use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRouteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    dest_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouteTimeoutsEl>,
}

struct ComputeRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouteData>,
}

#[derive(Clone)]
pub struct ComputeRoute(Rc<ComputeRoute_>);

impl ComputeRoute {
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

    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_gateway`.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn set_next_hop_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ilb`.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn set_next_hop_ilb(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ilb = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance`.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn set_next_hop_instance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_instance = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance_zone`.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn set_next_hop_instance_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_instance_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ip`.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn set_next_hop_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_vpn_tunnel`.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn set_next_hop_vpn_tunnel(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_vpn_tunnel = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\nA list of instance tags to which this route applies."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_range` after provisioning.\nThe destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network that this route applies to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_gateway` after provisioning.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_ilb` after provisioning.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance` after provisioning.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_ip` after provisioning.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_network` after provisioning.\nURL to a Network that should handle matching packets."]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nA list of instance tags to which this route applies."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeRoute {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeRoute { }

impl ToListMappable for ComputeRoute {
    type O = ListRef<ComputeRouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRoute {
    pub tf_id: String,
    #[doc= "The destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub dest_range: PrimField<String>,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "The network that this route applies to."]
    pub network: PrimField<String>,
}

impl BuildComputeRoute {
    pub fn build(self, stack: &mut Stack) -> ComputeRoute {
        let out = ComputeRoute(Rc::new(ComputeRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                dest_range: self.dest_range,
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                next_hop_gateway: core::default::Default::default(),
                next_hop_ilb: core::default::Default::default(),
                next_hop_instance: core::default::Default::default(),
                next_hop_instance_zone: core::default::Default::default(),
                next_hop_ip: core::default::Default::default(),
                next_hop_vpn_tunnel: core::default::Default::default(),
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_range` after provisioning.\nThe destination range of outgoing packets that this route applies to.\nOnly IPv4 is supported."]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network that this route applies to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_gateway` after provisioning.\nURL to a gateway that should handle matching packets.\nCurrently, you can only specify the internet gateway, using a full or\npartial valid URL:\n* 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'\n* 'projects/project/global/gateways/default-internet-gateway'\n* 'global/gateways/default-internet-gateway'\n* The string 'default-internet-gateway'."]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_ilb` after provisioning.\nThe IP address or URL to a forwarding rule of type\nloadBalancingScheme=INTERNAL that should handle matching\npackets.\n\nWith the GA provider you can only specify the forwarding\nrule as a partial or full URL. For example, the following\nare all valid values:\n* 10.128.0.56\n* https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule\n* regions/region/forwardingRules/forwardingRule\n\nWhen the beta provider, you can also specify the IP address\nof a forwarding rule from the same VPC or any peered VPC.\n\nNote that this can only be used when the destinationRange is\na public (non-RFC 1918) IP CIDR range."]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance` after provisioning.\nURL to an instance that should handle matching packets.\nYou can specify this as a full or partial URL. For example:\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'\n* 'projects/project/zones/zone/instances/instance'\n* 'zones/zone/instances/instance'\n* Just the instance name, with the zone in 'next_hop_instance_zone'."]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\nThe zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL."]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_ip` after provisioning.\nNetwork IP address of an instance that should handle matching packets."]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_network` after provisioning.\nURL to a Network that should handle matching packets."]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\nURL to a VpnTunnel that should handle matching packets."]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority of this route. Priority is used to break ties in cases\nwhere there is more than one matching route of equal prefix length.\n\nIn the case of two routes with equal prefix length, the one with the\nlowest-numbered priority value wins.\n\nDefault value is 1000. Valid range is 0 through 65535."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nA list of instance tags to which this route applies."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ComputeRouteTimeoutsEl {
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
}

impl ToListMappable for ComputeRouteTimeoutsEl {
    type O = BlockAssignable<ComputeRouteTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouteTimeoutsEl {}

impl BuildComputeRouteTimeoutsEl {
    pub fn build(self) -> ComputeRouteTimeoutsEl {
        ComputeRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouteTimeoutsElRef {
        ComputeRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouteTimeoutsElRef {
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
}
