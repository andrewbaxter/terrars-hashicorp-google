use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LoggingBillingAccountBucketConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    billing_account: PrimField<String>,
    bucket_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmek_settings: Option<Vec<LoggingBillingAccountBucketConfigCmekSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_configs: Option<Vec<LoggingBillingAccountBucketConfigIndexConfigsEl>>,
    dynamic: LoggingBillingAccountBucketConfigDynamic,
}

struct LoggingBillingAccountBucketConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingBillingAccountBucketConfigData>,
}

#[derive(Clone)]
pub struct LoggingBillingAccountBucketConfig(Rc<LoggingBillingAccountBucketConfig_>);

impl LoggingBillingAccountBucketConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description for this bucket."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_days`.\nLogs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used."]
    pub fn set_retention_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_days = Some(v.into());
        self
    }

    #[doc= "Set the field `cmek_settings`.\n"]
    pub fn set_cmek_settings(
        self,
        v: impl Into<BlockAssignable<LoggingBillingAccountBucketConfigCmekSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cmek_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cmek_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `index_configs`.\n"]
    pub fn set_index_configs(
        self,
        v: impl Into<BlockAssignable<LoggingBillingAccountBucketConfigIndexConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_configs = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `billing_account` after provisioning.\nThe parent resource that contains the logging bucket."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe name of the logging bucket. Logging automatically creates two log buckets: _Required and _Default."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description for this bucket."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_state` after provisioning.\nThe bucket's lifecycle such as active or deleted."]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the bucket."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the bucket"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_days` after provisioning.\nLogs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used."]
    pub fn retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmek_settings` after provisioning.\n"]
    pub fn cmek_settings(&self) -> ListRef<LoggingBillingAccountBucketConfigCmekSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cmek_settings", self.extract_ref()))
    }
}

impl Referable for LoggingBillingAccountBucketConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LoggingBillingAccountBucketConfig { }

