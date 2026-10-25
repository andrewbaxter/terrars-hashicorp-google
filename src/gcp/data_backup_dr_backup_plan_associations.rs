#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrBackupPlanAssociationsData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBackupDrBackupPlanAssociations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrBackupPlanAssociationsData_>,
}

#[derive(Clone)]
pub struct DataBackupDrBackupPlanAssociations(std::rc::Rc<DataBackupDrBackupPlanAssociations_>);

impl DataBackupDrBackupPlanAssociations {
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

    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `associations` after provisioning.\nA list of the backup plan associations found."]
    pub fn associations(&self) -> terrars::ListRef<DataBackupDrBackupPlanAssociations_AssociationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location to list the backup plan associations from."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrBackupPlanAssociations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrBackupPlanAssociations { }

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociations {
    type O = terrars::ListRef<DataBackupDrBackupPlanAssociationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrBackupPlanAssociations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_backup_plan_associations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrBackupPlanAssociations {
    pub tf_id: std::string::String,
    #[doc = "The location to list the backup plan associations from."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrBackupPlanAssociations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrBackupPlanAssociations {
        let out = DataBackupDrBackupPlanAssociations(std::rc::Rc::new(DataBackupDrBackupPlanAssociations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrBackupPlanAssociationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrBackupPlanAssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrBackupPlanAssociationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `associations` after provisioning.\nA list of the backup plan associations found."]
    pub fn associations(&self) -> terrars::ListRef<DataBackupDrBackupPlanAssociations_AssociationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location to list the backup plan associations from."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
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

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
    type O =
        terrars::BlockAssignable<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {}

impl BuildDataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
    pub fn build(self) -> DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
        DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef {
        DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef {
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
pub struct DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_error: std
    ::option
    ::Option<terrars::ListField<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_backup_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_successful_backup_consistency_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
    #[doc = "Set the field `last_backup_error`.\n"]
    pub fn set_last_backup_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupError>,
                    >,
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

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
    type O = terrars::BlockAssignable<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {}

impl BuildDataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
    pub fn build(self) -> DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
        DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo {
            last_backup_error: core::default::Default::default(),
            last_backup_state: core::default::Default::default(),
            last_successful_backup_consistency_time: core::default::Default::default(),
            rule_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef {
        DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_backup_error` after provisioning.\n"]
    pub fn last_backup_error(
        &self,
    ) -> terrars::ListRef<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo_LastBackupErrorRef> {
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

#[derive(Serialize)]
pub struct DataBackupDrBackupPlanAssociations_Associations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_config_info: std
    ::option
    ::Option<terrars::ListField<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo>>,
}

impl DataBackupDrBackupPlanAssociations_Associations {
    #[doc = "Set the field `backup_plan`.\n"]
    pub fn set_backup_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules_config_info`.\n"]
    pub fn set_rules_config_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfo>>,
    ) -> Self {
        self.rules_config_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrBackupPlanAssociations_Associations {
    type O = terrars::BlockAssignable<DataBackupDrBackupPlanAssociations_Associations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrBackupPlanAssociations_Associations {}

impl BuildDataBackupDrBackupPlanAssociations_Associations {
    pub fn build(self) -> DataBackupDrBackupPlanAssociations_Associations {
        DataBackupDrBackupPlanAssociations_Associations {
            backup_plan: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_source: core::default::Default::default(),
            name: core::default::Default::default(),
            resource: core::default::Default::default(),
            rules_config_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrBackupPlanAssociations_AssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrBackupPlanAssociations_AssociationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupDrBackupPlanAssociations_AssociationsRef {
        DataBackupDrBackupPlanAssociations_AssociationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrBackupPlanAssociations_AssociationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\n"]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `rules_config_info` after provisioning.\n"]
    pub fn rules_config_info(
        &self,
    ) -> terrars::ListRef<DataBackupDrBackupPlanAssociations_Associations_RulesConfigInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_config_info", self.base))
    }
}
