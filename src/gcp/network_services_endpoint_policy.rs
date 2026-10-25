#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesEndpointPolicyData_ {
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
    authorization_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_matcher: std::option::Option<std::vec::Vec<NetworkServicesEndpointPolicy_EndpointMatcher>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesEndpointPolicy_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_port_selector: std::option::Option<std::vec::Vec<NetworkServicesEndpointPolicy_TrafficPortSelector>>,
    dynamic: NetworkServicesEndpointPolicyDynamic,
}

struct NetworkServicesEndpointPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesEndpointPolicyData_>,
}

#[derive(Clone)]
pub struct NetworkServicesEndpointPolicy(std::rc::Rc<NetworkServicesEndpointPolicy_>);

impl NetworkServicesEndpointPolicy {
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
        "Set the field `authorization_policy`.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn set_authorization_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorization_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_tls_policy`.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn set_client_tls_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_tls_policy = std::option::Option::Some(v.into());
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
        "Set the field `labels`.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc =
        "Set the field `server_tls_policy`.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn set_server_tls_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_tls_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_matcher`.\n"]
    pub fn set_endpoint_matcher(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEndpointPolicy_EndpointMatcher>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_matcher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_matcher = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesEndpointPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_port_selector`.\n"]
    pub fn set_traffic_port_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEndpointPolicy_TrafficPortSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_port_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_port_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `authorization_policy` after provisioning.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn authorization_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_tls_policy` after provisioning.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn client_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_tls_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the EndpointPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn server_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_tls_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_matcher` after provisioning.\n"]
    pub fn endpoint_matcher(&self) -> terrars::ListRef<NetworkServicesEndpointPolicy_EndpointMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEndpointPolicy_TimeoutsRef {
        NetworkServicesEndpointPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_port_selector` after provisioning.\n"]
    pub fn traffic_port_selector(&self) -> terrars::ListRef<NetworkServicesEndpointPolicy_TrafficPortSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_port_selector", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkServicesEndpointPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesEndpointPolicy { }

impl terrars::ToListMappable for NetworkServicesEndpointPolicy {
    type O = terrars::ListRef<NetworkServicesEndpointPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesEndpointPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_endpoint_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesEndpointPolicy {
    pub tf_id: std::string::String,
    #[doc = "Name of the EndpointPolicy resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEndpointPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesEndpointPolicy {
        let out = NetworkServicesEndpointPolicy(std::rc::Rc::new(NetworkServicesEndpointPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesEndpointPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorization_policy: core::default::Default::default(),
                client_tls_policy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                server_tls_policy: core::default::Default::default(),
                type_: self.type_,
                endpoint_matcher: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic_port_selector: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesEndpointPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesEndpointPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorization_policy` after provisioning.\nThis field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints."]
    pub fn authorization_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_tls_policy` after provisioning.\nA URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints."]
    pub fn client_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_tls_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the TcpRoute was created in UTC."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the TcpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the EndpointPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_tls_policy` after provisioning.\nA URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends."]
    pub fn server_tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_tls_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of endpoint policy. This is primarily used to validate the configuration. Possible values: [\"SIDECAR_PROXY\", \"GRPC_SERVER\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the TcpRoute was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_matcher` after provisioning.\n"]
    pub fn endpoint_matcher(&self) -> terrars::ListRef<NetworkServicesEndpointPolicy_EndpointMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEndpointPolicy_TimeoutsRef {
        NetworkServicesEndpointPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_port_selector` after provisioning.\n"]
    pub fn traffic_port_selector(&self) -> terrars::ListRef<NetworkServicesEndpointPolicy_TrafficPortSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_port_selector", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicyDynamic {
    endpoint_matcher: std::option::Option<terrars::DynamicBlock<NetworkServicesEndpointPolicy_EndpointMatcher>>,
    traffic_port_selector: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEndpointPolicy_TrafficPortSelector>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
    label_name: terrars::PrimField<std::string::String>,
    label_value: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels { }

impl terrars::ToListMappable for NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
    type O =
        terrars::BlockAssignable<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
    #[doc = "Required. Label name presented as key in xDS Node Metadata."]
    pub label_name: terrars::PrimField<std::string::String>,
    #[doc = "Required. Label value presented as value corresponding to the above key, in xDS Node Metadata."]
    pub label_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
    pub fn build(self) -> NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
        NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels {
            label_name: self.label_name,
            label_value: self.label_value,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef {
        NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label_name` after provisioning.\nRequired. Label name presented as key in xDS Node Metadata."]
    pub fn label_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `label_value` after provisioning.\nRequired. Label value presented as value corresponding to the above key, in xDS Node Metadata."]
    pub fn label_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherDynamic {
    metadata_labels: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels>,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
    metadata_label_match_criteria: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_labels: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels>>,
    dynamic: NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherDynamic,
}

impl NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
    #[doc = "Set the field `metadata_labels`.\n"]
    pub fn set_metadata_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabels,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
    type O = terrars::BlockAssignable<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
    #[doc = "Specifies how matching should be done. Possible values: [\"MATCH_ANY\", \"MATCH_ALL\"]"]
    pub metadata_label_match_criteria: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
    pub fn build(self) -> NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
        NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher {
            metadata_label_match_criteria: self.metadata_label_match_criteria,
            metadata_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef {
        NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata_label_match_criteria` after provisioning.\nSpecifies how matching should be done. Possible values: [\"MATCH_ANY\", \"MATCH_ALL\"]"]
    pub fn metadata_label_match_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_label_match_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_labels` after provisioning.\n"]
    pub fn metadata_labels(
        &self,
    ) -> terrars::ListRef<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher_MetadataLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEndpointPolicy_EndpointMatcherDynamic {
    metadata_label_matcher: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcher {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_label_matcher: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher>>,
    dynamic: NetworkServicesEndpointPolicy_EndpointMatcherDynamic,
}

impl NetworkServicesEndpointPolicy_EndpointMatcher {
    #[doc = "Set the field `metadata_label_matcher`.\n"]
    pub fn set_metadata_label_matcher(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcher>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_label_matcher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_label_matcher = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEndpointPolicy_EndpointMatcher {
    type O = terrars::BlockAssignable<NetworkServicesEndpointPolicy_EndpointMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEndpointPolicy_EndpointMatcher {}

impl BuildNetworkServicesEndpointPolicy_EndpointMatcher {
    pub fn build(self) -> NetworkServicesEndpointPolicy_EndpointMatcher {
        NetworkServicesEndpointPolicy_EndpointMatcher {
            metadata_label_matcher: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEndpointPolicy_EndpointMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicy_EndpointMatcherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEndpointPolicy_EndpointMatcherRef {
        NetworkServicesEndpointPolicy_EndpointMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEndpointPolicy_EndpointMatcherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_label_matcher` after provisioning.\n"]
    pub fn metadata_label_matcher(
        &self,
    ) -> terrars::ListRef<NetworkServicesEndpointPolicy_EndpointMatcher_MetadataLabelMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_label_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEndpointPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEndpointPolicy_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesEndpointPolicy_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesEndpointPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEndpointPolicy_Timeouts {}

impl BuildNetworkServicesEndpointPolicy_Timeouts {
    pub fn build(self) -> NetworkServicesEndpointPolicy_Timeouts {
        NetworkServicesEndpointPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEndpointPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEndpointPolicy_TimeoutsRef {
        NetworkServicesEndpointPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEndpointPolicy_TimeoutsRef {
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
pub struct NetworkServicesEndpointPolicy_TrafficPortSelector {
    ports: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEndpointPolicy_TrafficPortSelector { }

impl terrars::ToListMappable for NetworkServicesEndpointPolicy_TrafficPortSelector {
    type O = terrars::BlockAssignable<NetworkServicesEndpointPolicy_TrafficPortSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEndpointPolicy_TrafficPortSelector {
    #[doc =
        "List of ports. Can be port numbers or port range (example, [80-90] specifies all ports from 80 to 90, including 80 and 90) or named ports or * to specify all ports. If the list is empty, all ports are selected."]
    pub ports: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkServicesEndpointPolicy_TrafficPortSelector {
    pub fn build(self) -> NetworkServicesEndpointPolicy_TrafficPortSelector {
        NetworkServicesEndpointPolicy_TrafficPortSelector { ports: self.ports }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEndpointPolicy_TrafficPortSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEndpointPolicy_TrafficPortSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEndpointPolicy_TrafficPortSelectorRef {
        NetworkServicesEndpointPolicy_TrafficPortSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEndpointPolicy_TrafficPortSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nList of ports. Can be port numbers or port range (example, [80-90] specifies all ports from 80 to 90, including 80 and 90) or named ports or * to specify all ports. If the list is empty, all ports are selected."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