impl ToListMappable for LoggingBillingAccountBucketConfig {
    type O = ListRef<LoggingBillingAccountBucketConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LoggingBillingAccountBucketConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_billing_account_bucket_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoggingBillingAccountBucketConfig {
    pub tf_id: String,
    #[doc= "The parent resource that contains the logging bucket."]
    pub billing_account: PrimField<String>,
    #[doc= "The name of the logging bucket. Logging automatically creates two log buckets: _Required and _Default."]
    pub bucket_id: PrimField<String>,
    #[doc= "The location of the bucket."]
    pub location: PrimField<String>,
}

impl BuildLoggingBillingAccountBucketConfig {
    pub fn build(self, stack: &mut Stack) -> LoggingBillingAccountBucketConfig {
        let out = LoggingBillingAccountBucketConfig(Rc::new(LoggingBillingAccountBucketConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingBillingAccountBucketConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_account: self.billing_account,
                bucket_id: self.bucket_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                retention_days: core::default::Default::default(),
                cmek_settings: core::default::Default::default(),
                index_configs: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoggingBillingAccountBucketConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingBillingAccountBucketConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoggingBillingAccountBucketConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `billing_account` after provisioning.\nThe parent resource that contains the logging bucket."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe name of the logging bucket. Logging automatically creates two log buckets: _Required and _Default."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description for this bucket."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_state` after provisioning.\nThe bucket's lifecycle such as active or deleted."]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the bucket."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the bucket"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_days` after provisioning.\nLogs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used."]
    pub fn retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmek_settings` after provisioning.\n"]
    pub fn cmek_settings(&self) -> ListRef<LoggingBillingAccountBucketConfigCmekSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cmek_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoggingBillingAccountBucketConfigCmekSettingsEl {
    kms_key_name: PrimField<String>,
}

impl LoggingBillingAccountBucketConfigCmekSettingsEl { }

impl ToListMappable for LoggingBillingAccountBucketConfigCmekSettingsEl {
    type O = BlockAssignable<LoggingBillingAccountBucketConfigCmekSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingBillingAccountBucketConfigCmekSettingsEl {
    #[doc= "The resource name for the configured Cloud KMS key.\nKMS key name format:\n\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]\"\nTo enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.\nThe Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.\nSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub kms_key_name: PrimField<String>,
}

impl BuildLoggingBillingAccountBucketConfigCmekSettingsEl {
    pub fn build(self) -> LoggingBillingAccountBucketConfigCmekSettingsEl {
        LoggingBillingAccountBucketConfigCmekSettingsEl { kms_key_name: self.kms_key_name }
    }
}

pub struct LoggingBillingAccountBucketConfigCmekSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingBillingAccountBucketConfigCmekSettingsElRef {
    fn new(shared: StackShared, base: String) -> LoggingBillingAccountBucketConfigCmekSettingsElRef {
        LoggingBillingAccountBucketConfigCmekSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingBillingAccountBucketConfigCmekSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name for the configured Cloud KMS key.\nKMS key name format:\n\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]\"\nTo enable CMEK for the bucket, set this field to a valid kmsKeyName for which the associated service account has the required cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.\nThe Cloud KMS key used by the bucket can be updated by changing the kmsKeyName to a new valid key name. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.\nSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_version_name` after provisioning.\nThe CryptoKeyVersion resource name for the configured Cloud KMS key.\nKMS key name format:\n\"projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]\"\nFor example:\n\"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1\"\nThis is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version."]
    pub fn kms_key_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_version_name", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the CMEK settings."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nThe service account associated with a project for which CMEK will apply.\nBefore enabling CMEK for a logging bucket, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account associated with the project for which CMEK will apply. Use [v2.getCmekSettings](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getCmekSettings#google.logging.v2.ConfigServiceV2.GetCmekSettings) to obtain the service account ID.\nSee [Enabling CMEK for Logging Buckets](https://cloud.google.com/logging/docs/routing/managed-encryption-storage) for more information."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingBillingAccountBucketConfigIndexConfigsEl {
    field_path: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl LoggingBillingAccountBucketConfigIndexConfigsEl { }

impl ToListMappable for LoggingBillingAccountBucketConfigIndexConfigsEl {
    type O = BlockAssignable<LoggingBillingAccountBucketConfigIndexConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingBillingAccountBucketConfigIndexConfigsEl {
    #[doc= "The LogEntry field path to index."]
    pub field_path: PrimField<String>,
    #[doc= "The type of data in this index\nNote that some paths are automatically indexed, and other paths are not eligible for indexing. See [indexing documentation]( https://cloud.google.com/logging/docs/view/advanced-queries#indexed-fields) for details.\nFor example: jsonPayload.request.status"]
    pub type_: PrimField<String>,
}

impl BuildLoggingBillingAccountBucketConfigIndexConfigsEl {
    pub fn build(self) -> LoggingBillingAccountBucketConfigIndexConfigsEl {
        LoggingBillingAccountBucketConfigIndexConfigsEl {
            field_path: self.field_path,
            type_: self.type_,
        }
    }
}

pub struct LoggingBillingAccountBucketConfigIndexConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoggingBillingAccountBucketConfigIndexConfigsElRef {
    fn new(shared: StackShared, base: String) -> LoggingBillingAccountBucketConfigIndexConfigsElRef {
        LoggingBillingAccountBucketConfigIndexConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingBillingAccountBucketConfigIndexConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `field_path` after provisioning.\nThe LogEntry field path to index."]
    pub fn field_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of data in this index\nNote that some paths are automatically indexed, and other paths are not eligible for indexing. See [indexing documentation]( https://cloud.google.com/logging/docs/view/advanced-queries#indexed-fields) for details.\nFor example: jsonPayload.request.status"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoggingBillingAccountBucketConfigDynamic {
    cmek_settings: Option<DynamicBlock<LoggingBillingAccountBucketConfigCmekSettingsEl>>,
    index_configs: Option<DynamicBlock<LoggingBillingAccountBucketConfigIndexConfigsEl>>,
}
