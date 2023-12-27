use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeVpnTunnelData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ike_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_traffic_selector: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_external_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_external_gateway_interface: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_gcp_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_traffic_selector: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    router: Option<PrimField<String>>,
    shared_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_vpn_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_gateway_interface: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeVpnTunnelTimeoutsEl>,
}

struct ComputeVpnTunnel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeVpnTunnelData>,
}

#[derive(Clone)]
pub struct ComputeVpnTunnel(Rc<ComputeVpnTunnel_>);

impl ComputeVpnTunnel {
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

    #[doc= "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ike_version`.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn set_ike_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ike_version = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `local_traffic_selector`.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn set_local_traffic_selector(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().local_traffic_selector = Some(v.into());
        self
    }

    #[doc= "Set the field `peer_external_gateway`.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn set_peer_external_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_external_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `peer_external_gateway_interface`.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn set_peer_external_gateway_interface(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().peer_external_gateway_interface = Some(v.into());
        self
    }

    #[doc= "Set the field `peer_gcp_gateway`.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn set_peer_gcp_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_gcp_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `peer_ip`.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn set_peer_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `remote_traffic_selector`.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn set_remote_traffic_selector(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().remote_traffic_selector = Some(v.into());
        self
    }

    #[doc= "Set the field `router`.\nURL of router resource to be used for dynamic routing."]
    pub fn set_router(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().router = Some(v.into());
        self
    }

    #[doc= "Set the field `target_vpn_gateway`.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn set_target_vpn_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_vpn_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `vpn_gateway`.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn set_vpn_gateway(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `vpn_gateway_interface`.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn set_vpn_gateway_interface(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway_interface = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeVpnTunnelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detailed_status` after provisioning.\nDetailed status message for the VPN tunnel."]
    pub fn detailed_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detailed_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ike_version` after provisioning.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn ike_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ike_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_traffic_selector` after provisioning.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn local_traffic_selector(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.local_traffic_selector", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_external_gateway` after provisioning.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_external_gateway_interface` after provisioning.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_gcp_gateway` after provisioning.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn peer_gcp_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_gcp_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_ip` after provisioning.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn peer_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_traffic_selector` after provisioning.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn remote_traffic_selector(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.remote_traffic_selector", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `router` after provisioning.\nURL of router resource to be used for dynamic routing."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret_hash` after provisioning.\nHash of the shared secret."]
    pub fn shared_secret_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_vpn_gateway` after provisioning.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn target_vpn_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_vpn_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tunnel_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn tunnel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_gateway` after provisioning.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn vpn_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_gateway_interface` after provisioning.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn vpn_gateway_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeVpnTunnelTimeoutsElRef {
        ComputeVpnTunnelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeVpnTunnel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeVpnTunnel { }

impl ToListMappable for ComputeVpnTunnel {
    type O = ListRef<ComputeVpnTunnelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeVpnTunnel_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_vpn_tunnel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeVpnTunnel {
    pub tf_id: String,
    #[doc= "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "Shared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub shared_secret: PrimField<String>,
}

impl BuildComputeVpnTunnel {
    pub fn build(self, stack: &mut Stack) -> ComputeVpnTunnel {
        let out = ComputeVpnTunnel(Rc::new(ComputeVpnTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeVpnTunnelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ike_version: core::default::Default::default(),
                labels: core::default::Default::default(),
                local_traffic_selector: core::default::Default::default(),
                name: self.name,
                peer_external_gateway: core::default::Default::default(),
                peer_external_gateway_interface: core::default::Default::default(),
                peer_gcp_gateway: core::default::Default::default(),
                peer_ip: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                remote_traffic_selector: core::default::Default::default(),
                router: core::default::Default::default(),
                shared_secret: self.shared_secret,
                target_vpn_gateway: core::default::Default::default(),
                vpn_gateway: core::default::Default::default(),
                vpn_gateway_interface: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeVpnTunnelRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeVpnTunnelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeVpnTunnelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detailed_status` after provisioning.\nDetailed status message for the VPN tunnel."]
    pub fn detailed_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detailed_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ike_version` after provisioning.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn ike_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ike_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_traffic_selector` after provisioning.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn local_traffic_selector(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.local_traffic_selector", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_external_gateway` after provisioning.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_external_gateway_interface` after provisioning.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_gcp_gateway` after provisioning.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn peer_gcp_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_gcp_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_ip` after provisioning.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn peer_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_traffic_selector` after provisioning.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn remote_traffic_selector(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.remote_traffic_selector", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `router` after provisioning.\nURL of router resource to be used for dynamic routing."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret_hash` after provisioning.\nHash of the shared secret."]
    pub fn shared_secret_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_vpn_gateway` after provisioning.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn target_vpn_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_vpn_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tunnel_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn tunnel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_gateway` after provisioning.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn vpn_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_gateway_interface` after provisioning.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn vpn_gateway_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeVpnTunnelTimeoutsElRef {
        ComputeVpnTunnelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeVpnTunnelTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeVpnTunnelTimeoutsEl {
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

impl ToListMappable for ComputeVpnTunnelTimeoutsEl {
    type O = BlockAssignable<ComputeVpnTunnelTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnelTimeoutsEl {}

impl BuildComputeVpnTunnelTimeoutsEl {
    pub fn build(self) -> ComputeVpnTunnelTimeoutsEl {
        ComputeVpnTunnelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeVpnTunnelTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeVpnTunnelTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeVpnTunnelTimeoutsElRef {
        ComputeVpnTunnelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnelTimeoutsElRef {
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
