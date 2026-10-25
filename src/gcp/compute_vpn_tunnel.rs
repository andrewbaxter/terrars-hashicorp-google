#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeVpnTunnelData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ike_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_traffic_selector: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_external_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_external_gateway_interface: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_gcp_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_traffic_selector: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_vpn_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway_interface: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cipher_suite: std::option::Option<std::vec::Vec<ComputeVpnTunnel_CipherSuite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeVpnTunnel_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeVpnTunnel_Timeouts>,
    dynamic: ComputeVpnTunnelDynamic,
}

struct ComputeVpnTunnel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeVpnTunnelData_>,
}

#[derive(Clone)]
pub struct ComputeVpnTunnel(std::rc::Rc<ComputeVpnTunnel_>);

impl ComputeVpnTunnel {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
        "Set the field `ike_version`.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn set_ike_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ike_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_traffic_selector`.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn set_local_traffic_selector(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().local_traffic_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_external_gateway`.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn set_peer_external_gateway(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_external_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_external_gateway_interface`.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn set_peer_external_gateway_interface(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().peer_external_gateway_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_gcp_gateway`.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn set_peer_gcp_gateway(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_gcp_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_ip`.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn set_peer_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remote_traffic_selector`.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn set_remote_traffic_selector(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().remote_traffic_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `router`.\nURL of router resource to be used for dynamic routing."]
    pub fn set_router(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().router = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_secret`.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn set_shared_secret(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shared_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_secret_wo`.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn set_shared_secret_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shared_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_secret_wo_version`.\nTriggers update of 'shared_secret_wo' write-only. Increment this value when an update to 'shared_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_shared_secret_wo_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shared_secret_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_vpn_gateway`.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn set_target_vpn_gateway(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_vpn_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpn_gateway`.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn set_vpn_gateway(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpn_gateway_interface`.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn set_vpn_gateway_interface(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cipher_suite`.\n"]
    pub fn set_cipher_suite(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cipher_suite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cipher_suite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeVpnTunnel_Params>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeVpnTunnel_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detailed_status` after provisioning.\nDetailed status message for the VPN tunnel."]
    pub fn detailed_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_status", self.extract_ref()))
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
        "Get a reference to the value of field `ike_version` after provisioning.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn ike_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ike_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_traffic_selector` after provisioning.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn local_traffic_selector(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_traffic_selector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_external_gateway` after provisioning.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_external_gateway_interface` after provisioning.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_external_gateway_interface", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `peer_gcp_gateway` after provisioning.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn peer_gcp_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_gcp_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip` after provisioning.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn peer_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_traffic_selector` after provisioning.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn remote_traffic_selector(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.remote_traffic_selector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nURL of router resource to be used for dynamic routing."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_hash` after provisioning.\nHash of the shared secret."]
    pub fn shared_secret_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_wo` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_wo_version` after provisioning.\nTriggers update of 'shared_secret_wo' write-only. Increment this value when an update to 'shared_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn shared_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_vpn_gateway` after provisioning.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn target_vpn_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_vpn_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpn_gateway` after provisioning.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn vpn_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpn_gateway_interface` after provisioning.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn vpn_gateway_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cipher_suite` after provisioning.\n"]
    pub fn cipher_suite(&self) -> terrars::ListRef<ComputeVpnTunnel_CipherSuiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cipher_suite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeVpnTunnel_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeVpnTunnel_TimeoutsRef {
        ComputeVpnTunnel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeVpnTunnel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeVpnTunnel { }

impl terrars::ToListMappable for ComputeVpnTunnel {
    type O = terrars::ListRef<ComputeVpnTunnelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeVpnTunnel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_vpn_tunnel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeVpnTunnel {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeVpnTunnel {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeVpnTunnel {
        let out = ComputeVpnTunnel(std::rc::Rc::new(ComputeVpnTunnel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeVpnTunnelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
                shared_secret: core::default::Default::default(),
                shared_secret_wo: core::default::Default::default(),
                shared_secret_wo_version: core::default::Default::default(),
                target_vpn_gateway: core::default::Default::default(),
                vpn_gateway: core::default::Default::default(),
                vpn_gateway_interface: core::default::Default::default(),
                cipher_suite: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeVpnTunnelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeVpnTunnelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detailed_status` after provisioning.\nDetailed status message for the VPN tunnel."]
    pub fn detailed_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_status", self.extract_ref()))
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
        "Get a reference to the value of field `ike_version` after provisioning.\nIKE protocol version to use when establishing the VPN tunnel with\npeer VPN gateway.\nAcceptable IKE versions are 1 or 2. Default version is 2."]
    pub fn ike_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ike_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this VpnTunnel.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_traffic_selector` after provisioning.\nLocal traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn local_traffic_selector(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_traffic_selector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63\ncharacters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\nmust be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_external_gateway` after provisioning.\nURL of the peer side external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_external_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_external_gateway_interface` after provisioning.\nThe interface ID of the external VPN gateway to which this VPN tunnel is connected."]
    pub fn peer_external_gateway_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_external_gateway_interface", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `peer_gcp_gateway` after provisioning.\nURL of the peer side HA GCP VPN gateway to which this VPN tunnel is connected.\nIf provided, the VPN tunnel will automatically use the same vpn_gateway_interface\nID in the peer GCP VPN gateway.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn peer_gcp_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_gcp_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip` after provisioning.\nIP address of the peer VPN gateway. Only IPv4 is supported."]
    pub fn peer_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the tunnel is located. If unset, is set to the region of 'target_vpn_gateway'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remote_traffic_selector` after provisioning.\nRemote traffic selector to use when establishing the VPN tunnel with\npeer VPN gateway. The value should be a CIDR formatted string,\nfor example '192.168.0.0/16'. The ranges should be disjoint.\nOnly IPv4 is supported."]
    pub fn remote_traffic_selector(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.remote_traffic_selector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nURL of router resource to be used for dynamic routing."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_hash` after provisioning.\nHash of the shared secret."]
    pub fn shared_secret_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_wo` after provisioning.\nShared secret used to set the secure session between the Cloud VPN\ngateway and the peer VPN gateway."]
    pub fn shared_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_secret_wo_version` after provisioning.\nTriggers update of 'shared_secret_wo' write-only. Increment this value when an update to 'shared_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn shared_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_vpn_gateway` after provisioning.\nURL of the Target VPN gateway with which this VPN tunnel is\nassociated."]
    pub fn target_vpn_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_vpn_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpn_gateway` after provisioning.\nURL of the VPN gateway with which this VPN tunnel is associated.\nThis must be used if a High Availability VPN gateway resource is created.\nThis field must reference a 'google_compute_ha_vpn_gateway' resource."]
    pub fn vpn_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpn_gateway_interface` after provisioning.\nThe interface ID of the VPN gateway with which this VPN tunnel is associated."]
    pub fn vpn_gateway_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cipher_suite` after provisioning.\n"]
    pub fn cipher_suite(&self) -> terrars::ListRef<ComputeVpnTunnel_CipherSuiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cipher_suite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeVpnTunnel_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeVpnTunnel_TimeoutsRef {
        ComputeVpnTunnel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeVpnTunnelDynamic {
    cipher_suite: std::option::Option<terrars::DynamicBlock<ComputeVpnTunnel_CipherSuite>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeVpnTunnel_Params>>,
}

#[derive(Serialize)]
pub struct ComputeVpnTunnel_CipherSuite_Phase1 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dh: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integrity: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prf: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeVpnTunnel_CipherSuite_Phase1 {
    #[doc = "Set the field `dh`.\nDiffie-Hellman groups."]
    pub fn set_dh(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption`.\nEncryption algorithms."]
    pub fn set_encryption(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integrity`.\nIntegrity algorithms."]
    pub fn set_integrity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.integrity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prf`.\nPseudo-random functions."]
    pub fn set_prf(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prf = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeVpnTunnel_CipherSuite_Phase1 {
    type O = terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite_Phase1>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnel_CipherSuite_Phase1 {}

impl BuildComputeVpnTunnel_CipherSuite_Phase1 {
    pub fn build(self) -> ComputeVpnTunnel_CipherSuite_Phase1 {
        ComputeVpnTunnel_CipherSuite_Phase1 {
            dh: core::default::Default::default(),
            encryption: core::default::Default::default(),
            integrity: core::default::Default::default(),
            prf: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeVpnTunnel_CipherSuite_Phase1Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnel_CipherSuite_Phase1Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeVpnTunnel_CipherSuite_Phase1Ref {
        ComputeVpnTunnel_CipherSuite_Phase1Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnel_CipherSuite_Phase1Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dh` after provisioning.\nDiffie-Hellman groups."]
    pub fn dh(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dh", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\nEncryption algorithms."]
    pub fn encryption(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `integrity` after provisioning.\nIntegrity algorithms."]
    pub fn integrity(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integrity", self.base))
    }

    #[doc = "Get a reference to the value of field `prf` after provisioning.\nPseudo-random functions."]
    pub fn prf(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prf", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeVpnTunnel_CipherSuite_Phase2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integrity: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pfs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeVpnTunnel_CipherSuite_Phase2 {
    #[doc = "Set the field `encryption`.\nEncryption algorithms."]
    pub fn set_encryption(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integrity`.\nIntegrity algorithms."]
    pub fn set_integrity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.integrity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pfs`.\nPerfect forward secrecy groups."]
    pub fn set_pfs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pfs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeVpnTunnel_CipherSuite_Phase2 {
    type O = terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite_Phase2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnel_CipherSuite_Phase2 {}

impl BuildComputeVpnTunnel_CipherSuite_Phase2 {
    pub fn build(self) -> ComputeVpnTunnel_CipherSuite_Phase2 {
        ComputeVpnTunnel_CipherSuite_Phase2 {
            encryption: core::default::Default::default(),
            integrity: core::default::Default::default(),
            pfs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeVpnTunnel_CipherSuite_Phase2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnel_CipherSuite_Phase2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeVpnTunnel_CipherSuite_Phase2Ref {
        ComputeVpnTunnel_CipherSuite_Phase2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnel_CipherSuite_Phase2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\nEncryption algorithms."]
    pub fn encryption(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `integrity` after provisioning.\nIntegrity algorithms."]
    pub fn integrity(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integrity", self.base))
    }

    #[doc = "Get a reference to the value of field `pfs` after provisioning.\nPerfect forward secrecy groups."]
    pub fn pfs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pfs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeVpnTunnel_CipherSuiteDynamic {
    phase1: std::option::Option<terrars::DynamicBlock<ComputeVpnTunnel_CipherSuite_Phase1>>,
    phase2: std::option::Option<terrars::DynamicBlock<ComputeVpnTunnel_CipherSuite_Phase2>>,
}

#[derive(Serialize)]
pub struct ComputeVpnTunnel_CipherSuite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase1: std::option::Option<std::vec::Vec<ComputeVpnTunnel_CipherSuite_Phase1>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase2: std::option::Option<std::vec::Vec<ComputeVpnTunnel_CipherSuite_Phase2>>,
    dynamic: ComputeVpnTunnel_CipherSuiteDynamic,
}

impl ComputeVpnTunnel_CipherSuite {
    #[doc = "Set the field `phase1`.\n"]
    pub fn set_phase1(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite_Phase1>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phase1 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phase1 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phase2`.\n"]
    pub fn set_phase2(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite_Phase2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phase2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phase2 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeVpnTunnel_CipherSuite {
    type O = terrars::BlockAssignable<ComputeVpnTunnel_CipherSuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnel_CipherSuite {}

impl BuildComputeVpnTunnel_CipherSuite {
    pub fn build(self) -> ComputeVpnTunnel_CipherSuite {
        ComputeVpnTunnel_CipherSuite {
            phase1: core::default::Default::default(),
            phase2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeVpnTunnel_CipherSuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnel_CipherSuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeVpnTunnel_CipherSuiteRef {
        ComputeVpnTunnel_CipherSuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnel_CipherSuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phase1` after provisioning.\n"]
    pub fn phase1(&self) -> terrars::ListRef<ComputeVpnTunnel_CipherSuite_Phase1Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phase1", self.base))
    }

    #[doc = "Get a reference to the value of field `phase2` after provisioning.\n"]
    pub fn phase2(&self) -> terrars::ListRef<ComputeVpnTunnel_CipherSuite_Phase2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phase2", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeVpnTunnel_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeVpnTunnel_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the Vpn Tunnel. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeVpnTunnel_Params {
    type O = terrars::BlockAssignable<ComputeVpnTunnel_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnel_Params {}

impl BuildComputeVpnTunnel_Params {
    pub fn build(self) -> ComputeVpnTunnel_Params {
        ComputeVpnTunnel_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeVpnTunnel_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnel_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeVpnTunnel_ParamsRef {
        ComputeVpnTunnel_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnel_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the Vpn Tunnel. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeVpnTunnel_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeVpnTunnel_Timeouts {
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

impl terrars::ToListMappable for ComputeVpnTunnel_Timeouts {
    type O = terrars::BlockAssignable<ComputeVpnTunnel_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeVpnTunnel_Timeouts {}

impl BuildComputeVpnTunnel_Timeouts {
    pub fn build(self) -> ComputeVpnTunnel_Timeouts {
        ComputeVpnTunnel_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeVpnTunnel_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeVpnTunnel_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeVpnTunnel_TimeoutsRef {
        ComputeVpnTunnel_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeVpnTunnel_TimeoutsRef {
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
