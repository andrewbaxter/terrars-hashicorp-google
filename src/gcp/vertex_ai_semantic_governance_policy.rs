#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiSemanticGovernancePolicyData_ {
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
    agent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    natural_language_constraint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    semantic_governance_policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_response_customization: std
    ::option
    ::Option<std::vec::Vec<VertexAiSemanticGovernancePolicy_AgentResponseCustomization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_tools: std::option::Option<std::vec::Vec<VertexAiSemanticGovernancePolicy_McpTools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiSemanticGovernancePolicy_Timeouts>,
    dynamic: VertexAiSemanticGovernancePolicyDynamic,
}

struct VertexAiSemanticGovernancePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiSemanticGovernancePolicyData_>,
}

#[derive(Clone)]
pub struct VertexAiSemanticGovernancePolicy(std::rc::Rc<VertexAiSemanticGovernancePolicy_>);

impl VertexAiSemanticGovernancePolicy {
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

    #[doc = "Set the field `description`.\nThe description of the SemanticGovernancePolicy."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe user-defined name of the SemanticGovernancePolicy."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `region`.\nThe region of the SemanticGovernancePolicy, e.g. 'us-central1'."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_response_customization`.\n"]
    pub fn set_agent_response_customization(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiSemanticGovernancePolicy_AgentResponseCustomization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_response_customization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_response_customization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp_tools`.\n"]
    pub fn set_mcp_tools(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiSemanticGovernancePolicy_McpTools>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mcp_tools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mcp_tools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiSemanticGovernancePolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe name of the agent in Agent Registry that is affected by this policy."]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_identity` after provisioning.\nRepresents the principal of the agent, used by the Policy Decision Point (PDP) for governance checks.\nFor more information, see https://docs.cloud.google.com/agent-builder/agent-engine/agent-identity.\nFormat: 'principal://TRUST_DOMAIN/NAMESPACE/AGENT_NAME'"]
    pub fn agent_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the SemanticGovernancePolicy was created, in RFC3339 UTC \"Zulu\" format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the SemanticGovernancePolicy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-defined name of the SemanticGovernancePolicy."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write transactions."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SemanticGovernancePolicy, in the form\n'projects/{project}/locations/{location}/semanticGovernancePolicies/{semantic_governance_policy}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `natural_language_constraint` after provisioning.\nThe natural language constraint of the SemanticGovernancePolicy."]
    pub fn natural_language_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.natural_language_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the SemanticGovernancePolicy, e.g. 'us-central1'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `semantic_governance_policy_id` after provisioning.\nThe ID of the SemanticGovernancePolicy, which will become the final component of the resource name.\nThis value may be up to 63 characters, and valid characters are [a-z0-9-]. The first character cannot be a number or hyphen. The last character must be a letter or a number."]
    pub fn semantic_governance_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.semantic_governance_policy_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the SemanticGovernancePolicy was last updated, in RFC3339 UTC \"Zulu\" format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_response_customization` after provisioning.\n"]
    pub fn agent_response_customization(
        &self,
    ) -> terrars::ListRef<VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_response_customization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_tools` after provisioning.\n"]
    pub fn mcp_tools(&self) -> terrars::ListRef<VertexAiSemanticGovernancePolicy_McpToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiSemanticGovernancePolicy_TimeoutsRef {
        VertexAiSemanticGovernancePolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiSemanticGovernancePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiSemanticGovernancePolicy { }

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicy {
    type O = terrars::ListRef<VertexAiSemanticGovernancePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiSemanticGovernancePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_semantic_governance_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiSemanticGovernancePolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the agent in Agent Registry that is affected by this policy."]
    pub agent: terrars::PrimField<std::string::String>,
    #[doc = "The natural language constraint of the SemanticGovernancePolicy."]
    pub natural_language_constraint: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the SemanticGovernancePolicy, which will become the final component of the resource name.\nThis value may be up to 63 characters, and valid characters are [a-z0-9-]. The first character cannot be a number or hyphen. The last character must be a letter or a number."]
    pub semantic_governance_policy_id: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiSemanticGovernancePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiSemanticGovernancePolicy {
        let out = VertexAiSemanticGovernancePolicy(std::rc::Rc::new(VertexAiSemanticGovernancePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiSemanticGovernancePolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent: self.agent,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                natural_language_constraint: self.natural_language_constraint,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                semantic_governance_policy_id: self.semantic_governance_policy_id,
                agent_response_customization: core::default::Default::default(),
                mcp_tools: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiSemanticGovernancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiSemanticGovernancePolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe name of the agent in Agent Registry that is affected by this policy."]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_identity` after provisioning.\nRepresents the principal of the agent, used by the Policy Decision Point (PDP) for governance checks.\nFor more information, see https://docs.cloud.google.com/agent-builder/agent-engine/agent-identity.\nFormat: 'principal://TRUST_DOMAIN/NAMESPACE/AGENT_NAME'"]
    pub fn agent_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the SemanticGovernancePolicy was created, in RFC3339 UTC \"Zulu\" format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the SemanticGovernancePolicy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-defined name of the SemanticGovernancePolicy."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write transactions."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SemanticGovernancePolicy, in the form\n'projects/{project}/locations/{location}/semanticGovernancePolicies/{semantic_governance_policy}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `natural_language_constraint` after provisioning.\nThe natural language constraint of the SemanticGovernancePolicy."]
    pub fn natural_language_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.natural_language_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the SemanticGovernancePolicy, e.g. 'us-central1'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `semantic_governance_policy_id` after provisioning.\nThe ID of the SemanticGovernancePolicy, which will become the final component of the resource name.\nThis value may be up to 63 characters, and valid characters are [a-z0-9-]. The first character cannot be a number or hyphen. The last character must be a letter or a number."]
    pub fn semantic_governance_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.semantic_governance_policy_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the SemanticGovernancePolicy was last updated, in RFC3339 UTC \"Zulu\" format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_response_customization` after provisioning.\n"]
    pub fn agent_response_customization(
        &self,
    ) -> terrars::ListRef<VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_response_customization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_tools` after provisioning.\n"]
    pub fn mcp_tools(&self) -> terrars::ListRef<VertexAiSemanticGovernancePolicy_McpToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiSemanticGovernancePolicy_TimeoutsRef {
        VertexAiSemanticGovernancePolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiSemanticGovernancePolicyDynamic {
    agent_response_customization: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiSemanticGovernancePolicy_AgentResponseCustomization>>,
    mcp_tools: std::option::Option<terrars::DynamicBlock<VertexAiSemanticGovernancePolicy_McpTools>>,
}

#[derive(Serialize)]
pub struct VertexAiSemanticGovernancePolicy_AgentResponseCustomization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denial_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiSemanticGovernancePolicy_AgentResponseCustomization {
    #[doc =
        "Set the field `denial_message`.\nCustom message shown to the end user when the policy check results in a denial. Use this\nto explain the rationale to the user. Max 1000 characters."]
    pub fn set_denial_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.denial_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicy_AgentResponseCustomization {
    type O = terrars::BlockAssignable<VertexAiSemanticGovernancePolicy_AgentResponseCustomization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiSemanticGovernancePolicy_AgentResponseCustomization {}

impl BuildVertexAiSemanticGovernancePolicy_AgentResponseCustomization {
    pub fn build(self) -> VertexAiSemanticGovernancePolicy_AgentResponseCustomization {
        VertexAiSemanticGovernancePolicy_AgentResponseCustomization {
            denial_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef {
        VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiSemanticGovernancePolicy_AgentResponseCustomizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `denial_message` after provisioning.\nCustom message shown to the end user when the policy check results in a denial. Use this\nto explain the rationale to the user. Max 1000 characters."]
    pub fn denial_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.denial_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiSemanticGovernancePolicy_McpTools {
    mcp_server: terrars::PrimField<std::string::String>,
    tools: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl VertexAiSemanticGovernancePolicy_McpTools { }

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicy_McpTools {
    type O = terrars::BlockAssignable<VertexAiSemanticGovernancePolicy_McpTools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiSemanticGovernancePolicy_McpTools {
    #[doc =
        "The resource name of the McpServer in Agent Registry that is affected by this policy.\nFormat: 'projects/{project}/locations/{location}/mcpServers/{mcpServer}'"]
    pub mcp_server: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource names of the McpTools used by the Agent that is affected by this policy.\nAt least one tool must be listed."]
    pub tools: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildVertexAiSemanticGovernancePolicy_McpTools {
    pub fn build(self) -> VertexAiSemanticGovernancePolicy_McpTools {
        VertexAiSemanticGovernancePolicy_McpTools {
            mcp_server: self.mcp_server,
            tools: self.tools,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiSemanticGovernancePolicy_McpToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicy_McpToolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiSemanticGovernancePolicy_McpToolsRef {
        VertexAiSemanticGovernancePolicy_McpToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiSemanticGovernancePolicy_McpToolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mcp_server` after provisioning.\nThe resource name of the McpServer in Agent Registry that is affected by this policy.\nFormat: 'projects/{project}/locations/{location}/mcpServers/{mcpServer}'"]
    pub fn mcp_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nThe resource names of the McpTools used by the Agent that is affected by this policy.\nAt least one tool must be listed."]
    pub fn tools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiSemanticGovernancePolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiSemanticGovernancePolicy_Timeouts {
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

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicy_Timeouts {
    type O = terrars::BlockAssignable<VertexAiSemanticGovernancePolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiSemanticGovernancePolicy_Timeouts {}

impl BuildVertexAiSemanticGovernancePolicy_Timeouts {
    pub fn build(self) -> VertexAiSemanticGovernancePolicy_Timeouts {
        VertexAiSemanticGovernancePolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiSemanticGovernancePolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiSemanticGovernancePolicy_TimeoutsRef {
        VertexAiSemanticGovernancePolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiSemanticGovernancePolicy_TimeoutsRef {
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
