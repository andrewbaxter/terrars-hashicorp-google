#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrDataSourcesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    backup_vault_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBackupDrDataSources_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrDataSourcesData_>,
}

#[derive(Clone)]
pub struct DataBackupDrDataSources(std::rc::Rc<DataBackupDrDataSources_>);

impl DataBackupDrDataSources {
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

    #[doc = "Set the field `filter`.\nThe filter to apply to list results."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order_by`.\nThe order to sort results by."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\nThe list of DataSources found."]
    pub fn data_sources(&self) -> terrars::ListRef<DataBackupDrDataSources_DataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to list results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `order_by` after provisioning.\nThe order to sort results by."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrDataSources {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrDataSources { }

impl terrars::ToListMappable for DataBackupDrDataSources {
    type O = terrars::ListRef<DataBackupDrDataSourcesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrDataSources_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_data_sources".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrDataSources {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub backup_vault_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrDataSources {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrDataSources {
        let out = DataBackupDrDataSources(std::rc::Rc::new(DataBackupDrDataSources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrDataSourcesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                backup_vault_id: self.backup_vault_id,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                order_by: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrDataSourcesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\n"]
    pub fn backup_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\nThe list of DataSources found."]
    pub fn data_sources(&self) -> terrars::ListRef<DataBackupDrDataSources_DataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to list results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `order_by` after provisioning.\nThe order to sort results by."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
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

impl DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
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

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
    type O =
        terrars::BlockAssignable<DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {}

impl BuildDataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
    pub fn build(self) -> DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
        DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig {
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
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef {
        DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef {
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
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_association: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan_rules: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
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

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
    type O = terrars::BlockAssignable<DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {}

impl BuildDataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
    pub fn build(self) -> DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
        DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig {
            backup_plan: core::default::Default::default(),
            backup_plan_association: core::default::Default::default(),
            backup_plan_description: core::default::Default::default(),
            backup_plan_rules: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef {
        DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef {
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
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_appliance_backup_config: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_backup_config: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_error: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_successful_backup_consistency_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSources_DataSources_BackupConfigInfo {
    #[doc = "Set the field `backup_appliance_backup_config`.\n"]
    pub fn set_backup_appliance_backup_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfig>,
                    >,
    ) -> Self {
        self.backup_appliance_backup_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_backup_config`.\n"]
    pub fn set_gcp_backup_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfig>>,
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

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources_BackupConfigInfo {
    type O = terrars::BlockAssignable<DataBackupDrDataSources_DataSources_BackupConfigInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_BackupConfigInfo {}

impl BuildDataBackupDrDataSources_DataSources_BackupConfigInfo {
    pub fn build(self) -> DataBackupDrDataSources_DataSources_BackupConfigInfo {
        DataBackupDrDataSources_DataSources_BackupConfigInfo {
            backup_appliance_backup_config: core::default::Default::default(),
            gcp_backup_config: core::default::Default::default(),
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSources_DataSources_BackupConfigInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSources_BackupConfigInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_BackupConfigInfoRef {
        DataBackupDrDataSources_DataSources_BackupConfigInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_BackupConfigInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_appliance_backup_config` after provisioning.\n"]
    pub fn backup_appliance_backup_config(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSources_DataSources_BackupConfigInfo_BackupApplianceBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_appliance_backup_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_backup_config` after provisioning.\n"]
    pub fn gcp_backup_config(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSources_DataSources_BackupConfigInfo_GcpBackupConfigRef> {
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
pub struct DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
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

impl DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
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

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
    type O = terrars::BlockAssignable<DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {}

impl BuildDataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
    pub fn build(self) -> DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
        DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication {
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
pub struct DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef {
        DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef {
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
pub struct DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
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

impl DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
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

impl terrars
::ToListMappable for DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    type O =
        terrars
        ::BlockAssignable<
            DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {}

impl BuildDataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
    pub fn build(
        self,
    ) -> DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
        DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties {
            description: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            name: core::default::Default::default(),
            total_disk_count: core::default::Default::default(),
            total_disk_size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
        DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef {
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
pub struct DataBackupDrDataSources_DataSources_DataSourceGcpResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_data_source_properties: std
    ::option
    ::Option<
        terrars
        ::ListField<DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_resourcename: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSources_DataSources_DataSourceGcpResource {
    #[doc = "Set the field `compute_instance_data_source_properties`.\n"]
    pub fn set_compute_instance_data_source_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourceProperties,
                        >,
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

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources_DataSourceGcpResource {
    type O = terrars::BlockAssignable<DataBackupDrDataSources_DataSources_DataSourceGcpResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources_DataSourceGcpResource {}

impl BuildDataBackupDrDataSources_DataSources_DataSourceGcpResource {
    pub fn build(self) -> DataBackupDrDataSources_DataSources_DataSourceGcpResource {
        DataBackupDrDataSources_DataSources_DataSourceGcpResource {
            compute_instance_data_source_properties: core::default::Default::default(),
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef {
        DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_instance_data_source_properties` after provisioning.\n"]
    pub fn compute_instance_data_source_properties(
        &self,
    ) -> terrars
    ::ListRef<DataBackupDrDataSources_DataSources_DataSourceGcpResource_ComputeInstanceDataSourcePropertiesRef> {
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

#[derive(Serialize)]
pub struct DataBackupDrDataSources_DataSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_blocked_by_vault_access_restriction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_config_info: std::option::Option<terrars::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_backup_appliance_application: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_gcp_resource: std
    ::option
    ::Option<terrars::ListField<DataBackupDrDataSources_DataSources_DataSourceGcpResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_stored_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrDataSources_DataSources {
    #[doc = "Set the field `backup_blocked_by_vault_access_restriction`.\n"]
    pub fn set_backup_blocked_by_vault_access_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.backup_blocked_by_vault_access_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_config_info`.\n"]
    pub fn set_backup_config_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupDrDataSources_DataSources_BackupConfigInfo>>,
    ) -> Self {
        self.backup_config_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_count`.\n"]
    pub fn set_backup_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config_state`.\n"]
    pub fn set_config_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.config_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_backup_appliance_application`.\n"]
    pub fn set_data_source_backup_appliance_application(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplication>,
                    >,
    ) -> Self {
        self.data_source_backup_appliance_application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_gcp_resource`.\n"]
    pub fn set_data_source_gcp_resource(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupDrDataSources_DataSources_DataSourceGcpResource>>,
    ) -> Self {
        self.data_source_gcp_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_stored_bytes`.\n"]
    pub fn set_total_stored_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_stored_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrDataSources_DataSources {
    type O = terrars::BlockAssignable<DataBackupDrDataSources_DataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrDataSources_DataSources {}

impl BuildDataBackupDrDataSources_DataSources {
    pub fn build(self) -> DataBackupDrDataSources_DataSources {
        DataBackupDrDataSources_DataSources {
            backup_blocked_by_vault_access_restriction: core::default::Default::default(),
            backup_config_info: core::default::Default::default(),
            backup_count: core::default::Default::default(),
            config_state: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source_backup_appliance_application: core::default::Default::default(),
            data_source_gcp_resource: core::default::Default::default(),
            etag: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            total_stored_bytes: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrDataSources_DataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrDataSources_DataSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupDrDataSources_DataSourcesRef {
        DataBackupDrDataSources_DataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrDataSources_DataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_blocked_by_vault_access_restriction` after provisioning.\n"]
    pub fn backup_blocked_by_vault_access_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_blocked_by_vault_access_restriction", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `backup_config_info` after provisioning.\n"]
    pub fn backup_config_info(&self) -> terrars::ListRef<DataBackupDrDataSources_DataSources_BackupConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config_info", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\n"]
    pub fn backup_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_count", self.base))
    }

    #[doc = "Get a reference to the value of field `config_state` after provisioning.\n"]
    pub fn config_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_state", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_backup_appliance_application` after provisioning.\n"]
    pub fn data_source_backup_appliance_application(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSources_DataSources_DataSourceBackupApplianceApplicationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_backup_appliance_application", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `data_source_gcp_resource` after provisioning.\n"]
    pub fn data_source_gcp_resource(
        &self,
    ) -> terrars::ListRef<DataBackupDrDataSources_DataSources_DataSourceGcpResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_gcp_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\n"]
    pub fn total_stored_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_stored_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
