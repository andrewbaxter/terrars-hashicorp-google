#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrBackupPlanAssociationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    backup_plan_association_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBackupDrBackupPlanAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrBackupPlanAssociationData_>,
}

#[derive(Clone)]
pub struct DataBackupDrBackupPlanAssociation(std::rc::Rc<DataBackupDrBackupPlanAssociation_>);

impl DataBackupDrBackupPlanAssociation {
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
        "Get a reference to the value of field `backup_plan` after provisioning.\nThe BP with which resource needs to be created\nNote:\n- A Backup Plan configured for 'compute.googleapis.com/Instance', can only protect instance type resources.\n- A Backup Plan configured for 'compute.googleapis.com/Disk' can be used to protect both standard Disks and Regional Disks resources.\n- A Backup Plan configured for 'file.googleapis.com/Instance' can only protect Filestore instances.\n- A Backup Plan configured for 'sqladmin.googleapis.com/Instance' can only protect Cloud SQL instances."]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_plan_association_id` after provisioning.\nThe id of backupplan association"]
    pub fn backup_plan_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_association_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nResource name of data source which will be used as storage location for backups taken"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the backupplan association"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan association resource created"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nThe resource for which BPA needs to be created"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backupplan is applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"compute.googleapis.com/RegionDisk\", and \"file.googleapis.com/Instance\""]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules_config_info` after provisioning.\nMessage for rules config info"]
    pub fn rules_config_info(&self) -> terrars::ListRef<DataBackupDrBackupPlanAssociation_RulesConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_config_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrBackupPlanAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrBackupPlanAssociation { }

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociation {
    type O = terrars::ListRef<DataBackupDrBackupPlanAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrBackupPlanAssociation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_backup_plan_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrBackupPlanAssociation {
    pub tf_id: std::string::String,
    #[doc = "The id of backupplan association"]
    pub backup_plan_association_id: terrars::PrimField<std::string::String>,
    #[doc = "The location for the backupplan association"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrBackupPlanAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrBackupPlanAssociation {
        let out = DataBackupDrBackupPlanAssociation(std::rc::Rc::new(DataBackupDrBackupPlanAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrBackupPlanAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                backup_plan_association_id: self.backup_plan_association_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrBackupPlanAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrBackupPlanAssociationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `backup_plan` after provisioning.\nThe BP with which resource needs to be created\nNote:\n- A Backup Plan configured for 'compute.googleapis.com/Instance', can only protect instance type resources.\n- A Backup Plan configured for 'compute.googleapis.com/Disk' can be used to protect both standard Disks and Regional Disks resources.\n- A Backup Plan configured for 'file.googleapis.com/Instance' can only protect Filestore instances.\n- A Backup Plan configured for 'sqladmin.googleapis.com/Instance' can only protect Cloud SQL instances."]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_plan_association_id` after provisioning.\nThe id of backupplan association"]
    pub fn backup_plan_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_association_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nResource name of data source which will be used as storage location for backups taken"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the backupplan association"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan association resource created"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nThe resource for which BPA needs to be created"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of workload on which backupplan is applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"compute.googleapis.com/RegionDisk\", and \"file.googleapis.com/Instance\""]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules_config_info` after provisioning.\nMessage for rules config info"]
    pub fn rules_config_info(&self) -> terrars::ListRef<DataBackupDrBackupPlanAssociation_RulesConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_config_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
    type O = terrars::BlockAssignable<DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {}

impl BuildDataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
    pub fn build(self) -> DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
        DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef {
        DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociation_RulesConfigInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_error: std
    ::option
    ::Option<terrars::ListField<DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_successful_backup_consistency_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrBackupPlanAssociation_RulesConfigInfo {
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupError>>,
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

    #[doc = "Set the field `rule_id`.\n"]
    pub fn set_rule_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociation_RulesConfigInfo {
    type O = terrars::BlockAssignable<DataBackupDrBackupPlanAssociation_RulesConfigInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackupPlanAssociation_RulesConfigInfo {}

impl BuildDataBackupDrBackupPlanAssociation_RulesConfigInfo {
    pub fn build(self) -> DataBackupDrBackupPlanAssociation_RulesConfigInfo {
        DataBackupDrBackupPlanAssociation_RulesConfigInfo {
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
            rule_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackupPlanAssociation_RulesConfigInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociation_RulesConfigInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackupPlanAssociation_RulesConfigInfoRef {
        DataBackupDrBackupPlanAssociation_RulesConfigInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackupPlanAssociation_RulesConfigInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(
        &self,
    ) -> terrars::ListRef<DataBackupDrBackupPlanAssociation_RulesConfigInfo_LastBackupErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_backup_error", self.base))
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

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }
}
