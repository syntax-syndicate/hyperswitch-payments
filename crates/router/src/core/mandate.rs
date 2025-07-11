pub mod helpers;
pub mod utils;
use api_models::payments;
use common_types::payments as common_payments_types;
use common_utils::{ext_traits::Encode, id_type};
use diesel_models::enums as storage_enums;
use error_stack::{report, ResultExt};
use futures::future;
use router_env::{instrument, logger, tracing};

use super::payments::helpers as payment_helper;
use crate::{
    core::{
        errors::{self, RouterResponse, StorageErrorExt},
        payments::CallConnectorAction,
    },
    db::StorageInterface,
    routes::{metrics, SessionState},
    services,
    types::{
        self,
        api::{
            mandates::{self, MandateResponseExt},
            ConnectorData, GetToken,
        },
        domain,
        storage::{self, enums::MerchantStorageScheme},
        transformers::ForeignFrom,
    },
    utils::OptionExt,
};

#[instrument(skip(state))]
pub async fn get_mandate(
    state: SessionState,
    merchant_context: domain::MerchantContext,
    req: mandates::MandateId,
) -> RouterResponse<mandates::MandateResponse> {
    let mandate = state
        .store
        .as_ref()
        .find_mandate_by_merchant_id_mandate_id(
            merchant_context.get_merchant_account().get_id(),
            &req.mandate_id,
            merchant_context.get_merchant_account().storage_scheme,
        )
        .await
        .to_not_found_response(errors::ApiErrorResponse::MandateNotFound)?;
    Ok(services::ApplicationResponse::Json(
        mandates::MandateResponse::from_db_mandate(
            &state,
            merchant_context.get_merchant_key_store().clone(),
            mandate,
            merchant_context.get_merchant_account(),
        )
        .await?,
    ))
}

#[cfg(feature = "v1")]
#[instrument(skip(state))]
pub async fn revoke_mandate(
    state: SessionState,
    merchant_context: domain::MerchantContext,
    req: mandates::MandateId,
) -> RouterResponse<mandates::MandateRevokedResponse> {
    let db = state.store.as_ref();
    let mandate = db
        .find_mandate_by_merchant_id_mandate_id(
            merchant_context.get_merchant_account().get_id(),
            &req.mandate_id,
            merchant_context.get_merchant_account().storage_scheme,
        )
        .await
        .to_not_found_response(errors::ApiErrorResponse::MandateNotFound)?;
    match mandate.mandate_status {
        common_enums::MandateStatus::Active
        | common_enums::MandateStatus::Inactive
        | common_enums::MandateStatus::Pending => {
            let profile_id =
                helpers::get_profile_id_for_mandate(&state, &merchant_context, mandate.clone())
                    .await?;

            let merchant_connector_account = payment_helper::get_merchant_connector_account(
                &state,
                merchant_context.get_merchant_account().get_id(),
                None,
                merchant_context.get_merchant_key_store(),
                &profile_id,
                &mandate.connector.clone(),
                mandate.merchant_connector_id.as_ref(),
            )
            .await?;

            let connector_data = ConnectorData::get_connector_by_name(
                &state.conf.connectors,
                &mandate.connector,
                GetToken::Connector,
                mandate.merchant_connector_id.clone(),
            )?;
            let connector_integration: services::BoxedMandateRevokeConnectorIntegrationInterface<
                types::api::MandateRevoke,
                types::MandateRevokeRequestData,
                types::MandateRevokeResponseData,
            > = connector_data.connector.get_connector_integration();

            let router_data = utils::construct_mandate_revoke_router_data(
                &state,
                merchant_connector_account,
                &merchant_context,
                mandate.clone(),
            )
            .await?;

            let response = services::execute_connector_processing_step(
                &state,
                connector_integration,
                &router_data,
                CallConnectorAction::Trigger,
                None,
                None,
            )
            .await
            .change_context(errors::ApiErrorResponse::InternalServerError)?;

            match response.response {
                Ok(_) => {
                    let update_mandate = db
                        .update_mandate_by_merchant_id_mandate_id(
                            merchant_context.get_merchant_account().get_id(),
                            &req.mandate_id,
                            storage::MandateUpdate::StatusUpdate {
                                mandate_status: storage::enums::MandateStatus::Revoked,
                            },
                            mandate,
                            merchant_context.get_merchant_account().storage_scheme,
                        )
                        .await
                        .to_not_found_response(errors::ApiErrorResponse::MandateNotFound)?;
                    Ok(services::ApplicationResponse::Json(
                        mandates::MandateRevokedResponse {
                            mandate_id: update_mandate.mandate_id,
                            status: update_mandate.mandate_status,
                            error_code: None,
                            error_message: None,
                        },
                    ))
                }

                Err(err) => Err(errors::ApiErrorResponse::ExternalConnectorError {
                    code: err.code,
                    message: err.message,
                    connector: mandate.connector,
                    status_code: err.status_code,
                    reason: err.reason,
                }
                .into()),
            }
        }
        common_enums::MandateStatus::Revoked => {
            Err(errors::ApiErrorResponse::MandateValidationFailed {
                reason: "Mandate has already been revoked".to_string(),
            }
            .into())
        }
    }
}

