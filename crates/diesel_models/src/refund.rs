use common_utils::{
    pii,
    types::{ChargeRefunds, ConnectorTransactionId, ConnectorTransactionIdTrait, MinorUnit},
};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::enums as storage_enums;
#[cfg(feature = "v1")]
use crate::schema::refund;
#[cfg(feature = "v2")]
use crate::schema_v2::refund;
#[cfg(feature = "v1")]
#[derive(
    Clone,
    Debug,
    Eq,
    Identifiable,
    Queryable,
    Selectable,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(table_name = refund, primary_key(refund_id), check_for_backend(diesel::pg::Pg))]
pub struct Refund {
    pub internal_reference_id: String,
    pub refund_id: String, //merchant_reference id
    pub payment_id: common_utils::id_type::PaymentId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub connector_transaction_id: ConnectorTransactionId,
    pub connector: String,
    pub connector_refund_id: Option<ConnectorTransactionId>,
    pub external_reference_id: Option<String>,
    pub refund_type: storage_enums::RefundType,
    pub total_amount: MinorUnit,
    pub currency: storage_enums::Currency,
    pub refund_amount: MinorUnit,
    pub refund_status: storage_enums::RefundStatus,
    pub sent_to_gateway: bool,
    pub refund_error_message: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub refund_arn: Option<String>,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub modified_at: PrimitiveDateTime,
    pub description: Option<String>,
    pub attempt_id: String,
    pub refund_reason: Option<String>,
    pub refund_error_code: Option<String>,
    pub profile_id: Option<common_utils::id_type::ProfileId>,
    pub updated_by: String,
    pub merchant_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub charges: Option<ChargeRefunds>,
    pub organization_id: common_utils::id_type::OrganizationId,
    /// INFO: This field is deprecated and replaced by processor_refund_data
    pub connector_refund_data: Option<String>,
    /// INFO: This field is deprecated and replaced by processor_transaction_data
    pub connector_transaction_data: Option<String>,
    pub split_refunds: Option<common_types::refunds::SplitRefund>,
    pub unified_code: Option<String>,
    pub unified_message: Option<String>,
    pub processor_refund_data: Option<String>,
    pub processor_transaction_data: Option<String>,
    pub issuer_error_code: Option<String>,
    pub issuer_error_message: Option<String>,
}

#[cfg(feature = "v2")]
#[derive(
    Clone,
    Debug,
    Eq,
    Identifiable,
    Queryable,
    Selectable,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(table_name = refund, primary_key(id), check_for_backend(diesel::pg::Pg))]
pub struct Refund {
    pub payment_id: common_utils::id_type::GlobalPaymentId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub connector_transaction_id: ConnectorTransactionId,
    pub connector: String,
    pub connector_refund_id: Option<ConnectorTransactionId>,
    pub external_reference_id: Option<String>,
    pub refund_type: storage_enums::RefundType,
    pub total_amount: MinorUnit,
    pub currency: storage_enums::Currency,
    pub refund_amount: MinorUnit,
    pub refund_status: storage_enums::RefundStatus,
    pub sent_to_gateway: bool,
    pub refund_error_message: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub refund_arn: Option<String>,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub modified_at: PrimitiveDateTime,
    pub description: Option<String>,
    pub attempt_id: common_utils::id_type::GlobalAttemptId,
    pub refund_reason: Option<String>,
    pub refund_error_code: Option<String>,
    pub profile_id: Option<common_utils::id_type::ProfileId>,
    pub updated_by: String,
    pub charges: Option<ChargeRefunds>,
    pub organization_id: common_utils::id_type::OrganizationId,
    pub split_refunds: Option<common_types::refunds::SplitRefund>,
    pub unified_code: Option<String>,
    pub unified_message: Option<String>,
    pub processor_refund_data: Option<String>,
    pub processor_transaction_data: Option<String>,
    pub id: common_utils::id_type::GlobalRefundId,
    pub merchant_reference_id: common_utils::id_type::RefundReferenceId,
    pub connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
}

