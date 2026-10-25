#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageBucketData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStorageBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageBucketData_>,
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
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `autoclass` after provisioning.\nThe bucket's autoclass configuration."]
    pub fn autoclass(&self) -> terrars::ListRef<DataStorageBucket_AutoclassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> terrars::ListRef<DataStorageBucket_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> terrars::ListRef<DataStorageBucket_CustomPlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
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
        "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> terrars::ListRef<DataStorageBucket_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hierarchical_namespace` after provisioning.\nThe bucket's HNS configuration, which defines bucket can organize folders in logical file system structure."]
    pub fn hierarchical_namespace(&self) -> terrars::ListRef<DataStorageBucket_HierarchicalNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_filter` after provisioning.\nThe bucket IP filtering configuration."]
    pub fn ip_filter(&self) -> terrars::ListRef<DataStorageBucket_IpFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<DataStorageBucket_LifecycleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> terrars::ListRef<DataStorageBucket_LoggingRef> {
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
        "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> terrars::ListRef<DataStorageBucket_RetentionPolicyRef> {
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
        "Get a reference to the value of field `soft_delete_policy` after provisioning.\nThe bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy"]
    pub fn soft_delete_policy(&self) -> terrars::ListRef<DataStorageBucket_SoftDeletePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.soft_delete_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> terrars::ListRef<DataStorageBucket_VersioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> terrars::ListRef<DataStorageBucket_WebsiteRef> {
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
            data: std::cell::RefCell::new(DataStorageBucketData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
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
    pub fn autoclass(&self) -> terrars::ListRef<DataStorageBucket_AutoclassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> terrars::ListRef<DataStorageBucket_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> terrars::ListRef<DataStorageBucket_CustomPlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_placement_config", self.extract_ref()))
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
        "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> terrars::ListRef<DataStorageBucket_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hierarchical_namespace` after provisioning.\nThe bucket's HNS configuration, which defines bucket can organize folders in logical file system structure."]
    pub fn hierarchical_namespace(&self) -> terrars::ListRef<DataStorageBucket_HierarchicalNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_filter` after provisioning.\nThe bucket IP filtering configuration."]
    pub fn ip_filter(&self) -> terrars::ListRef<DataStorageBucket_IpFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<DataStorageBucket_LifecycleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> terrars::ListRef<DataStorageBucket_LoggingRef> {
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
        "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> terrars::ListRef<DataStorageBucket_RetentionPolicyRef> {
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
        "Get a reference to the value of field `soft_delete_policy` after provisioning.\nThe bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy"]
    pub fn soft_delete_policy(&self) -> terrars::ListRef<DataStorageBucket_SoftDeletePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.soft_delete_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> terrars::ListRef<DataStorageBucket_VersioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> terrars::ListRef<DataStorageBucket_WebsiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Autoclass {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminal_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Autoclass {
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

impl terrars::ToListMappable for DataStorageBucket_Autoclass {
    type O = terrars::BlockAssignable<DataStorageBucket_Autoclass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Autoclass {}

impl BuildDataStorageBucket_Autoclass {
    pub fn build(self) -> DataStorageBucket_Autoclass {
        DataStorageBucket_Autoclass {
            enabled: core::default::Default::default(),
            terminal_storage_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_AutoclassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_AutoclassRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_AutoclassRef {
        DataStorageBucket_AutoclassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_AutoclassRef {
    #[allow(dead_code)]
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
pub struct DataStorageBucket_Cors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageBucket_Cors {
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

impl terrars::ToListMappable for DataStorageBucket_Cors {
    type O = terrars::BlockAssignable<DataStorageBucket_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Cors {}

impl BuildDataStorageBucket_Cors {
    pub fn build(self) -> DataStorageBucket_Cors {
        DataStorageBucket_Cors {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_CorsRef {
        DataStorageBucket_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_CorsRef {
    #[allow(dead_code)]
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
pub struct DataStorageBucket_CustomPlacementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageBucket_CustomPlacementConfig {
    #[doc = "Set the field `data_locations`.\n"]
    pub fn set_data_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_CustomPlacementConfig {
    type O = terrars::BlockAssignable<DataStorageBucket_CustomPlacementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_CustomPlacementConfig {}

impl BuildDataStorageBucket_CustomPlacementConfig {
    pub fn build(self) -> DataStorageBucket_CustomPlacementConfig {
        DataStorageBucket_CustomPlacementConfig { data_locations: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_CustomPlacementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_CustomPlacementConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_CustomPlacementConfigRef {
        DataStorageBucket_CustomPlacementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_CustomPlacementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_locations` after provisioning.\n"]
    pub fn data_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restriction_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restriction_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {}

impl BuildDataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
    pub fn build(self) -> DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
        DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
        DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restriction_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restriction_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {}