#[instrument(skip(db))]
pub async fn update_connector_mandate_id(
    db: &dyn StorageInterface,
    merchant_id: &id_type::MerchantId,
    mandate_ids_opt: Option<String>,
    payment_method_id: Option<String>,
    resp: Result<types::PaymentsResponseData, types::ErrorResponse>,
    storage_scheme: MerchantStorageScheme,
) -> RouterResponse<mandates::MandateResponse> {
    let mandate_details = Option::foreign_from(resp);
    let connector_mandate_id = mandate_details
        .clone()
        .map(|md| {
            md.encode_to_value()
                .change_context(errors::ApiErrorResponse::InternalServerError)
                .map(masking::Secret::new)
        })
        .transpose()?;

    //Ignore updation if the payment_attempt mandate_id or connector_mandate_id is not present
    if let Some((mandate_id, connector_id)) = mandate_ids_opt.zip(connector_mandate_id) {
        let mandate = db
            .find_mandate_by_merchant_id_mandate_id(merchant_id, &mandate_id, storage_scheme)
            .await
            .change_context(errors::ApiErrorResponse::MandateNotFound)?;

        let update_mandate_details = match payment_method_id {
            Some(pmd_id) => storage::MandateUpdate::ConnectorMandateIdUpdate {
                connector_mandate_id: mandate_details
                    .and_then(|mandate_reference| mandate_reference.connector_mandate_id),
                connector_mandate_ids: Some(connector_id),
                payment_method_id: pmd_id,
                original_payment_id: None,
            },
            None => storage::MandateUpdate::ConnectorReferenceUpdate {
                connector_mandate_ids: Some(connector_id),
            },
        };

        // only update the connector_mandate_id if existing is none
        if mandate.connector_mandate_id.is_none() {
            db.update_mandate_by_merchant_id_mandate_id(
                merchant_id,
                &mandate_id,
                update_mandate_details,
                mandate,
                storage_scheme,
            )
            .await
            .change_context(errors::ApiErrorResponse::MandateUpdateFailed)?;
        }
    }
    Ok(services::ApplicationResponse::StatusOk)
}
#[cfg(feature = "v1")]
#[instrument(skip(state))]
pub async fn get_customer_mandates(
    state: SessionState,
    merchant_context: domain::MerchantContext,
    customer_id: id_type::CustomerId,
) -> RouterResponse<Vec<mandates::MandateResponse>> {
    let mandates = state
        .store
        .find_mandate_by_merchant_id_customer_id(
            merchant_context.get_merchant_account().get_id(),
            &customer_id,
        )
        .await
        .change_context(errors::ApiErrorResponse::InternalServerError)
        .attach_printable_lazy(|| {
            format!(
                "Failed while finding mandate: merchant_id: {:?}, customer_id: {:?}",
                merchant_context.get_merchant_account().get_id(),
                customer_id,
            )
        })?;

    if mandates.is_empty() {
        Err(report!(errors::ApiErrorResponse::MandateNotFound).attach_printable("No Mandate found"))
    } else {
        let mut response_vec = Vec::with_capacity(mandates.len());
        for mandate in mandates {
            response_vec.push(
                mandates::MandateResponse::from_db_mandate(
                    &state,
                    merchant_context.get_merchant_key_store().clone(),
                    mandate,
                    merchant_context.get_merchant_account(),
                )
                .await?,
            );
        }
        Ok(services::ApplicationResponse::Json(response_vec))
    }
}

fn get_insensitive_payment_method_data_if_exists<F, FData>(
    router_data: &types::RouterData<F, FData, types::PaymentsResponseData>,
) -> Option<domain::PaymentMethodData>
where
    FData: MandateBehaviour,
{
    match &router_data.request.get_payment_method_data() {
        domain::PaymentMethodData::Card(_) => None,
        _ => Some(router_data.request.get_payment_method_data()),
    }
}

