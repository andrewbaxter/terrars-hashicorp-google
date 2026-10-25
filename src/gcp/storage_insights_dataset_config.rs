#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageInsightsDatasetConfigData_ {
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
    activity_data_retention_period_days: std::option::Option<terrars::PrimField<f64>>,
    dataset_config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_newly_created_buckets: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    link_dataset: std::option::Option<terrars::PrimField<bool>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_scope: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    retention_period_days: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_cloud_storage_locations: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_ExcludeCloudStorageLocations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<std::vec::Vec<StorageInsightsDatasetConfig_Identity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_cloud_storage_locations: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_IncludeCloudStorageLocations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_folders: std::option::Option<std::vec::Vec<StorageInsightsDatasetConfig_SourceFolders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_projects: std::option::Option<std::vec::Vec<StorageInsightsDatasetConfig_SourceProjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageInsightsDatasetConfig_Timeouts>,
    dynamic: StorageInsightsDatasetConfigDynamic,
}

struct StorageInsightsDatasetConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageInsightsDatasetConfigData_>,
}

#[derive(Clone)]
pub struct StorageInsightsDatasetConfig(std::rc::Rc<StorageInsightsDatasetConfig_>);

impl StorageInsightsDatasetConfig {
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
        "Set the field `activity_data_retention_period_days`.\nNumber of days of activity data that must be retained. If not specified, retentionPeriodDays will be used. Set to 0 to turn off the activity data."]
    pub fn set_activity_data_retention_period_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().activity_data_retention_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional user-provided description for the dataset configuration with a maximum length of 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_newly_created_buckets`.\nIf set to true, the request includes all the newly created buckets in the dataset that meet the inclusion and exclusion rules."]
    pub fn set_include_newly_created_buckets(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_newly_created_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `link_dataset`.\nA boolean terraform only flag to link/unlink dataset.\n\nSetting this field to true while creation will automatically link the created dataset as an additional functionality.\n-> **Note** A dataset config resource can only be destroyed once it is unlinked,\nso users must set this field to false to unlink the dataset and destroy the dataset config resource."]
    pub fn set_link_dataset(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().link_dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organization_number`.\nOrganization resource ID that the source projects should belong to.\nProjects that do not belong to the provided organization are not considered when creating the dataset."]
    pub fn set_organization_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().organization_number = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organization_scope`.\nDefines the options for providing a source organization for the DatasetConfig."]
    pub fn set_organization_scope(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().organization_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_cloud_storage_buckets`.\n"]
    pub fn set_exclude_cloud_storage_buckets(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_cloud_storage_locations`.\n"]
    pub fn set_exclude_cloud_storage_locations(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageLocations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_cloud_storage_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_cloud_storage_locations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_Identity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_cloud_storage_buckets`.\n"]
    pub fn set_include_cloud_storage_buckets(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_cloud_storage_locations`.\n"]
    pub fn set_include_cloud_storage_locations(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageLocations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_cloud_storage_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_cloud_storage_locations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_folders`.\n"]
    pub fn set_source_folders(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_SourceFolders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_folders = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_folders = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_projects`.\n"]
    pub fn set_source_projects(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsDatasetConfig_SourceProjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_projects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_projects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageInsightsDatasetConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn link(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_LinkRef> {
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
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\n"]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\n"]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\n"]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\n"]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\n"]
    pub fn source_folders(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_SourceFoldersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_folders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\n"]
    pub fn source_projects(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_SourceProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_projects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsDatasetConfig_TimeoutsRef {
        StorageInsightsDatasetConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StorageInsightsDatasetConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageInsightsDatasetConfig { }

impl terrars::ToListMappable for StorageInsightsDatasetConfig {
    type O = terrars::ListRef<StorageInsightsDatasetConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageInsightsDatasetConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_insights_dataset_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageInsightsDatasetConfig {
    pub tf_id: std::string::String,
    #[doc = "The user-defined ID of the DatasetConfig"]
    pub dataset_config_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the DatasetConfig."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Number of days of history that must be retained."]
    pub retention_period_days: terrars::PrimField<f64>,
}

impl BuildStorageInsightsDatasetConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageInsightsDatasetConfig {
        let out = StorageInsightsDatasetConfig(std::rc::Rc::new(StorageInsightsDatasetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageInsightsDatasetConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                activity_data_retention_period_days: core::default::Default::default(),
                dataset_config_id: self.dataset_config_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                include_newly_created_buckets: core::default::Default::default(),
                link_dataset: core::default::Default::default(),
                location: self.location,
                organization_number: core::default::Default::default(),
                organization_scope: core::default::Default::default(),
                project: core::default::Default::default(),
                retention_period_days: self.retention_period_days,
                exclude_cloud_storage_buckets: core::default::Default::default(),
                exclude_cloud_storage_locations: core::default::Default::default(),
                identity: core::default::Default::default(),
                include_cloud_storage_buckets: core::default::Default::default(),
                include_cloud_storage_locations: core::default::Default::default(),
                source_folders: core::default::Default::default(),
                source_projects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageInsightsDatasetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageInsightsDatasetConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn link(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_LinkRef> {
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
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe UTC time at which the DatasetConfig was updated. This is auto-populated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_cloud_storage_buckets` after provisioning.\n"]
    pub fn exclude_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_cloud_storage_locations` after provisioning.\n"]
    pub fn exclude_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_cloud_storage_buckets` after provisioning.\n"]
    pub fn include_cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_cloud_storage_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_cloud_storage_locations` after provisioning.\n"]
    pub fn include_cloud_storage_locations(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.include_cloud_storage_locations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_folders` after provisioning.\n"]
    pub fn source_folders(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_SourceFoldersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_folders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_projects` after provisioning.\n"]
    pub fn source_projects(&self) -> terrars::ListRef<StorageInsightsDatasetConfig_SourceProjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_projects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsDatasetConfig_TimeoutsRef {
        StorageInsightsDatasetConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsDatasetConfigDynamic {
    exclude_cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets>>,
    exclude_cloud_storage_locations: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_ExcludeCloudStorageLocations>>,
    identity: std::option::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_Identity>>,
    include_cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets>>,
    include_cloud_storage_locations: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_IncludeCloudStorageLocations>>,
    source_folders: std::option::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_SourceFolders>>,
    source_projects: std::option::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_SourceProjects>>,
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_Link {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked: std::option::Option<terrars::PrimField<bool>>,
}

impl StorageInsightsDatasetConfig_Link {
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

impl terrars::ToListMappable for StorageInsightsDatasetConfig_Link {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_Link>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_Link {}

impl BuildStorageInsightsDatasetConfig_Link {
    pub fn build(self) -> StorageInsightsDatasetConfig_Link {
        StorageInsightsDatasetConfig_Link {
            dataset: core::default::Default::default(),
            linked: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_LinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_LinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsDatasetConfig_LinkRef {
        StorageInsightsDatasetConfig_LinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_LinkRef {
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
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    #[doc =
        "Set the field `bucket_name`.\nThe list of cloud storage bucket names to exclude in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bucket_prefix_regex`.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_prefix_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_prefix_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {}

impl BuildStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
    pub fn build(self) -> StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
        StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
        StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe list of cloud storage bucket names to exclude in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_prefix_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsDynamic {
    cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>>,
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>>,
    dynamic: StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsDynamic,
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBuckets>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {}

impl BuildStorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
    pub fn build(self) -> StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
        StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets {
            cloud_storage_buckets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
        StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_ExcludeCloudStorageBuckets_CloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageLocations { }

impl terrars::ToListMappable for StorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_ExcludeCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    #[doc = "The list of cloud storage locations to exclude in the DatasetConfig."]
    pub locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
    pub fn build(self) -> StorageInsightsDatasetConfig_ExcludeCloudStorageLocations {
        StorageInsightsDatasetConfig_ExcludeCloudStorageLocations { locations: self.locations }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
        StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_ExcludeCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `locations` after provisioning.\nThe list of cloud storage locations to exclude in the DatasetConfig."]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_Identity {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl StorageInsightsDatasetConfig_Identity { }

impl terrars::ToListMappable for StorageInsightsDatasetConfig_Identity {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_Identity {
    #[doc =
        "Type of identity to use for the DatasetConfig. Possible values: [\"IDENTITY_TYPE_PER_CONFIG\", \"IDENTITY_TYPE_PER_PROJECT\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsDatasetConfig_Identity {
    pub fn build(self) -> StorageInsightsDatasetConfig_Identity {
        StorageInsightsDatasetConfig_Identity { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_IdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsDatasetConfig_IdentityRef {
        StorageInsightsDatasetConfig_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the identity."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of identity to use for the DatasetConfig. Possible values: [\"IDENTITY_TYPE_PER_CONFIG\", \"IDENTITY_TYPE_PER_PROJECT\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
    #[doc =
        "Set the field `bucket_name`.\nThe list of cloud storage bucket names to include in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bucket_prefix_regex`.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn set_bucket_prefix_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_prefix_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {}

impl BuildStorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
    pub fn build(self) -> StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
        StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets {
            bucket_name: core::default::Default::default(),
            bucket_prefix_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef {
        StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe list of cloud storage bucket names to include in the DatasetConfig.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bucket_prefix_regex` after provisioning.\nThe list of regex patterns for bucket names matching the regex.\nRegex should follow the syntax specified in google/re2 on GitHub.\nExactly one of the bucket_name and bucket_prefix_regex should be specified."]
    pub fn bucket_prefix_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsDatasetConfig_IncludeCloudStorageBucketsDynamic {
    cloud_storage_buckets: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets>>,
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_buckets: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets>>,
    dynamic: StorageInsightsDatasetConfig_IncludeCloudStorageBucketsDynamic,
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    #[doc = "Set the field `cloud_storage_buckets`.\n"]
    pub fn set_cloud_storage_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBuckets>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_buckets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {}

impl BuildStorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
    pub fn build(self) -> StorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
        StorageInsightsDatasetConfig_IncludeCloudStorageBuckets {
            cloud_storage_buckets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
        StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_buckets` after provisioning.\n"]
    pub fn cloud_storage_buckets(
        &self,
    ) -> terrars::ListRef<StorageInsightsDatasetConfig_IncludeCloudStorageBuckets_CloudStorageBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageLocations { }

impl terrars::ToListMappable for StorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_IncludeCloudStorageLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    #[doc = "The list of cloud storage locations to include in the DatasetConfig."]
    pub locations: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageInsightsDatasetConfig_IncludeCloudStorageLocations {
    pub fn build(self) -> StorageInsightsDatasetConfig_IncludeCloudStorageLocations {
        StorageInsightsDatasetConfig_IncludeCloudStorageLocations { locations: self.locations }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
        StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_IncludeCloudStorageLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `locations` after provisioning.\nThe list of cloud storage locations to include in the DatasetConfig."]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_SourceFolders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder_numbers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageInsightsDatasetConfig_SourceFolders {
    #[doc = "Set the field `folder_numbers`.\nThe list of folder numbers to include in the DatasetConfig."]
    pub fn set_folder_numbers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.folder_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_SourceFolders {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_SourceFolders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_SourceFolders {}

impl BuildStorageInsightsDatasetConfig_SourceFolders {
    pub fn build(self) -> StorageInsightsDatasetConfig_SourceFolders {
        StorageInsightsDatasetConfig_SourceFolders { folder_numbers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_SourceFoldersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_SourceFoldersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsDatasetConfig_SourceFoldersRef {
        StorageInsightsDatasetConfig_SourceFoldersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_SourceFoldersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `folder_numbers` after provisioning.\nThe list of folder numbers to include in the DatasetConfig."]
    pub fn folder_numbers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.folder_numbers", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_SourceProjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_numbers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl StorageInsightsDatasetConfig_SourceProjects {
    #[doc = "Set the field `project_numbers`.\nThe list of project numbers to include in the DatasetConfig."]
    pub fn set_project_numbers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_SourceProjects {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_SourceProjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_SourceProjects {}

impl BuildStorageInsightsDatasetConfig_SourceProjects {
    pub fn build(self) -> StorageInsightsDatasetConfig_SourceProjects {
        StorageInsightsDatasetConfig_SourceProjects { project_numbers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_SourceProjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_SourceProjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsDatasetConfig_SourceProjectsRef {
        StorageInsightsDatasetConfig_SourceProjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_SourceProjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_numbers` after provisioning.\nThe list of project numbers to include in the DatasetConfig."]
    pub fn project_numbers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_numbers", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsDatasetConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsDatasetConfig_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsDatasetConfig_Timeouts {
    type O = terrars::BlockAssignable<StorageInsightsDatasetConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsDatasetConfig_Timeouts {}

impl BuildStorageInsightsDatasetConfig_Timeouts {
    pub fn build(self) -> StorageInsightsDatasetConfig_Timeouts {
        StorageInsightsDatasetConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsDatasetConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsDatasetConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsDatasetConfig_TimeoutsRef {
        StorageInsightsDatasetConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsDatasetConfig_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
