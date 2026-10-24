use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionJobTriggerData {
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
    inspect_job: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionJobTriggerTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerTriggersEl>>,
    dynamic: DataLossPreventionJobTriggerDynamic,
}

struct DataLossPreventionJobTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionJobTriggerData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionJobTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersEl>>,
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
    pub fn inspect_job(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionJobTriggerTimeoutsElRef {
        DataLossPreventionJobTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<DataLossPreventionJobTriggerTriggersElRef> {
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
            data: std::cell::RefCell::new(DataLossPreventionJobTriggerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    pub fn inspect_job(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_job", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionJobTriggerTimeoutsElRef {
        DataLossPreventionJobTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<DataLossPreventionJobTriggerTriggersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_redact_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    structured_deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl {
            deidentify_template: core::default::Default::default(),
            image_redact_template: core::default::Default::default(),
            structured_deidentify_template: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
    #[doc =
        "Set the field `table_id`.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef {
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
struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElDynamic {
    table: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
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
                            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableEl,
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl {
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElTableElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElDynamic {
    transformation_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl>,
    >,
    transformation_details_storage_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
    cloud_storage_output: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_types_to_transform: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_details_storage_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
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
                            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigEl,
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
                            DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
    #[doc =
        "User settable Cloud Storage bucket and folders to store de-identified files.\n\nThis field must be set for cloud storage deidentification.\n\nThe output Cloud Storage bucket must be different from the input bucket.\n\nDe-identified files will overwrite files in the output path.\n\nForm of: gs://bucket/folder/ or gs://bucket"]
    pub cloud_storage_output: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl {
            cloud_storage_output: self.cloud_storage_output,
            file_types_to_transform: core::default::Default::default(),
            transformation_config: core::default::Default::default(),
            transformation_details_storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_details_storage_config` after provisioning.\n"]
    pub fn transformation_details_storage_config(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElTransformationDetailsStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_details_storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {}

impl DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {
        DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl {}
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl {
    topic: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElPubSubEl {
    #[doc = "Cloud Pub/Sub topic to send notifications to."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElPubSubEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl {
        DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl { topic: self.topic }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl {}
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl {}
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl {}
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
    #[doc =
        "Set the field `table_id`.\nName of the table. If is not set a new one will be generated for you with the following format:\n'dlp_googleapis_yyyy_mm_dd_[dlp_job_id]'. Pacific timezone will be used for generating the date details."]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
    #[doc = "Dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef {
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
struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElDynamic {
    table: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
    #[doc =
        "Set the field `output_schema`.\nSchema used for writing the findings for Inspect jobs. This field is only used for\nInspect and must be unspecified for Risk jobs. Columns are derived from the Finding\nobject. If appending to an existing table, any columns from the predefined schema\nthat are missing will be added. No columns in the existing table will be deleted.\n\nIf unspecified, then all available columns will be used for a new table or an (existing)\ntable with no schema, and no changes will be made to an existing table that has a schema.\nOnly for use with external storage. Possible values: [\"BASIC_COLUMNS\", \"GCS_COLUMNS\", \"DATASTORE_COLUMNS\", \"BIG_QUERY_COLUMNS\", \"ALL_COLUMNS\"]"]
    pub fn set_output_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_schema = std::option::Option::Some(v.into());
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
                            DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl {
            output_schema: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_schema` after provisioning.\nSchema used for writing the findings for Inspect jobs. This field is only used for\nInspect and must be unspecified for Risk jobs. Columns are derived from the Finding\nobject. If appending to an existing table, any columns from the predefined schema\nthat are missing will be added. No columns in the existing table will be deleted.\n\nIf unspecified, then all available columns will be used for a new table or an (existing)\ntable with no schema, and no changes will be made to an existing table that has a schema.\nOnly for use with external storage. Possible values: [\"BASIC_COLUMNS\", \"GCS_COLUMNS\", \"DATASTORE_COLUMNS\", \"BIG_QUERY_COLUMNS\", \"ALL_COLUMNS\"]"]
    pub fn output_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElDynamic {
    output_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigEl,
                        >,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl {
            output_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElOutputConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElActionsElDynamic {
    deidentify: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl>>,
    job_notification_emails: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl>>,
    pub_sub: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl>>,
    publish_findings_to_cloud_data_catalog: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl>,
    >,
    publish_summary_to_cscc: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl>>,
    publish_to_stackdriver: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl>>,
    save_findings: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_notification_emails: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub_sub: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_findings_to_cloud_data_catalog: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_summary_to_cscc: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_stackdriver: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    save_findings: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElActionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElActionsEl {
    #[doc = "Set the field `deidentify`.\n"]
    pub fn set_deidentify(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyEl>>,
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
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsEl>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPubSubEl>>,
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

    #[doc = "Set the field `publish_findings_to_cloud_data_catalog`.\n"]
    pub fn set_publish_findings_to_cloud_data_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_findings_to_cloud_data_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_findings_to_cloud_data_catalog = std::option::Option::Some(d);
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
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccEl>,
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
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverEl>,
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
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsEl>>,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElActionsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElActionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElActionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElActionsEl {
        DataLossPreventionJobTriggerInspectJobElActionsEl {
            deidentify: core::default::Default::default(),
            job_notification_emails: core::default::Default::default(),
            pub_sub: core::default::Default::default(),
            publish_findings_to_cloud_data_catalog: core::default::Default::default(),
            publish_summary_to_cscc: core::default::Default::default(),
            publish_to_stackdriver: core::default::Default::default(),
            save_findings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElActionsElRef {
        DataLossPreventionJobTriggerInspectJobElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deidentify` after provisioning.\n"]
    pub fn deidentify(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElDeidentifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deidentify", self.base))
    }

    #[doc = "Get a reference to the value of field `job_notification_emails` after provisioning.\n"]
    pub fn job_notification_emails(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElJobNotificationEmailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_notification_emails", self.base))
    }

    #[doc = "Get a reference to the value of field `pub_sub` after provisioning.\n"]
    pub fn pub_sub(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElPubSubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pub_sub", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_findings_to_cloud_data_catalog` after provisioning.\n"]
    pub fn publish_findings_to_cloud_data_catalog(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElPublishFindingsToCloudDataCatalogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_findings_to_cloud_data_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_summary_to_cscc` after provisioning.\n"]
    pub fn publish_summary_to_cscc(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElPublishSummaryToCsccElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_summary_to_cscc", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_stackdriver` after provisioning.\n"]
    pub fn publish_to_stackdriver(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElPublishToStackdriverElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_stackdriver", self.base))
    }

    #[doc = "Get a reference to the value of field `save_findings` after provisioning.\n"]
    pub fn save_findings(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElSaveFindingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.save_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
            path: self.path,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
            words: self.words,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
        >,
    >,
    word_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElWordListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef {
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
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl {
    #[doc =
        "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl { name: self.name }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl {}
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDynamic {
    dictionary: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl>,
    >,
    info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl>,
    >,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl>>,
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl>,
    >,
    stored_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl>,
    >,
    surrogate_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surrogate_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeEl,
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
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexEl,
                        >,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl {
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

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElStoredTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }

    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElSurrogateTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surrogate_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl { score: self.score }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {

}

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
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
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic {
    info_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl,
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

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
            max_findings: core::default::Default::default(),
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
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
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElDynamic {
    max_findings_per_info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_item: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_request: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl {
            max_findings_per_item: core::default::Default::default(),
            max_findings_per_request: core::default::Default::default(),
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_findings_per_info_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef {
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
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {

}

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
            path: self.path,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
            words: self.words,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >,
    >,
    word_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
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

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
            hotword_regex: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {

}

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
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
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic {
    info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
                        >,
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
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

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
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

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDynamic {
    dictionary: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
        >,
    >,
    exclude_by_hotword: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
        >,
    >,
    exclude_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
        >,
    >,
    regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
    matching_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_hotword: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
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
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl,
                        >,
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

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
    #[doc =
        "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef {
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
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]
    pub fn exclude_by_hotword(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_hotword", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]
    pub fn exclude_info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
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

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
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
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
        >,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElDynamic {
    exclusion_rule: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl>,
    >,
    hotword_rule: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleEl,
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
                            DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl {
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElExclusionRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElHotwordRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl>>,
    rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesEl>,
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
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesEl>,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElInspectConfigElDynamic {
    custom_info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl>>,
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl>>,
    limits: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl>>,
    rule_set: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_quote: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_set: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElInspectConfigElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigEl {
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
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesEl>,
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
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesEl>,
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

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetEl>,
                    >,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElInspectConfigEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElInspectConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElInspectConfigEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElInspectConfigEl {
        DataLossPreventionJobTriggerInspectJobElInspectConfigEl {
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

pub struct DataLossPreventionJobTriggerInspectJobElInspectConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElInspectConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElInspectConfigElRef {
        DataLossPreventionJobTriggerInspectJobElInspectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElInspectConfigElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElCustomInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRuleSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl {
    #[doc = "Name describing the field excluded from scanning."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl { name: self.name }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
    #[doc = "Name of a BigQuery field to be returned with the findings."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl {
            name: self.name,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl {
    #[doc = "Name describing the field to which scanning is limited."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl { name: self.name }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef {
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
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
    #[doc = "The dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef {
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
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElDynamic {
    excluded_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl>,
    >,
    identifying_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl>,
    >,
    included_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl>,
    >,
    table_reference: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rows_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rows_limit_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_fields: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifying_fields: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_fields: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_reference: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
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
        "Set the field `sample_method`.\nHow to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either\nrowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them. Default value: \"TOP\" Possible values: [\"TOP\", \"RANDOM_START\"]"]
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsEl,
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsEl,
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsEl,
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl {
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

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef {
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
        "Get a reference to the value of field `sample_method` after provisioning.\nHow to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either\nrowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them. Default value: \"TOP\" Possible values: [\"TOP\", \"RANDOM_START\"]"]
    pub fn sample_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_method", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_fields` after provisioning.\n"]
    pub fn excluded_fields(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElExcludedFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `identifying_fields` after provisioning.\n"]
    pub fn identifying_fields(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIdentifyingFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifying_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `included_fields` after provisioning.\n"]
    pub fn included_fields(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElIncludedFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `table_reference` after provisioning.\n"]
    pub fn table_reference(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElTableReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_regex: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regex: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
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
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
    #[doc = "The name of a Cloud Storage bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl {
            bucket_name: self.bucket_name,
            exclude_regex: core::default::Default::default(),
            include_regex: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef {
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
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElDynamic {
    regex_file_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_file_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl {
            url: core::default::Default::default(),
            regex_file_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef {
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
    ::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRegexFileSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_file_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElDynamic {
    file_set: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
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
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl {
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

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElFileSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl {
    #[doc = "The name of the Datastore kind."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl { name: self.name }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Datastore kind."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    project_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
    #[doc = "Set the field `namespace_id`.\nIf not empty, the ID of the namespace to which the entities belong."]
    pub fn set_namespace_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
    #[doc = "The ID of the project to which the entities belong."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl {
            namespace_id: core::default::Default::default(),
            project_id: self.project_id,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef {
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
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElDynamic {
    kind: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl>>,
    partition_id: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_id: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindEl,
                        >,
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl {
            kind: core::default::Default::default(),
            partition_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElKindElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_id` after provisioning.\n"]
    pub fn partition_id(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElPartitionIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
    #[doc = "Name describing the field."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
    pub fn build(
        self,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl {
            name: self.name,
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName describing the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElDynamic {
    identifying_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifying_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl>,
    >,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl {
            identifying_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifying_fields` after provisioning.\n"]
    pub fn identifying_fields(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElIdentifyingFieldsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifying_fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElDynamic {
    table_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_finding_label_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl {
            description: core::default::Default::default(),
            labels: core::default::Default::default(),
            required_finding_label_keys: core::default::Default::default(),
            table_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElTableOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl { }

impl terrars
::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl {
    #[doc =
        "Specification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery.\n\nFor BigQuery: Required to filter out rows based on the given start and end times. If not specified and the table was\nmodified between the given start and end times, the entire table will be scanned. The valid data types of the timestamp\nfield are: INTEGER, DATE, TIMESTAMP, or DATETIME BigQuery column.\n\nFor Datastore. Valid data types of the timestamp field are: TIMESTAMP. Datastore entity will be scanned if the\ntimestamp property does not exist or its value is empty or invalid."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl { name: self.name }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef {
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
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElDynamic {
    timestamp_field: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_population_of_timespan_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_field: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
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
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldEl,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl {
            enable_auto_population_of_timespan_config: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            timestamp_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef {
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
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElTimestampFieldElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElStorageConfigElDynamic {
    big_query_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl>>,
    cloud_storage_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl>>,
    datastore_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl>>,
    hybrid_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl>>,
    timespan_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hybrid_options: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timespan_config: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElStorageConfigElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigEl {
    #[doc = "Set the field `big_query_options`.\n"]
    pub fn set_big_query_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsEl>,
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
                        ::BlockAssignable<
                            DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsEl,
                        >,
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
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsEl>,
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
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsEl>,
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
                        terrars
                        ::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigEl>,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobElStorageConfigEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobElStorageConfigEl {}

impl BuildDataLossPreventionJobTriggerInspectJobElStorageConfigEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobElStorageConfigEl {
        DataLossPreventionJobTriggerInspectJobElStorageConfigEl {
            big_query_options: core::default::Default::default(),
            cloud_storage_options: core::default::Default::default(),
            datastore_options: core::default::Default::default(),
            hybrid_options: core::default::Default::default(),
            timespan_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElStorageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerInspectJobElStorageConfigElRef {
        DataLossPreventionJobTriggerInspectJobElStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `big_query_options` after provisioning.\n"]
    pub fn big_query_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElBigQueryOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_options", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_options` after provisioning.\n"]
    pub fn cloud_storage_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElCloudStorageOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `datastore_options` after provisioning.\n"]
    pub fn datastore_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElDatastoreOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_options", self.base))
    }

    #[doc = "Get a reference to the value of field `hybrid_options` after provisioning.\n"]
    pub fn hybrid_options(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElHybridOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timespan_config` after provisioning.\n"]
    pub fn timespan_config(
        &self,
    ) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElTimespanConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timespan_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerInspectJobElDynamic {
    actions: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElActionsEl>>,
    inspect_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElInspectConfigEl>>,
    storage_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobElStorageConfigEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerInspectJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElActionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_config: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElInspectConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerInspectJobElStorageConfigEl>>,
    dynamic: DataLossPreventionJobTriggerInspectJobElDynamic,
}

impl DataLossPreventionJobTriggerInspectJobEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElActionsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElInspectConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobElStorageConfigEl>>,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerInspectJobEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerInspectJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerInspectJobEl {}

impl BuildDataLossPreventionJobTriggerInspectJobEl {
    pub fn build(self) -> DataLossPreventionJobTriggerInspectJobEl {
        DataLossPreventionJobTriggerInspectJobEl {
            inspect_template_name: core::default::Default::default(),
            actions: core::default::Default::default(),
            inspect_config: core::default::Default::default(),
            storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerInspectJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerInspectJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTriggerInspectJobElRef {
        DataLossPreventionJobTriggerInspectJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerInspectJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inspect_template_name` after provisioning.\nThe name of the template to run when this job is triggered."]
    pub fn inspect_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElInspectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataLossPreventionJobTriggerInspectJobElStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerTimeoutsEl {
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerTimeoutsEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerTimeoutsEl {}

impl BuildDataLossPreventionJobTriggerTimeoutsEl {
    pub fn build(self) -> DataLossPreventionJobTriggerTimeoutsEl {
        DataLossPreventionJobTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTriggerTimeoutsElRef {
        DataLossPreventionJobTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerTimeoutsElRef {
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
pub struct DataLossPreventionJobTriggerTriggersElManualEl {}

impl DataLossPreventionJobTriggerTriggersElManualEl { }

impl terrars::ToListMappable for DataLossPreventionJobTriggerTriggersElManualEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersElManualEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerTriggersElManualEl {}

impl BuildDataLossPreventionJobTriggerTriggersElManualEl {
    pub fn build(self) -> DataLossPreventionJobTriggerTriggersElManualEl {
        DataLossPreventionJobTriggerTriggersElManualEl {}
    }
}

pub struct DataLossPreventionJobTriggerTriggersElManualElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerTriggersElManualElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerTriggersElManualElRef {
        DataLossPreventionJobTriggerTriggersElManualElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerTriggersElManualElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerTriggersElScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence_period_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionJobTriggerTriggersElScheduleEl {
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerTriggersElScheduleEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersElScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerTriggersElScheduleEl {}

impl BuildDataLossPreventionJobTriggerTriggersElScheduleEl {
    pub fn build(self) -> DataLossPreventionJobTriggerTriggersElScheduleEl {
        DataLossPreventionJobTriggerTriggersElScheduleEl {
            recurrence_period_duration: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerTriggersElScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerTriggersElScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionJobTriggerTriggersElScheduleElRef {
        DataLossPreventionJobTriggerTriggersElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerTriggersElScheduleElRef {
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
struct DataLossPreventionJobTriggerTriggersElDynamic {
    manual: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerTriggersElManualEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerTriggersElScheduleEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionJobTriggerTriggersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerTriggersElManualEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DataLossPreventionJobTriggerTriggersElScheduleEl>>,
    dynamic: DataLossPreventionJobTriggerTriggersElDynamic,
}

impl DataLossPreventionJobTriggerTriggersEl {
    #[doc = "Set the field `manual`.\n"]
    pub fn set_manual(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersElManualEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersElScheduleEl>>,
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

impl terrars::ToListMappable for DataLossPreventionJobTriggerTriggersEl {
    type O = terrars::BlockAssignable<DataLossPreventionJobTriggerTriggersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionJobTriggerTriggersEl {}

impl BuildDataLossPreventionJobTriggerTriggersEl {
    pub fn build(self) -> DataLossPreventionJobTriggerTriggersEl {
        DataLossPreventionJobTriggerTriggersEl {
            manual: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionJobTriggerTriggersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionJobTriggerTriggersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionJobTriggerTriggersElRef {
        DataLossPreventionJobTriggerTriggersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionJobTriggerTriggersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual` after provisioning.\n"]
    pub fn manual(&self) -> terrars::ListRef<DataLossPreventionJobTriggerTriggersElManualElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataLossPreventionJobTriggerTriggersElScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionJobTriggerDynamic {
    inspect_job: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerInspectJobEl>>,
    triggers: std::option::Option<terrars::DynamicBlock<DataLossPreventionJobTriggerTriggersEl>>,
}
