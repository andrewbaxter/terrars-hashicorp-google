use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageBucketData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct DataStorageBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageBucketData>,
}

#[derive(Clone)]
pub struct DataStorageBucket(std::rc::Rc<DataStorageBucket_>);

impl DataStorageBucket {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `autoclass` after provisioning.\nThe bucket's autoclass configuration."]
    pub fn autoclass(&self) -> terrars::ListRef<DataStorageBucketAutoclassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> terrars::ListRef<DataStorageBucketCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> terrars::ListRef<DataStorageBucketCustomPlacementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_event_based_hold` after provisioning.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn default_event_based_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_event_based_hold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_object_retention` after provisioning.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn enable_object_retention(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_object_retention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> terrars::ListRef<DataStorageBucketEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a bucket, this boolean option will delete all contained objects. If you try to delete a bucket that contains objects, Terraform will fail that run."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<DataStorageBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> terrars::ListRef<DataStorageBucketLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the bucket."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_access_prevention` after provisioning.\nPrevents public access to a bucket."]
    pub fn public_access_prevention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_access_prevention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requester_pays` after provisioning.\nEnables Requester Pays on a storage bucket."]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> terrars::ListRef<DataStorageBucketRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_bucket_level_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> terrars::ListRef<DataStorageBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> terrars::ListRef<DataStorageBucketWebsiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageBucket { }

impl terrars::ToListMappable for DataStorageBucket {
    type O = terrars::ListRef<DataStorageBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageBucket_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageBucket {
    pub tf_id: std::string::String,
    #[doc = "The name of the bucket."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataStorageBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageBucket {
        let out = DataStorageBucket(std::rc::Rc::new(DataStorageBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStorageBucketData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageBucketRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `autoclass` after provisioning.\nThe bucket's autoclass configuration."]
    pub fn autoclass(&self) -> terrars::ListRef<DataStorageBucketAutoclassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> terrars::ListRef<DataStorageBucketCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> terrars::ListRef<DataStorageBucketCustomPlacementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_event_based_hold` after provisioning.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn default_event_based_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_event_based_hold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_object_retention` after provisioning.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn enable_object_retention(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_object_retention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> terrars::ListRef<DataStorageBucketEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a bucket, this boolean option will delete all contained objects. If you try to delete a bucket that contains objects, Terraform will fail that run."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<DataStorageBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> terrars::ListRef<DataStorageBucketLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the bucket."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_access_prevention` after provisioning.\nPrevents public access to a bucket."]
    pub fn public_access_prevention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_access_prevention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requester_pays` after provisioning.\nEnables Requester Pays on a storage bucket."]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> terrars::ListRef<DataStorageBucketRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_bucket_level_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> terrars::ListRef<DataStorageBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> terrars::ListRef<DataStorageBucketWebsiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketAutoclassEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminal_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketAutoclassEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminal_storage_class`.\n"]
    pub fn set_terminal_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.terminal_storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketAutoclassEl {
    type O = terrars::BlockAssignable<DataStorageBucketAutoclassEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketAutoclassEl {}

impl BuildDataStorageBucketAutoclassEl {
    pub fn build(self) -> DataStorageBucketAutoclassEl {
        DataStorageBucketAutoclassEl {
            enabled: core::default::Default::default(),
            terminal_storage_class: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketAutoclassElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketAutoclassElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketAutoclassElRef {
        DataStorageBucketAutoclassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketAutoclassElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `terminal_storage_class` after provisioning.\n"]
    pub fn terminal_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminal_storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageBucketCorsEl {
    #[doc = "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_header`.\n"]
    pub fn set_response_header(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketCorsEl {
    type O = terrars::BlockAssignable<DataStorageBucketCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketCorsEl {}

impl BuildDataStorageBucketCorsEl {
    pub fn build(self) -> DataStorageBucketCorsEl {
        DataStorageBucketCorsEl {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketCorsElRef {
        DataStorageBucketCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header` after provisioning.\n"]
    pub fn response_header(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketCustomPlacementConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageBucketCustomPlacementConfigEl {
    #[doc = "Set the field `data_locations`.\n"]
    pub fn set_data_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketCustomPlacementConfigEl {
    type O = terrars::BlockAssignable<DataStorageBucketCustomPlacementConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketCustomPlacementConfigEl {}

impl BuildDataStorageBucketCustomPlacementConfigEl {
    pub fn build(self) -> DataStorageBucketCustomPlacementConfigEl {
        DataStorageBucketCustomPlacementConfigEl { data_locations: core::default::Default::default() }
    }
}

pub struct DataStorageBucketCustomPlacementConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketCustomPlacementConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketCustomPlacementConfigElRef {
        DataStorageBucketCustomPlacementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketCustomPlacementConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_locations` after provisioning.\n"]
    pub fn data_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketEncryptionEl {
    #[doc = "Set the field `default_kms_key_name`.\n"]
    pub fn set_default_kms_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketEncryptionEl {
    type O = terrars::BlockAssignable<DataStorageBucketEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketEncryptionEl {}

impl BuildDataStorageBucketEncryptionEl {
    pub fn build(self) -> DataStorageBucketEncryptionEl {
        DataStorageBucketEncryptionEl { default_kms_key_name: core::default::Default::default() }
    }
}

pub struct DataStorageBucketEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketEncryptionElRef {
        DataStorageBucketEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_kms_key_name` after provisioning.\n"]
    pub fn default_kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketLifecycleRuleElActionEl {
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketLifecycleRuleElActionEl {
    type O = terrars::BlockAssignable<DataStorageBucketLifecycleRuleElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketLifecycleRuleElActionEl {}

impl BuildDataStorageBucketLifecycleRuleElActionEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleElActionEl {
        DataStorageBucketLifecycleRuleElActionEl {
            storage_class: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketLifecycleRuleElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketLifecycleRuleElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketLifecycleRuleElActionElRef {
        DataStorageBucketLifecycleRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketLifecycleRuleElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleElConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_time_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_since_custom_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_since_noncurrent_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches_prefix: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches_storage_class: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches_suffix: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_age: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_time_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_newer_versions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketLifecycleRuleElConditionEl {
    #[doc = "Set the field `age`.\n"]
    pub fn set_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_before`.\n"]
    pub fn set_created_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_time_before`.\n"]
    pub fn set_custom_time_before(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_time_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_since_custom_time`.\n"]
    pub fn set_days_since_custom_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_since_custom_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_since_noncurrent_time`.\n"]
    pub fn set_days_since_noncurrent_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_since_noncurrent_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches_prefix`.\n"]
    pub fn set_matches_prefix(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches_storage_class`.\n"]
    pub fn set_matches_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches_suffix`.\n"]
    pub fn set_matches_suffix(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_age`.\n"]
    pub fn set_no_age(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `noncurrent_time_before`.\n"]
    pub fn set_noncurrent_time_before(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.noncurrent_time_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_newer_versions`.\n"]
    pub fn set_num_newer_versions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_newer_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_state`.\n"]
    pub fn set_with_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.with_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketLifecycleRuleElConditionEl {
    type O = terrars::BlockAssignable<DataStorageBucketLifecycleRuleElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketLifecycleRuleElConditionEl {}

impl BuildDataStorageBucketLifecycleRuleElConditionEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleElConditionEl {
        DataStorageBucketLifecycleRuleElConditionEl {
            age: core::default::Default::default(),
            created_before: core::default::Default::default(),
            custom_time_before: core::default::Default::default(),
            days_since_custom_time: core::default::Default::default(),
            days_since_noncurrent_time: core::default::Default::default(),
            matches_prefix: core::default::Default::default(),
            matches_storage_class: core::default::Default::default(),
            matches_suffix: core::default::Default::default(),
            no_age: core::default::Default::default(),
            noncurrent_time_before: core::default::Default::default(),
            num_newer_versions: core::default::Default::default(),
            with_state: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketLifecycleRuleElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketLifecycleRuleElConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucketLifecycleRuleElConditionElRef {
        DataStorageBucketLifecycleRuleElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketLifecycleRuleElConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `age` after provisioning.\n"]
    pub fn age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.age", self.base))
    }

    #[doc = "Get a reference to the value of field `created_before` after provisioning.\n"]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_time_before` after provisioning.\n"]
    pub fn custom_time_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_time_before", self.base))
    }

    #[doc = "Get a reference to the value of field `days_since_custom_time` after provisioning.\n"]
    pub fn days_since_custom_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_since_custom_time", self.base))
    }

    #[doc = "Get a reference to the value of field `days_since_noncurrent_time` after provisioning.\n"]
    pub fn days_since_noncurrent_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_since_noncurrent_time", self.base))
    }

    #[doc = "Get a reference to the value of field `matches_prefix` after provisioning.\n"]
    pub fn matches_prefix(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `matches_storage_class` after provisioning.\n"]
    pub fn matches_storage_class(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `matches_suffix` after provisioning.\n"]
    pub fn matches_suffix(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `no_age` after provisioning.\n"]
    pub fn no_age(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_age", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_time_before` after provisioning.\n"]
    pub fn noncurrent_time_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_time_before", self.base))
    }

    #[doc = "Get a reference to the value of field `num_newer_versions` after provisioning.\n"]
    pub fn num_newer_versions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_newer_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `with_state` after provisioning.\n"]
    pub fn with_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::SetField<DataStorageBucketLifecycleRuleElActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::SetField<DataStorageBucketLifecycleRuleElConditionEl>>,
}

impl DataStorageBucketLifecycleRuleEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataStorageBucketLifecycleRuleElActionEl>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataStorageBucketLifecycleRuleElConditionEl>>,
    ) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketLifecycleRuleEl {
    type O = terrars::BlockAssignable<DataStorageBucketLifecycleRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketLifecycleRuleEl {}

impl BuildDataStorageBucketLifecycleRuleEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleEl {
        DataStorageBucketLifecycleRuleEl {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketLifecycleRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketLifecycleRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketLifecycleRuleElRef {
        DataStorageBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketLifecycleRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::SetRef<DataStorageBucketLifecycleRuleElActionElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::SetRef<DataStorageBucketLifecycleRuleElConditionElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_object_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketLoggingEl {
    #[doc = "Set the field `log_bucket`.\n"]
    pub fn set_log_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_object_prefix`.\n"]
    pub fn set_log_object_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_object_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketLoggingEl {
    type O = terrars::BlockAssignable<DataStorageBucketLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketLoggingEl {}

impl BuildDataStorageBucketLoggingEl {
    pub fn build(self) -> DataStorageBucketLoggingEl {
        DataStorageBucketLoggingEl {
            log_bucket: core::default::Default::default(),
            log_object_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketLoggingElRef {
        DataStorageBucketLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_bucket` after provisioning.\n"]
    pub fn log_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `log_object_prefix` after provisioning.\n"]
    pub fn log_object_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_object_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketRetentionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<f64>>,
}

impl DataStorageBucketRetentionPolicyEl {
    #[doc = "Set the field `is_locked`.\n"]
    pub fn set_is_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketRetentionPolicyEl {
    type O = terrars::BlockAssignable<DataStorageBucketRetentionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketRetentionPolicyEl {}

impl BuildDataStorageBucketRetentionPolicyEl {
    pub fn build(self) -> DataStorageBucketRetentionPolicyEl {
        DataStorageBucketRetentionPolicyEl {
            is_locked: core::default::Default::default(),
            retention_period: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketRetentionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketRetentionPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketRetentionPolicyElRef {
        DataStorageBucketRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketRetentionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_locked` after provisioning.\n"]
    pub fn is_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_locked", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketVersioningEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataStorageBucketVersioningEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketVersioningEl {
    type O = terrars::BlockAssignable<DataStorageBucketVersioningEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketVersioningEl {}

impl BuildDataStorageBucketVersioningEl {
    pub fn build(self) -> DataStorageBucketVersioningEl {
        DataStorageBucketVersioningEl { enabled: core::default::Default::default() }
    }
}

pub struct DataStorageBucketVersioningElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketVersioningElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketVersioningElRef {
        DataStorageBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketVersioningElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketWebsiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_page_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_page: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucketWebsiteEl {
    #[doc = "Set the field `main_page_suffix`.\n"]
    pub fn set_main_page_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_page_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_found_page`.\n"]
    pub fn set_not_found_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_found_page = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucketWebsiteEl {
    type O = terrars::BlockAssignable<DataStorageBucketWebsiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketWebsiteEl {}

impl BuildDataStorageBucketWebsiteEl {
    pub fn build(self) -> DataStorageBucketWebsiteEl {
        DataStorageBucketWebsiteEl {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketWebsiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucketWebsiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucketWebsiteElRef {
        DataStorageBucketWebsiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketWebsiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `main_page_suffix` after provisioning.\n"]
    pub fn main_page_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_page_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `not_found_page` after provisioning.\n"]
    pub fn not_found_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_found_page", self.base))
    }
}
