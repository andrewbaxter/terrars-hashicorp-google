use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRouterPeerData {
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
    advertise_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_route_priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    interface: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_nexthop_address: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    peer_asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ipv6_nexthop_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    router: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router_appliance_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertised_ip_ranges: std::option::Option<std::vec::Vec<ComputeRouterPeerAdvertisedIpRangesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bfd: std::option::Option<std::vec::Vec<ComputeRouterPeerBfdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRouterPeerTimeoutsEl>,
    dynamic: ComputeRouterPeerDynamic,
}

struct ComputeRouterPeer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRouterPeerData>,
}

#[derive(Clone)]
pub struct ComputeRouterPeer(std::rc::Rc<ComputeRouterPeer_>);

impl ComputeRouterPeer {
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
        "Set the field `advertise_mode`.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn set_advertise_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().advertise_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `advertised_groups`.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn set_advertised_groups(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().advertised_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `advertised_route_priority`.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn set_advertised_route_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().advertised_route_priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable`.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn set_enable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_ipv6`.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn set_enable_ipv6(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_address`.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn set_ip_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_nexthop_address`.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn set_ipv6_nexthop_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_nexthop_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_ip_address`.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn set_peer_ip_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_ipv6_nexthop_address`.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn set_peer_ipv6_nexthop_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().peer_ipv6_nexthop_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `router_appliance_instance`.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn set_router_appliance_instance(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().router_appliance_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterPeerAdvertisedIpRangesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advertised_ip_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advertised_ip_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bfd`.\n"]
    pub fn set_bfd(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeRouterPeerBfdEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bfd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bfd = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRouterPeerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `advertise_mode` after provisioning.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn advertise_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_groups` after provisioning.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn advertised_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_route_priority` after provisioning.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn advertised_route_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised_route_priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_ipv6` after provisioning.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn enable_ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface` after provisioning.\nName of the interface the BGP peer is associated with."]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_nexthop_address` after provisioning.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn ipv6_nexthop_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_nexthop_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management_type` after provisioning.\nThe resource that configures and manages this BGP peer.\n\n* 'MANAGED_BY_USER' is the default value and can be managed by\nyou or other users\n* 'MANAGED_BY_ATTACHMENT' is a BGP peer that is configured and\nmanaged by Cloud Interconnect, specifically by an\nInterconnectAttachment of type PARTNER. Google automatically\ncreates, updates, and deletes this type of BGP peer when the\nPARTNER InterconnectAttachment is created, updated,\nor deleted."]
    pub fn management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_asn` after provisioning.\nPeer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub fn peer_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn peer_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ipv6_nexthop_address` after provisioning.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn peer_ipv6_nexthop_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ipv6_nexthop_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this BgpPeer will be configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router_appliance_instance` after provisioning.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn router_appliance_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_appliance_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advertised_ip_ranges` after provisioning.\n"]
    pub fn advertised_ip_ranges(&self) -> terrars::ListRef<ComputeRouterPeerAdvertisedIpRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_ip_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bfd` after provisioning.\n"]
    pub fn bfd(&self) -> terrars::ListRef<ComputeRouterPeerBfdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bfd", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRouterPeer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRouterPeer { }

impl terrars::ToListMappable for ComputeRouterPeer {
    type O = terrars::ListRef<ComputeRouterPeerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRouterPeer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_router_peer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRouterPeer {
    pub tf_id: std::string::String,
    #[doc = "Name of the interface the BGP peer is associated with."]
    pub interface: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Peer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub peer_asn: terrars::PrimField<f64>,
    #[doc = "The name of the Cloud Router in which this BgpPeer will be configured."]
    pub router: terrars::PrimField<std::string::String>,
}

impl BuildComputeRouterPeer {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRouterPeer {
        let out = ComputeRouterPeer(std::rc::Rc::new(ComputeRouterPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRouterPeerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                advertise_mode: core::default::Default::default(),
                advertised_groups: core::default::Default::default(),
                advertised_route_priority: core::default::Default::default(),
                enable: core::default::Default::default(),
                enable_ipv6: core::default::Default::default(),
                id: core::default::Default::default(),
                interface: self.interface,
                ip_address: core::default::Default::default(),
                ipv6_nexthop_address: core::default::Default::default(),
                name: self.name,
                peer_asn: self.peer_asn,
                peer_ip_address: core::default::Default::default(),
                peer_ipv6_nexthop_address: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                router_appliance_instance: core::default::Default::default(),
                advertised_ip_ranges: core::default::Default::default(),
                bfd: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRouterPeerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterPeerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRouterPeerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `advertise_mode` after provisioning.\nUser-specified flag to indicate which mode to use for advertisement.\nValid values of this enum field are: 'DEFAULT', 'CUSTOM' Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn advertise_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_groups` after provisioning.\nUser-specified list of prefix groups to advertise in custom\nmode, which currently supports the following option:\n\n* 'ALL_SUBNETS': Advertises all of the router's own VPC subnets.\nThis excludes any routes learned for subnets that use VPC Network\nPeering.\n\n\nNote that this field can only be populated if advertiseMode is 'CUSTOM'\nand overrides the list defined for the router (in the \"bgp\" message).\nThese groups are advertised in addition to any specified prefixes.\nLeave this field blank to advertise no custom groups."]
    pub fn advertised_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_route_priority` after provisioning.\nThe priority of routes advertised to this BGP peer.\nWhere there is more than one matching route of maximum\nlength, the routes with the lowest priority value win."]
    pub fn advertised_route_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised_route_priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nThe status of the BGP peer connection. If set to false, any active session\nwith the peer is terminated and all associated routing information is removed.\nIf set to true, the peer connection can be established with routing information.\nThe default is true."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_ipv6` after provisioning.\nEnable IPv6 traffic over BGP Peer. If not specified, it is disabled by default."]
    pub fn enable_ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interface` after provisioning.\nName of the interface the BGP peer is associated with."]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nIP address of the interface inside Google Cloud Platform.\nOnly IPv4 is supported."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_nexthop_address` after provisioning.\nIPv6 address of the interface inside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn ipv6_nexthop_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_nexthop_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management_type` after provisioning.\nThe resource that configures and manages this BGP peer.\n\n* 'MANAGED_BY_USER' is the default value and can be managed by\nyou or other users\n* 'MANAGED_BY_ATTACHMENT' is a BGP peer that is configured and\nmanaged by Cloud Interconnect, specifically by an\nInterconnectAttachment of type PARTNER. Google automatically\ncreates, updates, and deletes this type of BGP peer when the\nPARTNER InterconnectAttachment is created, updated,\nor deleted."]
    pub fn management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of this BGP peer. The name must be 1-63 characters long,\nand comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which\nmeans the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_asn` after provisioning.\nPeer BGP Autonomous System Number (ASN).\nEach BGP interface may use a different value."]
    pub fn peer_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address of the BGP interface outside Google Cloud Platform.\nOnly IPv4 is supported. Required if 'ip_address' is set."]
    pub fn peer_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ipv6_nexthop_address` after provisioning.\nIPv6 address of the BGP interface outside Google Cloud Platform.\nThe address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.\nIf you do not specify the next hop addresses, Google Cloud automatically\nassigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you."]
    pub fn peer_ipv6_nexthop_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ipv6_nexthop_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the router and BgpPeer reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this BgpPeer will be configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router_appliance_instance` after provisioning.\nThe URI of the VM instance that is used as third-party router appliances\nsuch as Next Gen Firewalls, Virtual Routers, or Router Appliances.\nThe VM instance must be located in zones contained in the same region as\nthis Cloud Router. The VM instance is the peer side of the BGP session."]
    pub fn router_appliance_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_appliance_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advertised_ip_ranges` after provisioning.\n"]
    pub fn advertised_ip_ranges(&self) -> terrars::ListRef<ComputeRouterPeerAdvertisedIpRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advertised_ip_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bfd` after provisioning.\n"]
    pub fn bfd(&self) -> terrars::ListRef<ComputeRouterPeerBfdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bfd", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterPeerAdvertisedIpRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    range: terrars::PrimField<std::string::String>,
}

