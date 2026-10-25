#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageBucketData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    autoclass: std::option::Option<std::vec::Vec<StorageBucket_Autoclass>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<StorageBucket_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_placement_config: std::option::Option<std::vec::Vec<StorageBucket_CustomPlacementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<std::vec::Vec<StorageBucket_Encryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchical_namespace: std::option::Option<std::vec::Vec<StorageBucket_HierarchicalNamespace>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_filter: std::option::Option<std::vec::Vec<StorageBucket_IpFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_rule: std::option::Option<std::vec::Vec<StorageBucket_LifecycleRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<StorageBucket_Logging>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<StorageBucket_RetentionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    soft_delete_policy: std::option::Option<std::vec::Vec<StorageBucket_SoftDeletePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageBucket_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioning: std::option::Option<std::vec::Vec<StorageBucket_Versioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    website: std::option::Option<std::vec::Vec<StorageBucket_Website>>,
    dynamic: StorageBucketDynamic,
}

struct StorageBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageBucketData_>,
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_object_retention`.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn set_enable_object_retention(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_object_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_destroy`.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
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
    pub fn set_autoclass(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Autoclass>>) -> Self {
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
    pub fn set_cors(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Cors>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_CustomPlacementConfig>>,
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
    pub fn set_encryption(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Encryption>>) -> Self {
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

    #[doc = "Set the field `hierarchical_namespace`.\n"]
    pub fn set_hierarchical_namespace(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_HierarchicalNamespace>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hierarchical_namespace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hierarchical_namespace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_filter`.\n"]
    pub fn set_ip_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_IpFilter>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ip_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ip_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_LifecycleRule>>,
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
    pub fn set_logging(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Logging>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_RetentionPolicy>>,
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

    #[doc = "Set the field `soft_delete_policy`.\n"]
    pub fn set_soft_delete_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_SoftDeletePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().soft_delete_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.soft_delete_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageBucket_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versioning`.\n"]
    pub fn set_versioning(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Versioning>>) -> Self {
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
    pub fn set_website(self, v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_Website>>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
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
        "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
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
        "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the project in which the resource belongs."]
    pub fn project_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_number", self.extract_ref()))
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
        "Get a reference to the value of field `time_created` after provisioning.\nThe creation time of the bucket in RFC 3339 format."]
    pub fn time_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_bucket_level_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nThe time at which the bucket's metadata or IAM policy was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> terrars::ListRef<StorageBucket_AutoclassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<StorageBucket_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> terrars::ListRef<StorageBucket_CustomPlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<StorageBucket_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(&self) -> terrars::ListRef<StorageBucket_HierarchicalNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\n"]
    pub fn ip_filter(&self) -> terrars::ListRef<StorageBucket_IpFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<StorageBucket_LifecycleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<StorageBucket_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<StorageBucket_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\n"]
    pub fn soft_delete_policy(&self) -> terrars::ListRef<StorageBucket_SoftDeletePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.soft_delete_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucket_TimeoutsRef {
        StorageBucket_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<StorageBucket_VersioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> terrars::ListRef<StorageBucket_WebsiteRef> {
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
    #[doc = "The Google Cloud Storage location or region."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the bucket."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageBucket {
        let out = StorageBucket(std::rc::Rc::new(StorageBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageBucketData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_event_based_hold: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
                hierarchical_namespace: core::default::Default::default(),
                ip_filter: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                logging: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                soft_delete_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
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
        "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
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
        "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the project in which the resource belongs."]
    pub fn project_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_number", self.extract_ref()))
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
        "Get a reference to the value of field `time_created` after provisioning.\nThe creation time of the bucket in RFC 3339 format."]
    pub fn time_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uniform_bucket_level_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nThe time at which the bucket's metadata or IAM policy was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> terrars::ListRef<StorageBucket_AutoclassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<StorageBucket_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> terrars::ListRef<StorageBucket_CustomPlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<StorageBucket_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(&self) -> terrars::ListRef<StorageBucket_HierarchicalNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\n"]
    pub fn ip_filter(&self) -> terrars::ListRef<StorageBucket_IpFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<StorageBucket_LifecycleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<StorageBucket_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<StorageBucket_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\n"]
    pub fn soft_delete_policy(&self) -> terrars::ListRef<StorageBucket_SoftDeletePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.soft_delete_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucket_TimeoutsRef {
        StorageBucket_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<StorageBucket_VersioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> terrars::ListRef<StorageBucket_WebsiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct StorageBucketDynamic {
    autoclass: std::option::Option<terrars::DynamicBlock<StorageBucket_Autoclass>>,
    cors: std::option::Option<terrars::DynamicBlock<StorageBucket_Cors>>,
    custom_placement_config: std::option::Option<terrars::DynamicBlock<StorageBucket_CustomPlacementConfig>>,
    encryption: std::option::Option<terrars::DynamicBlock<StorageBucket_Encryption>>,
    hierarchical_namespace: std::option::Option<terrars::DynamicBlock<StorageBucket_HierarchicalNamespace>>,
    ip_filter: std::option::Option<terrars::DynamicBlock<StorageBucket_IpFilter>>,
    lifecycle_rule: std::option::Option<terrars::DynamicBlock<StorageBucket_LifecycleRule>>,
    logging: std::option::Option<terrars::DynamicBlock<StorageBucket_Logging>>,
    retention_policy: std::option::Option<terrars::DynamicBlock<StorageBucket_RetentionPolicy>>,
    soft_delete_policy: std::option::Option<terrars::DynamicBlock<StorageBucket_SoftDeletePolicy>>,
    versioning: std::option::Option<terrars::DynamicBlock<StorageBucket_Versioning>>,
    website: std::option::Option<terrars::DynamicBlock<StorageBucket_Website>>,
}

#[derive(Serialize)]
pub struct StorageBucket_Autoclass {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminal_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_Autoclass {
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

impl terrars::ToListMappable for StorageBucket_Autoclass {
    type O = terrars::BlockAssignable<StorageBucket_Autoclass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Autoclass {
    #[doc =
        "While set to true, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildStorageBucket_Autoclass {
    pub fn build(self) -> StorageBucket_Autoclass {
        StorageBucket_Autoclass {
            enabled: self.enabled,
            terminal_storage_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_AutoclassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_AutoclassRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_AutoclassRef {
        StorageBucket_AutoclassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_AutoclassRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_Cors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageBucket_Cors {
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

impl terrars::ToListMappable for StorageBucket_Cors {
    type O = terrars::BlockAssignable<StorageBucket_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Cors {}

impl BuildStorageBucket_Cors {
    pub fn build(self) -> StorageBucket_Cors {
        StorageBucket_Cors {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_CorsRef {
        StorageBucket_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_CorsRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_CustomPlacementConfig {
    data_locations: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_CustomPlacementConfig { }

impl terrars::ToListMappable for StorageBucket_CustomPlacementConfig {
    type O = terrars::BlockAssignable<StorageBucket_CustomPlacementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_CustomPlacementConfig {
    #[doc =
        "The list of individual regions that comprise a dual-region bucket. See the docs for a list of acceptable regions. Note: If any of the data_locations changes, it will recreate the bucket."]
    pub data_locations: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageBucket_CustomPlacementConfig {
    pub fn build(self) -> StorageBucket_CustomPlacementConfig {
        StorageBucket_CustomPlacementConfig { data_locations: self.data_locations }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_CustomPlacementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_CustomPlacementConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_CustomPlacementConfigRef {
        StorageBucket_CustomPlacementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_CustomPlacementConfigRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    restriction_mode: terrars::PrimField<std::string::String>,
}

impl StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig { }

impl terrars::ToListMappable for StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    #[doc =
        "Whether CMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CMEK encryption. If NotRestricted or unset, creation of new objects with CMEK encryption is allowed."]
    pub restriction_mode: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    pub fn build(self) -> StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
        StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
            restriction_mode: self.restriction_mode,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
        StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether CMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CMEK encryption. If NotRestricted or unset, creation of new objects with CMEK encryption is allowed."]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    restriction_mode: terrars::PrimField<std::string::String>,
}

impl StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig { }

impl terrars::ToListMappable for StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    #[doc =
        "Whether CSEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CSEK encryption. If NotRestricted or unset, creation of new objects with CSEK encryption is allowed."]
    pub restriction_mode: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    pub fn build(self) -> StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
        StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
            restriction_mode: self.restriction_mode,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
        StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether CSEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CSEK encryption. If NotRestricted or unset, creation of new objects with CSEK encryption is allowed."]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    restriction_mode: terrars::PrimField<std::string::String>,
}

impl StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig { }

impl terrars::ToListMappable for StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    #[doc =
        "Whether GMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using GMEK encryption. If NotRestricted or unset, creation of new objects with GMEK encryption is allowed."]
    pub restriction_mode: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    pub fn build(self) -> StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
        StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig { restriction_mode: self.restriction_mode }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
        StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether GMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using GMEK encryption. If NotRestricted or unset, creation of new objects with GMEK encryption is allowed."]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBucket_EncryptionDynamic {
    customer_managed_encryption_enforcement_config: std
    ::option
    ::Option<terrars::DynamicBlock<StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>>,
    customer_supplied_encryption_enforcement_config: std
    ::option
    ::Option<terrars::DynamicBlock<StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>>,
    google_managed_encryption_enforcement_config: std
    ::option
    ::Option<terrars::DynamicBlock<StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>>,
}

#[derive(Serialize)]
pub struct StorageBucket_Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption_enforcement_config: std
    ::option
    ::Option<std::vec::Vec<StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_supplied_encryption_enforcement_config: std
    ::option
    ::Option<std::vec::Vec<StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_managed_encryption_enforcement_config: std
    ::option
    ::Option<std::vec::Vec<StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>>,
    dynamic: StorageBucket_EncryptionDynamic,
}

impl StorageBucket_Encryption {
    #[doc =
        "Set the field `default_kms_key_name`.\nA Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub fn set_default_kms_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_encryption_enforcement_config`.\n"]
    pub fn set_customer_managed_encryption_enforcement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_managed_encryption_enforcement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption_enforcement_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customer_supplied_encryption_enforcement_config`.\n"]
    pub fn set_customer_supplied_encryption_enforcement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_supplied_encryption_enforcement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_supplied_encryption_enforcement_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_managed_encryption_enforcement_config`.\n"]
    pub fn set_google_managed_encryption_enforcement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_managed_encryption_enforcement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_managed_encryption_enforcement_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageBucket_Encryption {
    type O = terrars::BlockAssignable<StorageBucket_Encryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Encryption {}

impl BuildStorageBucket_Encryption {
    pub fn build(self) -> StorageBucket_Encryption {
        StorageBucket_Encryption {
            default_kms_key_name: core::default::Default::default(),
            customer_managed_encryption_enforcement_config: core::default::Default::default(),
            customer_supplied_encryption_enforcement_config: core::default::Default::default(),
            google_managed_encryption_enforcement_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_EncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_EncryptionRef {
        StorageBucket_EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_kms_key_name` after provisioning.\nA Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub fn default_kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_managed_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<StorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_enforcement_config", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_supplied_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_supplied_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<StorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_supplied_encryption_enforcement_config", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `google_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn google_managed_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<StorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_managed_encryption_enforcement_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct StorageBucket_HierarchicalNamespace {
    enabled: terrars::PrimField<bool>,
}

impl StorageBucket_HierarchicalNamespace { }

impl terrars::ToListMappable for StorageBucket_HierarchicalNamespace {
    type O = terrars::BlockAssignable<StorageBucket_HierarchicalNamespace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_HierarchicalNamespace {
    #[doc = "Set this field true to organize bucket with logical file system structure."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildStorageBucket_HierarchicalNamespace {
    pub fn build(self) -> StorageBucket_HierarchicalNamespace {
        StorageBucket_HierarchicalNamespace { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_HierarchicalNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_HierarchicalNamespaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_HierarchicalNamespaceRef {
        StorageBucket_HierarchicalNamespaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_HierarchicalNamespaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSet this field true to organize bucket with logical file system structure."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_IpFilter_PublicNetworkSource {
    allowed_ip_cidr_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_IpFilter_PublicNetworkSource { }

impl terrars::ToListMappable for StorageBucket_IpFilter_PublicNetworkSource {
    type O = terrars::BlockAssignable<StorageBucket_IpFilter_PublicNetworkSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_IpFilter_PublicNetworkSource {
    #[doc = "The list of public IPv4, IPv6 cidr ranges that are allowed to access the bucket."]
    pub allowed_ip_cidr_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageBucket_IpFilter_PublicNetworkSource {
    pub fn build(self) -> StorageBucket_IpFilter_PublicNetworkSource {
        StorageBucket_IpFilter_PublicNetworkSource { allowed_ip_cidr_ranges: self.allowed_ip_cidr_ranges }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_IpFilter_PublicNetworkSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_IpFilter_PublicNetworkSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_IpFilter_PublicNetworkSourceRef {
        StorageBucket_IpFilter_PublicNetworkSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_IpFilter_PublicNetworkSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\nThe list of public IPv4, IPv6 cidr ranges that are allowed to access the bucket."]
    pub fn allowed_ip_cidr_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ip_cidr_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_IpFilter_VpcNetworkSources {
    allowed_ip_cidr_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
    network: terrars::PrimField<std::string::String>,
}

impl StorageBucket_IpFilter_VpcNetworkSources { }

impl terrars::ToListMappable for StorageBucket_IpFilter_VpcNetworkSources {
    type O = terrars::BlockAssignable<StorageBucket_IpFilter_VpcNetworkSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_IpFilter_VpcNetworkSources {
    #[doc = "The list of public or private IPv4 and IPv6 CIDR ranges that can access the bucket."]
    pub allowed_ip_cidr_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Name of the network. Format: projects/{PROJECT_ID}/global/networks/{NETWORK_NAME}"]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_IpFilter_VpcNetworkSources {
    pub fn build(self) -> StorageBucket_IpFilter_VpcNetworkSources {
        StorageBucket_IpFilter_VpcNetworkSources {
            allowed_ip_cidr_ranges: self.allowed_ip_cidr_ranges,
            network: self.network,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_IpFilter_VpcNetworkSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_IpFilter_VpcNetworkSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_IpFilter_VpcNetworkSourcesRef {
        StorageBucket_IpFilter_VpcNetworkSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_IpFilter_VpcNetworkSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\nThe list of public or private IPv4 and IPv6 CIDR ranges that can access the bucket."]
    pub fn allowed_ip_cidr_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ip_cidr_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nName of the network. Format: projects/{PROJECT_ID}/global/networks/{NETWORK_NAME}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBucket_IpFilterDynamic {
    public_network_source: std::option::Option<terrars::DynamicBlock<StorageBucket_IpFilter_PublicNetworkSource>>,
    vpc_network_sources: std::option::Option<terrars::DynamicBlock<StorageBucket_IpFilter_VpcNetworkSources>>,
}

#[derive(Serialize)]
pub struct StorageBucket_IpFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_service_agent_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_cross_org_vpcs: std::option::Option<terrars::PrimField<bool>>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_network_source: std::option::Option<std::vec::Vec<StorageBucket_IpFilter_PublicNetworkSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_network_sources: std::option::Option<std::vec::Vec<StorageBucket_IpFilter_VpcNetworkSources>>,
    dynamic: StorageBucket_IpFilterDynamic,
}

impl StorageBucket_IpFilter {
    #[doc =
        "Set the field `allow_all_service_agent_access`.\nWhether to allow all service agents to access the bucket regardless of the IP filter configuration."]
    pub fn set_allow_all_service_agent_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all_service_agent_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_cross_org_vpcs`.\nWhether to allow cross-org VPCs in the bucket's IP filter configuration."]
    pub fn set_allow_cross_org_vpcs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_cross_org_vpcs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_network_source`.\n"]
    pub fn set_public_network_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_IpFilter_PublicNetworkSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_network_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_network_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_IpFilter_VpcNetworkSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_network_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network_sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageBucket_IpFilter {
    type O = terrars::BlockAssignable<StorageBucket_IpFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_IpFilter {
    #[doc = "The mode of the IP filter. Valid values are 'Enabled' and 'Disabled'."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_IpFilter {
    pub fn build(self) -> StorageBucket_IpFilter {
        StorageBucket_IpFilter {
            allow_all_service_agent_access: core::default::Default::default(),
            allow_cross_org_vpcs: core::default::Default::default(),
            mode: self.mode,
            public_network_source: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_IpFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_IpFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_IpFilterRef {
        StorageBucket_IpFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_IpFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all_service_agent_access` after provisioning.\nWhether to allow all service agents to access the bucket regardless of the IP filter configuration."]
    pub fn allow_all_service_agent_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_service_agent_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_cross_org_vpcs` after provisioning.\nWhether to allow cross-org VPCs in the bucket's IP filter configuration."]
    pub fn allow_cross_org_vpcs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_cross_org_vpcs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of the IP filter. Valid values are 'Enabled' and 'Disabled'."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `public_network_source` after provisioning.\n"]
    pub fn public_network_source(&self) -> terrars::ListRef<StorageBucket_IpFilter_PublicNetworkSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_network_source", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(&self) -> terrars::ListRef<StorageBucket_IpFilter_VpcNetworkSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_network_sources", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_LifecycleRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl StorageBucket_LifecycleRule_Action {
    #[doc =
        "Set the field `storage_class`.\nThe target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucket_LifecycleRule_Action {
    type O = terrars::BlockAssignable<StorageBucket_LifecycleRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_LifecycleRule_Action {
    #[doc =
        "The type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_LifecycleRule_Action {
    pub fn build(self) -> StorageBucket_LifecycleRule_Action {
        StorageBucket_LifecycleRule_Action {
            storage_class: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_LifecycleRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_LifecycleRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_LifecycleRule_ActionRef {
        StorageBucket_LifecycleRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_LifecycleRule_ActionRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_LifecycleRule_Condition {
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
    noncurrent_time_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_newer_versions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_age_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_days_since_custom_time_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_days_since_noncurrent_time_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_num_newer_versions_if_zero: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_above_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_below_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_LifecycleRule_Condition {
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
        "Set the field `send_age_if_zero`.\nWhile set true, age value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the age field. It can be used alone or together with age."]
    pub fn set_send_age_if_zero(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_age_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `send_days_since_custom_time_if_zero`.\nWhile set true, days_since_custom_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_custom_time field. It can be used alone or together with days_since_custom_time."]
    pub fn set_send_days_since_custom_time_if_zero(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.send_days_since_custom_time_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `send_days_since_noncurrent_time_if_zero`.\nWhile set true, days_since_noncurrent_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_noncurrent_time field. It can be used alone or together with days_since_noncurrent_time."]
    pub fn set_send_days_since_noncurrent_time_if_zero(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.send_days_since_noncurrent_time_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `send_num_newer_versions_if_zero`.\nWhile set true, num_newer_versions value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the num_newer_versions field. It can be used alone or together with num_newer_versions."]
    pub fn set_send_num_newer_versions_if_zero(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_num_newer_versions_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_above_bytes`.\nObjects having a size greater than this value in bytes will be matched."]
    pub fn set_size_above_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_above_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_below_bytes`.\nObjects having a size less than this value in bytes will be matched."]
    pub fn set_size_below_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_below_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `with_state`.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn set_with_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.with_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucket_LifecycleRule_Condition {
    type O = terrars::BlockAssignable<StorageBucket_LifecycleRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_LifecycleRule_Condition {}

impl BuildStorageBucket_LifecycleRule_Condition {
    pub fn build(self) -> StorageBucket_LifecycleRule_Condition {
        StorageBucket_LifecycleRule_Condition {
            age: core::default::Default::default(),
            created_before: core::default::Default::default(),
            custom_time_before: core::default::Default::default(),
            days_since_custom_time: core::default::Default::default(),
            days_since_noncurrent_time: core::default::Default::default(),
            matches_prefix: core::default::Default::default(),
            matches_storage_class: core::default::Default::default(),
            matches_suffix: core::default::Default::default(),
            noncurrent_time_before: core::default::Default::default(),
            num_newer_versions: core::default::Default::default(),
            send_age_if_zero: core::default::Default::default(),
            send_days_since_custom_time_if_zero: core::default::Default::default(),
            send_days_since_noncurrent_time_if_zero: core::default::Default::default(),
            send_num_newer_versions_if_zero: core::default::Default::default(),
            size_above_bytes: core::default::Default::default(),
            size_below_bytes: core::default::Default::default(),
            with_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_LifecycleRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_LifecycleRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_LifecycleRule_ConditionRef {
        StorageBucket_LifecycleRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_LifecycleRule_ConditionRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `send_age_if_zero` after provisioning.\nWhile set true, age value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the age field. It can be used alone or together with age."]
    pub fn send_age_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_age_if_zero", self.base))
    }

    #[doc =
        "Get a reference to the value of field `send_days_since_custom_time_if_zero` after provisioning.\nWhile set true, days_since_custom_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_custom_time field. It can be used alone or together with days_since_custom_time."]
    pub fn send_days_since_custom_time_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_days_since_custom_time_if_zero", self.base))
    }

    #[doc =
        "Get a reference to the value of field `send_days_since_noncurrent_time_if_zero` after provisioning.\nWhile set true, days_since_noncurrent_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_noncurrent_time field. It can be used alone or together with days_since_noncurrent_time."]
    pub fn send_days_since_noncurrent_time_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_noncurrent_time_if_zero", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `send_num_newer_versions_if_zero` after provisioning.\nWhile set true, num_newer_versions value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the num_newer_versions field. It can be used alone or together with num_newer_versions."]
    pub fn send_num_newer_versions_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_num_newer_versions_if_zero", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_above_bytes` after provisioning.\nObjects having a size greater than this value in bytes will be matched."]
    pub fn size_above_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_above_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_below_bytes` after provisioning.\nObjects having a size less than this value in bytes will be matched."]
    pub fn size_below_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_below_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `with_state` after provisioning.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn with_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageBucket_LifecycleRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<StorageBucket_LifecycleRule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<StorageBucket_LifecycleRule_Condition>>,
}

#[derive(Serialize)]
pub struct StorageBucket_LifecycleRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<StorageBucket_LifecycleRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<StorageBucket_LifecycleRule_Condition>>,
    dynamic: StorageBucket_LifecycleRuleDynamic,
}

impl StorageBucket_LifecycleRule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_LifecycleRule_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucket_LifecycleRule_Condition>>,
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

impl terrars::ToListMappable for StorageBucket_LifecycleRule {
    type O = terrars::BlockAssignable<StorageBucket_LifecycleRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_LifecycleRule {}

impl BuildStorageBucket_LifecycleRule {
    pub fn build(self) -> StorageBucket_LifecycleRule {
        StorageBucket_LifecycleRule {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_LifecycleRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_LifecycleRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_LifecycleRuleRef {
        StorageBucket_LifecycleRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_LifecycleRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct StorageBucket_Logging {
    log_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_object_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_Logging {
    #[doc =
        "Set the field `log_object_prefix`.\nThe object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name."]
    pub fn set_log_object_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_object_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucket_Logging {
    type O = terrars::BlockAssignable<StorageBucket_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Logging {
    #[doc = "The bucket that will receive log objects."]
    pub log_bucket: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_Logging {
    pub fn build(self) -> StorageBucket_Logging {
        StorageBucket_Logging {
            log_bucket: self.log_bucket,
            log_object_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_LoggingRef {
        StorageBucket_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_LoggingRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_RetentionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_locked: std::option::Option<terrars::PrimField<bool>>,
    retention_period: terrars::PrimField<std::string::String>,
}

impl StorageBucket_RetentionPolicy {
    #[doc =
        "Set the field `is_locked`.\nIf set to true, the bucket will be locked and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action."]
    pub fn set_is_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_locked = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucket_RetentionPolicy {
    type O = terrars::BlockAssignable<StorageBucket_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_RetentionPolicy {
    #[doc =
        "The period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 3,155,760,000 seconds."]
    pub retention_period: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucket_RetentionPolicy {
    pub fn build(self) -> StorageBucket_RetentionPolicy {
        StorageBucket_RetentionPolicy {
            is_locked: core::default::Default::default(),
            retention_period: self.retention_period,
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_RetentionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_RetentionPolicyRef {
        StorageBucket_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_RetentionPolicyRef {
    #[allow(dead_code)]
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
    pub fn retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_SoftDeletePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_duration_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl StorageBucket_SoftDeletePolicy {
    #[doc =
        "Set the field `retention_duration_seconds`.\nThe duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800."]
    pub fn set_retention_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_duration_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucket_SoftDeletePolicy {
    type O = terrars::BlockAssignable<StorageBucket_SoftDeletePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_SoftDeletePolicy {}

impl BuildStorageBucket_SoftDeletePolicy {
    pub fn build(self) -> StorageBucket_SoftDeletePolicy {
        StorageBucket_SoftDeletePolicy { retention_duration_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_SoftDeletePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_SoftDeletePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_SoftDeletePolicyRef {
        StorageBucket_SoftDeletePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_SoftDeletePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_time` after provisioning.\nServer-determined value that indicates the time from which the policy, or one with a greater retention, was effective. This value is in RFC 3339 format."]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_duration_seconds` after provisioning.\nThe duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800."]
    pub fn retention_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucket_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_Timeouts {
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

impl terrars::ToListMappable for StorageBucket_Timeouts {
    type O = terrars::BlockAssignable<StorageBucket_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Timeouts {}

impl BuildStorageBucket_Timeouts {
    pub fn build(self) -> StorageBucket_Timeouts {
        StorageBucket_Timeouts {
            create: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_TimeoutsRef {
        StorageBucket_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_TimeoutsRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_Versioning {
    enabled: terrars::PrimField<bool>,
}

impl StorageBucket_Versioning { }

impl terrars::ToListMappable for StorageBucket_Versioning {
    type O = terrars::BlockAssignable<StorageBucket_Versioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Versioning {
    #[doc = "While set to true, versioning is fully enabled for this bucket."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildStorageBucket_Versioning {
    pub fn build(self) -> StorageBucket_Versioning {
        StorageBucket_Versioning { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_VersioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_VersioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_VersioningRef {
        StorageBucket_VersioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_VersioningRef {
    #[allow(dead_code)]
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
pub struct StorageBucket_Website {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_page_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_page: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucket_Website {
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

impl terrars::ToListMappable for StorageBucket_Website {
    type O = terrars::BlockAssignable<StorageBucket_Website>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucket_Website {}

impl BuildStorageBucket_Website {
    pub fn build(self) -> StorageBucket_Website {
        StorageBucket_Website {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageBucket_WebsiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucket_WebsiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucket_WebsiteRef {
        StorageBucket_WebsiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucket_WebsiteRef {
    #[allow(dead_code)]
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
