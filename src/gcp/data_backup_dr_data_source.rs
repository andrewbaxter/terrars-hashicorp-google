#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrDataSourceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    backup_vault_id: terrars::PrimField<std::string::String>,
    data_source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBackupDrDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrDataSourceData_>,
}

#[derive(Clone)]
pub struct DataBackupDrDataSource(std::rc::Rc<DataBackupDrDataSource_>);

impl DataBackupDrDataSource {
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
        "Get a reference to the value of field `backup_blocked_by_vault_access_restriction` after provisioning.\nThis field is set to true if the backup is blocked by vault access restriction."]
    pub fn backup_blocked_by_vault_access_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_blocked_by_vault_access_restriction", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_config_info` after provisioning.\nDetails of how the resource is configured for backup."]
    pub fn backup_config_info(&self) -> terrars::ListRef<DataBackupDrDataSource_BackupConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_count` after provisioning.\nNumber of backups in the data source."]
    pub fn backup_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_state` after provisioning.\nThe backup configuration state."]
    pub fn config_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\nThe backed up resource is a backup appliance application."]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_backup_appliance_application", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\nThe backed up resource is a Google Cloud resource.\n\t\tThe word 'DataSource' was included in the names to indicate that this is\n\t\tthe representation of the Google Cloud resource used within the\n\t\tDataSource object."]
    pub fn data_source_gcp_resource(&self) -> terrars::ListRef<DataBackupDrDataSource_DataSourceGcpResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_gcp_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the datasource to create.\n\t\tIt must have the format \"projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource}\".\n\t\t'{datasource}' cannot be changed after creation. It must be between 3-63 characters long and must be unique within the backup vault."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe DataSource resource instance state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_stored_bytes` after provisioning.\nThe number of bytes (metadata and data) stored in this datasource."]
    pub fn total_stored_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_stored_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrDataSource {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrDataSource { }

impl terrars::ToListMappable for DataBackupDrDataSource {
    type O = terrars::ListRef<DataBackupDrDataSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrDataSource_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_data_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrDataSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub backup_vault_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub data_source_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrDataSource {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrDataSource {
        let out = DataBackupDrDataSource(std::rc::Rc::new(DataBackupDrDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrDataSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                backup_vault_id: self.backup_vault_id,
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrDataSourceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `backup_blocked_by_vault_access_restriction` after provisioning.\nThis field is set to true if the backup is blocked by vault access restriction."]
    pub fn backup_blocked_by_vault_access_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_blocked_by_vault_access_restriction", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_config_info` after provisioning.\nDetails of how the resource is configured for backup."]
    pub fn backup_config_info(&self) -> terrars::ListRef<DataBackupDrDataSource_BackupConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_count` after provisioning.\nNumber of backups in the data source."]
    pub fn backup_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_state` after provisioning.\nThe backup configuration state."]
    pub fn config_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\nThe backed up resource is a backup appliance application."]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_backup_appliance_application", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\nThe backed up resource is a Google Cloud resource.\n\t\tThe word 'DataSource' was included in the names to indicate that this is\n\t\tthe representation of the Google Cloud resource used within the\n\t\tDataSource object."]
    pub fn data_source_gcp_resource(&self) -> terrars::ListRef<DataBackupDrDataSource_DataSourceGcpResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_gcp_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the datasource to create.\n\t\tIt must have the format \"projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource}\".\n\t\t'{datasource}' cannot be changed after creation. It must be between 3-63 characters long and must be unique within the backup vault."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe DataSource resource instance state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_stored_bytes` after provisioning.\nThe number of bytes (metadata and data) stored in this datasource."]
    pub fn total_stored_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_stored_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_appliance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_appliance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sla_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slp_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slt_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
    #[doc = "Set the field `application_name`.\n"]
    pub fn set_application_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_appliance_id`.\n"]
    pub fn set_backup_appliance_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_appliance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_appliance_name`.\n"]
    pub fn set_backup_appliance_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_appliance_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_name`.\n"]
    pub fn set_host_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sla_id`.\n"]
    pub fn set_sla_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sla_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slp_name`.\n"]
    pub fn set_slp_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slp_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slt_name`.\n"]
    pub fn set_slt_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slt_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
    type O = terrars::BlockAssignable<DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {}

impl BuildDataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
    pub fn build(self) -> DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
        DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig {
            application_name: core::default::Default::default(),
            backup_appliance_id: core::default::Default::default(),
            backup_appliance_name: core::default::Default::default(),
            host_name: core::default::Default::default(),
            sla_id: core::default::Default::default(),
            slp_name: core::default::Default::default(),
            slt_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef {
        DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_appliance_id` after provisioning.\n"]
    pub fn backup_appliance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_appliance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_appliance_name` after provisioning.\n"]
    pub fn backup_appliance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_appliance_name", self.base))
    }

    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc = "Get a reference to the value of field `sla_id` after provisioning.\n"]
    pub fn sla_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sla_id", self.base))
    }

    #[doc = "Get a reference to the value of field `slp_name` after provisioning.\n"]
    pub fn slp_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slp_name", self.base))
    }

    #[doc = "Get a reference to the value of field `slt_name` after provisioning.\n"]
    pub fn slt_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slt_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_association: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_rules: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
    #[doc = "Set the field `backup_plan`.\n"]
    pub fn set_backup_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_plan_association`.\n"]
    pub fn set_backup_plan_association(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_plan_association = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_plan_description`.\n"]
    pub fn set_backup_plan_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_plan_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_plan_rules`.\n"]
    pub fn set_backup_plan_rules(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.backup_plan_rules = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
    type O = terrars::BlockAssignable<DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {}

impl BuildDataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
    pub fn build(self) -> DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
        DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig {
            backup_plan: core::default::Default::default(),
            backup_plan_association: core::default::Default::default(),
            backup_plan_description: core::default::Default::default(),
            backup_plan_rules: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef {
        DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\n"]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_plan_association` after provisioning.\n"]
    pub fn backup_plan_association(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_association", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_plan_description` after provisioning.\n"]
    pub fn backup_plan_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_description", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_plan_rules` after provisioning.\n"]
    pub fn backup_plan_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_plan_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_BackupConfigInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_appliance_backup_config: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_backup_config: std::option::Option<terrars::ListField<DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_error: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_successful_backup_consistency_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSource_BackupConfigInfo {
    #[doc = "Set the field `backup_appliance_backup_config`.\n"]
    pub fn set_backup_appliance_backup_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfig>>,
    ) -> Self {
        self.backup_appliance_backup_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_backup_config`.\n"]
    pub fn set_gcp_backup_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfig>>,
    ) -> Self {
        self.gcp_backup_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.last_backup_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_backup_state`.\n"]
    pub fn set_last_backup_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_backup_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_successful_backup_consistency_time`.\n"]
    pub fn set_last_successful_backup_consistency_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_successful_backup_consistency_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_BackupConfigInfo {
    type O = terrars::BlockAssignable<DataBackupDrDataSource_BackupConfigInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_BackupConfigInfo {}

impl BuildDataBackupDrDataSource_BackupConfigInfo {
    pub fn build(self) -> DataBackupDrDataSource_BackupConfigInfo {
        DataBackupDrDataSource_BackupConfigInfo {
            backup_appliance_backup_config: core::default::Default::default(),
            gcp_backup_config: core::default::Default::default(),
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_BackupConfigInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_BackupConfigInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupDrDataSource_BackupConfigInfoRef {
        DataBackupDrDataSource_BackupConfigInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_BackupConfigInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_appliance_backup_config` after provisioning.\n"]
    pub fn backup_appliance_backup_config(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSource_BackupConfigInfo_BackupApplianceBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_appliance_backup_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_backup_config` after provisioning.\n"]
    pub fn gcp_backup_config(&self) -> terrars::ListRef<DataBackupDrDataSource_BackupConfigInfo_GcpBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_backup_config", self.base))
    }

    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.last_backup_error", self.base))
    }

    #[doc = "Get a reference to the value of field `last_backup_state` after provisioning.\n"]
    pub fn last_backup_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_backup_state", self.base))
    }

    #[doc = "Get a reference to the value of field `last_successful_backup_consistency_time` after provisioning.\n"]
    pub fn last_successful_backup_consistency_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_backup_consistency_time", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_DataSourceBackupApplianceApplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    appliance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_appliance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSource_DataSourceBackupApplianceApplication {
    #[doc = "Set the field `appliance_id`.\n"]
    pub fn set_appliance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.appliance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `application_id`.\n"]
    pub fn set_application_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `application_name`.\n"]
    pub fn set_application_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_appliance`.\n"]
    pub fn set_backup_appliance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_appliance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_DataSourceBackupApplianceApplication {
    type O = terrars::BlockAssignable<DataBackupDrDataSource_DataSourceBackupApplianceApplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_DataSourceBackupApplianceApplication {}

impl BuildDataBackupDrDataSource_DataSourceBackupApplianceApplication {
    pub fn build(self) -> DataBackupDrDataSource_DataSourceBackupApplianceApplication {
        DataBackupDrDataSource_DataSourceBackupApplianceApplication {
            appliance_id: core::default::Default::default(),
            application_id: core::default::Default::default(),
            application_name: core::default::Default::default(),
            backup_appliance: core::default::Default::default(),
            host_id: core::default::Default::default(),
            hostname: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef {
        DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_DataSourceBackupApplianceApplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `appliance_id` after provisioning.\n"]
    pub fn appliance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.appliance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_appliance` after provisioning.\n"]
    pub fn backup_appliance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_appliance", self.base))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_disk_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_disk_count`.\n"]
    pub fn set_total_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_disk_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_disk_size_gb`.\n"]
    pub fn set_total_disk_size_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_disk_size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    type O =
        terrars::BlockAssignable<DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {}

impl BuildDataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    pub fn build(self) -> DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
        DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
            description: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            name: core::default::Default::default(),
            total_disk_count: core::default::Default::default(),
            total_disk_size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
        DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `total_disk_count` after provisioning.\n"]
    pub fn total_disk_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_disk_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_disk_size_gb` after provisioning.\n"]
    pub fn total_disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_disk_size_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSource_DataSourceGcpResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_data_source_properties: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_resourcename: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSource_DataSourceGcpResource {
    #[doc = "Set the field `compute_instance_data_source_properties`.\n"]
    pub fn set_compute_instance_data_source_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourceProperties>,
                    >,
    ) -> Self {
        self.compute_instance_data_source_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_resourcename`.\n"]
    pub fn set_gcp_resourcename(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcp_resourcename = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSource_DataSourceGcpResource {
    type O = terrars::BlockAssignable<DataBackupDrDataSource_DataSourceGcpResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSource_DataSourceGcpResource {}

impl BuildDataBackupDrDataSource_DataSourceGcpResource {
    pub fn build(self) -> DataBackupDrDataSource_DataSourceGcpResource {
        DataBackupDrDataSource_DataSourceGcpResource {
            compute_instance_data_source_properties: core::default::Default::default(),
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSource_DataSourceGcpResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSource_DataSourceGcpResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSource_DataSourceGcpResourceRef {
        DataBackupDrDataSource_DataSourceGcpResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSource_DataSourceGcpResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_instance_data_source_properties` after provisioning.\n"]
    pub fn compute_instance_data_source_properties(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSource_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_data_source_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gcp_resourcename` after provisioning.\n"]
    pub fn gcp_resourcename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_resourcename", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
