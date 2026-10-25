#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesAgentConnectivityTemplateData_ {
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
    access_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_compute: std::option::Option<terrars::PrimField<std::string::String>>,
    agent_connectivity_template_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_network_config: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesAgentConnectivityTemplate_Timeouts>,
    dynamic: NetworkServicesAgentConnectivityTemplateDynamic,
}

struct NetworkServicesAgentConnectivityTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesAgentConnectivityTemplateData_>,
}

#[derive(Clone)]
pub struct NetworkServicesAgentConnectivityTemplate(std::rc::Rc<NetworkServicesAgentConnectivityTemplate_>);

impl NetworkServicesAgentConnectivityTemplate {
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
        "Set the field `access_types`.\nThe types of network access provided to the gateway.\nBoth PUBLIC and PRIVATE can be configured. Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn set_access_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().access_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `agent_compute`.\nThe compute environment where the agent is hosted.\nExactly one type of compute must be chosen.\nPossible values are: 'GKE', 'CLOUD_RUN'."]
    pub fn set_agent_compute(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_compute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deployment_model`.\nThe deployment model for the gateway.\nPossible values are: 'CENTRALIZED', 'AMBIENT'."]
    pub fn set_deployment_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_model = std::option::Option::Some(v.into());
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
        "Set the field `labels`.\nSet of label tags associated with the AgentConnectivityTemplate resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `egress_network_config`.\n"]
    pub fn set_egress_network_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesAgentConnectivityTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_path` after provisioning.\nThe path of the access.\nThe path is immutable once set. Exactly one path can be set. Possible values: [\"CLIENT_TO_AGENT\", \"AGENT_TO_ANYWHERE\"]"]
    pub fn access_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `access_types` after provisioning.\nThe types of network access provided to the gateway.\nBoth PUBLIC and PRIVATE can be configured. Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn access_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_compute` after provisioning.\nThe compute environment where the agent is hosted.\nExactly one type of compute must be chosen.\nPossible values are: 'GKE', 'CLOUD_RUN'."]
    pub fn agent_compute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_compute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_connectivity_template_id` after provisioning.\nShort name of the AgentConnectivityTemplate resource."]
    pub fn agent_connectivity_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_connectivity_template_id", self.extract_ref()),
        )
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
        "Get a reference to the value of field `deployment_model` after provisioning.\nThe deployment model for the gateway.\nPossible values are: 'CENTRALIZED', 'AMBIENT'."]
    pub fn deployment_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_model", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the AgentConnectivityTemplate resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the AgentConnectivityTemplate."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `egress_network_config` after provisioning.\n"]
    pub fn egress_network_config(
        &self,
    ) -> terrars::ListRef<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
        NetworkServicesAgentConnectivityTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesAgentConnectivityTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesAgentConnectivityTemplate { }

