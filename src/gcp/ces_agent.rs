#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesAgentData_ {
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
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    child_agents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_agent_callbacks: std::option::Option<std::vec::Vec<CesAgent_AfterAgentCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_model_callbacks: std::option::Option<std::vec::Vec<CesAgent_AfterModelCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_tool_callbacks: std::option::Option<std::vec::Vec<CesAgent_AfterToolCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_agent_callbacks: std::option::Option<std::vec::Vec<CesAgent_BeforeAgentCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_model_callbacks: std::option::Option<std::vec::Vec<CesAgent_BeforeModelCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_tool_callbacks: std::option::Option<std::vec::Vec<CesAgent_BeforeToolCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_agent: std::option::Option<std::vec::Vec<CesAgent_LlmAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<std::vec::Vec<CesAgent_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_a2a_agent: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_dialogflow_agent: std::option::Option<std::vec::Vec<CesAgent_RemoteDialogflowAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesAgent_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolsets: std::option::Option<std::vec::Vec<CesAgent_Toolsets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_rules: std::option::Option<std::vec::Vec<CesAgent_TransferRules>>,
    dynamic: CesAgentDynamic,
}

struct CesAgent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesAgentData_>,
}

#[derive(Clone)]
pub struct CesAgent(std::rc::Rc<CesAgent_>);