pub async fn mandate_procedure<F, FData>(
    state: &SessionState,
    resp: &types::RouterData<F, FData, types::PaymentsResponseData>,
    customer_id: &Option<id_type::CustomerId>,
    pm_id: Option<String>,
    merchant_connector_id: Option<id_type::MerchantConnectorAccountId>,
    storage_scheme: MerchantStorageScheme,
    payment_id: &id_type::PaymentId,
) -> errors::RouterResult<Option<String>>
where
    FData: MandateBehaviour,
{
    let Ok(ref response) = resp.response else {
        return Ok(None);
    };

    match resp.request.get_mandate_id() {
        Some(mandate_id) => {
            let Some(ref mandate_id) = mandate_id.mandate_id else {
                return Ok(None);
            };
            let orig_mandate = state
                .store
                .find_mandate_by_merchant_id_mandate_id(
                    &resp.merchant_id,
                    mandate_id,
                    storage_scheme,
                )
                .await
                .to_not_found_response(errors::ApiErrorResponse::MandateNotFound)?;
            let mandate = match orig_mandate.mandate_type {
                storage_enums::MandateType::SingleUse => state
                    .store
                    .update_mandate_by_merchant_id_mandate_id(
                        &resp.merchant_id,
                        mandate_id,
                        storage::MandateUpdate::StatusUpdate {
                            mandate_status: storage_enums::MandateStatus::Revoked,
                        },
                        orig_mandate,
                        storage_scheme,
                    )
                    .await
                    .change_context(errors::ApiErrorResponse::MandateUpdateFailed),
                storage_enums::MandateType::MultiUse => state
                    .store
                    .update_mandate_by_merchant_id_mandate_id(
                        &resp.merchant_id,
                        mandate_id,
                        storage::MandateUpdate::CaptureAmountUpdate {
                            amount_captured: Some(
                                orig_mandate.amount_captured.unwrap_or(0)
                                    + resp.request.get_amount(),
                            ),
                        },
                        orig_mandate,
                        storage_scheme,
                    )
                    .await
                    .change_context(errors::ApiErrorResponse::MandateUpdateFailed),
            }?;
            metrics::SUBSEQUENT_MANDATE_PAYMENT.add(
                1,
                router_env::metric_attributes!(("connector", mandate.connector)),
            );
            Ok(Some(mandate_id.clone()))
        }
        None => {
            let Some(_mandate_details) = resp.request.get_setup_mandate_details() else {
                return Ok(None);
            };
            let (mandate_reference, network_txn_id) = match &response {
                types::PaymentsResponseData::TransactionResponse {
                    mandate_reference,
                    network_txn_id,
                    ..
                } => (mandate_reference.clone(), network_txn_id.clone()),
                _ => (Box::new(None), None),
            };

            let mandate_ids = (*mandate_reference)
                .as_ref()
                .map(|md| {
                    md.encode_to_value()
                        .change_context(errors::ApiErrorResponse::MandateSerializationFailed)
                        .map(masking::Secret::new)
                })
                .transpose()?;

            let Some(new_mandate_data) = payment_helper::generate_mandate(
                resp.merchant_id.clone(),
                payment_id.to_owned(),
                resp.connector.clone(),
                resp.request.get_setup_mandate_details().cloned(),
                customer_id,
                pm_id.get_required_value("payment_method_id")?,
                mandate_ids,
                network_txn_id,
                get_insensitive_payment_method_data_if_exists(resp),
                *mandate_reference,
                merchant_connector_id,
            )?
            else {
                return Ok(None);
            };

            let connector = new_mandate_data.connector.clone();
            logger::debug!("{:?}", new_mandate_data);

            let res_mandate_id = new_mandate_data.mandate_id.clone();

            state
                .store
                .insert_mandate(new_mandate_data, storage_scheme)
                .await
                .to_duplicate_response(errors::ApiErrorResponse::DuplicateMandate)?;
            metrics::MANDATE_COUNT.add(1, router_env::metric_attributes!(("connector", connector)));
            Ok(Some(res_mandate_id))
        }
    }
}

#[instrument(skip(state))]
pub async fn retrieve_mandates_list(
    state: SessionState,
    merchant_context: domain::MerchantContext,
    constraints: api_models::mandates::MandateListConstraints,
) -> RouterResponse<Vec<api_models::mandates::MandateResponse>> {
    let mandates = state
        .store
        .as_ref()
        .find_mandates_by_merchant_id(
            merchant_context.get_merchant_account().get_id(),
            constraints,
        )
        .await
        .change_context(errors::ApiErrorResponse::InternalServerError)
        .attach_printable("Unable to retrieve mandates")?;
    let mandates_list = future::try_join_all(mandates.into_iter().map(|mandate| {
        mandates::MandateResponse::from_db_mandate(
            &state,
            merchant_context.get_merchant_key_store().clone(),
            mandate,
            merchant_context.get_merchant_account(),
        )
    }))
    .await?;
    Ok(services::ApplicationResponse::Json(mandates_list))
}

impl ForeignFrom<Result<types::PaymentsResponseData, types::ErrorResponse>>
    for Option<types::MandateReference>
{
    fn foreign_from(resp: Result<types::PaymentsResponseData, types::ErrorResponse>) -> Self {
        match resp {
            Ok(types::PaymentsResponseData::TransactionResponse {
                mandate_reference, ..
            }) => *mandate_reference,
            _ => None,
        }
    }
}

pub trait MandateBehaviour {
    fn get_amount(&self) -> i64;
    fn get_setup_future_usage(&self) -> Option<diesel_models::enums::FutureUsage>;
    fn get_mandate_id(&self) -> Option<&payments::MandateIds>;
    fn set_mandate_id(&mut self, new_mandate_id: Option<payments::MandateIds>);
    fn get_payment_method_data(&self) -> domain::payments::PaymentMethodData;
    fn get_setup_mandate_details(
        &self,
    ) -> Option<&hyperswitch_domain_models::mandates::MandateData>;
    fn get_customer_acceptance(&self) -> Option<common_payments_types::CustomerAcceptance>;
}
