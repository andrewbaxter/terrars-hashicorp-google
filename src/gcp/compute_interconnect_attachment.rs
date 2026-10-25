#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInterconnectAttachmentData_ {
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
    admin_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    candidate_cloud_router_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    candidate_cloud_router_ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    candidate_customer_router_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    candidate_customer_router_ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    candidate_subnets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_availability_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipsec_internal_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtu: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_tag8021q: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    l2_forwarding: std::option::Option<std::vec::Vec<ComputeInterconnectAttachment_L2Forwarding>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeInterconnectAttachment_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInterconnectAttachment_Timeouts>,
    dynamic: ComputeInterconnectAttachmentDynamic,
}

struct ComputeInterconnectAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInterconnectAttachmentData_>,
}

#[derive(Clone)]
pub struct ComputeInterconnectAttachment(std::rc::Rc<ComputeInterconnectAttachment_>);

impl ComputeInterconnectAttachment {
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
        "Set the field `admin_enabled`.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn set_admin_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bandwidth`.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\", \"BPS_100G\", \"BPS_400G\"]"]
    pub fn set_bandwidth(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bandwidth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `candidate_cloud_router_ip_address`.\nSingle IPv4 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 203.0.113.1/29"]
    pub fn set_candidate_cloud_router_ip_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().candidate_cloud_router_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `candidate_cloud_router_ipv6_address`.\nSingle IPv6 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 2001:db8::1/125"]
    pub fn set_candidate_cloud_router_ipv6_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().candidate_cloud_router_ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `candidate_customer_router_ip_address`.\nSingle IPv4 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 203.0.113.2/29"]
    pub fn set_candidate_customer_router_ip_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().candidate_customer_router_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `candidate_customer_router_ipv6_address`.\nSingle IPv6 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 2001:db8::2/125"]
    pub fn set_candidate_customer_router_ipv6_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().candidate_customer_router_ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `candidate_subnets`.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn set_candidate_subnets(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().candidate_subnets = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `edge_availability_domain`.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn set_edge_availability_domain(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().edge_availability_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption`.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn set_encryption(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interconnect`.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn set_interconnect(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().interconnect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipsec_internal_addresses`.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn set_ipsec_internal_addresses(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipsec_internal_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mtu`.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through this interconnect attachment.\nValid values are 1440, 1460, 1500, and 8896. If not specified, the value will default to 1440."]
    pub fn set_mtu(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mtu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where the regional interconnect attachment resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `router`.\nURL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub fn set_router(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().router = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stack_type`.\nThe stack type for this interconnect attachment to identify whether the IPv6\nfeature is enabled or not. If not specified, IPV4_ONLY will be used.\nThis field can be both set at interconnect attachments creation and update\ninterconnect attachment operations. Possible values: [\"IPV4_IPV6\", \"IPV4_ONLY\"]"]
    pub fn set_stack_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnet_length`.\nLength of the IPv4 subnet mask. Allowed values: 29 (default), 30. The default value is 29,\nexcept for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a\nconstraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure\nremote location fall into this category. In these cases, the default value is 30, and\nrequesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it\ngives Google Cloud Support more debugging visibility."]
    pub fn set_subnet_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().subnet_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\", \"L2_DEDICATED\"]"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vlan_tag8021q`.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn set_vlan_tag8021q(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vlan_tag8021q = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `l2_forwarding`.\n"]
    pub fn set_l2_forwarding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().l2_forwarding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.l2_forwarding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectAttachment_Params>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInterconnectAttachment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `admin_enabled` after provisioning.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn admin_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attachment_group` after provisioning.\nURL of the AttachmentGroup that includes this Attachment."]
    pub fn attachment_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bandwidth` after provisioning.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\", \"BPS_100G\", \"BPS_400G\"]"]
    pub fn bandwidth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `candidate_cloud_router_ip_address` after provisioning.\nSingle IPv4 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 203.0.113.1/29"]
    pub fn candidate_cloud_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_cloud_router_ip_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_cloud_router_ipv6_address` after provisioning.\nSingle IPv6 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 2001:db8::1/125"]
    pub fn candidate_cloud_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_cloud_router_ipv6_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_customer_router_ip_address` after provisioning.\nSingle IPv4 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 203.0.113.2/29"]
    pub fn candidate_customer_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_customer_router_ip_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_customer_router_ipv6_address` after provisioning.\nSingle IPv6 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 2001:db8::2/125"]
    pub fn candidate_customer_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_customer_router_ipv6_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_subnets` after provisioning.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn candidate_subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.candidate_subnets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_router_ipv6_address` after provisioning.\nIPv6 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ipv6_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_router_ipv6_address` after provisioning.\nIPv6 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ipv6_address", self.extract_ref()))
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
        "Get a reference to the value of field `edge_availability_domain` after provisioning.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn edge_availability_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_availability_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption` after provisioning.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID, to be used when raising support tickets with\nGoogle or otherwise to debug backend connectivity issues."]
    pub fn google_reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_reference_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interconnect` after provisioning.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn interconnect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipsec_internal_addresses` after provisioning.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn ipsec_internal_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipsec_internal_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nA fingerprint for the labels being applied to this Interconnect, which is essentially a hash\nof the labels set used for optimistic locking. The fingerprint is initially generated by\nCompute Engine and changes after every request to modify or update labels.\nYou must always provide an up-to-date fingerprint hash in order to update or change labels,\notherwise the request will fail with error 412 conditionNotMet."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through this interconnect attachment.\nValid values are 1440, 1460, 1500, and 8896. If not specified, the value will default to 1440."]
    pub fn mtu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pairing_key` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. The opaque\nidentifier of an PARTNER attachment used to initiate provisioning with\na selected partner. Of the form \"XXXXX/region/domain\""]
    pub fn pairing_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pairing_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `partner_asn` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. Optional\nBGP ASN for the router that should be supplied by a layer 3 Partner if\nthey configured BGP on behalf of the customer."]
    pub fn partner_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partner_asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_interconnect_info` after provisioning.\nInformation specific to an InterconnectAttachment. This property\nis populated if the interconnect that this is attached to is of type DEDICATED."]
    pub fn private_interconnect_info(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachment_PrivateInterconnectInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_interconnect_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the regional interconnect attachment resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nURL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this interconnect attachment to identify whether the IPv6\nfeature is enabled or not. If not specified, IPV4_ONLY will be used.\nThis field can be both set at interconnect attachments creation and update\ninterconnect attachment operations. Possible values: [\"IPV4_IPV6\", \"IPV4_ONLY\"]"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] The current state of this attachment's functionality."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_length` after provisioning.\nLength of the IPv4 subnet mask. Allowed values: 29 (default), 30. The default value is 29,\nexcept for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a\nconstraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure\nremote location fall into this category. In these cases, the default value is 30, and\nrequesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it\ngives Google Cloud Support more debugging visibility."]
    pub fn subnet_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\", \"L2_DEDICATED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vlan_tag8021q` after provisioning.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn vlan_tag8021q(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag8021q", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `l2_forwarding` after provisioning.\n"]
    pub fn l2_forwarding(&self) -> terrars::ListRef<ComputeInterconnectAttachment_L2ForwardingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.l2_forwarding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInterconnectAttachment_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachment_TimeoutsRef {
        ComputeInterconnectAttachment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeInterconnectAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInterconnectAttachment { }

impl terrars::ToListMappable for ComputeInterconnectAttachment {
    type O = terrars::ListRef<ComputeInterconnectAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInterconnectAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_interconnect_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInterconnectAttachment {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInterconnectAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInterconnectAttachment {
        let out = ComputeInterconnectAttachment(std::rc::Rc::new(ComputeInterconnectAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInterconnectAttachmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_enabled: core::default::Default::default(),
                bandwidth: core::default::Default::default(),
                candidate_cloud_router_ip_address: core::default::Default::default(),
                candidate_cloud_router_ipv6_address: core::default::Default::default(),
                candidate_customer_router_ip_address: core::default::Default::default(),
                candidate_customer_router_ipv6_address: core::default::Default::default(),
                candidate_subnets: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_availability_domain: core::default::Default::default(),
                encryption: core::default::Default::default(),
                id: core::default::Default::default(),
                interconnect: core::default::Default::default(),
                ipsec_internal_addresses: core::default::Default::default(),
                labels: core::default::Default::default(),
                mtu: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: core::default::Default::default(),
                stack_type: core::default::Default::default(),
                subnet_length: core::default::Default::default(),
                type_: core::default::Default::default(),
                vlan_tag8021q: core::default::Default::default(),
                l2_forwarding: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInterconnectAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInterconnectAttachmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_enabled` after provisioning.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn admin_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attachment_group` after provisioning.\nURL of the AttachmentGroup that includes this Attachment."]
    pub fn attachment_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bandwidth` after provisioning.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\", \"BPS_100G\", \"BPS_400G\"]"]
    pub fn bandwidth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `candidate_cloud_router_ip_address` after provisioning.\nSingle IPv4 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 203.0.113.1/29"]
    pub fn candidate_cloud_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_cloud_router_ip_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_cloud_router_ipv6_address` after provisioning.\nSingle IPv6 address + prefix length to be configured on the cloud router interface for this\ninterconnect attachment. Example: 2001:db8::1/125"]
    pub fn candidate_cloud_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_cloud_router_ipv6_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_customer_router_ip_address` after provisioning.\nSingle IPv4 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 203.0.113.2/29"]
    pub fn candidate_customer_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_customer_router_ip_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_customer_router_ipv6_address` after provisioning.\nSingle IPv6 address + prefix length to be configured on the customer router interface for this\ninterconnect attachment. Example: 2001:db8::2/125"]
    pub fn candidate_customer_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.candidate_customer_router_ipv6_address", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `candidate_subnets` after provisioning.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn candidate_subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.candidate_subnets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_router_ipv6_address` after provisioning.\nIPv6 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ipv6_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_router_ipv6_address` after provisioning.\nIPv6 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ipv6_address", self.extract_ref()))
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
        "Get a reference to the value of field `edge_availability_domain` after provisioning.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn edge_availability_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_availability_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption` after provisioning.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID, to be used when raising support tickets with\nGoogle or otherwise to debug backend connectivity issues."]
    pub fn google_reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_reference_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interconnect` after provisioning.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn interconnect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipsec_internal_addresses` after provisioning.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn ipsec_internal_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipsec_internal_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nA fingerprint for the labels being applied to this Interconnect, which is essentially a hash\nof the labels set used for optimistic locking. The fingerprint is initially generated by\nCompute Engine and changes after every request to modify or update labels.\nYou must always provide an up-to-date fingerprint hash in order to update or change labels,\notherwise the request will fail with error 412 conditionNotMet."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through this interconnect attachment.\nValid values are 1440, 1460, 1500, and 8896. If not specified, the value will default to 1440."]
    pub fn mtu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pairing_key` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. The opaque\nidentifier of an PARTNER attachment used to initiate provisioning with\na selected partner. Of the form \"XXXXX/region/domain\""]
    pub fn pairing_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pairing_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `partner_asn` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. Optional\nBGP ASN for the router that should be supplied by a layer 3 Partner if\nthey configured BGP on behalf of the customer."]
    pub fn partner_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partner_asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_interconnect_info` after provisioning.\nInformation specific to an InterconnectAttachment. This property\nis populated if the interconnect that this is attached to is of type DEDICATED."]
    pub fn private_interconnect_info(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachment_PrivateInterconnectInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_interconnect_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the regional interconnect attachment resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `router` after provisioning.\nURL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub fn router(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this interconnect attachment to identify whether the IPv6\nfeature is enabled or not. If not specified, IPV4_ONLY will be used.\nThis field can be both set at interconnect attachments creation and update\ninterconnect attachment operations. Possible values: [\"IPV4_IPV6\", \"IPV4_ONLY\"]"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] The current state of this attachment's functionality."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_length` after provisioning.\nLength of the IPv4 subnet mask. Allowed values: 29 (default), 30. The default value is 29,\nexcept for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a\nconstraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure\nremote location fall into this category. In these cases, the default value is 30, and\nrequesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it\ngives Google Cloud Support more debugging visibility."]
    pub fn subnet_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\", \"L2_DEDICATED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vlan_tag8021q` after provisioning.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn vlan_tag8021q(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag8021q", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `l2_forwarding` after provisioning.\n"]
    pub fn l2_forwarding(&self) -> terrars::ListRef<ComputeInterconnectAttachment_L2ForwardingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.l2_forwarding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInterconnectAttachment_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachment_TimeoutsRef {
        ComputeInterconnectAttachment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeInterconnectAttachmentDynamic {
    l2_forwarding: std::option::Option<terrars::DynamicBlock<ComputeInterconnectAttachment_L2Forwarding>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeInterconnectAttachment_Params>>,
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_PrivateInterconnectInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag8021q: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInterconnectAttachment_PrivateInterconnectInfo {
    #[doc = "Set the field `tag8021q`.\n"]
    pub fn set_tag8021q(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tag8021q = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachment_PrivateInterconnectInfo {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_PrivateInterconnectInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_PrivateInterconnectInfo {}

impl BuildComputeInterconnectAttachment_PrivateInterconnectInfo {
    pub fn build(self) -> ComputeInterconnectAttachment_PrivateInterconnectInfo {
        ComputeInterconnectAttachment_PrivateInterconnectInfo { tag8021q: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_PrivateInterconnectInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_PrivateInterconnectInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachment_PrivateInterconnectInfoRef {
        ComputeInterconnectAttachment_PrivateInterconnectInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_PrivateInterconnectInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tag8021q` after provisioning.\n"]
    pub fn tag8021q(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag8021q", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inner_appliance_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inner_vlan_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
    #[doc = "Set the field `inner_appliance_ip_address`.\nThe inner appliance IP address."]
    pub fn set_inner_appliance_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inner_appliance_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inner_vlan_tags`.\nList of inner VLAN tags."]
    pub fn set_inner_vlan_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inner_vlan_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
    type O =
        terrars
        ::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {}

impl BuildComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
    pub fn build(self) -> ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
        ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings {
            inner_appliance_ip_address: core::default::Default::default(),
            inner_vlan_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef {
        ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inner_appliance_ip_address` after provisioning.\nThe inner appliance IP address."]
    pub fn inner_appliance_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inner_appliance_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `inner_vlan_tags` after provisioning.\nList of inner VLAN tags."]
    pub fn inner_vlan_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inner_vlan_tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsDynamic {
    inner_vlan_to_appliance_mappings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings>,
    >,
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    appliance_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inner_vlan_to_appliance_mappings: std
    ::option
    ::Option<
        std::vec::Vec<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings>,
    >,
    dynamic: ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsDynamic,
}

impl ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
    #[doc = "Set the field `appliance_ip_address`.\nThe appliance IP address."]
    pub fn set_appliance_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.appliance_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of this appliance mapping rule."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vlan_id`.\nThe VLAN tag."]
    pub fn set_vlan_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vlan_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inner_vlan_to_appliance_mappings`.\n"]
    pub fn set_inner_vlan_to_appliance_mappings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inner_vlan_to_appliance_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inner_vlan_to_appliance_mappings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {}

impl BuildComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
    pub fn build(self) -> ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
        ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings {
            appliance_ip_address: core::default::Default::default(),
            name: core::default::Default::default(),
            vlan_id: core::default::Default::default(),
            inner_vlan_to_appliance_mappings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef {
        ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `appliance_ip_address` after provisioning.\nThe appliance IP address."]
    pub fn appliance_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.appliance_ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this appliance mapping rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `vlan_id` after provisioning.\nThe VLAN tag."]
    pub fn vlan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_id", self.base))
    }

    #[doc = "Get a reference to the value of field `inner_vlan_to_appliance_mappings` after provisioning.\n"]
    pub fn inner_vlan_to_appliance_mappings(
        &self,
    ) -> terrars
    ::ListRef<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings_InnerVlanToApplianceMappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inner_vlan_to_appliance_mappings", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_L2Forwarding_GeneveHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vni: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInterconnectAttachment_L2Forwarding_GeneveHeader {
    #[doc = "Set the field `vni`.\nVNI is a 24-bit unique virtual network identifier."]
    pub fn set_vni(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vni = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachment_L2Forwarding_GeneveHeader {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding_GeneveHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_L2Forwarding_GeneveHeader {}

impl BuildComputeInterconnectAttachment_L2Forwarding_GeneveHeader {
    pub fn build(self) -> ComputeInterconnectAttachment_L2Forwarding_GeneveHeader {
        ComputeInterconnectAttachment_L2Forwarding_GeneveHeader { vni: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef {
        ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vni` after provisioning.\nVNI is a 24-bit unique virtual network identifier."]
    pub fn vni(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vni", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInterconnectAttachment_L2ForwardingDynamic {
    appliance_mappings: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings>>,
    geneve_header: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInterconnectAttachment_L2Forwarding_GeneveHeader>>,
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_L2Forwarding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_appliance_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_endpoint_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    appliance_mappings: std::option::Option<std::vec::Vec<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geneve_header: std::option::Option<std::vec::Vec<ComputeInterconnectAttachment_L2Forwarding_GeneveHeader>>,
    dynamic: ComputeInterconnectAttachment_L2ForwardingDynamic,
}

impl ComputeInterconnectAttachment_L2Forwarding {
    #[doc = "Set the field `default_appliance_ip_address`.\nThe default appliance IP address."]
    pub fn set_default_appliance_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_appliance_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nURL of the network to which this attachment belongs."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_endpoint_ip_address`.\nThe tunnel endpoint IP address."]
    pub fn set_tunnel_endpoint_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tunnel_endpoint_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `appliance_mappings`.\n"]
    pub fn set_appliance_mappings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.appliance_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.appliance_mappings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geneve_header`.\n"]
    pub fn set_geneve_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding_GeneveHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geneve_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geneve_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachment_L2Forwarding {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_L2Forwarding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_L2Forwarding {}

impl BuildComputeInterconnectAttachment_L2Forwarding {
    pub fn build(self) -> ComputeInterconnectAttachment_L2Forwarding {
        ComputeInterconnectAttachment_L2Forwarding {
            default_appliance_ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            tunnel_endpoint_ip_address: core::default::Default::default(),
            appliance_mappings: core::default::Default::default(),
            geneve_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_L2ForwardingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_L2ForwardingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectAttachment_L2ForwardingRef {
        ComputeInterconnectAttachment_L2ForwardingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_L2ForwardingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_appliance_ip_address` after provisioning.\nThe default appliance IP address."]
    pub fn default_appliance_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_appliance_ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nURL of the network to which this attachment belongs."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_endpoint_ip_address` after provisioning.\nThe tunnel endpoint IP address."]
    pub fn tunnel_endpoint_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_endpoint_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `appliance_mappings` after provisioning.\n"]
    pub fn appliance_mappings(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachment_L2Forwarding_ApplianceMappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.appliance_mappings", self.base))
    }

    #[doc = "Get a reference to the value of field `geneve_header` after provisioning.\n"]
    pub fn geneve_header(&self) -> terrars::ListRef<ComputeInterconnectAttachment_L2Forwarding_GeneveHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geneve_header", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInterconnectAttachment_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the interconnect attachment. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachment_Params {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_Params {}

impl BuildComputeInterconnectAttachment_Params {
    pub fn build(self) -> ComputeInterconnectAttachment_Params {
        ComputeInterconnectAttachment_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectAttachment_ParamsRef {
        ComputeInterconnectAttachment_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the interconnect attachment. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachment_Timeouts {
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

impl terrars::ToListMappable for ComputeInterconnectAttachment_Timeouts {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachment_Timeouts {}

impl BuildComputeInterconnectAttachment_Timeouts {
    pub fn build(self) -> ComputeInterconnectAttachment_Timeouts {
        ComputeInterconnectAttachment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectAttachment_TimeoutsRef {
        ComputeInterconnectAttachment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachment_TimeoutsRef {
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