#[cfg(feature = "v1")]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Insertable,
    router_derive::DebugAsDisplay,
    serde::Serialize,
    serde::Deserialize,
    router_derive::Setter,
)]
#[diesel(table_name = refund)]
pub struct RefundNew {
    pub refund_id: String,
    pub payment_id: common_utils::id_type::PaymentId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub internal_reference_id: String,
    pub external_reference_id: Option<String>,
    pub connector_transaction_id: ConnectorTransactionId,
    pub connector: String,
    pub connector_refund_id: Option<ConnectorTransactionId>,
    pub refund_type: storage_enums::RefundType,
    pub total_amount: MinorUnit,
    pub currency: storage_enums::Currency,
    pub refund_amount: MinorUnit,
    pub refund_status: storage_enums::RefundStatus,
    pub sent_to_gateway: bool,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub refund_arn: Option<String>,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub modified_at: PrimitiveDateTime,
    pub description: Option<String>,
    pub attempt_id: String,
    pub refund_reason: Option<String>,
    pub profile_id: Option<common_utils::id_type::ProfileId>,
    pub updated_by: String,
    pub merchant_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub charges: Option<ChargeRefunds>,
    pub organization_id: common_utils::id_type::OrganizationId,
    pub split_refunds: Option<common_types::refunds::SplitRefund>,
    pub processor_refund_data: Option<String>,
    pub processor_transaction_data: Option<String>,
}

#[cfg(feature = "v2")]
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Insertable,
    router_derive::DebugAsDisplay,
    serde::Serialize,
    serde::Deserialize,
    router_derive::Setter,
)]
#[diesel(table_name = refund)]
pub struct RefundNew {
    pub merchant_reference_id: common_utils::id_type::RefundReferenceId,
    pub payment_id: common_utils::id_type::GlobalPaymentId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub id: common_utils::id_type::GlobalRefundId,
    pub external_reference_id: Option<String>,
    pub connector_transaction_id: ConnectorTransactionId,
    pub connector: String,
    pub connector_refund_id: Option<ConnectorTransactionId>,
    pub refund_type: storage_enums::RefundType,
    pub total_amount: MinorUnit,
    pub currency: storage_enums::Currency,
    pub refund_amount: MinorUnit,
    pub refund_status: storage_enums::RefundStatus,
    pub sent_to_gateway: bool,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub refund_arn: Option<String>,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "common_utils::custom_serde::iso8601")]
    pub modified_at: PrimitiveDateTime,
    pub description: Option<String>,
    pub attempt_id: common_utils::id_type::GlobalAttemptId,
    pub refund_reason: Option<String>,
    pub profile_id: Option<common_utils::id_type::ProfileId>,
    pub updated_by: String,
    pub connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub charges: Option<ChargeRefunds>,
    pub organization_id: common_utils::id_type::OrganizationId,
    pub split_refunds: Option<common_types::refunds::SplitRefund>,
    pub processor_refund_data: Option<String>,
    pub processor_transaction_data: Option<String>,
}

#[cfg(feature = "v1")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RefundUpdate {
    Update {
        connector_refund_id: ConnectorTransactionId,
        refund_status: storage_enums::RefundStatus,
        sent_to_gateway: bool,
        refund_error_message: Option<String>,
        refund_arn: String,
        updated_by: String,
        processor_refund_data: Option<String>,
    },
    MetadataAndReasonUpdate {
        metadata: Option<pii::SecretSerdeValue>,
        reason: Option<String>,
        updated_by: String,
    },
    StatusUpdate {
        connector_refund_id: Option<ConnectorTransactionId>,
        sent_to_gateway: bool,
        refund_status: storage_enums::RefundStatus,
        updated_by: String,
        processor_refund_data: Option<String>,
    },
    ErrorUpdate {
        refund_status: Option<storage_enums::RefundStatus>,
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        updated_by: String,
        connector_refund_id: Option<ConnectorTransactionId>,
        processor_refund_data: Option<String>,
        unified_code: Option<String>,
        unified_message: Option<String>,
        issuer_error_code: Option<String>,
        issuer_error_message: Option<String>,
    },
    ManualUpdate {
        refund_status: Option<storage_enums::RefundStatus>,
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        updated_by: String,
    },
}

