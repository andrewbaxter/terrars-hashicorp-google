#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxPlaybookData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    goal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    playbook_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referenced_tools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<std::vec::Vec<DialogflowCxPlaybook_Instruction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_model_settings: std::option::Option<std::vec::Vec<DialogflowCxPlaybook_LlmModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxPlaybook_Timeouts>,
    dynamic: DialogflowCxPlaybookDynamic,
}

struct DialogflowCxPlaybook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxPlaybookData_>,
}

#[derive(Clone)]
pub struct DialogflowCxPlaybook(std::rc::Rc<DialogflowCxPlaybook_>);

impl DialogflowCxPlaybook {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `playbook_type`.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn set_playbook_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().playbook_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `referenced_tools`.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn set_referenced_tools(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().referenced_tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPlaybook_Instruction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instruction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instruction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPlaybook_LlmModelSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_model_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxPlaybook_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of initial playbook creation.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the playbook, unique within an agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goal` after provisioning.\nHigh level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub fn goal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Playbook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/playbooks/<Playbook ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `playbook_type` after provisioning.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn playbook_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.playbook_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_flows` after provisioning.\nThe resource name of flows referenced by the current playbook in the instructions."]
    pub fn referenced_flows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_flows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_playbooks` after provisioning.\nThe resource name of other playbooks referenced by the current playbook in the instructions."]
    pub fn referenced_playbooks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_playbooks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_tools` after provisioning.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn referenced_tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_count` after provisioning.\nEstimated number of tokes current playbook takes when sent to the LLM."]
    pub fn token_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast time the playbook version was updated.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::ListRef<DialogflowCxPlaybook_InstructionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxPlaybook_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPlaybook_TimeoutsRef {
        DialogflowCxPlaybook_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxPlaybook {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxPlaybook { }

impl terrars::ToListMappable for DialogflowCxPlaybook {
    type O = terrars::ListRef<DialogflowCxPlaybookRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxPlaybook_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_playbook".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxPlaybook {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the playbook, unique within an agent."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub goal: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPlaybook {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxPlaybook {
        let out = DialogflowCxPlaybook(std::rc::Rc::new(DialogflowCxPlaybook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxPlaybookData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                goal: self.goal,
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                playbook_type: core::default::Default::default(),
                referenced_tools: core::default::Default::default(),
                instruction: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxPlaybookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPlaybookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxPlaybookRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of initial playbook creation.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the playbook, unique within an agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goal` after provisioning.\nHigh level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook."]
    pub fn goal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Playbook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/playbooks/<Playbook ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Playbook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `playbook_type` after provisioning.\nType of the playbook. Possible values: [\"PLAYBOOK_TYPE_UNSPECIFIED\", \"TASK\", \"ROUTINE\"]"]
    pub fn playbook_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.playbook_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_flows` after provisioning.\nThe resource name of flows referenced by the current playbook in the instructions."]
    pub fn referenced_flows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_flows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_playbooks` after provisioning.\nThe resource name of other playbooks referenced by the current playbook in the instructions."]
    pub fn referenced_playbooks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_playbooks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `referenced_tools` after provisioning.\nThe resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps."]
    pub fn referenced_tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_count` after provisioning.\nEstimated number of tokes current playbook takes when sent to the LLM."]
    pub fn token_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast time the playbook version was updated.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::ListRef<DialogflowCxPlaybook_InstructionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxPlaybook_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPlaybook_TimeoutsRef {
        DialogflowCxPlaybook_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPlaybookDynamic {
    instruction: std::option::Option<terrars::DynamicBlock<DialogflowCxPlaybook_Instruction>>,
    llm_model_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxPlaybook_LlmModelSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPlaybook_Instruction_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPlaybook_Instruction_Steps {
    #[doc =
        "Set the field `steps`.\nSub-processing needed to execute the current step.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Step](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.playbooks#step)."]
    pub fn set_steps(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nStep instruction in text format."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPlaybook_Instruction_Steps {
    type O = terrars::BlockAssignable<DialogflowCxPlaybook_Instruction_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPlaybook_Instruction_Steps {}

impl BuildDialogflowCxPlaybook_Instruction_Steps {
    pub fn build(self) -> DialogflowCxPlaybook_Instruction_Steps {
        DialogflowCxPlaybook_Instruction_Steps {
            steps: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxPlaybook_Instruction_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPlaybook_Instruction_StepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPlaybook_Instruction_StepsRef {
        DialogflowCxPlaybook_Instruction_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPlaybook_Instruction_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `steps` after provisioning.\nSub-processing needed to execute the current step.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Step](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.playbooks#step)."]
    pub fn steps(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steps", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nStep instruction in text format."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPlaybook_InstructionDynamic {
    steps: std::option::Option<terrars::DynamicBlock<DialogflowCxPlaybook_Instruction_Steps>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPlaybook_Instruction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guidelines: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<DialogflowCxPlaybook_Instruction_Steps>>,
    dynamic: DialogflowCxPlaybook_InstructionDynamic,
}

impl DialogflowCxPlaybook_Instruction {
    #[doc =
        "Set the field `guidelines`.\nGeneral guidelines for the playbook. These are unstructured instructions that are not directly part of the goal, e.g. \"Always be polite\". It's valid for this text to be long and used instead of steps altogether."]
    pub fn set_guidelines(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guidelines = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPlaybook_Instruction_Steps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.steps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPlaybook_Instruction {
    type O = terrars::BlockAssignable<DialogflowCxPlaybook_Instruction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPlaybook_Instruction {}

impl BuildDialogflowCxPlaybook_Instruction {
    pub fn build(self) -> DialogflowCxPlaybook_Instruction {
        DialogflowCxPlaybook_Instruction {
            guidelines: core::default::Default::default(),
            steps: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxPlaybook_InstructionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPlaybook_InstructionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPlaybook_InstructionRef {
        DialogflowCxPlaybook_InstructionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPlaybook_InstructionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `guidelines` after provisioning.\nGeneral guidelines for the playbook. These are unstructured instructions that are not directly part of the goal, e.g. \"Always be polite\". It's valid for this text to be long and used instead of steps altogether."]
    pub fn guidelines(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guidelines", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<DialogflowCxPlaybook_Instruction_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPlaybook_LlmModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPlaybook_LlmModelSettings {
    #[doc = "Set the field `model`.\nThe selected LLM model."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_text`.\nThe custom prompt to use."]
    pub fn set_prompt_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPlaybook_LlmModelSettings {
    type O = terrars::BlockAssignable<DialogflowCxPlaybook_LlmModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPlaybook_LlmModelSettings {}

impl BuildDialogflowCxPlaybook_LlmModelSettings {
    pub fn build(self) -> DialogflowCxPlaybook_LlmModelSettings {
        DialogflowCxPlaybook_LlmModelSettings {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxPlaybook_LlmModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPlaybook_LlmModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPlaybook_LlmModelSettingsRef {
        DialogflowCxPlaybook_LlmModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPlaybook_LlmModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe selected LLM model."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\nThe custom prompt to use."]
    pub fn prompt_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPlaybook_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPlaybook_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxPlaybook_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxPlaybook_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPlaybook_Timeouts {}

impl BuildDialogflowCxPlaybook_Timeouts {
    pub fn build(self) -> DialogflowCxPlaybook_Timeouts {
        DialogflowCxPlaybook_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxPlaybook_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPlaybook_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPlaybook_TimeoutsRef {
        DialogflowCxPlaybook_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPlaybook_TimeoutsRef {
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