impl ComputeRouterPeerAdvertisedIpRangesEl {
    #[doc = "Set the field `description`.\nUser-specified description for the IP range."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRouterPeerAdvertisedIpRangesEl {
    type O = terrars::BlockAssignable<ComputeRouterPeerAdvertisedIpRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterPeerAdvertisedIpRangesEl {
    #[doc = "The IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub range: terrars::PrimField<std::string::String>,
}

impl BuildComputeRouterPeerAdvertisedIpRangesEl {
    pub fn build(self) -> ComputeRouterPeerAdvertisedIpRangesEl {
        ComputeRouterPeerAdvertisedIpRangesEl {
            description: core::default::Default::default(),
            range: self.range,
        }
    }
}

pub struct ComputeRouterPeerAdvertisedIpRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterPeerAdvertisedIpRangesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterPeerAdvertisedIpRangesElRef {
        ComputeRouterPeerAdvertisedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterPeerAdvertisedIpRangesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-specified description for the IP range."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `range` after provisioning.\nThe IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub fn range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterPeerBfdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_receive_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_transmit_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplier: std::option::Option<terrars::PrimField<f64>>,
    session_initialization_mode: terrars::PrimField<std::string::String>,
}

impl ComputeRouterPeerBfdEl {
    #[doc =
        "Set the field `min_receive_interval`.\nThe minimum interval, in milliseconds, between BFD control packets\nreceived from the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the transmit interval of the other router. If set, this value\nmust be between 1000 and 30000."]
    pub fn set_min_receive_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_receive_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_transmit_interval`.\nThe minimum interval, in milliseconds, between BFD control packets\ntransmitted to the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the corresponding receive interval of the other router. If set,\nthis value must be between 1000 and 30000."]
    pub fn set_min_transmit_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_transmit_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multiplier`.\nThe number of consecutive BFD packets that must be missed before\nBFD declares that a peer is unavailable. If set, the value must\nbe a value between 5 and 16."]
    pub fn set_multiplier(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.multiplier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRouterPeerBfdEl {
    type O = terrars::BlockAssignable<ComputeRouterPeerBfdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterPeerBfdEl {
    #[doc =
        "The BFD session initialization mode for this BGP peer.\nIf set to 'ACTIVE', the Cloud Router will initiate the BFD session\nfor this BGP peer. If set to 'PASSIVE', the Cloud Router will wait\nfor the peer router to initiate the BFD session for this BGP peer.\nIf set to 'DISABLED', BFD is disabled for this BGP peer. Possible values: [\"ACTIVE\", \"DISABLED\", \"PASSIVE\"]"]
    pub session_initialization_mode: terrars::PrimField<std::string::String>,
}

impl BuildComputeRouterPeerBfdEl {
    pub fn build(self) -> ComputeRouterPeerBfdEl {
        ComputeRouterPeerBfdEl {
            min_receive_interval: core::default::Default::default(),
            min_transmit_interval: core::default::Default::default(),
            multiplier: core::default::Default::default(),
            session_initialization_mode: self.session_initialization_mode,
        }
    }
}

pub struct ComputeRouterPeerBfdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterPeerBfdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterPeerBfdElRef {
        ComputeRouterPeerBfdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterPeerBfdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_receive_interval` after provisioning.\nThe minimum interval, in milliseconds, between BFD control packets\nreceived from the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the transmit interval of the other router. If set, this value\nmust be between 1000 and 30000."]
    pub fn min_receive_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_receive_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_transmit_interval` after provisioning.\nThe minimum interval, in milliseconds, between BFD control packets\ntransmitted to the peer router. The actual value is negotiated\nbetween the two routers and is equal to the greater of this value\nand the corresponding receive interval of the other router. If set,\nthis value must be between 1000 and 30000."]
    pub fn min_transmit_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_transmit_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `multiplier` after provisioning.\nThe number of consecutive BFD packets that must be missed before\nBFD declares that a peer is unavailable. If set, the value must\nbe a value between 5 and 16."]
    pub fn multiplier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_initialization_mode` after provisioning.\nThe BFD session initialization mode for this BGP peer.\nIf set to 'ACTIVE', the Cloud Router will initiate the BFD session\nfor this BGP peer. If set to 'PASSIVE', the Cloud Router will wait\nfor the peer router to initiate the BFD session for this BGP peer.\nIf set to 'DISABLED', BFD is disabled for this BGP peer. Possible values: [\"ACTIVE\", \"DISABLED\", \"PASSIVE\"]"]
    pub fn session_initialization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_initialization_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRouterPeerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRouterPeerTimeoutsEl {
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

impl terrars::ToListMappable for ComputeRouterPeerTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeRouterPeerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRouterPeerTimeoutsEl {}

impl BuildComputeRouterPeerTimeoutsEl {
    pub fn build(self) -> ComputeRouterPeerTimeoutsEl {
        ComputeRouterPeerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRouterPeerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRouterPeerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRouterPeerTimeoutsElRef {
        ComputeRouterPeerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRouterPeerTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComputeRouterPeerDynamic {
    advertised_ip_ranges: std::option::Option<terrars::DynamicBlock<ComputeRouterPeerAdvertisedIpRangesEl>>,
    bfd: std::option::Option<terrars::DynamicBlock<ComputeRouterPeerBfdEl>>,
}