#[cfg(feature = "v2")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RefundUpdate {
    Update {
        connector_refund_id: ConnectorTransactionId,
        refund_status: storage_enums::RefundStatus,
        sent_to_gateway: bool,
        refund_error_message: Option<String>,
        refund_arn: String,
        updated_by: String,
        processor_refund_data: Option<String>,
    },
    MetadataAndReasonUpdate {
        metadata: Option<pii::SecretSerdeValue>,
        reason: Option<String>,
        updated_by: String,
    },
    StatusUpdate {
        connector_refund_id: Option<ConnectorTransactionId>,
        sent_to_gateway: bool,
        refund_status: storage_enums::RefundStatus,
        updated_by: String,
        processor_refund_data: Option<String>,
    },
    ErrorUpdate {
        refund_status: Option<storage_enums::RefundStatus>,
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        updated_by: String,
        connector_refund_id: Option<ConnectorTransactionId>,
        processor_refund_data: Option<String>,
        unified_code: Option<String>,
        unified_message: Option<String>,
    },
    ManualUpdate {
        refund_status: Option<storage_enums::RefundStatus>,
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        updated_by: String,
    },
}

#[cfg(feature = "v1")]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = refund)]
pub struct RefundUpdateInternal {
    connector_refund_id: Option<ConnectorTransactionId>,
    refund_status: Option<storage_enums::RefundStatus>,
    sent_to_gateway: Option<bool>,
    refund_error_message: Option<String>,
    refund_arn: Option<String>,
    metadata: Option<pii::SecretSerdeValue>,
    refund_reason: Option<String>,
    refund_error_code: Option<String>,
    updated_by: String,
    modified_at: PrimitiveDateTime,
    processor_refund_data: Option<String>,
    unified_code: Option<String>,
    unified_message: Option<String>,
    issuer_error_code: Option<String>,
    issuer_error_message: Option<String>,
}

#[cfg(feature = "v2")]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = refund)]
pub struct RefundUpdateInternal {
    connector_refund_id: Option<ConnectorTransactionId>,
    refund_status: Option<storage_enums::RefundStatus>,
    sent_to_gateway: Option<bool>,
    refund_error_message: Option<String>,
    refund_arn: Option<String>,
    metadata: Option<pii::SecretSerdeValue>,
    refund_reason: Option<String>,
    refund_error_code: Option<String>,
    updated_by: String,
    modified_at: PrimitiveDateTime,
    processor_refund_data: Option<String>,
    unified_code: Option<String>,
    unified_message: Option<String>,
}

#[cfg(feature = "v1")]
impl RefundUpdateInternal {
    pub fn create_refund(self, source: Refund) -> Refund {
        Refund {
            connector_refund_id: self.connector_refund_id,
            refund_status: self.refund_status.unwrap_or_default(),
            sent_to_gateway: self.sent_to_gateway.unwrap_or_default(),
            refund_error_message: self.refund_error_message,
            refund_arn: self.refund_arn,
            metadata: self.metadata,
            refund_reason: self.refund_reason,
            refund_error_code: self.refund_error_code,
            updated_by: self.updated_by,
            modified_at: self.modified_at,
            processor_refund_data: self.processor_refund_data,
            unified_code: self.unified_code,
            unified_message: self.unified_message,
            ..source
        }
    }
}

#[cfg(feature = "v2")]
impl RefundUpdateInternal {
    pub fn create_refund(self, source: Refund) -> Refund {
        Refund {
            connector_refund_id: self.connector_refund_id,
            refund_status: self.refund_status.unwrap_or_default(),
            sent_to_gateway: self.sent_to_gateway.unwrap_or_default(),
            refund_error_message: self.refund_error_message,
            refund_arn: self.refund_arn,
            metadata: self.metadata,
            refund_reason: self.refund_reason,
            refund_error_code: self.refund_error_code,
            updated_by: self.updated_by,
            modified_at: self.modified_at,
            processor_refund_data: self.processor_refund_data,
            unified_code: self.unified_code,
            unified_message: self.unified_message,
            ..source
        }
    }
}

