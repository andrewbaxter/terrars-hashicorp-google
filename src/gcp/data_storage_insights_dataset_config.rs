#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageInsightsDatasetConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    dataset_config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStorageInsightsDatasetConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageInsightsDatasetConfigData_>,
}

#[derive(Clone)]
pub struct DataStorageInsightsDatasetConfig(std::rc::Rc<DataStorageInsightsDatasetConfig_>);

impl DataStorageInsightsDatasetConfig {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `activity_data_retention_period_days` after provisioning.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn activity_data_retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.activity_data_retention_period_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe UTC time at which the DatasetConfig was created. This is auto-populated."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_config_id` after provisioning.\nThe user-defined ID of the DatasetConfig"]
    pub fn dataset_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_config_state` after provisioning.\nState of the DatasetConfig."]
    pub fn dataset_config_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_config_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\nDefined the options for excluding cloud storage buckets for the DatasetConfig."]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\nDefines the options for excluding cloud storage locations for the DatasetConfig."]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\nIdentity used by DatasetConfig."]
    pub fn identity(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\nDefines the options for including cloud storage buckets for the DatasetConfig."]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\nDefines the options for including cloud storage locations for the DatasetConfig."]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_LinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `link_dataset` after provisioning.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn link_dataset(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DatasetConfig."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full canonical resource name of the DatasetConfig (e.g., projects/P/locations/L/datasetConfigs/ID)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_number` after provisioning.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn organization_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_scope` after provisioning.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn organization_scope(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_period_days` after provisioning.\nNumber of days of history that must be retained."]
    pub fn retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_folders` after provisioning.\nDefines the options for providing source folders for the DatasetConfig."]
    pub fn source_folders(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_SourceFoldersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_folders", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_projects` after provisioning.\nDefines the options for providing source projects for the DatasetConfig."]
    pub fn source_projects(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_SourceProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_projects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageInsightsDatasetConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageInsightsDatasetConfig { }

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig {
    type O = terrars::ListRef<DataStorageInsightsDatasetConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageInsightsDatasetConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_insights_dataset_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageInsightsDatasetConfig {
    pub tf_id: std::string::String,
    #[doc = "The user-defined ID of the DatasetConfig"]
    pub dataset_config_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the DatasetConfig."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataStorageInsightsDatasetConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageInsightsDatasetConfig {
        let out = DataStorageInsightsDatasetConfig(std::rc::Rc::new(DataStorageInsightsDatasetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStorageInsightsDatasetConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                dataset_config_id: self.dataset_config_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageInsightsDatasetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageInsightsDatasetConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `activity_data_retention_period_days` after provisioning.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn activity_data_retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.activity_data_retention_period_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe UTC time at which the DatasetConfig was created. This is auto-populated."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_config_id` after provisioning.\nThe user-defined ID of the DatasetConfig"]
    pub fn dataset_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_config_state` after provisioning.\nState of the DatasetConfig."]
    pub fn dataset_config_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_config_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\nDefined the options for excluding cloud storage buckets for the DatasetConfig."]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\nDefines the options for excluding cloud storage locations for the DatasetConfig."]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\nIdentity used by DatasetConfig."]
    pub fn identity(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\nDefines the options for including cloud storage buckets for the DatasetConfig."]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\nDefines the options for including cloud storage locations for the DatasetConfig."]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `include_newly_created_buckets` after provisioning.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn include_newly_created_buckets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_newly_created_buckets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link` after provisioning.\nDetails of the linked DatasetConfig."]
    pub fn link(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_LinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `link_dataset` after provisioning.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn link_dataset(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DatasetConfig."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full canonical resource name of the DatasetConfig (e.g., projects/P/locations/L/datasetConfigs/ID)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_number` after provisioning.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn organization_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_scope` after provisioning.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn organization_scope(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_period_days` after provisioning.\nNumber of days of history that must be retained."]
    pub fn retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_folders` after provisioning.\nDefines the options for providing source folders for the DatasetConfig."]
    pub fn source_folders(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_SourceFoldersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_folders", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_projects` after provisioning.\nDefines the options for providing source projects for the DatasetConfig."]
    pub fn source_projects(&self) -> terrars::ListRef<DataStorageInsightsDatasetConfig_SourceProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_projects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix_regex`.\n"]
    pub fn set_bucket_prefix_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_prefix_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    type O =
        terrars::BlockAssignable<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {}

impl BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\n"]
    pub fn bucket_prefix_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>>,
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>,
                    >,
    ) -> Self {
        self.cloud_storage_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {}

impl BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
            cloud_storage_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {}

impl BuildDataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
            locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
        DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_Identity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageInsightsDatasetConfig_Identity {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_Identity {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_Identity {}

impl BuildDataStorageInsightsDatasetConfig_Identity {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_Identity {
        DataStorageInsightsDatasetConfig_Identity {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_IdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageInsightsDatasetConfig_IdentityRef {
        DataStorageInsightsDatasetConfig_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>>,
}

impl DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>,
                    >,
    ) -> Self {
        self.cloud_storage_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {}

impl BuildDataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
        DataStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
            cloud_storage_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
        DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {}

impl BuildDataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
        DataStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
            locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
        DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_Link {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked: std::option::Option<terrars::PrimField<bool>>,
}

impl DataStorageInsightsDatasetConfig_Link {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked`.\n"]
    pub fn set_linked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.linked = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_Link {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_Link>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_Link {}

impl BuildDataStorageInsightsDatasetConfig_Link {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_Link {
        DataStorageInsightsDatasetConfig_Link {
            dataset: core::default::Default::default(),
            linked: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_LinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_LinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataStorageInsightsDatasetConfig_LinkRef {
        DataStorageInsightsDatasetConfig_LinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_LinkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `linked` after provisioning.\n"]
    pub fn linked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_SourceFolders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder_numbers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageInsightsDatasetConfig_SourceFolders {
    #[doc = "Set the field `folder_numbers`.\n"]
    pub fn set_folder_numbers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.folder_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_SourceFolders {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_SourceFolders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_SourceFolders {}

impl BuildDataStorageInsightsDatasetConfig_SourceFolders {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_SourceFolders {
        DataStorageInsightsDatasetConfig_SourceFolders { folder_numbers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_SourceFoldersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_SourceFoldersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_SourceFoldersRef {
        DataStorageInsightsDatasetConfig_SourceFoldersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_SourceFoldersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `folder_numbers` after provisioning.\n"]
    pub fn folder_numbers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.folder_numbers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageInsightsDatasetConfig_SourceProjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_numbers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataStorageInsightsDatasetConfig_SourceProjects {
    #[doc = "Set the field `project_numbers`.\n"]
    pub fn set_project_numbers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageInsightsDatasetConfig_SourceProjects {
    type O = terrars::BlockAssignable<DataStorageInsightsDatasetConfig_SourceProjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageInsightsDatasetConfig_SourceProjects {}

impl BuildDataStorageInsightsDatasetConfig_SourceProjects {
    pub fn build(self) -> DataStorageInsightsDatasetConfig_SourceProjects {
        DataStorageInsightsDatasetConfig_SourceProjects { project_numbers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataStorageInsightsDatasetConfig_SourceProjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageInsightsDatasetConfig_SourceProjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageInsightsDatasetConfig_SourceProjectsRef {
        DataStorageInsightsDatasetConfig_SourceProjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageInsightsDatasetConfig_SourceProjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `project_numbers` after provisioning.\n"]
    pub fn project_numbers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_numbers", self.base))
    }
}
