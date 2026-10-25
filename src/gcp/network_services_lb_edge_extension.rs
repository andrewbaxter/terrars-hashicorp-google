#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesLbEdgeExtensionData_ {
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
    extension_chains: std::option::Option<std::vec::Vec<NetworkServicesLbEdgeExtension_ExtensionChains>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesLbEdgeExtension_Timeouts>,
    dynamic: NetworkServicesLbEdgeExtensionDynamic,
}

struct NetworkServicesLbEdgeExtension_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesLbEdgeExtensionData_>,
}

#[derive(Clone)]
pub struct NetworkServicesLbEdgeExtension(std::rc::Rc<NetworkServicesLbEdgeExtension_>);

impl NetworkServicesLbEdgeExtension {
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
        "Set the field `labels`.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesLbEdgeExtension_Timeouts>) -> Self {
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
        "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
    pub fn forwarding_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the edge extension"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
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
    pub fn extension_chains(&self) -> terrars::ListRef<NetworkServicesLbEdgeExtension_ExtensionChainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extension_chains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbEdgeExtension_TimeoutsRef {
        NetworkServicesLbEdgeExtension_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesLbEdgeExtension {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesLbEdgeExtension { }

impl terrars::ToListMappable for NetworkServicesLbEdgeExtension {
    type O = terrars::ListRef<NetworkServicesLbEdgeExtensionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesLbEdgeExtension_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_lb_edge_extension".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesLbEdgeExtension {
    pub tf_id: std::string::String,
    #[doc =
        "A list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
    pub forwarding_rules: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub load_balancing_scheme: terrars::PrimField<std::string::String>,
    #[doc = "The location of the edge extension"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbEdgeExtension {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesLbEdgeExtension {
        let out = NetworkServicesLbEdgeExtension(std::rc::Rc::new(NetworkServicesLbEdgeExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesLbEdgeExtensionData_ {
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

pub struct NetworkServicesLbEdgeExtensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbEdgeExtensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesLbEdgeExtensionRef {
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
        "Get a reference to the value of field `forwarding_rules` after provisioning.\nA list of references to the forwarding rules to which this service extension is attached.\nAt least one forwarding rule is required. Only one LbEdgeExtension resource can be associated with a forwarding rule."]
    pub fn forwarding_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the LbEdgeExtension resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nAll forwarding rules referenced by this extension must share the same load balancing scheme. Possible values: [\"EXTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the edge extension"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the LbEdgeExtension resource in the following format: projects/{project}/locations/{location}/lbEdgeExtensions/{lbEdgeExtensions}"]
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
    pub fn extension_chains(&self) -> terrars::ListRef<NetworkServicesLbEdgeExtension_ExtensionChainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extension_chains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesLbEdgeExtension_TimeoutsRef {
        NetworkServicesLbEdgeExtension_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesLbEdgeExtensionDynamic {
    extension_chains: std::option::Option<terrars::DynamicBlock<NetworkServicesLbEdgeExtension_ExtensionChains>>,
}

#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
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
        "Set the field `supported_events`.\nA set of events during request or response processing for which this extension is called.\nThis field is required for the LbEdgeExtension resource and only supports the value 'REQUEST_HEADERS'."]
    pub fn set_supported_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
    type O = terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains_Extensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
    #[doc =
        "The name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
    pub fn build(self) -> NetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
        NetworkServicesLbEdgeExtension_ExtensionChains_Extensions {
            fail_open: core::default::Default::default(),
            forward_attributes: core::default::Default::default(),
            forward_headers: core::default::Default::default(),
            name: self.name,
            service: self.service,
            supported_events: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef {
        NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `name` after provisioning.\nThe name for this extension. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last a letter or a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe reference to the service that runs the extension.\n\n* To configure a callout extension, service must be a fully-qualified reference to a backend service.\n* To configure a plugin extension, service must be a reference to a WasmPlugin resource."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported_events` after provisioning.\nA set of events during request or response processing for which this extension is called.\nThis field is required for the LbEdgeExtension resource and only supports the value 'REQUEST_HEADERS'."]
    pub fn supported_events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_events", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition {
    cel_expression: terrars::PrimField<std::string::String>,
}

impl NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition { }

impl terrars::ToListMappable for NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition {
    type O = terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition {
    #[doc =
        "A Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed."]
    pub cel_expression: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition {
    pub fn build(self) -> NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition {
        NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition { cel_expression: self.cel_expression }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef {
        NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef {
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
struct NetworkServicesLbEdgeExtension_ExtensionChainsDynamic {
    extensions: std::option::Option<terrars::DynamicBlock<NetworkServicesLbEdgeExtension_ExtensionChains_Extensions>>,
    match_condition: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition>>,
}

#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChains {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extensions: std::option::Option<std::vec::Vec<NetworkServicesLbEdgeExtension_ExtensionChains_Extensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_condition: std::option::Option<std::vec::Vec<NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition>>,
    dynamic: NetworkServicesLbEdgeExtension_ExtensionChainsDynamic,
}

impl NetworkServicesLbEdgeExtension_ExtensionChains {
    #[doc = "Set the field `extensions`.\n"]
    pub fn set_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains_Extensions>>,
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
                    ::Into<terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains_MatchCondition>>,
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

impl terrars::ToListMappable for NetworkServicesLbEdgeExtension_ExtensionChains {
    type O = terrars::BlockAssignable<NetworkServicesLbEdgeExtension_ExtensionChains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbEdgeExtension_ExtensionChains {
    #[doc =
        "The name for this extension chain. The name is logged as part of the HTTP request logs.\nThe name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,\nand can have a maximum length of 63 characters. Additionally, the first character must be a letter\nand the last character must be a letter or a number."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesLbEdgeExtension_ExtensionChains {
    pub fn build(self) -> NetworkServicesLbEdgeExtension_ExtensionChains {
        NetworkServicesLbEdgeExtension_ExtensionChains {
            name: self.name,
            extensions: core::default::Default::default(),
            match_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbEdgeExtension_ExtensionChainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbEdgeExtension_ExtensionChainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesLbEdgeExtension_ExtensionChainsRef {
        NetworkServicesLbEdgeExtension_ExtensionChainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbEdgeExtension_ExtensionChainsRef {
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
    pub fn extensions(&self) -> terrars::ListRef<NetworkServicesLbEdgeExtension_ExtensionChains_ExtensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `match_condition` after provisioning.\n"]
    pub fn match_condition(&self) -> terrars::ListRef<NetworkServicesLbEdgeExtension_ExtensionChains_MatchConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesLbEdgeExtension_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesLbEdgeExtension_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesLbEdgeExtension_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesLbEdgeExtension_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesLbEdgeExtension_Timeouts {}

impl BuildNetworkServicesLbEdgeExtension_Timeouts {
    pub fn build(self) -> NetworkServicesLbEdgeExtension_Timeouts {
        NetworkServicesLbEdgeExtension_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesLbEdgeExtension_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesLbEdgeExtension_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesLbEdgeExtension_TimeoutsRef {
        NetworkServicesLbEdgeExtension_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesLbEdgeExtension_TimeoutsRef {
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
