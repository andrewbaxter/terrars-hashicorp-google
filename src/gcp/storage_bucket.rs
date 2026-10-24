use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageBucketData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_event_based_hold: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_object_retention: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access_prevention: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_pays: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uniform_bucket_level_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoclass: std::option::Option<std::vec::Vec<StorageBucketAutoclassEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<StorageBucketCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_placement_config: std::option::Option<std::vec::Vec<StorageBucketCustomPlacementConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<std::vec::Vec<StorageBucketEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_rule: std::option::Option<std::vec::Vec<StorageBucketLifecycleRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<StorageBucketLoggingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<StorageBucketRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageBucketTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioning: std::option::Option<std::vec::Vec<StorageBucketVersioningEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    website: std::option::Option<std::vec::Vec<StorageBucketWebsiteEl>>,
    dynamic: StorageBucketDynamic,
}

struct StorageBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageBucketData>,
}

#[derive(Clone)]
pub struct StorageBucket(std::rc::Rc<StorageBucket_>);

impl StorageBucket {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc =
        "Set the field `default_event_based_hold`.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn set_default_event_based_hold(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_event_based_hold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_object_retention`.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn set_enable_object_retention(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_object_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_destroy`.\nWhen deleting a bucket, this boolean option will delete all contained objects. If you try to delete a bucket that contains objects, Terraform will fail that run."]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to the bucket."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access_prevention`.\nPrevents public access to a bucket."]
    pub fn set_public_access_prevention(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().public_access_prevention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays`.\nEnables Requester Pays on a storage bucket."]
    pub fn set_requester_pays(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().requester_pays = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rpo`.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn set_rpo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rpo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_class`.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn set_storage_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uniform_bucket_level_access`.\nEnables uniform bucket-level access on a bucket."]
    pub fn set_uniform_bucket_level_access(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().uniform_bucket_level_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoclass`.\n"]
    pub fn set_autoclass(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketAutoclassEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoclass = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoclass = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketCorsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_placement_config`.\n"]
    pub fn set_custom_placement_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketCustomPlacementConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_placement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_placement_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketLifecycleRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketLoggingEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketRetentionPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageBucketTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versioning`.\n"]
    pub fn set_versioning(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketVersioningEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `website`.\n"]
    pub fn set_website(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketWebsiteEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().website = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.website = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> terrars::ListRef<StorageBucketAutoclassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<StorageBucketCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> terrars::ListRef<StorageBucketCustomPlacementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<StorageBucketEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<StorageBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<StorageBucketLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<StorageBucketRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<StorageBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> terrars::ListRef<StorageBucketWebsiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

impl terrars::Referable for StorageBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageBucket { }

impl terrars::ToListMappable for StorageBucket {
    type O = terrars::ListRef<StorageBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageBucket_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageBucket {
    pub tf_id: std::string::String,
    #[doc = "The Google Cloud Storage location"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the bucket."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageBucket {
        let out = StorageBucket(std::rc::Rc::new(StorageBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageBucketData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_event_based_hold: core::default::Default::default(),
                enable_object_retention: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                public_access_prevention: core::default::Default::default(),
                requester_pays: core::default::Default::default(),
                rpo: core::default::Default::default(),
                storage_class: core::default::Default::default(),
                uniform_bucket_level_access: core::default::Default::default(),
                autoclass: core::default::Default::default(),
                cors: core::default::Default::default(),
                custom_placement_config: core::default::Default::default(),
                encryption: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                logging: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                versioning: core::default::Default::default(),
                website: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageBucketRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> terrars::ListRef<StorageBucketAutoclassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<StorageBucketCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> terrars::ListRef<StorageBucketCustomPlacementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<StorageBucketEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<StorageBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<StorageBucketLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<StorageBucketRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<StorageBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> terrars::ListRef<StorageBucketWebsiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct StorageBucketAutoclassEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminal_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucketAutoclassEl {
    #[doc =
        "Set the field `terminal_storage_class`.\nThe storage class that objects in the bucket eventually transition to if they are not read for a certain length of time. Supported values include: NEARLINE, ARCHIVE."]
    pub fn set_terminal_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.terminal_storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketAutoclassEl {
    type O = terrars::BlockAssignable<StorageBucketAutoclassEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketAutoclassEl {
    #[doc =
        "While set to true, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildStorageBucketAutoclassEl {
    pub fn build(self) -> StorageBucketAutoclassEl {
        StorageBucketAutoclassEl {
            enabled: self.enabled,
            terminal_storage_class: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketAutoclassElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketAutoclassElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketAutoclassElRef {
        StorageBucketAutoclassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketAutoclassElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhile set to true, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminal_storage_class` after provisioning.\nThe storage class that objects in the bucket eventually transition to if they are not read for a certain length of time. Supported values include: NEARLINE, ARCHIVE."]
    pub fn terminal_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminal_storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageBucketCorsEl {
    #[doc =
        "Set the field `max_age_seconds`.\nThe value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses."]
    pub fn set_max_age_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nThe list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin`.\nThe list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
    pub fn set_origin(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_header`.\nThe list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
    pub fn set_response_header(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketCorsEl {
    type O = terrars::BlockAssignable<StorageBucketCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketCorsEl {}

impl BuildStorageBucketCorsEl {
    pub fn build(self) -> StorageBucketCorsEl {
        StorageBucketCorsEl {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketCorsElRef {
        StorageBucketCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_age_seconds` after provisioning.\nThe value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses."]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
    pub fn method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nThe list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
    pub fn origin(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_header` after provisioning.\nThe list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
    pub fn response_header(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketCustomPlacementConfigEl {
    data_locations: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl StorageBucketCustomPlacementConfigEl { }

impl terrars::ToListMappable for StorageBucketCustomPlacementConfigEl {
    type O = terrars::BlockAssignable<StorageBucketCustomPlacementConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketCustomPlacementConfigEl {
    #[doc =
        "The list of individual regions that comprise a dual-region bucket. See the docs for a list of acceptable regions. Note: If any of the data_locations changes, it will recreate the bucket."]
    pub data_locations: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageBucketCustomPlacementConfigEl {
    pub fn build(self) -> StorageBucketCustomPlacementConfigEl {
        StorageBucketCustomPlacementConfigEl { data_locations: self.data_locations }
    }
}

pub struct StorageBucketCustomPlacementConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketCustomPlacementConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketCustomPlacementConfigElRef {
        StorageBucketCustomPlacementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketCustomPlacementConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_locations` after provisioning.\nThe list of individual regions that comprise a dual-region bucket. See the docs for a list of acceptable regions. Note: If any of the data_locations changes, it will recreate the bucket."]
    pub fn data_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketEncryptionEl {
    default_kms_key_name: terrars::PrimField<std::string::String>,
}

impl StorageBucketEncryptionEl { }

impl terrars::ToListMappable for StorageBucketEncryptionEl {
    type O = terrars::BlockAssignable<StorageBucketEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketEncryptionEl {
    #[doc =
        "A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub default_kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketEncryptionEl {
    pub fn build(self) -> StorageBucketEncryptionEl {
        StorageBucketEncryptionEl { default_kms_key_name: self.default_kms_key_name }
    }
}

pub struct StorageBucketEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketEncryptionElRef {
        StorageBucketEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_kms_key_name` after provisioning.\nA Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub fn default_kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl StorageBucketLifecycleRuleElActionEl {
    #[doc =
        "Set the field `storage_class`.\nThe target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketLifecycleRuleElActionEl {
    type O = terrars::BlockAssignable<StorageBucketLifecycleRuleElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketLifecycleRuleElActionEl {
    #[doc =
        "The type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketLifecycleRuleElActionEl {
    pub fn build(self) -> StorageBucketLifecycleRuleElActionEl {
        StorageBucketLifecycleRuleElActionEl {
            storage_class: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct StorageBucketLifecycleRuleElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketLifecycleRuleElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketLifecycleRuleElActionElRef {
        StorageBucketLifecycleRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketLifecycleRuleElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleElConditionEl {
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

impl StorageBucketLifecycleRuleElConditionEl {
    #[doc = "Set the field `age`.\nMinimum age of an object in days to satisfy this condition."]
    pub fn set_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `created_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_created_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_time_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_custom_time_before(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_time_before = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_since_custom_time`.\nNumber of days elapsed since the user-specified timestamp set on an object."]
    pub fn set_days_since_custom_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_since_custom_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_since_noncurrent_time`.\nNumber of days elapsed since the noncurrent timestamp of an object. This\n\t\t\t\t\t\t\t\t\t\tcondition is relevant only for versioned objects."]
    pub fn set_days_since_noncurrent_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_since_noncurrent_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches_prefix`.\nOne or more matching name prefixes to satisfy this condition."]
    pub fn set_matches_prefix(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `matches_storage_class`.\nStorage Class of objects to satisfy this condition. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, DURABLE_REDUCED_AVAILABILITY."]
    pub fn set_matches_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches_suffix`.\nOne or more matching name suffixes to satisfy this condition."]
    pub fn set_matches_suffix(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_age`.\nWhile set true, age value will be omitted.Required to set true when age is unset in the config file."]
    pub fn set_no_age(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `noncurrent_time_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_noncurrent_time_before(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.noncurrent_time_before = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_newer_versions`.\nRelevant only for versioned objects. The number of newer versions of an object to satisfy this condition."]
    pub fn set_num_newer_versions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_newer_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `with_state`.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn set_with_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.with_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketLifecycleRuleElConditionEl {
    type O = terrars::BlockAssignable<StorageBucketLifecycleRuleElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketLifecycleRuleElConditionEl {}

impl BuildStorageBucketLifecycleRuleElConditionEl {
    pub fn build(self) -> StorageBucketLifecycleRuleElConditionEl {
        StorageBucketLifecycleRuleElConditionEl {
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

pub struct StorageBucketLifecycleRuleElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketLifecycleRuleElConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketLifecycleRuleElConditionElRef {
        StorageBucketLifecycleRuleElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketLifecycleRuleElConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `age` after provisioning.\nMinimum age of an object in days to satisfy this condition."]
    pub fn age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_time_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn custom_time_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_time_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `days_since_custom_time` after provisioning.\nNumber of days elapsed since the user-specified timestamp set on an object."]
    pub fn days_since_custom_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_since_custom_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `days_since_noncurrent_time` after provisioning.\nNumber of days elapsed since the noncurrent timestamp of an object. This\n\t\t\t\t\t\t\t\t\t\tcondition is relevant only for versioned objects."]
    pub fn days_since_noncurrent_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_since_noncurrent_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matches_prefix` after provisioning.\nOne or more matching name prefixes to satisfy this condition."]
    pub fn matches_prefix(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matches_storage_class` after provisioning.\nStorage Class of objects to satisfy this condition. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, DURABLE_REDUCED_AVAILABILITY."]
    pub fn matches_storage_class(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_storage_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matches_suffix` after provisioning.\nOne or more matching name suffixes to satisfy this condition."]
    pub fn matches_suffix(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_age` after provisioning.\nWhile set true, age value will be omitted.Required to set true when age is unset in the config file."]
    pub fn no_age(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `noncurrent_time_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn noncurrent_time_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_time_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_newer_versions` after provisioning.\nRelevant only for versioned objects. The number of newer versions of an object to satisfy this condition."]
    pub fn num_newer_versions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_newer_versions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `with_state` after provisioning.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn with_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBucketLifecycleRuleElDynamic {
    action: std::option::Option<terrars::DynamicBlock<StorageBucketLifecycleRuleElActionEl>>,
    condition: std::option::Option<terrars::DynamicBlock<StorageBucketLifecycleRuleElConditionEl>>,
}

#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<StorageBucketLifecycleRuleElActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<StorageBucketLifecycleRuleElConditionEl>>,
    dynamic: StorageBucketLifecycleRuleElDynamic,
}

impl StorageBucketLifecycleRuleEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketLifecycleRuleElActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketLifecycleRuleElConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageBucketLifecycleRuleEl {
    type O = terrars::BlockAssignable<StorageBucketLifecycleRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketLifecycleRuleEl {}

impl BuildStorageBucketLifecycleRuleEl {
    pub fn build(self) -> StorageBucketLifecycleRuleEl {
        StorageBucketLifecycleRuleEl {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketLifecycleRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketLifecycleRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketLifecycleRuleElRef {
        StorageBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketLifecycleRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct StorageBucketLoggingEl {
    log_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_object_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucketLoggingEl {
    #[doc =
        "Set the field `log_object_prefix`.\nThe object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name."]
    pub fn set_log_object_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_object_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketLoggingEl {
    type O = terrars::BlockAssignable<StorageBucketLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketLoggingEl {
    #[doc = "The bucket that will receive log objects."]
    pub log_bucket: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketLoggingEl {
    pub fn build(self) -> StorageBucketLoggingEl {
        StorageBucketLoggingEl {
            log_bucket: self.log_bucket,
            log_object_prefix: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketLoggingElRef {
        StorageBucketLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_bucket` after provisioning.\nThe bucket that will receive log objects."]
    pub fn log_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_object_prefix` after provisioning.\nThe object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name."]
    pub fn log_object_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_object_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketRetentionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_locked: std::option::Option<terrars::PrimField<bool>>,
    retention_period: terrars::PrimField<f64>,
}

impl StorageBucketRetentionPolicyEl {
    #[doc =
        "Set the field `is_locked`.\nIf set to true, the bucket will be locked and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action."]
    pub fn set_is_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_locked = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketRetentionPolicyEl {
    type O = terrars::BlockAssignable<StorageBucketRetentionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketRetentionPolicyEl {
    #[doc =
        "The period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 3,155,760,000 seconds."]
    pub retention_period: terrars::PrimField<f64>,
}

impl BuildStorageBucketRetentionPolicyEl {
    pub fn build(self) -> StorageBucketRetentionPolicyEl {
        StorageBucketRetentionPolicyEl {
            is_locked: core::default::Default::default(),
            retention_period: self.retention_period,
        }
    }
}

pub struct StorageBucketRetentionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketRetentionPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketRetentionPolicyElRef {
        StorageBucketRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketRetentionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_locked` after provisioning.\nIf set to true, the bucket will be locked and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action."]
    pub fn is_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_locked", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_period` after provisioning.\nThe period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 3,155,760,000 seconds."]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucketTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketTimeoutsEl {
    type O = terrars::BlockAssignable<StorageBucketTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketTimeoutsEl {}

impl BuildStorageBucketTimeoutsEl {
    pub fn build(self) -> StorageBucketTimeoutsEl {
        StorageBucketTimeoutsEl {
            create: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketVersioningEl {
    enabled: terrars::PrimField<bool>,
}

impl StorageBucketVersioningEl { }

impl terrars::ToListMappable for StorageBucketVersioningEl {
    type O = terrars::BlockAssignable<StorageBucketVersioningEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketVersioningEl {
    #[doc = "While set to true, versioning is fully enabled for this bucket."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildStorageBucketVersioningEl {
    pub fn build(self) -> StorageBucketVersioningEl {
        StorageBucketVersioningEl { enabled: self.enabled }
    }
}

pub struct StorageBucketVersioningElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketVersioningElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketVersioningElRef {
        StorageBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketVersioningElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhile set to true, versioning is fully enabled for this bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketWebsiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_page_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_page: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucketWebsiteEl {
    #[doc =
        "Set the field `main_page_suffix`.\nBehaves as the bucket's directory index where missing objects are treated as potential directories."]
    pub fn set_main_page_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_page_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_found_page`.\nThe custom object to return when a requested resource is not found."]
    pub fn set_not_found_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_found_page = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketWebsiteEl {
    type O = terrars::BlockAssignable<StorageBucketWebsiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketWebsiteEl {}

impl BuildStorageBucketWebsiteEl {
    pub fn build(self) -> StorageBucketWebsiteEl {
        StorageBucketWebsiteEl {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketWebsiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketWebsiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketWebsiteElRef {
        StorageBucketWebsiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketWebsiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `main_page_suffix` after provisioning.\nBehaves as the bucket's directory index where missing objects are treated as potential directories."]
    pub fn main_page_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_page_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_found_page` after provisioning.\nThe custom object to return when a requested resource is not found."]
    pub fn not_found_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_found_page", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBucketDynamic {
    autoclass: std::option::Option<terrars::DynamicBlock<StorageBucketAutoclassEl>>,
    cors: std::option::Option<terrars::DynamicBlock<StorageBucketCorsEl>>,
    custom_placement_config: std::option::Option<terrars::DynamicBlock<StorageBucketCustomPlacementConfigEl>>,
    encryption: std::option::Option<terrars::DynamicBlock<StorageBucketEncryptionEl>>,
    lifecycle_rule: std::option::Option<terrars::DynamicBlock<StorageBucketLifecycleRuleEl>>,
    logging: std::option::Option<terrars::DynamicBlock<StorageBucketLoggingEl>>,
    retention_policy: std::option::Option<terrars::DynamicBlock<StorageBucketRetentionPolicyEl>>,
    versioning: std::option::Option<terrars::DynamicBlock<StorageBucketVersioningEl>>,
    website: std::option::Option<terrars::DynamicBlock<StorageBucketWebsiteEl>>,
}
