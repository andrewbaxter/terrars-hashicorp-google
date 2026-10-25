#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesAgentGatewayData_ {
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
    agent_connectivity_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registries: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_managed: std::option::Option<std::vec::Vec<NetworkServicesAgentGateway_GoogleManaged>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<NetworkServicesAgentGateway_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed: std::option::Option<std::vec::Vec<NetworkServicesAgentGateway_SelfManaged>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesAgentGateway_Timeouts>,
    dynamic: NetworkServicesAgentGatewayDynamic,
}

struct NetworkServicesAgentGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesAgentGatewayData_>,
}

#[derive(Clone)]
pub struct NetworkServicesAgentGateway(std::rc::Rc<NetworkServicesAgentGateway_>);

impl NetworkServicesAgentGateway {
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
        "Set the field `agent_connectivity_template`.\nThe resource name of the AgentConnectivityTemplate.\nMust be of format\n'projects/{{project}}/locations/{{location}}/agentConnectivityTemplates/{{agent_connectivity_template}}'"]
    pub fn set_agent_connectivity_template(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().agent_connectivity_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nSet of label tags associated with the AgentGateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\nList of protocols supported by an Agent Gateway. Possible values: [\"MCP\"]"]
    pub fn set_protocols(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `registries`.\nA list of Agent registries containing the agents, MCP servers and tools governed by the Agent Gateway.\nNote: Currently limited to project-scoped registries Must be of format\n'//agentregistry.googleapis.com/{version}/projects/{{project}}/locations/{{location}}'"]
    pub fn set_registries(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().registries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_managed`.\n"]
    pub fn set_google_managed(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesAgentGateway_GoogleManaged>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().google_managed = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.google_managed = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed`.\n"]
    pub fn set_self_managed(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesAgentGateway_SelfManaged>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesAgentGateway_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `agent_connectivity_template` after provisioning.\nThe resource name of the AgentConnectivityTemplate.\nMust be of format\n'projects/{{project}}/locations/{{location}}/agentConnectivityTemplates/{{agent_connectivity_template}}'"]
    pub fn agent_connectivity_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_connectivity_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_gateway_card` after provisioning.\nAgentGatewayOutputCard contains informational output-only fields."]
    pub fn agent_gateway_card(&self) -> terrars::ListRef<NetworkServicesAgentGateway_AgentGatewayCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_gateway_card", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag of the resource.\nIf this is provided, it must match the server's etag. If the provided etag\ndoes not match the server's etag, the request will fail with a 409 ABORTED\nerror."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the AgentGateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the agent gateway."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the AgentGateway resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocols` after provisioning.\nList of protocols supported by an Agent Gateway. Possible values: [\"MCP\"]"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registries` after provisioning.\nA list of Agent registries containing the agents, MCP servers and tools governed by the Agent Gateway.\nNote: Currently limited to project-scoped registries Must be of format\n'//agentregistry.googleapis.com/{version}/projects/{{project}}/locations/{{location}}'"]
    pub fn registries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_managed` after provisioning.\n"]
    pub fn google_managed(&self) -> terrars::ListRef<NetworkServicesAgentGateway_GoogleManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<NetworkServicesAgentGateway_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> terrars::ListRef<NetworkServicesAgentGateway_SelfManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAgentGateway_TimeoutsRef {
        NetworkServicesAgentGateway_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesAgentGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesAgentGateway { }

impl terrars::ToListMappable for NetworkServicesAgentGateway {
    type O = terrars::ListRef<NetworkServicesAgentGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesAgentGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_agent_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesAgentGateway {
    pub tf_id: std::string::String,
    #[doc = "The location of the agent gateway."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the AgentGateway resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesAgentGateway {
        let out = NetworkServicesAgentGateway(std::rc::Rc::new(NetworkServicesAgentGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesAgentGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_connectivity_template: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                protocols: core::default::Default::default(),
                registries: core::default::Default::default(),
                google_managed: core::default::Default::default(),
                network_config: core::default::Default::default(),
                self_managed: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesAgentGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesAgentGatewayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_connectivity_template` after provisioning.\nThe resource name of the AgentConnectivityTemplate.\nMust be of format\n'projects/{{project}}/locations/{{location}}/agentConnectivityTemplates/{{agent_connectivity_template}}'"]
    pub fn agent_connectivity_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_connectivity_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_gateway_card` after provisioning.\nAgentGatewayOutputCard contains informational output-only fields."]
    pub fn agent_gateway_card(&self) -> terrars::ListRef<NetworkServicesAgentGateway_AgentGatewayCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_gateway_card", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag of the resource.\nIf this is provided, it must match the server's etag. If the provided etag\ndoes not match the server's etag, the request will fail with a 409 ABORTED\nerror."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the AgentGateway resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the agent gateway."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the AgentGateway resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocols` after provisioning.\nList of protocols supported by an Agent Gateway. Possible values: [\"MCP\"]"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registries` after provisioning.\nA list of Agent registries containing the agents, MCP servers and tools governed by the Agent Gateway.\nNote: Currently limited to project-scoped registries Must be of format\n'//agentregistry.googleapis.com/{version}/projects/{{project}}/locations/{{location}}'"]
    pub fn registries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `google_managed` after provisioning.\n"]
    pub fn google_managed(&self) -> terrars::ListRef<NetworkServicesAgentGateway_GoogleManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<NetworkServicesAgentGateway_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> terrars::ListRef<NetworkServicesAgentGateway_SelfManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAgentGateway_TimeoutsRef {
        NetworkServicesAgentGateway_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesAgentGatewayDynamic {
    google_managed: std::option::Option<terrars::DynamicBlock<NetworkServicesAgentGateway_GoogleManaged>>,
    network_config: std::option::Option<terrars::DynamicBlock<NetworkServicesAgentGateway_NetworkConfig>>,
    self_managed: std::option::Option<terrars::DynamicBlock<NetworkServicesAgentGateway_SelfManaged>>,
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_AgentGatewayCard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_extensions_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesAgentGateway_AgentGatewayCard {
    #[doc = "Set the field `mtls_endpoint`.\n"]
    pub fn set_mtls_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mtls_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_certificates`.\n"]
    pub fn set_root_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.root_certificates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_extensions_service_account`.\n"]
    pub fn set_service_extensions_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_extensions_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesAgentGateway_AgentGatewayCard {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_AgentGatewayCard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_AgentGatewayCard {}

impl BuildNetworkServicesAgentGateway_AgentGatewayCard {
    pub fn build(self) -> NetworkServicesAgentGateway_AgentGatewayCard {
        NetworkServicesAgentGateway_AgentGatewayCard {
            mtls_endpoint: core::default::Default::default(),
            root_certificates: core::default::Default::default(),
            service_extensions_service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_AgentGatewayCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_AgentGatewayCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentGateway_AgentGatewayCardRef {
        NetworkServicesAgentGateway_AgentGatewayCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_AgentGatewayCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mtls_endpoint` after provisioning.\n"]
    pub fn mtls_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtls_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `root_certificates` after provisioning.\n"]
    pub fn root_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_certificates", self.base))
    }

    #[doc = "Get a reference to the value of field `service_extensions_service_account` after provisioning.\n"]
    pub fn service_extensions_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_extensions_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_GoogleManaged {
    governed_access_path: terrars::PrimField<std::string::String>,
}

impl NetworkServicesAgentGateway_GoogleManaged { }

impl terrars::ToListMappable for NetworkServicesAgentGateway_GoogleManaged {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_GoogleManaged>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_GoogleManaged {
    #[doc = "Operating Mode of Agent Gateway. Possible values: [\"AGENT_TO_ANYWHERE\", \"CLIENT_TO_AGENT\"]"]
    pub governed_access_path: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentGateway_GoogleManaged {
    pub fn build(self) -> NetworkServicesAgentGateway_GoogleManaged {
        NetworkServicesAgentGateway_GoogleManaged { governed_access_path: self.governed_access_path }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_GoogleManagedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_GoogleManagedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesAgentGateway_GoogleManagedRef {
        NetworkServicesAgentGateway_GoogleManagedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_GoogleManagedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `governed_access_path` after provisioning.\nOperating Mode of Agent Gateway. Possible values: [\"AGENT_TO_ANYWHERE\", \"CLIENT_TO_AGENT\"]"]
    pub fn governed_access_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.governed_access_path", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
    domains: terrars::ListField<terrars::PrimField<std::string::String>>,
    target_network: terrars::PrimField<std::string::String>,
    target_project: terrars::PrimField<std::string::String>,
}

impl NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig { }

impl terrars::ToListMappable for NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
    #[doc =
        "The list of domain names to peer for DNS resolution. Each entry\nmust be a fully qualified domain name ending with a dot\n(for example, 'example.com.')."]
    pub domains: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The URI of the target VPC network for DNS peering. Must be of the\nform 'projects/{project}/global/networks/{network}'."]
    pub target_network: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project that hosts the target VPC network for DNS\npeering."]
    pub target_project: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
    pub fn build(self) -> NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
        NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig {
            domains: self.domains,
            target_network: self.target_network,
            target_project: self.target_project,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef {
        NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nThe list of domain names to peer for DNS resolution. Each entry\nmust be a fully qualified domain name ending with a dot\n(for example, 'example.com.')."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_network` after provisioning.\nThe URI of the target VPC network for DNS peering. Must be of the\nform 'projects/{project}/global/networks/{network}'."]
    pub fn target_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_project` after provisioning.\nThe ID of the project that hosts the target VPC network for DNS\npeering."]
    pub fn target_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_project", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_NetworkConfig_Egress {
    network_attachment: terrars::PrimField<std::string::String>,
}

impl NetworkServicesAgentGateway_NetworkConfig_Egress { }

impl terrars::ToListMappable for NetworkServicesAgentGateway_NetworkConfig_Egress {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig_Egress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_NetworkConfig_Egress {
    #[doc = "The URI of the Network Attachment resource."]
    pub network_attachment: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentGateway_NetworkConfig_Egress {
    pub fn build(self) -> NetworkServicesAgentGateway_NetworkConfig_Egress {
        NetworkServicesAgentGateway_NetworkConfig_Egress { network_attachment: self.network_attachment }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_NetworkConfig_EgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_NetworkConfig_EgressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentGateway_NetworkConfig_EgressRef {
        NetworkServicesAgentGateway_NetworkConfig_EgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_NetworkConfig_EgressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe URI of the Network Attachment resource."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesAgentGateway_NetworkConfigDynamic {
    dns_peering_config: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig>>,
    egress: std::option::Option<terrars::DynamicBlock<NetworkServicesAgentGateway_NetworkConfig_Egress>>,
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_peering_config: std::option::Option<std::vec::Vec<NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<std::vec::Vec<NetworkServicesAgentGateway_NetworkConfig_Egress>>,
    dynamic: NetworkServicesAgentGateway_NetworkConfigDynamic,
}

impl NetworkServicesAgentGateway_NetworkConfig {
    #[doc = "Set the field `dns_peering_config`.\n"]
    pub fn set_dns_peering_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_peering_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_peering_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig_Egress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesAgentGateway_NetworkConfig {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_NetworkConfig {}

impl BuildNetworkServicesAgentGateway_NetworkConfig {
    pub fn build(self) -> NetworkServicesAgentGateway_NetworkConfig {
        NetworkServicesAgentGateway_NetworkConfig {
            dns_peering_config: core::default::Default::default(),
            egress: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesAgentGateway_NetworkConfigRef {
        NetworkServicesAgentGateway_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_peering_config` after provisioning.\n"]
    pub fn dns_peering_config(&self) -> terrars::ListRef<NetworkServicesAgentGateway_NetworkConfig_DnsPeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_peering_config", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::ListRef<NetworkServicesAgentGateway_NetworkConfig_EgressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_SelfManaged {
    resource_uri: terrars::PrimField<std::string::String>,
}

impl NetworkServicesAgentGateway_SelfManaged { }

impl terrars::ToListMappable for NetworkServicesAgentGateway_SelfManaged {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_SelfManaged>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_SelfManaged {
    #[doc = "A supported Google Cloud networking proxy in the Project and Location."]
    pub resource_uri: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentGateway_SelfManaged {
    pub fn build(self) -> NetworkServicesAgentGateway_SelfManaged {
        NetworkServicesAgentGateway_SelfManaged { resource_uri: self.resource_uri }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_SelfManagedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_SelfManagedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesAgentGateway_SelfManagedRef {
        NetworkServicesAgentGateway_SelfManagedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_SelfManagedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_uri` after provisioning.\nA supported Google Cloud networking proxy in the Project and Location."]
    pub fn resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentGateway_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesAgentGateway_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesAgentGateway_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesAgentGateway_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentGateway_Timeouts {}

impl BuildNetworkServicesAgentGateway_Timeouts {
    pub fn build(self) -> NetworkServicesAgentGateway_Timeouts {
        NetworkServicesAgentGateway_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentGateway_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentGateway_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesAgentGateway_TimeoutsRef {
        NetworkServicesAgentGateway_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentGateway_TimeoutsRef {
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
