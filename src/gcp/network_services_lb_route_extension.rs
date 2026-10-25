#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesLbRouteExtensionData_ {
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
    forwarding_rules: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    load_balancing_scheme: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extension_chains: std::option::Option<std::vec::Vec<NetworkServicesLbRouteExtension_ExtensionChains>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesLbRouteExtension_Timeouts>,
    dynamic: NetworkServicesLbRouteExtensionDynamic,
}

struct NetworkServicesLbRouteExtension_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesLbRouteExtensionData_>,
}

#[derive(Clone)]
pub struct NetworkServicesLbRouteExtension(std::rc::Rc<NetworkServicesLbRouteExtension_>);

impl NetworkServicesLbRouteExtension {
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

    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
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
        "Set the field `labels`.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `extension_chains`.\n"]
    pub fn set_extension_chains(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extension_chains = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extension_chains = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesLbRouteExtension_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub fn forwarding_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the route extension"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
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

    #[doc = "Get a reference to the value of field `extension_chains` after provisioning.\n"]
    pub fn extension_chains(&self) -> terrars::ListRef<NetworkServicesLbRouteExtension_ExtensionChainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extension_chains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbRouteExtension_TimeoutsRef {
        NetworkServicesLbRouteExtension_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesLbRouteExtension {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesLbRouteExtension { }

impl terrars::ToListMappable for NetworkServicesLbRouteExtension {
    type O = terrars::ListRef<NetworkServicesLbRouteExtensionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesLbRouteExtension_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_lb_route_extension".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesLbRouteExtension {
    pub tf_id: std::string::String,
    #[doc =
        "A list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub forwarding_rules: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub load_balancing_scheme: terrars::PrimField<std::string::String>,
    #[doc = "The location of the route extension"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbRouteExtension {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesLbRouteExtension {
        let out = NetworkServicesLbRouteExtension(std::rc::Rc::new(NetworkServicesLbRouteExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesLbRouteExtensionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                forwarding_rules: self.forwarding_rules,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                load_balancing_scheme: self.load_balancing_scheme,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                extension_chains: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesLbRouteExtensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbRouteExtensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesLbRouteExtensionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached to.\nAt least one forwarding rule is required. There can be only one LbRouteExtension resource per forwarding rule."]
    pub fn forwarding_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbRouteExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll backend services and forwarding rules referenced by this extension must share the same load balancing scheme.\nFor more information, refer to [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service) and\n[Supported application load balancers](https://cloud.google.com/service-extensions/docs/callouts-overview#supported-lbs). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the route extension"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the LbRouteExtension resource in the following format: projects/{project}/locations/{location}/lbRouteExtensions/{lbRouteExtension}"]
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

    #[doc = "Get a reference to the value of field `extension_chains` after provisioning.\n"]
    pub fn extension_chains(&self) -> terrars::ListRef<NetworkServicesLbRouteExtension_ExtensionChainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extension_chains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbRouteExtension_TimeoutsRef {
        NetworkServicesLbRouteExtension_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesLbRouteExtensionDynamic {
    extension_chains: std::option::Option<terrars::DynamicBlock<NetworkServicesLbRouteExtension_ExtensionChains>>,
}

#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtension_ExtensionChains_Extensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body_send_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesLbRouteExtension_ExtensionChains_Extensions {
    #[doc =
        "Set the field `authority`.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn set_authority(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.authority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fail_open`.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error.\nAny subsequent extensions in the extension chain are also executed.\nWhen set to FALSE: * If response headers have not been delivered to the downstream client,\na generic 500 error is returned to the client. The error response can be tailored by\nconfiguring a custom error response in the load balancer."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forward_attributes`.\nList of the Envoy attributes to forward to the extension server. The attributes\nprovided here are included as part of the 'ProcessingRequest.attributes' field\n(of type 'map'), where the keys are the attribute names. Refer to the\n[documentation](https://docs.cloud.google.com/service-extensions/docs/attributes)\nfor the names of attributes that can be forwarded. If omitted, no attributes\nare sent. Each element is a string indicating the attribute name."]
    pub fn set_forward_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.forward_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forward_headers`.\nList of the HTTP headers to forward to the extension (from the client or backend).\nIf omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn set_forward_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.forward_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata`.\nThe metadata provided here is included as part of the 'metadata_context' (of type 'google.protobuf.Struct')\nin the 'ProcessingRequest' message sent to the extension server.\nThe metadata is available under the namespace 'com.google.lb_route_extension.<resource_name>.<chain_name>.<extension_name>'.\nThe following variables are supported in the metadata: '{forwarding_rule_id}' - substituted with the forwarding rule's fully qualified resource name.\nThis field must not be set for plugin extensions. Setting it results in a validation error."]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `observability_mode`.\nWhen set to 'TRUE', enables 'observability_mode' on the 'ext_proc' filter.\nThis makes 'ext_proc' calls asynchronous. Envoy doesn't check for the response from 'ext_proc' calls.\nFor more information about the filter, see: https://www.envoyproxy.io/docs/envoy/v1.32.3/api-v3/extensions/filters/http/ext_proc/v3/ext_proc.proto\nThis field is helpful when you want to try out the extension in async log-only mode.\nSupported by regional 'LbTrafficExtension' and 'LbRouteExtension' resources.\nOnly 'STREAMED' (default) body processing mode is supported."]
    pub fn set_observability_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.observability_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_body_send_mode`.\nConfigures the send mode for request body processing.\nThe field can only be set if 'supported_events' includes 'REQUEST_BODY'.\nIf 'supported_events' includes 'REQUEST_BODY', but 'request_body_send_mode' is unset, the default value 'STREAMED' is used.\nWhen this field is set to 'FULL_DUPLEX_STREAMED', 'supported_events' must include both 'REQUEST_BODY' and 'REQUEST_TRAILERS'.\nThis field can be set only when the 'service' field of the extension points to a 'BackendService'.\nOnly 'FULL_DUPLEX_STREAMED' mode is supported for 'LbRouteExtension' resources. Possible values: [\"BODY_SEND_MODE_UNSPECIFIED\", \"BODY_SEND_MODE_STREAMED\", \"BODY_SEND_MODE_FULL_DUPLEX_STREAMED\"]"]
    pub fn set_request_body_send_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.request_body_send_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `supported_events`.\nA set of events during request or response processing for which this extension is called.\nThis field is optional for the LbRouteExtension resource. If unspecified, 'REQUEST_HEADERS' event is assumed as supported.\nPossible values: 'REQUEST_HEADERS', 'REQUEST_BODY', 'REQUEST_TRAILERS'."]
    pub fn set_supported_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_events = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesLbRouteExtension_ExtensionChains_Extensions {
    type O = terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains_Extensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbRouteExtension_ExtensionChains_Extensions {
    #[doc =
        "The name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbRouteExtension_ExtensionChains_Extensions {
    pub fn build(self) -> NetworkServicesLbRouteExtension_ExtensionChains_Extensions {
        NetworkServicesLbRouteExtension_ExtensionChains_Extensions {
            authority: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            forward_attributes: core::default::Default::default(),
            forward_headers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: self.name,
            observability_mode: core::default::Default::default(),
            request_body_send_mode: core::default::Default::default(),
            service: self.service,
            supported_events: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef {
        NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authority` after provisioning.\nThe :authority header in the gRPC request sent from Envoy to the extension service."]
    pub fn authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nDetermines how the proxy behaves if the call to the extension fails or times out.\nWhen set to TRUE, request or response processing continues without error.\nAny subsequent extensions in the extension chain are also executed.\nWhen set to FALSE: * If response headers have not been delivered to the downstream client,\na generic 500 error is returned to the client. The error response can be tailored by\nconfiguring a custom error response in the load balancer."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forward_attributes` after provisioning.\nList of the Envoy attributes to forward to the extension server. The attributes\nprovided here are included as part of the 'ProcessingRequest.attributes' field\n(of type 'map'), where the keys are the attribute names. Refer to the\n[documentation](https://docs.cloud.google.com/service-extensions/docs/attributes)\nfor the names of attributes that can be forwarded. If omitted, no attributes\nare sent. Each element is a string indicating the attribute name."]
    pub fn forward_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forward_headers` after provisioning.\nList of the HTTP headers to forward to the extension (from the client or backend).\nIf omitted, all headers are sent. Each element is a string indicating the header name."]
    pub fn forward_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe metadata provided here is included as part of the 'metadata_context' (of type 'google.protobuf.Struct')\nin the 'ProcessingRequest' message sent to the extension server.\nThe metadata is available under the namespace 'com.google.lb_route_extension.<resource_name>.<chain_name>.<extension_name>'.\nThe following variables are supported in the metadata: '{forwarding_rule_id}' - substituted with the forwarding rule's fully qualified resource name.\nThis field must not be set for plugin extensions. Setting it results in a validation error."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `observability_mode` after provisioning.\nWhen set to 'TRUE', enables 'observability_mode' on the 'ext_proc' filter.\nThis makes 'ext_proc' calls asynchronous. Envoy doesn't check for the response from 'ext_proc' calls.\nFor more information about the filter, see: https://www.envoyproxy.io/docs/envoy/v1.32.3/api-v3/extensions/filters/http/ext_proc/v3/ext_proc.proto\nThis field is helpful when you want to try out the extension in async log-only mode.\nSupported by regional 'LbTrafficExtension' and 'LbRouteExtension' resources.\nOnly 'STREAMED' (default) body processing mode is supported."]
    pub fn observability_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observability_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body_send_mode` after provisioning.\nConfigures the send mode for request body processing.\nThe field can only be set if 'supported_events' includes 'REQUEST_BODY'.\nIf 'supported_events' includes 'REQUEST_BODY', but 'request_body_send_mode' is unset, the default value 'STREAMED' is used.\nWhen this field is set to 'FULL_DUPLEX_STREAMED', 'supported_events' must include both 'REQUEST_BODY' and 'REQUEST_TRAILERS'.\nThis field can be set only when the 'service' field of the extension points to a 'BackendService'.\nOnly 'FULL_DUPLEX_STREAMED' mode is supported for 'LbRouteExtension' resources. Possible values: [\"BODY_SEND_MODE_UNSPECIFIED\", \"BODY_SEND_MODE_STREAMED\", \"BODY_SEND_MODE_FULL_DUPLEX_STREAMED\"]"]
    pub fn request_body_send_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body_send_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported_events` after provisioning.\nA set of events during request or response processing for which this extension is called.\nThis field is optional for the LbRouteExtension resource. If unspecified, 'REQUEST_HEADERS' event is assumed as supported.\nPossible values: 'REQUEST_HEADERS', 'REQUEST_BODY', 'REQUEST_TRAILERS'."]
    pub fn supported_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_events", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition {
    cel_expression: terrars::PrimField<std::string::String>,
}

impl NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition { }

impl terrars::ToListMappable for NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition {
    type O = terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbRouteExtension_ExtensionChains_MatchCondition {
    #[doc =
        "A Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub cel_expression: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbRouteExtension_ExtensionChains_MatchCondition {
    pub fn build(self) -> NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition {
        NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition { cel_expression: self.cel_expression }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef {
        NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cel_expression` after provisioning.\nA Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub fn cel_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesLbRouteExtension_ExtensionChainsDynamic {
    extensions: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesLbRouteExtension_ExtensionChains_Extensions>>,
    match_condition: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition>>,
}

#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtension_ExtensionChains {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extensions: std::option::Option<std::vec::Vec<NetworkServicesLbRouteExtension_ExtensionChains_Extensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_condition: std::option::Option<std::vec::Vec<NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition>>,
    dynamic: NetworkServicesLbRouteExtension_ExtensionChainsDynamic,
}

impl NetworkServicesLbRouteExtension_ExtensionChains {
    #[doc = "Set the field `extensions`.\n"]
    pub fn set_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains_Extensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_condition`.\n"]
    pub fn set_match_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains_MatchCondition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesLbRouteExtension_ExtensionChains {
    type O = terrars::BlockAssignable<NetworkServicesLbRouteExtension_ExtensionChains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbRouteExtension_ExtensionChains {
    #[doc =
        "The name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbRouteExtension_ExtensionChains {
    pub fn build(self) -> NetworkServicesLbRouteExtension_ExtensionChains {
        NetworkServicesLbRouteExtension_ExtensionChains {
            name: self.name,
            extensions: core::default::Default::default(),
            match_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbRouteExtension_ExtensionChainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbRouteExtension_ExtensionChainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbRouteExtension_ExtensionChainsRef {
        NetworkServicesLbRouteExtension_ExtensionChainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbRouteExtension_ExtensionChainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `extensions` after provisioning.\n"]
    pub fn extensions(&self) -> terrars::ListRef<NetworkServicesLbRouteExtension_ExtensionChains_ExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `match_condition` after provisioning.\n"]
    pub fn match_condition(&self) -> terrars::ListRef<NetworkServicesLbRouteExtension_ExtensionChains_MatchConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesLbRouteExtension_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesLbRouteExtension_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesLbRouteExtension_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesLbRouteExtension_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbRouteExtension_Timeouts {}

impl BuildNetworkServicesLbRouteExtension_Timeouts {
    pub fn build(self) -> NetworkServicesLbRouteExtension_Timeouts {
        NetworkServicesLbRouteExtension_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbRouteExtension_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbRouteExtension_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesLbRouteExtension_TimeoutsRef {
        NetworkServicesLbRouteExtension_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbRouteExtension_TimeoutsRef {
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
