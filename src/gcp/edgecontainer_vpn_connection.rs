use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EdgecontainerVpnConnectionData {
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
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_high_availability: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EdgecontainerVpnConnectionTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_project: std::option::Option<std::vec::Vec<EdgecontainerVpnConnectionVpcProjectEl>>,
    dynamic: EdgecontainerVpnConnectionDynamic,
}

struct EdgecontainerVpnConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EdgecontainerVpnConnectionData>,
}

#[derive(Clone)]
pub struct EdgecontainerVpnConnection(std::rc::Rc<EdgecontainerVpnConnection_>);

impl EdgecontainerVpnConnection {
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
        "Set the field `enable_high_availability`.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn set_enable_high_availability(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_high_availability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nat_gateway_ip`.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn set_nat_gateway_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nat_gateway_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `router`.\nThe VPN connection Cloud Router name."]
    pub fn set_router(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().router = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\nThe network ID of VPC to connect to."]
    pub fn set_vpc(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EdgecontainerVpnConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_project`.\n"]
    pub fn set_vpc_project(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerVpnConnectionVpcProjectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_project = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_project = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the VPN connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\nA nested object resource"]
    pub fn details(&self) -> terrars::ListRef<EdgecontainerVpnConnectionDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_high_availability` after provisioning.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn enable_high_availability(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_high_availability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nGoogle Cloud Platform location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of VPN connection"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_gateway_ip` after provisioning.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn nat_gateway_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe VPN connection Cloud Router name."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the VPN connection was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe network ID of VPC to connect to."]
    pub fn vpc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_project` after provisioning.\n"]
    pub fn vpc_project(&self) -> terrars::ListRef<EdgecontainerVpnConnectionVpcProjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_project", self.extract_ref()))
    }
}

impl terrars::Referable for EdgecontainerVpnConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EdgecontainerVpnConnection { }

impl terrars::ToListMappable for EdgecontainerVpnConnection {
    type O = terrars::ListRef<EdgecontainerVpnConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EdgecontainerVpnConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_edgecontainer_vpn_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEdgecontainerVpnConnection {
    pub tf_id: std::string::String,
    #[doc =
        "The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "Google Cloud Platform location."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The resource name of VPN connection"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerVpnConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> EdgecontainerVpnConnection {
        let out = EdgecontainerVpnConnection(std::rc::Rc::new(EdgecontainerVpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EdgecontainerVpnConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: self.cluster,
                enable_high_availability: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                nat_gateway_ip: core::default::Default::default(),
                project: core::default::Default::default(),
                router: core::default::Default::default(),
                vpc: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_project: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EdgecontainerVpnConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EdgecontainerVpnConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the VPN connection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\nA nested object resource"]
    pub fn details(&self) -> terrars::ListRef<EdgecontainerVpnConnectionDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_high_availability` after provisioning.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn enable_high_availability(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_high_availability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nGoogle Cloud Platform location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of VPN connection"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_gateway_ip` after provisioning.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn nat_gateway_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe VPN connection Cloud Router name."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the VPN connection was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe network ID of VPC to connect to."]
    pub fn vpc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_project` after provisioning.\n"]
    pub fn vpc_project(&self) -> terrars::ListRef<EdgecontainerVpnConnectionVpcProjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    type O = terrars::BlockAssignable<EdgecontainerVpnConnectionDetailsElCloudRouterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerVpnConnectionDetailsElCloudRouterEl {}

impl BuildEdgecontainerVpnConnectionDetailsElCloudRouterEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsElCloudRouterEl {
        EdgecontainerVpnConnectionDetailsElCloudRouterEl { name: core::default::Default::default() }
    }
}

pub struct EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
        EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    type O = terrars::BlockAssignable<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerVpnConnectionDetailsElCloudVpnsEl {}

impl BuildEdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
        EdgecontainerVpnConnectionDetailsElCloudVpnsEl { gateway: core::default::Default::default() }
    }
}

pub struct EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
        EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_router: std::option::Option<terrars::ListField<EdgecontainerVpnConnectionDetailsElCloudRouterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_vpns: std::option::Option<terrars::ListField<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerVpnConnectionDetailsEl {
    #[doc = "Set the field `cloud_router`.\n"]
    pub fn set_cloud_router(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EdgecontainerVpnConnectionDetailsElCloudRouterEl>>,
    ) -> Self {
        self.cloud_router = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_vpns`.\n"]
    pub fn set_cloud_vpns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>>,
    ) -> Self {
        self.cloud_vpns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerVpnConnectionDetailsEl {
    type O = terrars::BlockAssignable<EdgecontainerVpnConnectionDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerVpnConnectionDetailsEl {}

impl BuildEdgecontainerVpnConnectionDetailsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsEl {
        EdgecontainerVpnConnectionDetailsEl {
            cloud_router: core::default::Default::default(),
            cloud_vpns: core::default::Default::default(),
            error: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerVpnConnectionDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerVpnConnectionDetailsElRef {
        EdgecontainerVpnConnectionDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerVpnConnectionDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_router` after provisioning.\n"]
    pub fn cloud_router(&self) -> terrars::ListRef<EdgecontainerVpnConnectionDetailsElCloudRouterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_router", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_vpns` after provisioning.\n"]
    pub fn cloud_vpns(&self) -> terrars::ListRef<EdgecontainerVpnConnectionDetailsElCloudVpnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_vpns", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerVpnConnectionTimeoutsEl {
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

impl terrars::ToListMappable for EdgecontainerVpnConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<EdgecontainerVpnConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerVpnConnectionTimeoutsEl {}

impl BuildEdgecontainerVpnConnectionTimeoutsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionTimeoutsEl {
        EdgecontainerVpnConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerVpnConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerVpnConnectionTimeoutsElRef {
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
pub struct EdgecontainerVpnConnectionVpcProjectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerVpnConnectionVpcProjectEl {
    #[doc =
        "Set the field `project_id`.\nThe project of the VPC to connect to. If not specified, it is the same as the cluster project."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerVpnConnectionVpcProjectEl {
    type O = terrars::BlockAssignable<EdgecontainerVpnConnectionVpcProjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerVpnConnectionVpcProjectEl {}

impl BuildEdgecontainerVpnConnectionVpcProjectEl {
    pub fn build(self) -> EdgecontainerVpnConnectionVpcProjectEl {
        EdgecontainerVpnConnectionVpcProjectEl { project_id: core::default::Default::default() }
    }
}

pub struct EdgecontainerVpnConnectionVpcProjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerVpnConnectionVpcProjectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerVpnConnectionVpcProjectElRef {
        EdgecontainerVpnConnectionVpcProjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerVpnConnectionVpcProjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project of the VPC to connect to. If not specified, it is the same as the cluster project."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerVpnConnectionDynamic {
    vpc_project: std::option::Option<terrars::DynamicBlock<EdgecontainerVpnConnectionVpcProjectEl>>,
}
