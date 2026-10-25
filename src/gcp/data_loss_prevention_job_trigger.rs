#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionJobTriggerData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_job: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionJobTrigger_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_Triggers>>,
    dynamic: DataLossPreventionJobTriggerDynamic,
}

struct DataLossPreventionJobTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionJobTriggerData_>,
}

#[derive(Clone)]
pub struct DataLossPreventionJobTrigger(std::rc::Rc<DataLossPreventionJobTrigger_>);

impl DataLossPreventionJobTrigger {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the job trigger."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser set display name of the job trigger."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nWhether the trigger is currently active. Default value: \"HEALTHY\" Possible values: [\"PAUSED\", \"HEALTHY\", \"CANCELLED\"]"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trigger_id`.\nThe trigger id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+.\nThe maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub fn set_trigger_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().trigger_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_job`.\n"]
    pub fn set_inspect_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inspect_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inspect_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionJobTrigger_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().triggers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.triggers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of an inspectTemplate. Set by the server."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the job trigger."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the job trigger."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_run_time` after provisioning.\nThe timestamp of the last time this trigger executed."]
    pub fn last_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job trigger. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the trigger, either in the format 'projects/{{project}}'\nor 'projects/{{project}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nWhether the trigger is currently active. Default value: \"HEALTHY\" Possible values: [\"PAUSED\", \"HEALTHY\", \"CANCELLED\"]"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_id` after provisioning.\nThe trigger id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+.\nThe maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub fn trigger_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of an inspectTemplate. Set by the server."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_job` after provisioning.\n"]
    pub fn inspect_job(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionJobTrigger_TimeoutsRef {
        DataLossPreventionJobTrigger_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_TriggersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

impl terrars::Referable for DataLossPreventionJobTrigger {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLossPreventionJobTrigger { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger {
    type O = terrars::ListRef<DataLossPreventionJobTriggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLossPreventionJobTrigger_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_loss_prevention_job_trigger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLossPreventionJobTrigger {
    pub tf_id: std::string::String,
    #[doc =
        "The parent of the trigger, either in the format 'projects/{{project}}'\nor 'projects/{{project}}/locations/{{location}}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLossPreventionJobTrigger {
        let out = DataLossPreventionJobTrigger(std::rc::Rc::new(DataLossPreventionJobTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLossPreventionJobTriggerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                status: core::default::Default::default(),
                trigger_id: core::default::Default::default(),
                inspect_job: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                triggers: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLossPreventionJobTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLossPreventionJobTriggerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of an inspectTemplate. Set by the server."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the job trigger."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the job trigger."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_run_time` after provisioning.\nThe timestamp of the last time this trigger executed."]
    pub fn last_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the job trigger. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the trigger, either in the format 'projects/{{project}}'\nor 'projects/{{project}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nWhether the trigger is currently active. Default value: \"HEALTHY\" Possible values: [\"PAUSED\", \"HEALTHY\", \"CANCELLED\"]"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_id` after provisioning.\nThe trigger id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+.\nThe maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub fn trigger_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of an inspectTemplate. Set by the server."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_job` after provisioning.\n"]
    pub fn inspect_job(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionJobTrigger_TimeoutsRef {
        DataLossPreventionJobTrigger_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_TriggersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerDynamic {
    inspect_job: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob>>,
    triggers: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_Triggers>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_redact_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    structured_deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
    #[doc =
        "Set the field `deidentify_template`.\nIf this template is specified, it will serve as the default de-identify template."]
    pub fn set_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_redact_template`.\nIf this template is specified, it will serve as the de-identify template for images."]
    pub fn set_image_redact_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_redact_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `structured_deidentify_template`.\nIf this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables."]
    pub fn set_structured_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.structured_deidentify_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig {
            deidentify_template: core::default::Default::default(),
            image_redact_template: core::default::Default::default(),
            structured_deidentify_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\nIf this template is specified, it will serve as the default de-identify template."]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_redact_template` after provisioning.\nIf this template is specified, it will serve as the de-identify template for images."]
    pub fn image_redact_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_redact_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `structured_deidentify_template` after provisioning.\nIf this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables."]
    pub fn structured_deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.structured_deidentify_template", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
    #[doc =
        "Set the field `table_id`.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigDynamic {
    table: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_Table,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig {
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyDynamic {
    transformation_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig>>,
    transformation_details_storage_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
    cloud_storage_output: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_types_to_transform: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_details_storage_config: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
    #[doc =
        "Set the field `file_types_to_transform`.\nList of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed.\n\nIf empty, all supported files will be transformed. Supported types may be automatically added over time.\n\nIf a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started. Possible values: [\"IMAGE\", \"TEXT_FILE\", \"CSV\", \"TSV\"]"]
    pub fn set_file_types_to_transform(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_types_to_transform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transformation_config`.\n"]
    pub fn set_transformation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transformation_details_storage_config`.\n"]
    pub fn set_transformation_details_storage_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_details_storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_details_storage_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
    #[doc =
        "User settable Cloud Storage bucket and folders to store de-identified files.\n\nThis field must be set for cloud storage deidentification.\n\nThe output Cloud Storage bucket must be different from the input bucket.\n\nDe-identified files will overwrite files in the output path.\n\nForm of: gs://bucket/folder/ or gs://bucket"]
    pub cloud_storage_output: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
        DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify {
            cloud_storage_output: self.cloud_storage_output,
            file_types_to_transform: core::default::Default::default(),
            transformation_config: core::default::Default::default(),
            transformation_details_storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_storage_output` after provisioning.\nUser settable Cloud Storage bucket and folders to store de-identified files.\n\nThis field must be set for cloud storage deidentification.\n\nThe output Cloud Storage bucket must be different from the input bucket.\n\nDe-identified files will overwrite files in the output path.\n\nForm of: gs://bucket/folder/ or gs://bucket"]
    pub fn cloud_storage_output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_storage_output", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_types_to_transform` after provisioning.\nList of user-specified file type groups to transform. If specified, only the files with these filetypes will be transformed.\n\nIf empty, all supported files will be transformed. Supported types may be automatically added over time.\n\nIf a file type is set in this field that isn't supported by the Deidentify action then the job will fail and will not be successfully created/started. Possible values: [\"IMAGE\", \"TEXT_FILE\", \"CSV\", \"TSV\"]"]
    pub fn file_types_to_transform(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_types_to_transform", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_config` after provisioning.\n"]
    pub fn transformation_config(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_details_storage_config` after provisioning.\n"]
    pub fn transformation_details_storage_config(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify_TransformationDetailsStorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_details_storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {}

impl DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {
        DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PubSub {
    topic: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PubSub { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_PubSub {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PubSub>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_PubSub {
    #[doc = "Cloud Pub/Sub topic to send notifications to."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_PubSub {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_PubSub {
        DataLossPreventionJobTrigger_InspectJob_Actions_PubSub { topic: self.topic }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nCloud Pub/Sub topic to send notifications to."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath { }

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath {
    #[doc =
        "A URL representing a file or path (no wildcards) in Cloud Storage.\nExample: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath { path: self.path }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA URL representing a file or path (no wildcards) in Cloud Storage.\nExample: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
    #[doc =
        "Set the field `table_id`.\nName of the table. If is not set a new one will be generated for you with the following format:\n'dlp_googleapis_yyyy_mm_dd_[dlp_job_id]'. Pacific timezone will be used for generating the date details."]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
    #[doc = "Dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset ID of the table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nName of the table. If is not set a new one will be generated for you with the following format:\n'dlp_googleapis_yyyy_mm_dd_[dlp_job_id]'. Pacific timezone will be used for generating the date details."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigDynamic {
    storage_path: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath>,
    >,
    table: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_path: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
    #[doc =
        "Set the field `output_schema`.\nSchema used for writing the findings for Inspect jobs. This field is only used for\nInspect and must be unspecified for Risk jobs. Columns are derived from the Finding\nobject. If appending to an existing table, any columns from the predefined schema\nthat are missing will be added. No columns in the existing table will be deleted.\n\nIf unspecified, then all available columns will be used for a new table or an (existing)\ntable with no schema, and no changes will be made to an existing table that has a schema.\nOnly for use with external storage. Possible values: [\"BASIC_COLUMNS\", \"GCS_COLUMNS\", \"DATASTORE_COLUMNS\", \"BIG_QUERY_COLUMNS\", \"ALL_COLUMNS\"]"]
    pub fn set_output_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_path`.\n"]
    pub fn set_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePath,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_Table,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig {
            output_schema: core::default::Default::default(),
            storage_path: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_schema` after provisioning.\nSchema used for writing the findings for Inspect jobs. This field is only used for\nInspect and must be unspecified for Risk jobs. Columns are derived from the Finding\nobject. If appending to an existing table, any columns from the predefined schema\nthat are missing will be added. No columns in the existing table will be deleted.\n\nIf unspecified, then all available columns will be used for a new table or an (existing)\ntable with no schema, and no changes will be made to an existing table that has a schema.\nOnly for use with external storage. Possible values: [\"BASIC_COLUMNS\", \"GCS_COLUMNS\", \"DATASTORE_COLUMNS\", \"BIG_QUERY_COLUMNS\", \"ALL_COLUMNS\"]"]
    pub fn output_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_path` after provisioning.\n"]
    pub fn storage_path(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_StoragePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsDynamic {
    output_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings {
            output_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef {
        DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_ActionsDynamic {
    deidentify: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify>>,
    job_notification_emails: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails>>,
    pub_sub: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_PubSub>>,
    publish_findings_to_dataplex_catalog: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog>>,
    publish_summary_to_cscc: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc>>,
    publish_to_stackdriver: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver>>,
    save_findings: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_Actions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_notification_emails: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub_sub: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_PubSub>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_findings_to_dataplex_catalog: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_summary_to_cscc: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_stackdriver: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    save_findings: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_ActionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_Actions {
    #[doc = "Set the field `deidentify`.\n"]
    pub fn set_deidentify(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_Deidentify>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deidentify = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deidentify = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_notification_emails`.\n"]
    pub fn set_job_notification_emails(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmails>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_notification_emails = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_notification_emails = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pub_sub`.\n"]
    pub fn set_pub_sub(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PubSub>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pub_sub = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pub_sub = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_findings_to_dataplex_catalog`.\n"]
    pub fn set_publish_findings_to_dataplex_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_findings_to_dataplex_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_findings_to_dataplex_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_summary_to_cscc`.\n"]
    pub fn set_publish_summary_to_cscc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCscc>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_summary_to_cscc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_summary_to_cscc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_to_stackdriver`.\n"]
    pub fn set_publish_to_stackdriver(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriver>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_to_stackdriver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_stackdriver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `save_findings`.\n"]
    pub fn set_save_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.save_findings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.save_findings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_Actions {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_Actions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_Actions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_Actions {
        DataLossPreventionJobTrigger_InspectJob_Actions {
            deidentify: core::default::Default::default(),
            job_notification_emails: core::default::Default::default(),
            pub_sub: core::default::Default::default(),
            publish_findings_to_dataplex_catalog: core::default::Default::default(),
            publish_summary_to_cscc: core::default::Default::default(),
            publish_to_stackdriver: core::default::Default::default(),
            save_findings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_ActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_ActionsRef {
        DataLossPreventionJobTrigger_InspectJob_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deidentify` after provisioning.\n"]
    pub fn deidentify(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_DeidentifyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deidentify", self.base))
    }

    #[doc = "Get a reference to the value of field `job_notification_emails` after provisioning.\n"]
    pub fn job_notification_emails(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_JobNotificationEmailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_notification_emails", self.base))
    }

    #[doc = "Get a reference to the value of field `pub_sub` after provisioning.\n"]
    pub fn pub_sub(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_PubSubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pub_sub", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_findings_to_dataplex_catalog` after provisioning.\n"]
    pub fn publish_findings_to_dataplex_catalog(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_PublishFindingsToDataplexCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_findings_to_dataplex_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_summary_to_cscc` after provisioning.\n"]
    pub fn publish_summary_to_cscc(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_PublishSummaryToCsccRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_summary_to_cscc", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_stackdriver` after provisioning.\n"]
    pub fn publish_to_stackdriver(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_PublishToStackdriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_stackdriver", self.base))
    }

    #[doc = "Get a reference to the value of field `save_findings` after provisioning.\n"]
    pub fn save_findings(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_Actions_SaveFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.save_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath { }

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList { }

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList {
            words: self.words,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub fn words(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
        >,
    >,
    word_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `word_list`.\n"]
    pub fn set_word_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.word_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.word_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary_WordListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore { score: self.score }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType {
    #[doc =
        "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of an inspectTemplate. Set by the server."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary>>,
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType>>,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>>,
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    stored_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType>>,
    surrogate_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surrogate_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
    #[doc =
        "Set the field `exclusion_type`.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn set_exclusion_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exclusion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `likelihood`.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stored_type`.\n"]
    pub fn set_stored_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stored_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stored_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `surrogate_type`.\n"]
    pub fn set_surrogate_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.surrogate_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.surrogate_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes {
            exclusion_type: core::default::Default::default(),
            likelihood: core::default::Default::default(),
            dictionary: core::default::Default::default(),
            info_type: core::default::Default::default(),
            regex: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            stored_type: core::default::Default::default(),
            surrogate_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclusion_type` after provisioning.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn exclusion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclusion_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `likelihood` after provisioning.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_StoredTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }

    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_SurrogateTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surrogate_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic {
    info_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
    #[doc = "Set the field `max_findings`.\nMax findings limit for the given infoType."]
    pub fn set_max_findings(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_findings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType {
            max_findings: core::default::Default::default(),
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings` after provisioning.\nMax findings limit for the given infoType."]
    pub fn max_findings(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsDynamic {
    max_findings_per_info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_item: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_request: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
    #[doc =
        "Set the field `max_findings_per_item`.\nMax number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub fn set_max_findings_per_item(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_findings_per_item = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_findings_per_request`.\nMax number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub fn set_max_findings_per_request(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_findings_per_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_findings_per_info_type`.\n"]
    pub fn set_max_findings_per_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_findings_per_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_findings_per_info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits {
            max_findings_per_item: core::default::Default::default(),
            max_findings_per_request: core::default::Default::default(),
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_item` after provisioning.\nMax number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub fn max_findings_per_item(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_item", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_request` after provisioning.\nMax number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub fn max_findings_per_request(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_request", self.base))
    }

    #[doc = "Get a reference to the value of field `max_findings_per_info_type` after provisioning.\n"]
    pub fn max_findings_per_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits_MaxFindingsPerInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_findings_per_info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pattern`.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn set_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
            group_indexes: core::default::Default::default(),
            pattern: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    #[doc =
        "Set the field `window_after`.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn set_window_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `window_before`.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn set_window_before(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn window_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn window_before(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_before", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
            hotword_regex: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary>>,
    exclude_by_hotword: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
        >,
    >,
    exclude_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
        >,
    >,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
    matching_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_hotword: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Dictionary,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_by_hotword`.\n"]
    pub fn set_exclude_by_hotword(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_by_hotword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_by_hotword = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_info_types`.\n"]
    pub fn set_exclude_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_Regex>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc =
        "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `matching_type` after provisioning.\nHow the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub fn matching_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]
    pub fn exclude_by_hotword(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_hotword", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]
    pub fn exclude_info_types(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    #[doc =
        "Set the field `fixed_likelihood`.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_fixed_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `relative_likelihood`.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn set_relative_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relative_likelihood = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn fixed_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_likelihood", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relative_likelihood` after provisioning.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn relative_likelihood(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relative_likelihood", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_Proximity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesDynamic {
    exclusion_rule: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule>,
    >,
    hotword_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
    #[doc = "Set the field `exclusion_rule`.\n"]
    pub fn set_exclusion_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hotword_rule`.\n"]
    pub fn set_hotword_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules {
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_ExclusionRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules_HotwordRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>>,
    rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypes>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_Rules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_InspectConfigDynamic {
    custom_info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes>>,
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes>>,
    limits: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits>>,
    rule_set: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_quote: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_set: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_InspectConfigDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfig {
    #[doc = "Set the field `exclude_info_types`.\nWhen true, excludes type information of the findings."]
    pub fn set_exclude_info_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_info_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_quote`.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn set_include_quote(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_quote = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_likelihood`.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_min_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_info_types`.\n"]
    pub fn set_custom_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypes>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_Limits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_set`.\n"]
    pub fn set_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_InspectConfig {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_InspectConfig {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_InspectConfig {
        DataLossPreventionJobTrigger_InspectJob_InspectConfig {
            exclude_info_types: core::default::Default::default(),
            include_quote: core::default::Default::default(),
            min_likelihood: core::default::Default::default(),
            custom_info_types: core::default::Default::default(),
            info_types: core::default::Default::default(),
            limits: core::default::Default::default(),
            rule_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_InspectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_InspectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_InspectConfigRef {
        DataLossPreventionJobTrigger_InspectJob_InspectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_InspectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude_info_types` after provisioning.\nWhen true, excludes type information of the findings."]
    pub fn exclude_info_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_quote` after provisioning.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn include_quote(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_info_types` after provisioning.\n"]
    pub fn custom_info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_CustomInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfig_RuleSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields {
    #[doc = "Name describing the field excluded from scanning."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName describing the field excluded from scanning."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields { }

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields {
    #[doc = "Name of a BigQuery field to be returned with the findings."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of a BigQuery field to be returned with the findings."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields {
    #[doc = "Name describing the field to which scanning is limited."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName describing the field to which scanning is limited."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
    #[doc = "The dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID of the table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe name of the table."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsDynamic {
    excluded_fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields>,
    >,
    identifying_fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields>,
    >,
    included_fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields>,
    >,
    table_reference: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rows_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rows_limit_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_fields: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifying_fields: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_fields: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_reference: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
    #[doc =
        "Set the field `rows_limit`.\nMax number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted.\nIf not set, or if set to 0, all rows will be scanned. Only one of rowsLimit and rowsLimitPercent can be\nspecified. Cannot be used in conjunction with TimespanConfig."]
    pub fn set_rows_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rows_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rows_limit_percent`.\nMax percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of\nrowsLimit and rowsLimitPercent can be specified. Cannot be used in conjunction with TimespanConfig."]
    pub fn set_rows_limit_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rows_limit_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_method`.\nHow to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either\nrowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them.\nIf TimespanConfig is set, set this to an empty string to avoid using the default value. Default value: \"TOP\" Possible values: [\"TOP\", \"RANDOM_START\"]"]
    pub fn set_sample_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sample_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_fields`.\n"]
    pub fn set_excluded_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excluded_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identifying_fields`.\n"]
    pub fn set_identifying_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identifying_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identifying_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `included_fields`.\n"]
    pub fn set_included_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.included_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.included_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_reference`.\n"]
    pub fn set_table_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions {
            rows_limit: core::default::Default::default(),
            rows_limit_percent: core::default::Default::default(),
            sample_method: core::default::Default::default(),
            excluded_fields: core::default::Default::default(),
            identifying_fields: core::default::Default::default(),
            included_fields: core::default::Default::default(),
            table_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rows_limit` after provisioning.\nMax number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted.\nIf not set, or if set to 0, all rows will be scanned. Only one of rowsLimit and rowsLimitPercent can be\nspecified. Cannot be used in conjunction with TimespanConfig."]
    pub fn rows_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rows_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rows_limit_percent` after provisioning.\nMax percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of\nrowsLimit and rowsLimitPercent can be specified. Cannot be used in conjunction with TimespanConfig."]
    pub fn rows_limit_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rows_limit_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_method` after provisioning.\nHow to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either\nrowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them.\nIf TimespanConfig is set, set this to an empty string to avoid using the default value. Default value: \"TOP\" Possible values: [\"TOP\", \"RANDOM_START\"]"]
    pub fn sample_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_method", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_fields` after provisioning.\n"]
    pub fn excluded_fields(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_ExcludedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `identifying_fields` after provisioning.\n"]
    pub fn identifying_fields(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IdentifyingFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifying_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `included_fields` after provisioning.\n"]
    pub fn included_fields(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_IncludedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `table_reference` after provisioning.\n"]
    pub fn table_reference(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions_TableReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_regex: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regex: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
    #[doc =
        "Set the field `exclude_regex`.\nA list of regular expressions matching file paths to exclude. All files in the bucket that match at\nleast one of these regular expressions will be excluded from the scan."]
    pub fn set_exclude_regex(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_regex`.\nA list of regular expressions matching file paths to include. All files in the bucket\nthat match at least one of these regular expressions will be included in the set of files,\nexcept for those that also match an item in excludeRegex. Leaving this field empty will\nmatch all files by default (this is equivalent to including .* in the list)"]
    pub fn set_include_regex(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
    #[doc = "The name of a Cloud Storage bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet {
            bucket_name: self.bucket_name,
            exclude_regex: core::default::Default::default(),
            include_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of a Cloud Storage bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_regex` after provisioning.\nA list of regular expressions matching file paths to exclude. All files in the bucket that match at\nleast one of these regular expressions will be excluded from the scan."]
    pub fn exclude_regex(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_regex` after provisioning.\nA list of regular expressions matching file paths to include. All files in the bucket\nthat match at least one of these regular expressions will be included in the set of files,\nexcept for those that also match an item in excludeRegex. Leaving this field empty will\nmatch all files by default (this is equivalent to including .* in the list)"]
    pub fn include_regex(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetDynamic {
    regex_file_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_file_set: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
    #[doc =
        "Set the field `url`.\nThe Cloud Storage url of the file(s) to scan, in the format 'gs://<bucket>/<path>'. Trailing wildcard\nin the path is allowed.\n\nIf the url ends in a trailing slash, the bucket or directory represented by the url will be scanned\nnon-recursively (content in sub-directories will not be scanned). This means that 'gs://mybucket/' is\nequivalent to 'gs://mybucket/*', and 'gs://mybucket/directory/' is equivalent to 'gs://mybucket/directory/*'."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex_file_set`.\n"]
    pub fn set_regex_file_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_file_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_file_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet {
            url: core::default::Default::default(),
            regex_file_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe Cloud Storage url of the file(s) to scan, in the format 'gs://<bucket>/<path>'. Trailing wildcard\nin the path is allowed.\n\nIf the url ends in a trailing slash, the bucket or directory represented by the url will be scanned\nnon-recursively (content in sub-directories will not be scanned). This means that 'gs://mybucket/' is\nequivalent to 'gs://mybucket/*', and 'gs://mybucket/directory/' is equivalent to 'gs://mybucket/directory/*'."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_file_set` after provisioning.\n"]
    pub fn regex_file_set(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet_RegexFileSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_file_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsDynamic {
    file_set: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes_limit_per_file: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes_limit_per_file_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files_limit_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_set: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
    #[doc =
        "Set the field `bytes_limit_per_file`.\nMax number of bytes to scan from a file. If a scanned file's size is bigger than this value\nthen the rest of the bytes are omitted."]
    pub fn set_bytes_limit_per_file(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bytes_limit_per_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bytes_limit_per_file_percent`.\nMax percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit."]
    pub fn set_bytes_limit_per_file_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bytes_limit_per_file_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_types`.\nList of file type groups to include in the scan. If empty, all files are scanned and available data\nformat processors are applied. In addition, the binary content of the selected files is always scanned as well.\nImages are scanned only as binary if the specified region does not support image inspection and no fileTypes were specified. Possible values: [\"BINARY_FILE\", \"TEXT_FILE\", \"IMAGE\", \"WORD\", \"PDF\", \"AVRO\", \"CSV\", \"TSV\", \"POWERPOINT\", \"EXCEL\"]"]
    pub fn set_file_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `files_limit_percent`.\nLimits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit."]
    pub fn set_files_limit_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.files_limit_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_method`.\nHow to sample bytes if not all bytes are scanned. Meaningful only when used in conjunction with bytesLimitPerFile.\nIf not specified, scanning would start from the top. Possible values: [\"TOP\", \"RANDOM_START\"]"]
    pub fn set_sample_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sample_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_set`.\n"]
    pub fn set_file_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions {
            bytes_limit_per_file: core::default::Default::default(),
            bytes_limit_per_file_percent: core::default::Default::default(),
            file_types: core::default::Default::default(),
            files_limit_percent: core::default::Default::default(),
            sample_method: core::default::Default::default(),
            file_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bytes_limit_per_file` after provisioning.\nMax number of bytes to scan from a file. If a scanned file's size is bigger than this value\nthen the rest of the bytes are omitted."]
    pub fn bytes_limit_per_file(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_limit_per_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bytes_limit_per_file_percent` after provisioning.\nMax percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit."]
    pub fn bytes_limit_per_file_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_limit_per_file_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_types` after provisioning.\nList of file type groups to include in the scan. If empty, all files are scanned and available data\nformat processors are applied. In addition, the binary content of the selected files is always scanned as well.\nImages are scanned only as binary if the specified region does not support image inspection and no fileTypes were specified. Possible values: [\"BINARY_FILE\", \"TEXT_FILE\", \"IMAGE\", \"WORD\", \"PDF\", \"AVRO\", \"CSV\", \"TSV\", \"POWERPOINT\", \"EXCEL\"]"]
    pub fn file_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `files_limit_percent` after provisioning.\nLimits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down.\nMust be between 0 and 100, inclusively. Both 0 and 100 means no limit."]
    pub fn files_limit_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.files_limit_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_method` after provisioning.\nHow to sample bytes if not all bytes are scanned. Meaningful only when used in conjunction with bytesLimitPerFile.\nIf not specified, scanning would start from the top. Possible values: [\"TOP\", \"RANDOM_START\"]"]
    pub fn sample_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_method", self.base))
    }

    #[doc = "Get a reference to the value of field `file_set` after provisioning.\n"]
    pub fn file_set(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions_FileSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind {
    #[doc = "The name of the Datastore kind."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Datastore kind."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    project_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
    #[doc = "Set the field `namespace_id`.\nIf not empty, the ID of the namespace to which the entities belong."]
    pub fn set_namespace_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
    #[doc = "The ID of the project to which the entities belong."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId {
            namespace_id: core::default::Default::default(),
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nIf not empty, the ID of the namespace to which the entities belong."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project to which the entities belong."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsDynamic {
    kind: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind>>,
    partition_id: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_id: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_Kind>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kind = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kind = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_id`.\n"]
    pub fn set_partition_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionId,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partition_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partition_id = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions {
            kind: core::default::Default::default(),
            partition_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_KindRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_id` after provisioning.\n"]
    pub fn partition_id(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions_PartitionIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields { }

impl terrars
::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
    #[doc = "Name describing the field."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName describing the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsDynamic {
    identifying_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifying_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields>,
    >,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
    #[doc = "Set the field `identifying_fields`.\n"]
    pub fn set_identifying_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identifying_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identifying_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions {
            identifying_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifying_fields` after provisioning.\n"]
    pub fn identifying_fields(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions_IdentifyingFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifying_fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsDynamic {
    table_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_finding_label_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
    #[doc =
        "Set the field `description`.\nA short description of where the data is coming from. Will be stored once in the job. 256 max length."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nTo organize findings, these labels will be added to each finding.\n\nLabel keys must be between 1 and 63 characters long and must conform to the following regular expression: '[a-z]([-a-z0-9]*[a-z0-9])?'.\n\nLabel values must be between 0 and 63 characters long and must conform to the regular expression '([a-z]([-a-z0-9]*[a-z0-9])?)?'.\n\nNo more than 10 labels can be associated with a given finding.\n\nExamples:\n* '\"environment\" : \"production\"'\n* '\"pipeline\" : \"etl\"'"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required_finding_label_keys`.\nThese are labels that each inspection request must include within their 'finding_labels' map. Request\nmay contain others, but any missing one of these will be rejected.\n\nLabel keys must be between 1 and 63 characters long and must conform to the following regular expression: '[a-z]([-a-z0-9]*[a-z0-9])?'.\n\nNo more than 10 keys can be required."]
    pub fn set_required_finding_label_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required_finding_label_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_options`.\n"]
    pub fn set_table_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions {
            description: core::default::Default::default(),
            labels: core::default::Default::default(),
            required_finding_label_keys: core::default::Default::default(),
            table_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of where the data is coming from. Will be stored once in the job. 256 max length."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nTo organize findings, these labels will be added to each finding.\n\nLabel keys must be between 1 and 63 characters long and must conform to the following regular expression: '[a-z]([-a-z0-9]*[a-z0-9])?'.\n\nLabel values must be between 0 and 63 characters long and must conform to the regular expression '([a-z]([-a-z0-9]*[a-z0-9])?)?'.\n\nNo more than 10 labels can be associated with a given finding.\n\nExamples:\n* '\"environment\" : \"production\"'\n* '\"pipeline\" : \"etl\"'"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_finding_label_keys` after provisioning.\nThese are labels that each inspection request must include within their 'finding_labels' map. Request\nmay contain others, but any missing one of these will be rejected.\n\nLabel keys must be between 1 and 63 characters long and must conform to the following regular expression: '[a-z]([-a-z0-9]*[a-z0-9])?'.\n\nNo more than 10 keys can be required."]
    pub fn required_finding_label_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required_finding_label_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `table_options` after provisioning.\n"]
    pub fn table_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions_TableOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField {
    #[doc =
        "Specification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery.\n\nFor BigQuery: Required to filter out rows based on the given start and end times. If not specified and the table was\nmodified between the given start and end times, the entire table will be scanned. The valid data types of the timestamp\nfield are: INTEGER, DATE, TIMESTAMP, or DATETIME BigQuery column.\n\nFor Datastore. Valid data types of the timestamp field are: TIMESTAMP. Datastore entity will be scanned if the\ntimestamp property does not exist or its value is empty or invalid."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery.\n\nFor BigQuery: Required to filter out rows based on the given start and end times. If not specified and the table was\nmodified between the given start and end times, the entire table will be scanned. The valid data types of the timestamp\nfield are: INTEGER, DATE, TIMESTAMP, or DATETIME BigQuery column.\n\nFor Datastore. Valid data types of the timestamp field are: TIMESTAMP. Datastore entity will be scanned if the\ntimestamp property does not exist or its value is empty or invalid."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigDynamic {
    timestamp_field: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_population_of_timespan_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_field: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
    #[doc =
        "Set the field `enable_auto_population_of_timespan_config`.\nWhen the job is started by a JobTrigger we will automatically figure out a valid startTime to avoid\nscanning files that have not been modified since the last time the JobTrigger executed. This will\nbe based on the time of the execution of the last run of the JobTrigger or the timespan endTime\nused in the last run of the JobTrigger."]
    pub fn set_enable_auto_population_of_timespan_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_auto_population_of_timespan_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `end_time`.\nExclude files, tables, or rows newer than this value. If not set, no upper time limit is applied."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nExclude files, tables, or rows older than this value. If not set, no lower time limit is applied."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_field`.\n"]
    pub fn set_timestamp_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestamp_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig {
            enable_auto_population_of_timespan_config: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            timestamp_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_auto_population_of_timespan_config` after provisioning.\nWhen the job is started by a JobTrigger we will automatically figure out a valid startTime to avoid\nscanning files that have not been modified since the last time the JobTrigger executed. This will\nbe based on the time of the execution of the last run of the JobTrigger or the timespan endTime\nused in the last run of the JobTrigger."]
    pub fn enable_auto_population_of_timespan_config(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_auto_population_of_timespan_config", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nExclude files, tables, or rows newer than this value. If not set, no upper time limit is applied."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nExclude files, tables, or rows older than this value. If not set, no lower time limit is applied."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_field` after provisioning.\n"]
    pub fn timestamp_field(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig_TimestampFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJob_StorageConfigDynamic {
    big_query_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions>>,
    cloud_storage_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions>>,
    datastore_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions>>,
    hybrid_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions>>,
    timespan_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hybrid_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timespan_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig>>,
    dynamic: DataLossPreventionJobTrigger_InspectJob_StorageConfigDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfig {
    #[doc = "Set the field `big_query_options`.\n"]
    pub fn set_big_query_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.big_query_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.big_query_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_options`.\n"]
    pub fn set_cloud_storage_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datastore_options`.\n"]
    pub fn set_datastore_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datastore_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datastore_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hybrid_options`.\n"]
    pub fn set_hybrid_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hybrid_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hybrid_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timespan_config`.\n"]
    pub fn set_timespan_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timespan_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timespan_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob_StorageConfig {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig {}

impl BuildDataLossPreventionJobTrigger_InspectJob_StorageConfig {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob_StorageConfig {
        DataLossPreventionJobTrigger_InspectJob_StorageConfig {
            big_query_options: core::default::Default::default(),
            cloud_storage_options: core::default::Default::default(),
            datastore_options: core::default::Default::default(),
            hybrid_options: core::default::Default::default(),
            timespan_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJob_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJob_StorageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_InspectJob_StorageConfigRef {
        DataLossPreventionJobTrigger_InspectJob_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJob_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `big_query_options` after provisioning.\n"]
    pub fn big_query_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_BigQueryOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_options", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_options` after provisioning.\n"]
    pub fn cloud_storage_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_CloudStorageOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `datastore_options` after provisioning.\n"]
    pub fn datastore_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_DatastoreOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_options", self.base))
    }

    #[doc = "Get a reference to the value of field `hybrid_options` after provisioning.\n"]
    pub fn hybrid_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_HybridOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timespan_config` after provisioning.\n"]
    pub fn timespan_config(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfig_TimespanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timespan_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_InspectJobDynamic {
    actions: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_Actions>>,
    inspect_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_InspectConfig>>,
    storage_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_InspectJob_StorageConfig>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_InspectJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_Actions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_config: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_InspectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_InspectJob_StorageConfig>>,
    dynamic: DataLossPreventionJobTrigger_InspectJobDynamic,
}

impl DataLossPreventionJobTrigger_InspectJob {
    #[doc = "Set the field `inspect_template_name`.\nThe name of the template to run when this job is triggered."]
    pub fn set_inspect_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inspect_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_Actions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inspect_config`.\n"]
    pub fn set_inspect_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_InspectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob_StorageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_InspectJob {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_InspectJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_InspectJob {}

impl BuildDataLossPreventionJobTrigger_InspectJob {
    pub fn build(self) -> DataLossPreventionJobTrigger_InspectJob {
        DataLossPreventionJobTrigger_InspectJob {
            inspect_template_name: core::default::Default::default(),
            actions: core::default::Default::default(),
            inspect_config: core::default::Default::default(),
            storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_InspectJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_InspectJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTrigger_InspectJobRef {
        DataLossPreventionJobTrigger_InspectJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_InspectJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inspect_template_name` after provisioning.\nThe name of the template to run when this job is triggered."]
    pub fn inspect_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_InspectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_InspectJob_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_Timeouts {
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

impl terrars::ToListMappable for DataLossPreventionJobTrigger_Timeouts {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_Timeouts {}

impl BuildDataLossPreventionJobTrigger_Timeouts {
    pub fn build(self) -> DataLossPreventionJobTrigger_Timeouts {
        DataLossPreventionJobTrigger_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTrigger_TimeoutsRef {
        DataLossPreventionJobTrigger_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_Triggers_Manual {}

impl DataLossPreventionJobTrigger_Triggers_Manual { }

impl terrars::ToListMappable for DataLossPreventionJobTrigger_Triggers_Manual {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers_Manual>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_Triggers_Manual {}

impl BuildDataLossPreventionJobTrigger_Triggers_Manual {
    pub fn build(self) -> DataLossPreventionJobTrigger_Triggers_Manual {
        DataLossPreventionJobTrigger_Triggers_Manual {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_Triggers_ManualRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_Triggers_ManualRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_Triggers_ManualRef {
        DataLossPreventionJobTrigger_Triggers_ManualRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_Triggers_ManualRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_Triggers_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence_period_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTrigger_Triggers_Schedule {
    #[doc =
        "Set the field `recurrence_period_duration`.\nWith this option a job is started a regular periodic basis. For example: every day (86400 seconds).\n\nA scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs.\n\nThis value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_recurrence_period_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.recurrence_period_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_Triggers_Schedule {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_Triggers_Schedule {}

impl BuildDataLossPreventionJobTrigger_Triggers_Schedule {
    pub fn build(self) -> DataLossPreventionJobTrigger_Triggers_Schedule {
        DataLossPreventionJobTrigger_Triggers_Schedule {
            recurrence_period_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_Triggers_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_Triggers_ScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTrigger_Triggers_ScheduleRef {
        DataLossPreventionJobTrigger_Triggers_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_Triggers_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recurrence_period_duration` after provisioning.\nWith this option a job is started a regular periodic basis. For example: every day (86400 seconds).\n\nA scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs.\n\nThis value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn recurrence_period_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence_period_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTrigger_TriggersDynamic {
    manual: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_Triggers_Manual>>,
    schedule: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTrigger_Triggers_Schedule>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTrigger_Triggers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_Triggers_Manual>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DataLossPreventionJobTrigger_Triggers_Schedule>>,
    dynamic: DataLossPreventionJobTrigger_TriggersDynamic,
}

impl DataLossPreventionJobTrigger_Triggers {
    #[doc = "Set the field `manual`.\n"]
    pub fn set_manual(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers_Manual>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manual = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manual = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers_Schedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTrigger_Triggers {
    type O = terrars::BlockAssignable<DataLossPreventionJobTrigger_Triggers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTrigger_Triggers {}

impl BuildDataLossPreventionJobTrigger_Triggers {
    pub fn build(self) -> DataLossPreventionJobTrigger_Triggers {
        DataLossPreventionJobTrigger_Triggers {
            manual: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionJobTrigger_TriggersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTrigger_TriggersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTrigger_TriggersRef {
        DataLossPreventionJobTrigger_TriggersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTrigger_TriggersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual` after provisioning.\n"]
    pub fn manual(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_Triggers_ManualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataLossPreventionJobTrigger_Triggers_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