#[cfg(feature = "v1")]
impl From<RefundUpdate> for RefundUpdateInternal {
    fn from(refund_update: RefundUpdate) -> Self {
        match refund_update {
            RefundUpdate::Update {
                connector_refund_id,
                refund_status,
                sent_to_gateway,
                refund_error_message,
                refund_arn,
                updated_by,
                processor_refund_data,
            } => Self {
                connector_refund_id: Some(connector_refund_id),
                refund_status: Some(refund_status),
                sent_to_gateway: Some(sent_to_gateway),
                refund_error_message,
                refund_arn: Some(refund_arn),
                updated_by,
                processor_refund_data,
                metadata: None,
                refund_reason: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                unified_code: None,
                unified_message: None,
                issuer_error_code: None,
                issuer_error_message: None,
            },
            RefundUpdate::MetadataAndReasonUpdate {
                metadata,
                reason,
                updated_by,
            } => Self {
                metadata,
                refund_reason: reason,
                updated_by,
                connector_refund_id: None,
                refund_status: None,
                sent_to_gateway: None,
                refund_error_message: None,
                refund_arn: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                processor_refund_data: None,
                unified_code: None,
                unified_message: None,
                issuer_error_code: None,
                issuer_error_message: None,
            },
            RefundUpdate::StatusUpdate {
                connector_refund_id,
                sent_to_gateway,
                refund_status,
                updated_by,
                processor_refund_data,
            } => Self {
                connector_refund_id,
                sent_to_gateway: Some(sent_to_gateway),
                refund_status: Some(refund_status),
                updated_by,
                processor_refund_data,
                refund_error_message: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                unified_code: None,
                unified_message: None,
                issuer_error_code: None,
                issuer_error_message: None,
            },
            RefundUpdate::ErrorUpdate {
                refund_status,
                refund_error_message,
                refund_error_code,
                unified_code,
                unified_message,
                updated_by,
                connector_refund_id,
                processor_refund_data,
                issuer_error_code,
                issuer_error_message,
            } => Self {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
                connector_refund_id,
                processor_refund_data,
                sent_to_gateway: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                modified_at: common_utils::date_time::now(),
                unified_code,
                unified_message,
                issuer_error_code,
                issuer_error_message,
            },
            RefundUpdate::ManualUpdate {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
            } => Self {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
                connector_refund_id: None,
                sent_to_gateway: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                modified_at: common_utils::date_time::now(),
                processor_refund_data: None,
                unified_code: None,
                unified_message: None,
                issuer_error_code: None,
                issuer_error_message: None,
            },
        }
    }
}

#[cfg(feature = "v2")]
impl From<RefundUpdate> for RefundUpdateInternal {
    fn from(refund_update: RefundUpdate) -> Self {
        match refund_update {
            RefundUpdate::Update {
                connector_refund_id,
                refund_status,
                sent_to_gateway,
                refund_error_message,
                refund_arn,
                updated_by,
                processor_refund_data,
            } => Self {
                connector_refund_id: Some(connector_refund_id),
                refund_status: Some(refund_status),
                sent_to_gateway: Some(sent_to_gateway),
                refund_error_message,
                refund_arn: Some(refund_arn),
                updated_by,
                processor_refund_data,
                metadata: None,
                refund_reason: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                unified_code: None,
                unified_message: None,
            },
            RefundUpdate::MetadataAndReasonUpdate {
                metadata,
                reason,
                updated_by,
            } => Self {
                metadata,
                refund_reason: reason,
                updated_by,
                connector_refund_id: None,
                refund_status: None,
                sent_to_gateway: None,
                refund_error_message: None,
                refund_arn: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                processor_refund_data: None,
                unified_code: None,
                unified_message: None,
            },
            RefundUpdate::StatusUpdate {
                connector_refund_id,
                sent_to_gateway,
                refund_status,
                updated_by,
                processor_refund_data,
            } => Self {
                connector_refund_id,
                sent_to_gateway: Some(sent_to_gateway),
                refund_status: Some(refund_status),
                updated_by,
                processor_refund_data,
                refund_error_message: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                refund_error_code: None,
                modified_at: common_utils::date_time::now(),
                unified_code: None,
                unified_message: None,
            },
            RefundUpdate::ErrorUpdate {
                refund_status,
                refund_error_message,
                refund_error_code,
                unified_code,
                unified_message,
                updated_by,
                connector_refund_id,
                processor_refund_data,
            } => Self {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
                connector_refund_id,
                processor_refund_data,
                sent_to_gateway: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                modified_at: common_utils::date_time::now(),
                unified_code,
                unified_message,
            },
            RefundUpdate::ManualUpdate {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
            } => Self {
                refund_status,
                refund_error_message,
                refund_error_code,
                updated_by,
                connector_refund_id: None,
                sent_to_gateway: None,
                refund_arn: None,
                metadata: None,
                refund_reason: None,
                modified_at: common_utils::date_time::now(),
                processor_refund_data: None,
                unified_code: None,
                unified_message: None,
            },
        }
    }
}

