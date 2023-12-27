use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_create_subnetworks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_default_routes_on_create: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ula_internal_ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ipv6_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_firewall_policy_enforcement_order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeNetworkTimeoutsEl>,
}

struct ComputeNetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeNetworkData>,
}

#[derive(Clone)]
pub struct ComputeNetwork(Rc<ComputeNetwork_>);

impl ComputeNetwork {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `auto_create_subnetworks`.\nWhen set to 'true', the network is created in \"auto subnet mode\" and\nit will create a subnet for each region automatically across the\n'10.128.0.0/9' address range.\n\nWhen set to 'false', the network is created in \"custom subnet mode\" so\nthe user can explicitly connect subnetwork resources."]
    pub fn set_auto_create_subnetworks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_create_subnetworks = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_default_routes_on_create`.\nIf set to 'true', default routes ('0.0.0.0/0') will be deleted\nimmediately after network creation. Defaults to 'false'."]
    pub fn set_delete_default_routes_on_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_default_routes_on_create = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this resource. The resource must be\nrecreated to modify this field."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_ula_internal_ipv6`.\nEnable ULA internal ipv6 on this network. Enabling this feature will assign\na /48 from google defined ULA prefix fd20::/20."]
    pub fn set_enable_ula_internal_ipv6(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ula_internal_ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ipv6_range`.\nWhen enabling ula internal ipv6, caller optionally can specify the /48 range\nthey want from the google defined ULA prefix fd20::/20. The input must be a\nvalid /48 ULA IPv6 address and must be within the fd20::/20. Operation will\nfail if the speficied /48 is already in used by another resource.\nIf the field is not speficied, then a /48 range will be randomly allocated from fd20::/20 and returned via this field."]
    pub fn set_internal_ipv6_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().internal_ipv6_range = Some(v.into());
        self
    }

    #[doc= "Set the field `mtu`.\nMaximum Transmission Unit in bytes. The default value is 1460 bytes.\nThe minimum value for this field is 1300 and the maximum value is 8896 bytes (jumbo frames).\nNote that packets larger than 1500 bytes (standard Ethernet) can be subject to TCP-MSS clamping or dropped\nwith an ICMP 'Fragmentation-Needed' message if the packets are routed to the Internet or other VPCs\nwith varying MTUs."]
    pub fn set_mtu(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mtu = Some(v.into());
        self
    }

    #[doc= "Set the field `network_firewall_policy_enforcement_order`.\nSet the order that Firewall Rules and Firewall Policies are evaluated. Default value: \"AFTER_CLASSIC_FIREWALL\" Possible values: [\"BEFORE_CLASSIC_FIREWALL\", \"AFTER_CLASSIC_FIREWALL\"]"]
    pub fn set_network_firewall_policy_enforcement_order(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_firewall_policy_enforcement_order = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `routing_mode`.\nThe network-wide routing mode to use. If set to 'REGIONAL', this\nnetwork's cloud routers will only advertise routes with subnetworks\nof this network in the same region as the router. If set to 'GLOBAL',\nthis network's cloud routers will advertise routes with all\nsubnetworks of this network, across regions. Possible values: [\"REGIONAL\", \"GLOBAL\"]"]
    pub fn set_routing_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().routing_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeNetworkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `auto_create_subnetworks` after provisioning.\nWhen set to 'true', the network is created in \"auto subnet mode\" and\nit will create a subnet for each region automatically across the\n'10.128.0.0/9' address range.\n\nWhen set to 'false', the network is created in \"custom subnet mode\" so\nthe user can explicitly connect subnetwork resources."]
    pub fn auto_create_subnetworks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_create_subnetworks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_default_routes_on_create` after provisioning.\nIf set to 'true', default routes ('0.0.0.0/0') will be deleted\nimmediately after network creation. Defaults to 'false'."]
    pub fn delete_default_routes_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_default_routes_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. The resource must be\nrecreated to modify this field."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ula_internal_ipv6` after provisioning.\nEnable ULA internal ipv6 on this network. Enabling this feature will assign\na /48 from google defined ULA prefix fd20::/20."]
    pub fn enable_ula_internal_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ula_internal_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_ipv4` after provisioning.\nThe gateway address for default routing out of the network. This value\nis selected by GCP."]
    pub fn gateway_ipv4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_range` after provisioning.\nWhen enabling ula internal ipv6, caller optionally can specify the /48 range\nthey want from the google defined ULA prefix fd20::/20. The input must be a\nvalid /48 ULA IPv6 address and must be within the fd20::/20. Operation will\nfail if the speficied /48 is already in used by another resource.\nIf the field is not speficied, then a /48 range will be randomly allocated from fd20::/20 and returned via this field."]
    pub fn internal_ipv6_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit in bytes. The default value is 1460 bytes.\nThe minimum value for this field is 1300 and the maximum value is 8896 bytes (jumbo frames).\nNote that packets larger than 1500 bytes (standard Ethernet) can be subject to TCP-MSS clamping or dropped\nwith an ICMP 'Fragmentation-Needed' message if the packets are routed to the Internet or other VPCs\nwith varying MTUs."]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_firewall_policy_enforcement_order` after provisioning.\nSet the order that Firewall Rules and Firewall Policies are evaluated. Default value: \"AFTER_CLASSIC_FIREWALL\" Possible values: [\"BEFORE_CLASSIC_FIREWALL\", \"AFTER_CLASSIC_FIREWALL\"]"]
    pub fn network_firewall_policy_enforcement_order(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_firewall_policy_enforcement_order", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `numeric_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn numeric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.numeric_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `routing_mode` after provisioning.\nThe network-wide routing mode to use. If set to 'REGIONAL', this\nnetwork's cloud routers will only advertise routes with subnetworks\nof this network in the same region as the router. If set to 'GLOBAL',\nthis network's cloud routers will advertise routes with all\nsubnetworks of this network, across regions. Possible values: [\"REGIONAL\", \"GLOBAL\"]"]
    pub fn routing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkTimeoutsElRef {
        ComputeNetworkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeNetwork {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeNetwork { }

impl ToListMappable for ComputeNetwork {
    type O = ListRef<ComputeNetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeNetwork_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_network".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNetwork {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}

impl BuildComputeNetwork {
    pub fn build(self, stack: &mut Stack) -> ComputeNetwork {
        let out = ComputeNetwork(Rc::new(ComputeNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeNetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_create_subnetworks: core::default::Default::default(),
                delete_default_routes_on_create: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_ula_internal_ipv6: core::default::Default::default(),
                id: core::default::Default::default(),
                internal_ipv6_range: core::default::Default::default(),
                mtu: core::default::Default::default(),
                name: self.name,
                network_firewall_policy_enforcement_order: core::default::Default::default(),
                project: core::default::Default::default(),
                routing_mode: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeNetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNetworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_create_subnetworks` after provisioning.\nWhen set to 'true', the network is created in \"auto subnet mode\" and\nit will create a subnet for each region automatically across the\n'10.128.0.0/9' address range.\n\nWhen set to 'false', the network is created in \"custom subnet mode\" so\nthe user can explicitly connect subnetwork resources."]
    pub fn auto_create_subnetworks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_create_subnetworks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_default_routes_on_create` after provisioning.\nIf set to 'true', default routes ('0.0.0.0/0') will be deleted\nimmediately after network creation. Defaults to 'false'."]
    pub fn delete_default_routes_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_default_routes_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. The resource must be\nrecreated to modify this field."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ula_internal_ipv6` after provisioning.\nEnable ULA internal ipv6 on this network. Enabling this feature will assign\na /48 from google defined ULA prefix fd20::/20."]
    pub fn enable_ula_internal_ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ula_internal_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_ipv4` after provisioning.\nThe gateway address for default routing out of the network. This value\nis selected by GCP."]
    pub fn gateway_ipv4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_range` after provisioning.\nWhen enabling ula internal ipv6, caller optionally can specify the /48 range\nthey want from the google defined ULA prefix fd20::/20. The input must be a\nvalid /48 ULA IPv6 address and must be within the fd20::/20. Operation will\nfail if the speficied /48 is already in used by another resource.\nIf the field is not speficied, then a /48 range will be randomly allocated from fd20::/20 and returned via this field."]
    pub fn internal_ipv6_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit in bytes. The default value is 1460 bytes.\nThe minimum value for this field is 1300 and the maximum value is 8896 bytes (jumbo frames).\nNote that packets larger than 1500 bytes (standard Ethernet) can be subject to TCP-MSS clamping or dropped\nwith an ICMP 'Fragmentation-Needed' message if the packets are routed to the Internet or other VPCs\nwith varying MTUs."]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_firewall_policy_enforcement_order` after provisioning.\nSet the order that Firewall Rules and Firewall Policies are evaluated. Default value: \"AFTER_CLASSIC_FIREWALL\" Possible values: [\"BEFORE_CLASSIC_FIREWALL\", \"AFTER_CLASSIC_FIREWALL\"]"]
    pub fn network_firewall_policy_enforcement_order(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_firewall_policy_enforcement_order", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `numeric_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn numeric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.numeric_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `routing_mode` after provisioning.\nThe network-wide routing mode to use. If set to 'REGIONAL', this\nnetwork's cloud routers will only advertise routes with subnetworks\nof this network in the same region as the router. If set to 'GLOBAL',\nthis network's cloud routers will advertise routes with all\nsubnetworks of this network, across regions. Possible values: [\"REGIONAL\", \"GLOBAL\"]"]
    pub fn routing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkTimeoutsElRef {
        ComputeNetworkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeNetworkTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeNetworkTimeoutsEl {
    type O = BlockAssignable<ComputeNetworkTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkTimeoutsEl {}

impl BuildComputeNetworkTimeoutsEl {
    pub fn build(self) -> ComputeNetworkTimeoutsEl {
        ComputeNetworkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeNetworkTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeNetworkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNetworkTimeoutsElRef {
        ComputeNetworkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