impl BuildDataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
    pub fn build(self) -> DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
        DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
        DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restriction_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restriction_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    type O = terrars::BlockAssignable<DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {}

impl BuildDataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
    pub fn build(self) -> DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
        DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
        DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption_enforcement_config: std
    ::option
    ::Option<terrars::ListField<DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_supplied_encryption_enforcement_config: std
    ::option
    ::Option<terrars::ListField<DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_managed_encryption_enforcement_config: std
    ::option
    ::Option<terrars::ListField<DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>>,
}

impl DataStorageBucket_Encryption {
    #[doc = "Set the field `customer_managed_encryption_enforcement_config`.\n"]
    pub fn set_customer_managed_encryption_enforcement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfig>,
                    >,
    ) -> Self {
        self.customer_managed_encryption_enforcement_config = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfig>,
                    >,
    ) -> Self {
        self.customer_supplied_encryption_enforcement_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_kms_key_name`.\n"]
    pub fn set_default_kms_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_managed_encryption_enforcement_config`.\n"]
    pub fn set_google_managed_encryption_enforcement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfig>>,
    ) -> Self {
        self.google_managed_encryption_enforcement_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_Encryption {
    type O = terrars::BlockAssignable<DataStorageBucket_Encryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Encryption {}

impl BuildDataStorageBucket_Encryption {
    pub fn build(self) -> DataStorageBucket_Encryption {
        DataStorageBucket_Encryption {
            customer_managed_encryption_enforcement_config: core::default::Default::default(),
            customer_supplied_encryption_enforcement_config: core::default::Default::default(),
            default_kms_key_name: core::default::Default::default(),
            google_managed_encryption_enforcement_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_EncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_EncryptionRef {
        DataStorageBucket_EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_managed_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<DataStorageBucket_Encryption_CustomerManagedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_enforcement_config", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_supplied_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_supplied_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<DataStorageBucket_Encryption_CustomerSuppliedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_supplied_encryption_enforcement_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_kms_key_name` after provisioning.\n"]
    pub fn default_kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `google_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn google_managed_encryption_enforcement_config(
        &self,
    ) -> terrars::ListRef<DataStorageBucket_Encryption_GoogleManagedEncryptionEnforcementConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_managed_encryption_enforcement_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_HierarchicalNamespace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataStorageBucket_HierarchicalNamespace {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_HierarchicalNamespace {
    type O = terrars::BlockAssignable<DataStorageBucket_HierarchicalNamespace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_HierarchicalNamespace {}

impl BuildDataStorageBucket_HierarchicalNamespace {
    pub fn build(self) -> DataStorageBucket_HierarchicalNamespace {
        DataStorageBucket_HierarchicalNamespace { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_HierarchicalNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_HierarchicalNamespaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_HierarchicalNamespaceRef {
        DataStorageBucket_HierarchicalNamespaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_HierarchicalNamespaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_IpFilter_PublicNetworkSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ip_cidr_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageBucket_IpFilter_PublicNetworkSource {
    #[doc = "Set the field `allowed_ip_cidr_ranges`.\n"]
    pub fn set_allowed_ip_cidr_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_ip_cidr_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_IpFilter_PublicNetworkSource {
    type O = terrars::BlockAssignable<DataStorageBucket_IpFilter_PublicNetworkSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_IpFilter_PublicNetworkSource {}

impl BuildDataStorageBucket_IpFilter_PublicNetworkSource {
    pub fn build(self) -> DataStorageBucket_IpFilter_PublicNetworkSource {
        DataStorageBucket_IpFilter_PublicNetworkSource { allowed_ip_cidr_ranges: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_IpFilter_PublicNetworkSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_IpFilter_PublicNetworkSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucket_IpFilter_PublicNetworkSourceRef {
        DataStorageBucket_IpFilter_PublicNetworkSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_IpFilter_PublicNetworkSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\n"]
    pub fn allowed_ip_cidr_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ip_cidr_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_IpFilter_VpcNetworkSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ip_cidr_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_IpFilter_VpcNetworkSources {
    #[doc = "Set the field `allowed_ip_cidr_ranges`.\n"]
    pub fn set_allowed_ip_cidr_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_ip_cidr_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_IpFilter_VpcNetworkSources {
    type O = terrars::BlockAssignable<DataStorageBucket_IpFilter_VpcNetworkSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_IpFilter_VpcNetworkSources {}

impl BuildDataStorageBucket_IpFilter_VpcNetworkSources {
    pub fn build(self) -> DataStorageBucket_IpFilter_VpcNetworkSources {
        DataStorageBucket_IpFilter_VpcNetworkSources {
            allowed_ip_cidr_ranges: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_IpFilter_VpcNetworkSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_IpFilter_VpcNetworkSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageBucket_IpFilter_VpcNetworkSourcesRef {
        DataStorageBucket_IpFilter_VpcNetworkSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_IpFilter_VpcNetworkSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\n"]
    pub fn allowed_ip_cidr_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ip_cidr_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_IpFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_service_agent_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_cross_org_vpcs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_network_source: std::option::Option<terrars::ListField<DataStorageBucket_IpFilter_PublicNetworkSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_network_sources: std::option::Option<terrars::ListField<DataStorageBucket_IpFilter_VpcNetworkSources>>,
}

impl DataStorageBucket_IpFilter {
    #[doc = "Set the field `allow_all_service_agent_access`.\n"]
    pub fn set_allow_all_service_agent_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all_service_agent_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_cross_org_vpcs`.\n"]
    pub fn set_allow_cross_org_vpcs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_cross_org_vpcs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_network_source`.\n"]
    pub fn set_public_network_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataStorageBucket_IpFilter_PublicNetworkSource>>,
    ) -> Self {
        self.public_network_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataStorageBucket_IpFilter_VpcNetworkSources>>,
    ) -> Self {
        self.vpc_network_sources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_IpFilter {
    type O = terrars::BlockAssignable<DataStorageBucket_IpFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_IpFilter {}

impl BuildDataStorageBucket_IpFilter {
    pub fn build(self) -> DataStorageBucket_IpFilter {
        DataStorageBucket_IpFilter {
            allow_all_service_agent_access: core::default::Default::default(),
            allow_cross_org_vpcs: core::default::Default::default(),
            mode: core::default::Default::default(),
            public_network_source: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_IpFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_IpFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_IpFilterRef {
        DataStorageBucket_IpFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_IpFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_all_service_agent_access` after provisioning.\n"]
    pub fn allow_all_service_agent_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_service_agent_access", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_cross_org_vpcs` after provisioning.\n"]
    pub fn allow_cross_org_vpcs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_cross_org_vpcs", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `public_network_source` after provisioning.\n"]
    pub fn public_network_source(&self) -> terrars::ListRef<DataStorageBucket_IpFilter_PublicNetworkSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_network_source", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(&self) -> terrars::ListRef<DataStorageBucket_IpFilter_VpcNetworkSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_network_sources", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_LifecycleRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_LifecycleRule_Action {
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

impl terrars::ToListMappable for DataStorageBucket_LifecycleRule_Action {
    type O = terrars::BlockAssignable<DataStorageBucket_LifecycleRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_LifecycleRule_Action {}

impl BuildDataStorageBucket_LifecycleRule_Action {
    pub fn build(self) -> DataStorageBucket_LifecycleRule_Action {
        DataStorageBucket_LifecycleRule_Action {
            storage_class: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_LifecycleRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_LifecycleRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_LifecycleRule_ActionRef {
        DataStorageBucket_LifecycleRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_LifecycleRule_ActionRef {
    #[allow(dead_code)]
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
pub struct DataStorageBucket_LifecycleRule_Condition {
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

impl DataStorageBucket_LifecycleRule_Condition {
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

    #[doc = "Set the field `send_age_if_zero`.\n"]
    pub fn set_send_age_if_zero(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_age_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_days_since_custom_time_if_zero`.\n"]
    pub fn set_send_days_since_custom_time_if_zero(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.send_days_since_custom_time_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_days_since_noncurrent_time_if_zero`.\n"]
    pub fn set_send_days_since_noncurrent_time_if_zero(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.send_days_since_noncurrent_time_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_num_newer_versions_if_zero`.\n"]
    pub fn set_send_num_newer_versions_if_zero(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_num_newer_versions_if_zero = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_above_bytes`.\n"]
    pub fn set_size_above_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_above_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_below_bytes`.\n"]
    pub fn set_size_below_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_below_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_state`.\n"]
    pub fn set_with_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.with_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_LifecycleRule_Condition {
    type O = terrars::BlockAssignable<DataStorageBucket_LifecycleRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_LifecycleRule_Condition {}

impl BuildDataStorageBucket_LifecycleRule_Condition {
    pub fn build(self) -> DataStorageBucket_LifecycleRule_Condition {
        DataStorageBucket_LifecycleRule_Condition {
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
pub struct DataStorageBucket_LifecycleRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_LifecycleRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_LifecycleRule_ConditionRef {
        DataStorageBucket_LifecycleRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_LifecycleRule_ConditionRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `noncurrent_time_before` after provisioning.\n"]
    pub fn noncurrent_time_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_time_before", self.base))
    }

    #[doc = "Get a reference to the value of field `num_newer_versions` after provisioning.\n"]
    pub fn num_newer_versions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_newer_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `send_age_if_zero` after provisioning.\n"]
    pub fn send_age_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_age_if_zero", self.base))
    }

    #[doc = "Get a reference to the value of field `send_days_since_custom_time_if_zero` after provisioning.\n"]
    pub fn send_days_since_custom_time_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_days_since_custom_time_if_zero", self.base))
    }

    #[doc = "Get a reference to the value of field `send_days_since_noncurrent_time_if_zero` after provisioning.\n"]
    pub fn send_days_since_noncurrent_time_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_noncurrent_time_if_zero", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `send_num_newer_versions_if_zero` after provisioning.\n"]
    pub fn send_num_newer_versions_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_num_newer_versions_if_zero", self.base))
    }

    #[doc = "Get a reference to the value of field `size_above_bytes` after provisioning.\n"]
    pub fn size_above_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_above_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `size_below_bytes` after provisioning.\n"]
    pub fn size_below_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_below_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `with_state` after provisioning.\n"]
    pub fn with_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_LifecycleRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::SetField<DataStorageBucket_LifecycleRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::SetField<DataStorageBucket_LifecycleRule_Condition>>,
}

impl DataStorageBucket_LifecycleRule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataStorageBucket_LifecycleRule_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataStorageBucket_LifecycleRule_Condition>>,
    ) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_LifecycleRule {
    type O = terrars::BlockAssignable<DataStorageBucket_LifecycleRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_LifecycleRule {}

impl BuildDataStorageBucket_LifecycleRule {
    pub fn build(self) -> DataStorageBucket_LifecycleRule {
        DataStorageBucket_LifecycleRule {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_LifecycleRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_LifecycleRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_LifecycleRuleRef {
        DataStorageBucket_LifecycleRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_LifecycleRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::SetRef<DataStorageBucket_LifecycleRule_ActionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::SetRef<DataStorageBucket_LifecycleRule_ConditionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_object_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Logging {
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

impl terrars::ToListMappable for DataStorageBucket_Logging {
    type O = terrars::BlockAssignable<DataStorageBucket_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Logging {}

impl BuildDataStorageBucket_Logging {
    pub fn build(self) -> DataStorageBucket_Logging {
        DataStorageBucket_Logging {
            log_bucket: core::default::Default::default(),
            log_object_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_LoggingRef {
        DataStorageBucket_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_LoggingRef {
    #[allow(dead_code)]
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
pub struct DataStorageBucket_RetentionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_RetentionPolicy {
    #[doc = "Set the field `is_locked`.\n"]
    pub fn set_is_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_RetentionPolicy {
    type O = terrars::BlockAssignable<DataStorageBucket_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_RetentionPolicy {}

impl BuildDataStorageBucket_RetentionPolicy {
    pub fn build(self) -> DataStorageBucket_RetentionPolicy {
        DataStorageBucket_RetentionPolicy {
            is_locked: core::default::Default::default(),
            retention_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_RetentionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_RetentionPolicyRef {
        DataStorageBucket_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_RetentionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_locked` after provisioning.\n"]
    pub fn is_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_locked", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_SoftDeletePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_duration_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataStorageBucket_SoftDeletePolicy {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_duration_seconds`.\n"]
    pub fn set_retention_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_duration_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_SoftDeletePolicy {
    type O = terrars::BlockAssignable<DataStorageBucket_SoftDeletePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_SoftDeletePolicy {}

impl BuildDataStorageBucket_SoftDeletePolicy {
    pub fn build(self) -> DataStorageBucket_SoftDeletePolicy {
        DataStorageBucket_SoftDeletePolicy {
            effective_time: core::default::Default::default(),
            retention_duration_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_SoftDeletePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_SoftDeletePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_SoftDeletePolicyRef {
        DataStorageBucket_SoftDeletePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_SoftDeletePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_time", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_duration_seconds` after provisioning.\n"]
    pub fn retention_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Versioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataStorageBucket_Versioning {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageBucket_Versioning {
    type O = terrars::BlockAssignable<DataStorageBucket_Versioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Versioning {}

impl BuildDataStorageBucket_Versioning {
    pub fn build(self) -> DataStorageBucket_Versioning {
        DataStorageBucket_Versioning { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_VersioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_VersioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_VersioningRef {
        DataStorageBucket_VersioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_VersioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucket_Website {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_page_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_page: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageBucket_Website {
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

impl terrars::ToListMappable for DataStorageBucket_Website {
    type O = terrars::BlockAssignable<DataStorageBucket_Website>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucket_Website {}

impl BuildDataStorageBucket_Website {
    pub fn build(self) -> DataStorageBucket_Website {
        DataStorageBucket_Website {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageBucket_WebsiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageBucket_WebsiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageBucket_WebsiteRef {
        DataStorageBucket_WebsiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucket_WebsiteRef {
    #[allow(dead_code)]
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