#[cfg(feature = "v1")]
impl RefundUpdate {
    pub fn apply_changeset(self, source: Refund) -> Refund {
        let RefundUpdateInternal {
            connector_refund_id,
            refund_status,
            sent_to_gateway,
            refund_error_message,
            refund_arn,
            metadata,
            refund_reason,
            refund_error_code,
            updated_by,
            modified_at: _,
            processor_refund_data,
            unified_code,
            unified_message,
            issuer_error_code,
            issuer_error_message,
        } = self.into();
        Refund {
            connector_refund_id: connector_refund_id.or(source.connector_refund_id),
            refund_status: refund_status.unwrap_or(source.refund_status),
            sent_to_gateway: sent_to_gateway.unwrap_or(source.sent_to_gateway),
            refund_error_message: refund_error_message.or(source.refund_error_message),
            refund_error_code: refund_error_code.or(source.refund_error_code),
            refund_arn: refund_arn.or(source.refund_arn),
            metadata: metadata.or(source.metadata),
            refund_reason: refund_reason.or(source.refund_reason),
            updated_by,
            modified_at: common_utils::date_time::now(),
            processor_refund_data: processor_refund_data.or(source.processor_refund_data),
            unified_code: unified_code.or(source.unified_code),
            unified_message: unified_message.or(source.unified_message),
            issuer_error_code: issuer_error_code.or(source.issuer_error_code),
            issuer_error_message: issuer_error_message.or(source.issuer_error_message),
            ..source
        }
    }
}

#[cfg(feature = "v2")]
impl RefundUpdate {
    pub fn apply_changeset(self, source: Refund) -> Refund {
        let RefundUpdateInternal {
            connector_refund_id,
            refund_status,
            sent_to_gateway,
            refund_error_message,
            refund_arn,
            metadata,
            refund_reason,
            refund_error_code,
            updated_by,
            modified_at: _,
            processor_refund_data,
            unified_code,
            unified_message,
        } = self.into();
        Refund {
            connector_refund_id: connector_refund_id.or(source.connector_refund_id),
            refund_status: refund_status.unwrap_or(source.refund_status),
            sent_to_gateway: sent_to_gateway.unwrap_or(source.sent_to_gateway),
            refund_error_message: refund_error_message.or(source.refund_error_message),
            refund_error_code: refund_error_code.or(source.refund_error_code),
            refund_arn: refund_arn.or(source.refund_arn),
            metadata: metadata.or(source.metadata),
            refund_reason: refund_reason.or(source.refund_reason),
            updated_by,
            modified_at: common_utils::date_time::now(),
            processor_refund_data: processor_refund_data.or(source.processor_refund_data),
            unified_code: unified_code.or(source.unified_code),
            unified_message: unified_message.or(source.unified_message),
            ..source
        }
    }

    pub fn build_error_update_for_unified_error_and_message(
        unified_error_object: (String, String),
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        storage_scheme: &storage_enums::MerchantStorageScheme,
    ) -> Self {
        let (unified_code, unified_message) = unified_error_object;

        Self::ErrorUpdate {
            refund_status: Some(storage_enums::RefundStatus::Failure),
            refund_error_message,
            refund_error_code,
            updated_by: storage_scheme.to_string(),
            connector_refund_id: None,
            processor_refund_data: None,
            unified_code: Some(unified_code),
            unified_message: Some(unified_message),
        }
    }

    pub fn build_error_update_for_integrity_check_failure(
        integrity_check_failed_fields: String,
        connector_refund_id: Option<ConnectorTransactionId>,
        storage_scheme: &storage_enums::MerchantStorageScheme,
    ) -> Self {
        Self::ErrorUpdate {
            refund_status: Some(storage_enums::RefundStatus::ManualReview),
            refund_error_message: Some(format!(
                "Integrity Check Failed! as data mismatched for fields {}",
                integrity_check_failed_fields
            )),
            refund_error_code: Some("IE".to_string()),
            updated_by: storage_scheme.to_string(),
            connector_refund_id: connector_refund_id.clone(),
            processor_refund_data: connector_refund_id.and_then(|x| x.extract_hashed_data()),
            unified_code: None,
            unified_message: None,
        }
    }

    pub fn build_refund_update(
        connector_refund_id: ConnectorTransactionId,
        refund_status: storage_enums::RefundStatus,
        storage_scheme: &storage_enums::MerchantStorageScheme,
    ) -> Self {
        Self::Update {
            connector_refund_id: connector_refund_id.clone(),
            refund_status,
            sent_to_gateway: true,
            refund_error_message: None,
            refund_arn: "".to_string(),
            updated_by: storage_scheme.to_string(),
            processor_refund_data: connector_refund_id.extract_hashed_data(),
        }
    }