impl terrars::ToListMappable for NetworkServicesAgentConnectivityTemplate {
    type O = terrars::ListRef<NetworkServicesAgentConnectivityTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesAgentConnectivityTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_agent_connectivity_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesAgentConnectivityTemplate {
    pub tf_id: std::string::String,
    #[doc =
        "The path of the access.\nThe path is immutable once set. Exactly one path can be set. Possible values: [\"CLIENT_TO_AGENT\", \"AGENT_TO_ANYWHERE\"]"]
    pub access_path: terrars::PrimField<std::string::String>,
    #[doc = "Short name of the AgentConnectivityTemplate resource."]
    pub agent_connectivity_template_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the AgentConnectivityTemplate."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentConnectivityTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesAgentConnectivityTemplate {
        let out =
            NetworkServicesAgentConnectivityTemplate(std::rc::Rc::new(NetworkServicesAgentConnectivityTemplate_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(NetworkServicesAgentConnectivityTemplateData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    access_path: self.access_path,
                    access_types: core::default::Default::default(),
                    agent_compute: core::default::Default::default(),
                    agent_connectivity_template_id: self.agent_connectivity_template_id,
                    deletion_policy: core::default::Default::default(),
                    deployment_model: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    egress_network_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesAgentConnectivityTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentConnectivityTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesAgentConnectivityTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_path` after provisioning.\nThe path of the access.\nThe path is immutable once set. Exactly one path can be set. Possible values: [\"CLIENT_TO_AGENT\", \"AGENT_TO_ANYWHERE\"]"]
    pub fn access_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `access_types` after provisioning.\nThe types of network access provided to the gateway.\nBoth PUBLIC and PRIVATE can be configured. Possible values: [\"PUBLIC\", \"PRIVATE\"]"]
    pub fn access_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_compute` after provisioning.\nThe compute environment where the agent is hosted.\nExactly one type of compute must be chosen.\nPossible values are: 'GKE', 'CLOUD_RUN'."]
    pub fn agent_compute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_compute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `agent_connectivity_template_id` after provisioning.\nShort name of the AgentConnectivityTemplate resource."]
    pub fn agent_connectivity_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.agent_connectivity_template_id", self.extract_ref()),
        )
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
        "Get a reference to the value of field `deployment_model` after provisioning.\nThe deployment model for the gateway.\nPossible values are: 'CENTRALIZED', 'AMBIENT'."]
    pub fn deployment_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_model", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the AgentConnectivityTemplate resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the AgentConnectivityTemplate."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `egress_network_config` after provisioning.\n"]
    pub fn egress_network_config(
        &self,
    ) -> terrars::ListRef<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
        NetworkServicesAgentConnectivityTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesAgentConnectivityTemplateDynamic {
    egress_network_config: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig>>,
}

#[derive(Serialize)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    target_network: terrars::PrimField<std::string::String>,
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
    #[doc =
        "Set the field `domain`.\nThe domain name to peer for DNS resolution. Must be a fully\nqualified domain name ending with a dot (for example, 'example.com.')."]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `domains`.\nThe list of domain names to peer for DNS resolution. Each entry\nmust be a fully qualified domain name ending with a dot\n(for example, 'example.com.'). At least one domain must be\nspecified between 'domain' and 'domains'."]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
    type O = terrars::BlockAssignable<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
    #[doc =
        "The URI of the target VPC network for DNS peering. Must be of the\nform 'projects/{project}/global/networks/{network}'."]
    pub target_network: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
    pub fn build(self) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig {
            domain: core::default::Default::default(),
            domains: core::default::Default::default(),
            target_network: self.target_network,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name to peer for DNS resolution. Must be a fully\nqualified domain name ending with a dot (for example, 'example.com.')."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nThe list of domain names to peer for DNS resolution. Each entry\nmust be a fully qualified domain name ending with a dot\n(for example, 'example.com.'). At least one domain must be\nspecified between 'domain' and 'domains'."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_network` after provisioning.\nThe URI of the target VPC network for DNS peering. Must be of the\nform 'projects/{project}/global/networks/{network}'."]
    pub fn target_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_network", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
    additional_roots: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
    #[doc =
        "Set the field `trust_config`.\nThe trust config resource name.\nFormat: projects/{project}/locations/{location}/trustConfigs/{trust_config}"]
    pub fn set_trust_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
    type O = terrars::BlockAssignable<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
    #[doc =
        "Defines whether additional roots should be trusted. Possible values: [\"NO_ADDITIONAL_ROOTS\", \"PUBLICLY_TRUSTED_ROOTS\"]"]
    pub additional_roots: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
    pub fn build(self) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig {
            additional_roots: self.additional_roots,
            trust_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_roots` after provisioning.\nDefines whether additional roots should be trusted. Possible values: [\"NO_ADDITIONAL_ROOTS\", \"PUBLICLY_TRUSTED_ROOTS\"]"]
    pub fn additional_roots(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_roots", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_config` after provisioning.\nThe trust config resource name.\nFormat: projects/{project}/locations/{location}/trustConfigs/{trust_config}"]
    pub fn trust_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigDynamic {
    dns_peering_config: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig>>,
    tls_config: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig>>,
}

#[derive(Serialize)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_peering_config: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig>>,
    dynamic: NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigDynamic,
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
    #[doc =
        "Set the field `network_attachment`.\nThe network attachment resource name.\nFormat: projects/{project}/regions/{region}/networkAttachments/{network_attachment_id}"]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpc_egress`.\nThe VPC egress setting. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn set_vpc_egress(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_peering_config`.\n"]
    pub fn set_dns_peering_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfig,
                        >,
                    >,
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

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
    type O = terrars::BlockAssignable<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {}

impl BuildNetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
    pub fn build(self) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig {
            network_attachment: core::default::Default::default(),
            vpc_egress: core::default::Default::default(),
            dns_peering_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef {
        NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentConnectivityTemplate_EgressNetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe network attachment resource name.\nFormat: projects/{project}/regions/{region}/networkAttachments/{network_attachment_id}"]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_egress` after provisioning.\nThe VPC egress setting. Possible values: [\"ALL_TRAFFIC\", \"PRIVATE_RANGES_ONLY\"]"]
    pub fn vpc_egress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_egress", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_peering_config` after provisioning.\n"]
    pub fn dns_peering_config(
        &self,
    ) -> terrars::ListRef<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_DnsPeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_peering_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(
        &self,
    ) -> terrars::ListRef<NetworkServicesAgentConnectivityTemplate_EgressNetworkConfig_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesAgentConnectivityTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesAgentConnectivityTemplate_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesAgentConnectivityTemplate_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesAgentConnectivityTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesAgentConnectivityTemplate_Timeouts {}

impl BuildNetworkServicesAgentConnectivityTemplate_Timeouts {
    pub fn build(self) -> NetworkServicesAgentConnectivityTemplate_Timeouts {
        NetworkServicesAgentConnectivityTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
        NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesAgentConnectivityTemplate_TimeoutsRef {
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