impl CesAgent {
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
        "Set the field `agent_id`.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn set_agent_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `child_agents`.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_child_agents(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().child_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nHuman-readable description of the agent."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `guardrails`.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn set_guardrails(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn set_instruction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tools`.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tools(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after_agent_callbacks`.\n"]
    pub fn set_after_agent_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_AfterAgentCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_agent_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_agent_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `after_model_callbacks`.\n"]
    pub fn set_after_model_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_AfterModelCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_model_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_model_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `after_tool_callbacks`.\n"]
    pub fn set_after_tool_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_AfterToolCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().after_tool_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.after_tool_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `before_agent_callbacks`.\n"]
    pub fn set_before_agent_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_BeforeAgentCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_agent_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_agent_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `before_model_callbacks`.\n"]
    pub fn set_before_model_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_BeforeModelCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_model_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_model_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `before_tool_callbacks`.\n"]
    pub fn set_before_tool_callbacks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_BeforeToolCallbacks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().before_tool_callbacks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.before_tool_callbacks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_agent`.\n"]
    pub fn set_llm_agent(self, v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_LlmAgent>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_ModelSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_a2a_agent`.\n"]
    pub fn set_remote_a2a_agent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_a2a_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_a2a_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_dialogflow_agent`.\n"]
    pub fn set_remote_dialogflow_agent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteDialogflowAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_dialogflow_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_dialogflow_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesAgent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(self, v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_Toolsets>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().toolsets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.toolsets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_rules`.\n"]
    pub fn set_transfer_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_TransferRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transfer_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transfer_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `agent_id` after provisioning.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn child_agents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the agent was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the agent."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_summary` after provisioning.\nIf the agent is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instruction` after provisioning.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the agent was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(&self) -> terrars::ListRef<CesAgent_AfterAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_agent_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(&self) -> terrars::ListRef<CesAgent_AfterModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_model_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(&self) -> terrars::ListRef<CesAgent_AfterToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_tool_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_agent_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_model_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_tool_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> terrars::ListRef<CesAgent_LlmAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAgent_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_a2a_agent` after provisioning.\n"]
    pub fn remote_a2a_agent(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_a2a_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(&self) -> terrars::ListRef<CesAgent_RemoteDialogflowAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_dialogflow_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAgent_TimeoutsRef {
        CesAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> terrars::ListRef<CesAgent_ToolsetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolsets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_rules` after provisioning.\n"]
    pub fn transfer_rules(&self) -> terrars::ListRef<CesAgent_TransferRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_rules", self.extract_ref()))
    }
}

impl terrars::Referable for CesAgent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesAgent { }

impl terrars::ToListMappable for CesAgent {
    type O = terrars::ListRef<CesAgentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesAgent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_agent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesAgent {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the agent."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent {
    pub fn build(self, stack: &mut terrars::Stack) -> CesAgent {
        let out = CesAgent(std::rc::Rc::new(CesAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesAgentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_id: core::default::Default::default(),
                app: self.app,
                child_agents: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                guardrails: core::default::Default::default(),
                id: core::default::Default::default(),
                instruction: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                tools: core::default::Default::default(),
                after_agent_callbacks: core::default::Default::default(),
                after_model_callbacks: core::default::Default::default(),
                after_tool_callbacks: core::default::Default::default(),
                before_agent_callbacks: core::default::Default::default(),
                before_model_callbacks: core::default::Default::default(),
                before_tool_callbacks: core::default::Default::default(),
                llm_agent: core::default::Default::default(),
                model_settings: core::default::Default::default(),
                remote_a2a_agent: core::default::Default::default(),
                remote_dialogflow_agent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                toolsets: core::default::Default::default(),
                transfer_rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesAgentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_id` after provisioning.\nThe ID to use for the agent, which will become the final component of\nthe agent's resource name. If not provided, a unique ID will be\nautomatically assigned for the agent."]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents in the agent tree.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn child_agents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the agent was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the agent."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_summary` after provisioning.\nIf the agent is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instruction` after provisioning.\nInstructions for the LLM model to guide the agent's behavior."]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nList of available tools for the agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the agent was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(&self) -> terrars::ListRef<CesAgent_AfterAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_agent_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(&self) -> terrars::ListRef<CesAgent_AfterModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_model_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(&self) -> terrars::ListRef<CesAgent_AfterToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_tool_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_agent_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_model_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(&self) -> terrars::ListRef<CesAgent_BeforeToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_tool_callbacks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> terrars::ListRef<CesAgent_LlmAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAgent_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_a2a_agent` after provisioning.\n"]
    pub fn remote_a2a_agent(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_a2a_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(&self) -> terrars::ListRef<CesAgent_RemoteDialogflowAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_dialogflow_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAgent_TimeoutsRef {
        CesAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> terrars::ListRef<CesAgent_ToolsetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolsets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_rules` after provisioning.\n"]
    pub fn transfer_rules(&self) -> terrars::ListRef<CesAgent_TransferRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_rules", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesAgentDynamic {
    after_agent_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_AfterAgentCallbacks>>,
    after_model_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_AfterModelCallbacks>>,
    after_tool_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_AfterToolCallbacks>>,
    before_agent_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_BeforeAgentCallbacks>>,
    before_model_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_BeforeModelCallbacks>>,
    before_tool_callbacks: std::option::Option<terrars::DynamicBlock<CesAgent_BeforeToolCallbacks>>,
    llm_agent: std::option::Option<terrars::DynamicBlock<CesAgent_LlmAgent>>,
    model_settings: std::option::Option<terrars::DynamicBlock<CesAgent_ModelSettings>>,
    remote_a2a_agent: std::option::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent>>,
    remote_dialogflow_agent: std::option::Option<terrars::DynamicBlock<CesAgent_RemoteDialogflowAgent>>,
    toolsets: std::option::Option<terrars::DynamicBlock<CesAgent_Toolsets>>,
    transfer_rules: std::option::Option<terrars::DynamicBlock<CesAgent_TransferRules>>,
}

#[derive(Serialize)]
pub struct CesAgent_AfterAgentCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_AfterAgentCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_AfterAgentCallbacks {
    type O = terrars::BlockAssignable<CesAgent_AfterAgentCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_AfterAgentCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_AfterAgentCallbacks {
    pub fn build(self) -> CesAgent_AfterAgentCallbacks {
        CesAgent_AfterAgentCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_AfterAgentCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_AfterAgentCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_AfterAgentCallbacksRef {
        CesAgent_AfterAgentCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_AfterAgentCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_AfterModelCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_AfterModelCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_AfterModelCallbacks {
    type O = terrars::BlockAssignable<CesAgent_AfterModelCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_AfterModelCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_AfterModelCallbacks {
    pub fn build(self) -> CesAgent_AfterModelCallbacks {
        CesAgent_AfterModelCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_AfterModelCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_AfterModelCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_AfterModelCallbacksRef {
        CesAgent_AfterModelCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_AfterModelCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_AfterToolCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_AfterToolCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_AfterToolCallbacks {
    type O = terrars::BlockAssignable<CesAgent_AfterToolCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_AfterToolCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_AfterToolCallbacks {
    pub fn build(self) -> CesAgent_AfterToolCallbacks {
        CesAgent_AfterToolCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_AfterToolCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_AfterToolCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_AfterToolCallbacksRef {
        CesAgent_AfterToolCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_AfterToolCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_BeforeAgentCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_BeforeAgentCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_BeforeAgentCallbacks {
    type O = terrars::BlockAssignable<CesAgent_BeforeAgentCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_BeforeAgentCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_BeforeAgentCallbacks {
    pub fn build(self) -> CesAgent_BeforeAgentCallbacks {
        CesAgent_BeforeAgentCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_BeforeAgentCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_BeforeAgentCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_BeforeAgentCallbacksRef {
        CesAgent_BeforeAgentCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_BeforeAgentCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_BeforeModelCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_BeforeModelCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_BeforeModelCallbacks {
    type O = terrars::BlockAssignable<CesAgent_BeforeModelCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_BeforeModelCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_BeforeModelCallbacks {
    pub fn build(self) -> CesAgent_BeforeModelCallbacks {
        CesAgent_BeforeModelCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_BeforeModelCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_BeforeModelCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_BeforeModelCallbacksRef {
        CesAgent_BeforeModelCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_BeforeModelCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_BeforeToolCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_BeforeToolCallbacks {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_BeforeToolCallbacks {
    type O = terrars::BlockAssignable<CesAgent_BeforeToolCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_BeforeToolCallbacks {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_BeforeToolCallbacks {
    pub fn build(self) -> CesAgent_BeforeToolCallbacks {
        CesAgent_BeforeToolCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_BeforeToolCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_BeforeToolCallbacksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_BeforeToolCallbacksRef {
        CesAgent_BeforeToolCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_BeforeToolCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_LlmAgent {}

impl CesAgent_LlmAgent { }

impl terrars::ToListMappable for CesAgent_LlmAgent {
    type O = terrars::BlockAssignable<CesAgent_LlmAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_LlmAgent {}

impl BuildCesAgent_LlmAgent {
    pub fn build(self) -> CesAgent_LlmAgent {
        CesAgent_LlmAgent {}
    }
}

#[allow(dead_code)]
pub struct CesAgent_LlmAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_LlmAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_LlmAgentRef {
        CesAgent_LlmAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_LlmAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CesAgent_ModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thinking_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAgent_ModelSettings {
    #[doc =
        "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `thinking_level`.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_thinking_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thinking_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_ModelSettings {
    type O = terrars::BlockAssignable<CesAgent_ModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_ModelSettings {}

impl BuildCesAgent_ModelSettings {
    pub fn build(self) -> CesAgent_ModelSettings {
        CesAgent_ModelSettings {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
            thinking_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_ModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_ModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_ModelSettingsRef {
        CesAgent_ModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_ModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thinking_level` after provisioning.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn thinking_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thinking_level", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    examples: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    tags: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
    #[doc = "Set the field `examples`.\nExample prompts or scenarios that this skill can handle."]
    pub fn set_examples(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.examples = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input_modes`.\nThe set of supported input media types for this skill, overriding the\nagent's defaults."]
    pub fn set_input_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_modes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_modes`.\nThe set of supported output media types for this skill, overriding the\nagent's defaults."]
    pub fn set_output_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_modes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
    #[doc = "A detailed description of the skill."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "A unique identifier for the agent's skill."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "A human-readable name for the skill."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "A set of keywords describing the skill's capabilities."]
    pub tags: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills {
            description: self.description,
            examples: core::default::Default::default(),
            id: self.id,
            input_modes: core::default::Default::default(),
            name: self.name,
            output_modes: core::default::Default::default(),
            tags: self.tags,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA detailed description of the skill."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `examples` after provisioning.\nExample prompts or scenarios that this skill can handle."]
    pub fn examples(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nA unique identifier for the agent's skill."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_modes` after provisioning.\nThe set of supported input media types for this skill, overriding the\nagent's defaults."]
    pub fn input_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the skill."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_modes` after provisioning.\nThe set of supported output media types for this skill, overriding the\nagent's defaults."]
    pub fn output_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA set of keywords describing the skill's capabilities."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
    protocol_binding: terrars::PrimField<std::string::String>,
    protocol_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
    #[doc = "Set the field `tenant`.\nTenant ID to be used in the request when calling the agent."]
    pub fn set_tenant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
    #[doc =
        "The protocol binding supported at this URL. The core ones officially\nsupported are JSONRPC, GRPC and HTTP+JSON."]
    pub protocol_binding: terrars::PrimField<std::string::String>,
    #[doc = "The version of the A2A protocol this interface exposes.\nExamples: \"0.3\", \"1.0\""]
    pub protocol_version: terrars::PrimField<std::string::String>,
    #[doc = "The URL where this interface is available. Must be a valid absolute HTTPS\nURL in production."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces {
            protocol_binding: self.protocol_binding,
            protocol_version: self.protocol_version,
            tenant: core::default::Default::default(),
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `protocol_binding` after provisioning.\nThe protocol binding supported at this URL. The core ones officially\nsupported are JSONRPC, GRPC and HTTP+JSON."]
    pub fn protocol_binding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_binding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol_version` after provisioning.\nThe version of the A2A protocol this interface exposes.\nExamples: \"0.3\", \"1.0\""]
    pub fn protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nTenant ID to be used in the request when calling the agent."]
    pub fn tenant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL where this interface is available. Must be a valid absolute HTTPS\nURL in production."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCardDynamic {
    skills: std::option::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills>>,
    supported_interfaces: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces>>,
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
    description: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skills: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_interfaces: std
    ::option
    ::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces>>,
    dynamic: CesAgent_RemoteA2aAgent_A2aConfig_AgentCardDynamic,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
    #[doc = "Set the field `skills`.\n"]
    pub fn set_skills(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_Skills>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.skills = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.skills = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `supported_interfaces`.\n"]
    pub fn set_supported_interfaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfaces>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.supported_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.supported_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
    #[doc = "A description of the agent's domain of action/solution space."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "A human-readable name for the agent."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The version of the agent."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCard {
            description: self.description,
            name: self.name,
            version: self.version,
            skills: core::default::Default::default(),
            supported_interfaces: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef {
        CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the agent's domain of action/solution space."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the agent."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the agent."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `skills` after provisioning.\n"]
    pub fn skills(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SkillsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skills", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_interfaces` after provisioning.\n"]
    pub fn supported_interfaces(
        &self,
    ) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard_SupportedInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_interfaces", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
    api_key_secret_version: terrars::PrimField<std::string::String>,
    key_name: terrars::PrimField<std::string::String>,
    request_location: terrars::PrimField<std::string::String>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig { }

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
    #[doc =
        "The name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub api_key_secret_version: terrars::PrimField<std::string::String>,
    #[doc =
        "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: terrars::PrimField<std::string::String>,
    #[doc = "Key location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub request_location: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig {
            api_key_secret_version: self.api_key_secret_version,
            key_name: self.key_name,
            request_location: self.request_location,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig {
    token: terrars::PrimField<std::string::String>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig { }

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig {
    #[doc = "The bearer token.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig { token: self.token }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `token` after provisioning.\nThe bearer token.\nMust be in the format '$context.variables.<name_of_variable>'."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
    client_id: terrars::PrimField<std::string::String>,
    client_secret_version: terrars::PrimField<std::string::String>,
    oauth_grant_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    token_endpoint: terrars::PrimField<std::string::String>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub client_secret_version: terrars::PrimField<std::string::String>,
    #[doc = "OAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub oauth_grant_type: terrars::PrimField<std::string::String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig {
            client_id: self.client_id,
            client_secret_version: self.client_secret_version,
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID from the OAuth provider."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint in the OAuth provider to exchange for an access token."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    service_account: terrars::PrimField<std::string::String>,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc =
        "Set the field `scopes`.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc =
        "The email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig {
            scopes: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationDynamic {
    api_key_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig>>,
    bearer_token_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig>>,
    oauth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig>>,
    service_account_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig>>,
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std
    ::option
    ::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig>>,
    dynamic: CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationDynamic,
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bearer_token_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bearer_token_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef {
        CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_RemoteA2aAgent_A2aConfigDynamic {
    agent_card: std::option::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard>>,
    api_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication>>,
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent_A2aConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    streaming_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_card: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication>>,
    dynamic: CesAgent_RemoteA2aAgent_A2aConfigDynamic,
}

impl CesAgent_RemoteA2aAgent_A2aConfig {
    #[doc =
        "Set the field `agent_registry`.\nReference to the agent in the Agent Registry.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub fn set_agent_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_registry = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `context_id`.\nIf not empty, interactions with the remote A2A agent will use this context\nID. This context_id field can refer to a session variable like\n'$context.variables.order_agent_session_id'."]
    pub fn set_context_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.context_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input_variable_mapping`.\nMapping of input variable names of remote agent to GECX variable names."]
    pub fn set_input_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_variable_mapping`.\nMapping of output variable names of remote agent to GECX variable names."]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `streaming_enabled`.\nWhether streaming is enabled for the remote agent."]
    pub fn set_streaming_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.streaming_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_card`.\n"]
    pub fn set_agent_card(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_AgentCard>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_card = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_card = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent_A2aConfig {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent_A2aConfig {}

impl BuildCesAgent_RemoteA2aAgent_A2aConfig {
    pub fn build(self) -> CesAgent_RemoteA2aAgent_A2aConfig {
        CesAgent_RemoteA2aAgent_A2aConfig {
            agent_registry: core::default::Default::default(),
            context_id: core::default::Default::default(),
            input_variable_mapping: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
            streaming_enabled: core::default::Default::default(),
            agent_card: core::default::Default::default(),
            api_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgent_A2aConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgent_A2aConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_RemoteA2aAgent_A2aConfigRef {
        CesAgent_RemoteA2aAgent_A2aConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgent_A2aConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_registry` after provisioning.\nReference to the agent in the Agent Registry.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub fn agent_registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_registry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `context_id` after provisioning.\nIf not empty, interactions with the remote A2A agent will use this context\nID. This context_id field can refer to a session variable like\n'$context.variables.order_agent_session_id'."]
    pub fn context_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_variable_mapping` after provisioning.\nMapping of input variable names of remote agent to GECX variable names."]
    pub fn input_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.input_variable_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_variable_mapping` after provisioning.\nMapping of output variable names of remote agent to GECX variable names."]
    pub fn output_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.output_variable_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `streaming_enabled` after provisioning.\nWhether streaming is enabled for the remote agent."]
    pub fn streaming_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.streaming_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_card` after provisioning.\n"]
    pub fn agent_card(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_AgentCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_card", self.base))
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfig_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_RemoteA2aAgentDynamic {
    a2a_config: std::option::Option<terrars::DynamicBlock<CesAgent_RemoteA2aAgent_A2aConfig>>,
}

#[derive(Serialize)]
pub struct CesAgent_RemoteA2aAgent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    a2a_config: std::option::Option<std::vec::Vec<CesAgent_RemoteA2aAgent_A2aConfig>>,
    dynamic: CesAgent_RemoteA2aAgentDynamic,
}

impl CesAgent_RemoteA2aAgent {
    #[doc = "Set the field `a2a_config`.\n"]
    pub fn set_a2a_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_RemoteA2aAgent_A2aConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.a2a_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.a2a_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteA2aAgent {
    type O = terrars::BlockAssignable<CesAgent_RemoteA2aAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteA2aAgent {}

impl BuildCesAgent_RemoteA2aAgent {
    pub fn build(self) -> CesAgent_RemoteA2aAgent {
        CesAgent_RemoteA2aAgent {
            a2a_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteA2aAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteA2aAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_RemoteA2aAgentRef {
        CesAgent_RemoteA2aAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteA2aAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `a2a_config` after provisioning.\n"]
    pub fn a2a_config(&self) -> terrars::ListRef<CesAgent_RemoteA2aAgent_A2aConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.a2a_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_RemoteDialogflowAgent {
    agent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    flow_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code_variable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_response_interruption_settings: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAgent_RemoteDialogflowAgent {
    #[doc =
        "Set the field `environment_id`.\nThe environment ID of the Dialogflow agent be used for the agent\nexecution. If not specified, the draft environment will be used."]
    pub fn set_environment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input_variable_mapping`.\nThe mapping of the app variables names to the Dialogflow session\nparameters names to be sent to the Dialogflow agent as input."]
    pub fn set_input_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `language_code_variable`.\nThe name of the variable that contains the language code to be used for\nthe Dialogflow session. If unspecified, the default language code of the\nDialogflow agent will be used."]
    pub fn set_language_code_variable(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_code_variable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_variable_mapping`.\nThe mapping of the Dialogflow session parameters names to the app\nvariables names to be sent back to the CES agent after the Dialogflow\nagent execution ends."]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `respect_response_interruption_settings`.\nIndicates whether to respect the message-level interruption settings configured in the Dialogflow agent. * If false: all response messages from the Dialogflow agent follow the app-level barge-in settings. * If true: only response messages with ['allow_playback_interruption'](https://docs.cloud.google.com/dialogflow/cx/docs/reference/rpc/google.cloud.dialogflow.cx.v3#text) set to true will be interruptable, all other messages follow the app-level barge-in settings."]
    pub fn set_respect_response_interruption_settings(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.respect_response_interruption_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_RemoteDialogflowAgent {
    type O = terrars::BlockAssignable<CesAgent_RemoteDialogflowAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_RemoteDialogflowAgent {
    #[doc =
        "The\n[Dialogflow](https://cloud.google.com/dialogflow/cx/docs/concept/console-conversational-agents\nagent resource name.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub agent: terrars::PrimField<std::string::String>,
    #[doc = "The flow ID of the flow in the Dialogflow agent."]
    pub flow_id: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_RemoteDialogflowAgent {
    pub fn build(self) -> CesAgent_RemoteDialogflowAgent {
        CesAgent_RemoteDialogflowAgent {
            agent: self.agent,
            environment_id: core::default::Default::default(),
            flow_id: self.flow_id,
            input_variable_mapping: core::default::Default::default(),
            language_code_variable: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
            respect_response_interruption_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_RemoteDialogflowAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_RemoteDialogflowAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_RemoteDialogflowAgentRef {
        CesAgent_RemoteDialogflowAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_RemoteDialogflowAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe\n[Dialogflow](https://cloud.google.com/dialogflow/cx/docs/concept/console-conversational-agents\nagent resource name.\nFormat: 'projects/{project}/locations/{location}/agents/{agent}'"]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_id` after provisioning.\nThe environment ID of the Dialogflow agent be used for the agent\nexecution. If not specified, the draft environment will be used."]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flow_id` after provisioning.\nThe flow ID of the flow in the Dialogflow agent."]
    pub fn flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_variable_mapping` after provisioning.\nThe mapping of the app variables names to the Dialogflow session\nparameters names to be sent to the Dialogflow agent as input."]
    pub fn input_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.input_variable_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language_code_variable` after provisioning.\nThe name of the variable that contains the language code to be used for\nthe Dialogflow session. If unspecified, the default language code of the\nDialogflow agent will be used."]
    pub fn language_code_variable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code_variable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_variable_mapping` after provisioning.\nThe mapping of the Dialogflow session parameters names to the app\nvariables names to be sent back to the CES agent after the Dialogflow\nagent execution ends."]
    pub fn output_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.output_variable_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `respect_response_interruption_settings` after provisioning.\nIndicates whether to respect the message-level interruption settings configured in the Dialogflow agent. * If false: all response messages from the Dialogflow agent follow the app-level barge-in settings. * If true: only response messages with ['allow_playback_interruption'](https://docs.cloud.google.com/dialogflow/cx/docs/reference/rpc/google.cloud.dialogflow.cx.v3#text) set to true will be interruptable, all other messages follow the app-level barge-in settings."]
    pub fn respect_response_interruption_settings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.respect_response_interruption_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CesAgent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAgent_Timeouts {
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

impl terrars::ToListMappable for CesAgent_Timeouts {
    type O = terrars::BlockAssignable<CesAgent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_Timeouts {}

impl BuildCesAgent_Timeouts {
    pub fn build(self) -> CesAgent_Timeouts {
        CesAgent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_TimeoutsRef {
        CesAgent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TimeoutsRef {
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
pub struct CesAgent_Toolsets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    toolset: terrars::PrimField<std::string::String>,
}

impl CesAgent_Toolsets {
    #[doc = "Set the field `tool_ids`.\nThe tools IDs to filter the toolset."]
    pub fn set_tool_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tool_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAgent_Toolsets {
    type O = terrars::BlockAssignable<CesAgent_Toolsets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_Toolsets {
    #[doc =
        "The resource name of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub toolset: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_Toolsets {
    pub fn build(self) -> CesAgent_Toolsets {
        CesAgent_Toolsets {
            tool_ids: core::default::Default::default(),
            toolset: self.toolset,
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_ToolsetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_ToolsetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_ToolsetsRef {
        CesAgent_ToolsetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_ToolsetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tool_ids` after provisioning.\nThe tools IDs to filter the toolset."]
    pub fn tool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `toolset` after provisioning.\nThe resource name of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn toolset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition {
    expression: terrars::PrimField<std::string::String>,
}

impl CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition { }

impl terrars::ToListMappable for CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition {
    type O = terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_TransferRules_DeterministicTransfer_ExpressionCondition {
    #[doc = "The string representation of cloud.api.Expression condition."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_TransferRules_DeterministicTransfer_ExpressionCondition {
    pub fn build(self) -> CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition {
        CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition { expression: self.expression }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef {
        CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe string representation of cloud.api.Expression condition."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition {
    python_code: terrars::PrimField<std::string::String>,
}

impl CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition { }

impl terrars::ToListMappable for CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition {
    type O = terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition {
    #[doc = "The python code to execute. The function must be named\n'should_trigger_transfer_callback'."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition {
    pub fn build(self) -> CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition {
        CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition { python_code: self.python_code }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
        CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute. The function must be named\n'should_trigger_transfer_callback'."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_TransferRules_DeterministicTransferDynamic {
    expression_condition: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>>,
    python_code_condition: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition>>,
}

#[derive(Serialize)]
pub struct CesAgent_TransferRules_DeterministicTransfer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression_condition: std
    ::option
    ::Option<std::vec::Vec<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code_condition: std
    ::option
    ::Option<std::vec::Vec<CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition>>,
    dynamic: CesAgent_TransferRules_DeterministicTransferDynamic,
}

impl CesAgent_TransferRules_DeterministicTransfer {
    #[doc = "Set the field `expression_condition`.\n"]
    pub fn set_expression_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expression_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expression_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `python_code_condition`.\n"]
    pub fn set_python_code_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer_PythonCodeCondition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python_code_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python_code_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_TransferRules_DeterministicTransfer {
    type O = terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_TransferRules_DeterministicTransfer {}

impl BuildCesAgent_TransferRules_DeterministicTransfer {
    pub fn build(self) -> CesAgent_TransferRules_DeterministicTransfer {
        CesAgent_TransferRules_DeterministicTransfer {
            expression_condition: core::default::Default::default(),
            python_code_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TransferRules_DeterministicTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TransferRules_DeterministicTransferRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_TransferRules_DeterministicTransferRef {
        CesAgent_TransferRules_DeterministicTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TransferRules_DeterministicTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression_condition` after provisioning.\n"]
    pub fn expression_condition(
        &self,
    ) -> terrars::ListRef<CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code_condition` after provisioning.\n"]
    pub fn python_code_condition(
        &self,
    ) -> terrars::ListRef<CesAgent_TransferRules_DeterministicTransfer_PythonCodeConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_code_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_TransferRules_DisablePlannerTransferDynamic {
    expression_condition: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>>,
}

#[derive(Serialize)]
pub struct CesAgent_TransferRules_DisablePlannerTransfer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression_condition: std
    ::option
    ::Option<std::vec::Vec<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>>,
    dynamic: CesAgent_TransferRules_DisablePlannerTransferDynamic,
}

impl CesAgent_TransferRules_DisablePlannerTransfer {
    #[doc = "Set the field `expression_condition`.\n"]
    pub fn set_expression_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer_ExpressionCondition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expression_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expression_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_TransferRules_DisablePlannerTransfer {
    type O = terrars::BlockAssignable<CesAgent_TransferRules_DisablePlannerTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_TransferRules_DisablePlannerTransfer {}

impl BuildCesAgent_TransferRules_DisablePlannerTransfer {
    pub fn build(self) -> CesAgent_TransferRules_DisablePlannerTransfer {
        CesAgent_TransferRules_DisablePlannerTransfer {
            expression_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TransferRules_DisablePlannerTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TransferRules_DisablePlannerTransferRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAgent_TransferRules_DisablePlannerTransferRef {
        CesAgent_TransferRules_DisablePlannerTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TransferRules_DisablePlannerTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression_condition` after provisioning.\n"]
    pub fn expression_condition(
        &self,
    ) -> terrars::ListRef<CesAgent_TransferRules_DeterministicTransfer_ExpressionConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesAgent_TransferRulesDynamic {
    deterministic_transfer: std::option::Option<terrars::DynamicBlock<CesAgent_TransferRules_DeterministicTransfer>>,
    disable_planner_transfer: std
    ::option
    ::Option<terrars::DynamicBlock<CesAgent_TransferRules_DisablePlannerTransfer>>,
}

#[derive(Serialize)]
pub struct CesAgent_TransferRules {
    child_agent: terrars::PrimField<std::string::String>,
    direction: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deterministic_transfer: std::option::Option<std::vec::Vec<CesAgent_TransferRules_DeterministicTransfer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_planner_transfer: std::option::Option<std::vec::Vec<CesAgent_TransferRules_DisablePlannerTransfer>>,
    dynamic: CesAgent_TransferRulesDynamic,
}

impl CesAgent_TransferRules {
    #[doc = "Set the field `deterministic_transfer`.\n"]
    pub fn set_deterministic_transfer(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_TransferRules_DeterministicTransfer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deterministic_transfer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deterministic_transfer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disable_planner_transfer`.\n"]
    pub fn set_disable_planner_transfer(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesAgent_TransferRules_DisablePlannerTransfer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disable_planner_transfer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disable_planner_transfer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesAgent_TransferRules {
    type O = terrars::BlockAssignable<CesAgent_TransferRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAgent_TransferRules {
    #[doc =
        "The resource name of the child agent the rule applies to.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub child_agent: terrars::PrimField<std::string::String>,
    #[doc = "The direction of the transfer. Possible values: [\"PARENT_TO_CHILD\", \"CHILD_TO_PARENT\"]"]
    pub direction: terrars::PrimField<std::string::String>,
}

impl BuildCesAgent_TransferRules {
    pub fn build(self) -> CesAgent_TransferRules {
        CesAgent_TransferRules {
            child_agent: self.child_agent,
            direction: self.direction,
            deterministic_transfer: core::default::Default::default(),
            disable_planner_transfer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAgent_TransferRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAgent_TransferRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAgent_TransferRulesRef {
        CesAgent_TransferRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAgent_TransferRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `child_agent` after provisioning.\nThe resource name of the child agent the rule applies to.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn child_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.child_agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction of the transfer. Possible values: [\"PARENT_TO_CHILD\", \"CHILD_TO_PARENT\"]"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `deterministic_transfer` after provisioning.\n"]
    pub fn deterministic_transfer(&self) -> terrars::ListRef<CesAgent_TransferRules_DeterministicTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deterministic_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_planner_transfer` after provisioning.\n"]
    pub fn disable_planner_transfer(&self) -> terrars::ListRef<CesAgent_TransferRules_DisablePlannerTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disable_planner_transfer", self.base))
    }
}