    pub fn build_error_update_for_refund_failure(
        refund_status: Option<storage_enums::RefundStatus>,
        refund_error_message: Option<String>,
        refund_error_code: Option<String>,
        storage_scheme: &storage_enums::MerchantStorageScheme,
    ) -> Self {
        Self::ErrorUpdate {
            refund_status,
            refund_error_message,
            refund_error_code,
            updated_by: storage_scheme.to_string(),
            connector_refund_id: None,
            processor_refund_data: None,
            unified_code: None,
            unified_message: None,
        }
    }
}

#[cfg(feature = "v1")]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RefundCoreWorkflow {
    pub refund_internal_reference_id: String,
    pub connector_transaction_id: ConnectorTransactionId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub payment_id: common_utils::id_type::PaymentId,
    pub processor_transaction_data: Option<String>,
}

#[cfg(feature = "v2")]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RefundCoreWorkflow {
    pub refund_id: common_utils::id_type::GlobalRefundId,
    pub connector_transaction_id: ConnectorTransactionId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub payment_id: common_utils::id_type::GlobalPaymentId,
    pub processor_transaction_data: Option<String>,
}

#[cfg(feature = "v1")]
impl common_utils::events::ApiEventMetric for Refund {
    fn get_api_event_type(&self) -> Option<common_utils::events::ApiEventsType> {
        Some(common_utils::events::ApiEventsType::Refund {
            payment_id: Some(self.payment_id.clone()),
            refund_id: self.refund_id.clone(),
        })
    }
}

#[cfg(feature = "v2")]
impl common_utils::events::ApiEventMetric for Refund {
    fn get_api_event_type(&self) -> Option<common_utils::events::ApiEventsType> {
        Some(common_utils::events::ApiEventsType::Refund {
            payment_id: Some(self.payment_id.clone()),
            refund_id: self.id.clone(),
        })
    }
}

impl ConnectorTransactionIdTrait for Refund {
    fn get_optional_connector_refund_id(&self) -> Option<&String> {
        match self
            .connector_refund_id
            .as_ref()
            .map(|refund_id| refund_id.get_txn_id(self.processor_refund_data.as_ref()))
            .transpose()
        {
            Ok(refund_id) => refund_id,

            // In case hashed data is missing from DB, use the hashed ID as connector transaction ID
            Err(_) => self
                .connector_refund_id
                .as_ref()
                .map(|txn_id| txn_id.get_id()),
        }
    }

    fn get_connector_transaction_id(&self) -> &String {
        match self
            .connector_transaction_id
            .get_txn_id(self.processor_transaction_data.as_ref())
        {
            Ok(txn_id) => txn_id,

            // In case hashed data is missing from DB, use the hashed ID as connector transaction ID
            Err(_) => self.connector_transaction_id.get_id(),
        }
    }
}

mod tests {
    #[test]
    fn test_backwards_compatibility() {
        let serialized_refund = r#"{
    "internal_reference_id": "internal_ref_123",
    "refund_id": "refund_456",
    "payment_id": "payment_789",
    "merchant_id": "merchant_123",
    "connector_transaction_id": "connector_txn_789",
    "connector": "stripe",
    "connector_refund_id": null,
    "external_reference_id": null,
    "refund_type": "instant_refund",
    "total_amount": 10000,
    "currency": "USD",
    "refund_amount": 9500,
    "refund_status": "Success",
    "sent_to_gateway": true,
    "refund_error_message": null,
    "metadata": null,
    "refund_arn": null,
    "created_at": "2024-02-26T12:00:00Z",
    "updated_at": "2024-02-26T12:00:00Z",
    "description": null,
    "attempt_id": "attempt_123",
    "refund_reason": null,
    "refund_error_code": null,
    "profile_id": null,
    "updated_by": "admin",
    "merchant_connector_id": null,
    "charges": null,
    "connector_transaction_data": null
    "unified_code": null,
    "unified_message": null,
    "processor_transaction_data": null,
}"#;
        let deserialized = serde_json::from_str::<super::Refund>(serialized_refund);

        assert!(deserialized.is_ok());
    }
}
