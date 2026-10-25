#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AgentRegistryServiceData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_spec: std::option::Option<std::vec::Vec<AgentRegistryService_AgentSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_spec: std::option::Option<std::vec::Vec<AgentRegistryService_EndpointSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interfaces: std::option::Option<std::vec::Vec<AgentRegistryService_Interfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_server_spec: std::option::Option<std::vec::Vec<AgentRegistryService_McpServerSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AgentRegistryService_Timeouts>,
    dynamic: AgentRegistryServiceDynamic,
}

struct AgentRegistryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentRegistryServiceData_>,
}

#[derive(Clone)]
pub struct AgentRegistryService(std::rc::Rc<AgentRegistryService_>);

impl AgentRegistryService {
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

    #[doc = "Set the field `description`.\nThe description of the Service."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nUser-defined display name for the Service. Can have a maximum length of 63 characters."]
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

    #[doc = "Set the field `agent_spec`.\n"]
    pub fn set_agent_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentRegistryService_AgentSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_spec`.\n"]
    pub fn set_endpoint_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentRegistryService_EndpointSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interfaces`.\n"]
    pub fn set_interfaces(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentRegistryService_Interfaces>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interfaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp_server_spec`.\n"]
    pub fn set_mcp_server_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentRegistryService_McpServerSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mcp_server_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mcp_server_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AgentRegistryService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Service."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-defined display name for the Service. Can have a maximum length of 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_resource` after provisioning.\nThe resource name of the resulting Agent, MCP Server, or Endpoint."]
    pub fn registry_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe name of the Service."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_spec` after provisioning.\n"]
    pub fn agent_spec(&self) -> terrars::ListRef<AgentRegistryService_AgentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<AgentRegistryService_EndpointSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interfaces` after provisioning.\n"]
    pub fn interfaces(&self) -> terrars::ListRef<AgentRegistryService_InterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_server_spec` after provisioning.\n"]
    pub fn mcp_server_spec(&self) -> terrars::ListRef<AgentRegistryService_McpServerSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_server_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentRegistryService_TimeoutsRef {
        AgentRegistryService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AgentRegistryService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentRegistryService { }

impl terrars::ToListMappable for AgentRegistryService {
    type O = terrars::ListRef<AgentRegistryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentRegistryService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_agent_registry_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentRegistryService {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Service."]
    pub service_id: terrars::PrimField<std::string::String>,
}

impl BuildAgentRegistryService {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentRegistryService {
        let out = AgentRegistryService(std::rc::Rc::new(AgentRegistryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentRegistryServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_id: self.service_id,
                agent_spec: core::default::Default::default(),
                endpoint_spec: core::default::Default::default(),
                interfaces: core::default::Default::default(),
                mcp_server_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentRegistryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentRegistryServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Service."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-defined display name for the Service. Can have a maximum length of 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_resource` after provisioning.\nThe resource name of the resulting Agent, MCP Server, or Endpoint."]
    pub fn registry_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe name of the Service."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_spec` after provisioning.\n"]
    pub fn agent_spec(&self) -> terrars::ListRef<AgentRegistryService_AgentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<AgentRegistryService_EndpointSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interfaces` after provisioning.\n"]
    pub fn interfaces(&self) -> terrars::ListRef<AgentRegistryService_InterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_server_spec` after provisioning.\n"]
    pub fn mcp_server_spec(&self) -> terrars::ListRef<AgentRegistryService_McpServerSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_server_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentRegistryService_TimeoutsRef {
        AgentRegistryService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgentRegistryServiceDynamic {
    agent_spec: std::option::Option<terrars::DynamicBlock<AgentRegistryService_AgentSpec>>,
    endpoint_spec: std::option::Option<terrars::DynamicBlock<AgentRegistryService_EndpointSpec>>,
    interfaces: std::option::Option<terrars::DynamicBlock<AgentRegistryService_Interfaces>>,
    mcp_server_spec: std::option::Option<terrars::DynamicBlock<AgentRegistryService_McpServerSpec>>,
}

#[derive(Serialize)]
pub struct AgentRegistryService_AgentSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AgentRegistryService_AgentSpec {
    #[doc =
        "Set the field `content`.\nThe content of the Agent spec in the JSON format. This payload is validated against the schema for the specified type."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentRegistryService_AgentSpec {
    type O = terrars::BlockAssignable<AgentRegistryService_AgentSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentRegistryService_AgentSpec {
    #[doc = "The type of the Agent spec content. Possible values: [\"NO_SPEC\", \"A2A_AGENT_CARD\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAgentRegistryService_AgentSpec {
    pub fn build(self) -> AgentRegistryService_AgentSpec {
        AgentRegistryService_AgentSpec {
            content: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct AgentRegistryService_AgentSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryService_AgentSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentRegistryService_AgentSpecRef {
        AgentRegistryService_AgentSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentRegistryService_AgentSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe content of the Agent spec in the JSON format. This payload is validated against the schema for the specified type."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the Agent spec content. Possible values: [\"NO_SPEC\", \"A2A_AGENT_CARD\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentRegistryService_EndpointSpec {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AgentRegistryService_EndpointSpec { }

impl terrars::ToListMappable for AgentRegistryService_EndpointSpec {
    type O = terrars::BlockAssignable<AgentRegistryService_EndpointSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentRegistryService_EndpointSpec {
    #[doc = "The type of the Endpoint spec content. Possible values: [\"NO_SPEC\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAgentRegistryService_EndpointSpec {
    pub fn build(self) -> AgentRegistryService_EndpointSpec {
        AgentRegistryService_EndpointSpec { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct AgentRegistryService_EndpointSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryService_EndpointSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentRegistryService_EndpointSpecRef {
        AgentRegistryService_EndpointSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentRegistryService_EndpointSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the Endpoint spec content. Possible values: [\"NO_SPEC\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentRegistryService_Interfaces {
    protocol_binding: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl AgentRegistryService_Interfaces { }

impl terrars::ToListMappable for AgentRegistryService_Interfaces {
    type O = terrars::BlockAssignable<AgentRegistryService_Interfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentRegistryService_Interfaces {
    #[doc = "The protocol binding of the interface. Possible values: [\"JSONRPC\", \"GRPC\", \"HTTP_JSON\"]"]
    pub protocol_binding: terrars::PrimField<std::string::String>,
    #[doc = "The destination URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildAgentRegistryService_Interfaces {
    pub fn build(self) -> AgentRegistryService_Interfaces {
        AgentRegistryService_Interfaces {
            protocol_binding: self.protocol_binding,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct AgentRegistryService_InterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryService_InterfacesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentRegistryService_InterfacesRef {
        AgentRegistryService_InterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentRegistryService_InterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `protocol_binding` after provisioning.\nThe protocol binding of the interface. Possible values: [\"JSONRPC\", \"GRPC\", \"HTTP_JSON\"]"]
    pub fn protocol_binding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_binding", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe destination URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentRegistryService_McpServerSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AgentRegistryService_McpServerSpec {
    #[doc =
        "Set the field `content`.\nThe content of the MCP Server spec. This payload is validated against the schema for the specified type."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentRegistryService_McpServerSpec {
    type O = terrars::BlockAssignable<AgentRegistryService_McpServerSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentRegistryService_McpServerSpec {
    #[doc = "The type of the MCP Server spec content. Possible values: [\"NO_SPEC\", \"TOOL_SPEC\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAgentRegistryService_McpServerSpec {
    pub fn build(self) -> AgentRegistryService_McpServerSpec {
        AgentRegistryService_McpServerSpec {
            content: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct AgentRegistryService_McpServerSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryService_McpServerSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentRegistryService_McpServerSpecRef {
        AgentRegistryService_McpServerSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentRegistryService_McpServerSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe content of the MCP Server spec. This payload is validated against the schema for the specified type."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the MCP Server spec content. Possible values: [\"NO_SPEC\", \"TOOL_SPEC\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentRegistryService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentRegistryService_Timeouts {
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

impl terrars::ToListMappable for AgentRegistryService_Timeouts {
    type O = terrars::BlockAssignable<AgentRegistryService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentRegistryService_Timeouts {}

impl BuildAgentRegistryService_Timeouts {
    pub fn build(self) -> AgentRegistryService_Timeouts {
        AgentRegistryService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentRegistryService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentRegistryService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentRegistryService_TimeoutsRef {
        AgentRegistryService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentRegistryService_TimeoutsRef {
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
