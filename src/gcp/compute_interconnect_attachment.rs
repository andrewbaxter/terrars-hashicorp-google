use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInterconnectAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_subnets: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_availability_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipsec_internal_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vlan_tag8021q: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInterconnectAttachmentTimeoutsEl>,
}

struct ComputeInterconnectAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInterconnectAttachmentData>,
}

#[derive(Clone)]
pub struct ComputeInterconnectAttachment(Rc<ComputeInterconnectAttachment_>);

impl ComputeInterconnectAttachment {
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

    #[doc= "Set the field `admin_enabled`.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn set_admin_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `bandwidth`.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\"]"]
    pub fn set_bandwidth(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bandwidth = Some(v.into());
        self
    }

    #[doc= "Set the field `candidate_subnets`.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn set_candidate_subnets(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().candidate_subnets = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `edge_availability_domain`.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn set_edge_availability_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edge_availability_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption`.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn set_encryption(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `interconnect`.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn set_interconnect(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().interconnect = Some(v.into());
        self
    }

    #[doc= "Set the field `ipsec_internal_addresses`.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\n\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\n\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\n\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn set_ipsec_internal_addresses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipsec_internal_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `mtu`.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through\nthis interconnect attachment. Currently, only 1440 and 1500 are allowed. If not specified, the value will default to 1440."]
    pub fn set_mtu(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mtu = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nRegion where the regional interconnect attachment resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `vlan_tag8021q`.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn set_vlan_tag8021q(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vlan_tag8021q = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeInterconnectAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `admin_enabled` after provisioning.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bandwidth` after provisioning.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\"]"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `candidate_subnets` after provisioning.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn candidate_subnets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.candidate_subnets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edge_availability_domain` after provisioning.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn edge_availability_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_availability_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption` after provisioning.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID, to be used when raising support tickets with\nGoogle or otherwise to debug backend connectivity issues."]
    pub fn google_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.google_reference_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interconnect` after provisioning.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn interconnect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipsec_internal_addresses` after provisioning.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\n\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\n\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\n\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn ipsec_internal_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipsec_internal_addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through\nthis interconnect attachment. Currently, only 1440 and 1500 are allowed. If not specified, the value will default to 1440."]
    pub fn mtu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pairing_key` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. The opaque\nidentifier of an PARTNER attachment used to initiate provisioning with\na selected partner. Of the form \"XXXXX/region/domain\""]
    pub fn pairing_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pairing_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `partner_asn` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. Optional\nBGP ASN for the router that should be supplied by a layer 3 Partner if\nthey configured BGP on behalf of the customer."]
    pub fn partner_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partner_asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_interconnect_info` after provisioning.\nInformation specific to an InterconnectAttachment. This property\nis populated if the interconnect that this is attached to is of type DEDICATED."]
    pub fn private_interconnect_info(&self) -> ListRef<ComputeInterconnectAttachmentPrivateInterconnectInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_interconnect_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where the regional interconnect attachment resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `router` after provisioning.\nURL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n[Output Only] The current state of this attachment's functionality."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vlan_tag8021q` after provisioning.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn vlan_tag8021q(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag8021q", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentTimeoutsElRef {
        ComputeInterconnectAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeInterconnectAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeInterconnectAttachment { }

impl ToListMappable for ComputeInterconnectAttachment {
    type O = ListRef<ComputeInterconnectAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeInterconnectAttachment_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_interconnect_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInterconnectAttachment {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "URL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub router: PrimField<String>,
}

impl BuildComputeInterconnectAttachment {
    pub fn build(self, stack: &mut Stack) -> ComputeInterconnectAttachment {
        let out = ComputeInterconnectAttachment(Rc::new(ComputeInterconnectAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeInterconnectAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_enabled: core::default::Default::default(),
                bandwidth: core::default::Default::default(),
                candidate_subnets: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_availability_domain: core::default::Default::default(),
                encryption: core::default::Default::default(),
                id: core::default::Default::default(),
                interconnect: core::default::Default::default(),
                ipsec_internal_addresses: core::default::Default::default(),
                mtu: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                type_: core::default::Default::default(),
                vlan_tag8021q: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInterconnectAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeInterconnectAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInterconnectAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `admin_enabled` after provisioning.\nWhether the VLAN attachment is enabled or disabled.  When using\nPARTNER type this will Pre-Activate the interconnect attachment"]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bandwidth` after provisioning.\nProvisioned bandwidth capacity for the interconnect attachment.\nFor attachments of type DEDICATED, the user can set the bandwidth.\nFor attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.\nOutput only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,\nDefaults to BPS_10G Possible values: [\"BPS_50M\", \"BPS_100M\", \"BPS_200M\", \"BPS_300M\", \"BPS_400M\", \"BPS_500M\", \"BPS_1G\", \"BPS_2G\", \"BPS_5G\", \"BPS_10G\", \"BPS_20G\", \"BPS_50G\"]"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `candidate_subnets` after provisioning.\nUp to 16 candidate prefixes that can be used to restrict the allocation\nof cloudRouterIpAddress and customerRouterIpAddress for this attachment.\nAll prefixes must be within link-local address space (169.254.0.0/16)\nand must be /29 or shorter (/28, /27, etc). Google will attempt to select\nan unused /29 from the supplied candidate prefix(es). The request will\nfail if all possible /29s are in use on Google's edge. If not supplied,\nGoogle will randomly select an unused /29 from all of link-local space."]
    pub fn candidate_subnets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.candidate_subnets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on Cloud Router\nInterface for this interconnect attachment."]
    pub fn cloud_router_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_router_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_router_ip_address` after provisioning.\nIPv4 address + prefix length to be configured on the customer\nrouter subinterface for this interconnect attachment."]
    pub fn customer_router_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_router_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edge_availability_domain` after provisioning.\nDesired availability domain for the attachment. Only available for type\nPARTNER, at creation time. For improved reliability, customers should\nconfigure a pair of attachments with one per availability domain. The\nselected availability domain will be provided to the Partner via the\npairing key so that the provisioned circuit will lie in the specified\ndomain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY."]
    pub fn edge_availability_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_availability_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption` after provisioning.\nIndicates the user-supplied encryption option of this interconnect\nattachment. Can only be specified at attachment creation for PARTNER or\nDEDICATED attachments.\n\n* NONE - This is the default value, which means that the VLAN attachment\ncarries unencrypted traffic. VMs are able to send traffic to, or receive\ntraffic from, such a VLAN attachment.\n\n* IPSEC - The VLAN attachment carries only encrypted traffic that is\nencrypted by an IPsec device, such as an HA VPN gateway or third-party\nIPsec VPN. VMs cannot directly send traffic to, or receive traffic from,\nsuch a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN\nattachment must be created with this option. Default value: \"NONE\" Possible values: [\"NONE\", \"IPSEC\"]"]
    pub fn encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID, to be used when raising support tickets with\nGoogle or otherwise to debug backend connectivity issues."]
    pub fn google_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.google_reference_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interconnect` after provisioning.\nURL of the underlying Interconnect object that this attachment's\ntraffic will traverse through. Required if type is DEDICATED, must not\nbe set if type is PARTNER."]
    pub fn interconnect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipsec_internal_addresses` after provisioning.\nURL of addresses that have been reserved for the interconnect attachment,\nUsed only for interconnect attachment that has the encryption option as\nIPSEC.\n\nThe addresses must be RFC 1918 IP address ranges. When creating HA VPN\ngateway over the interconnect attachment, if the attachment is configured\nto use an RFC 1918 IP address, then the VPN gateway's IP address will be\nallocated from the IP address range specified here.\n\nFor example, if the HA VPN gateway's interface 0 is paired to this\ninterconnect attachment, then an RFC 1918 IP address for the VPN gateway\ninterface 0 will be allocated from the IP address specified for this\ninterconnect attachment.\n\nIf this field is not specified for interconnect attachment that has\nencryption option as IPSEC, later on when creating HA VPN gateway on this\ninterconnect attachment, the HA VPN gateway's IP address will be\nallocated from regional external IP address pool."]
    pub fn ipsec_internal_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipsec_internal_addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mtu` after provisioning.\nMaximum Transmission Unit (MTU), in bytes, of packets passing through\nthis interconnect attachment. Currently, only 1440 and 1500 are allowed. If not specified, the value will default to 1440."]
    pub fn mtu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a\nlowercase letter, and all following characters must be a dash, lowercase\nletter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pairing_key` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. The opaque\nidentifier of an PARTNER attachment used to initiate provisioning with\na selected partner. Of the form \"XXXXX/region/domain\""]
    pub fn pairing_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pairing_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `partner_asn` after provisioning.\n[Output only for type PARTNER. Not present for DEDICATED]. Optional\nBGP ASN for the router that should be supplied by a layer 3 Partner if\nthey configured BGP on behalf of the customer."]
    pub fn partner_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partner_asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_interconnect_info` after provisioning.\nInformation specific to an InterconnectAttachment. This property\nis populated if the interconnect that this is attached to is of type DEDICATED."]
    pub fn private_interconnect_info(&self) -> ListRef<ComputeInterconnectAttachmentPrivateInterconnectInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_interconnect_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where the regional interconnect attachment resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `router` after provisioning.\nURL of the cloud router to be used for dynamic routing. This router must be in\nthe same region as this InterconnectAttachment. The InterconnectAttachment will\nautomatically connect the Interconnect to the network & region within which the\nCloud Router is configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n[Output Only] The current state of this attachment's functionality."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of InterconnectAttachment you wish to create. Defaults to\nDEDICATED. Possible values: [\"DEDICATED\", \"PARTNER\", \"PARTNER_PROVIDER\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vlan_tag8021q` after provisioning.\nThe IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When\nusing PARTNER type this will be managed upstream."]
    pub fn vlan_tag8021q(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag8021q", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentTimeoutsElRef {
        ComputeInterconnectAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentPrivateInterconnectInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tag8021q: Option<PrimField<f64>>,
}

impl ComputeInterconnectAttachmentPrivateInterconnectInfoEl {
    #[doc= "Set the field `tag8021q`.\n"]
    pub fn set_tag8021q(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tag8021q = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeInterconnectAttachmentPrivateInterconnectInfoEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentPrivateInterconnectInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentPrivateInterconnectInfoEl {}

impl BuildComputeInterconnectAttachmentPrivateInterconnectInfoEl {
    pub fn build(self) -> ComputeInterconnectAttachmentPrivateInterconnectInfoEl {
        ComputeInterconnectAttachmentPrivateInterconnectInfoEl { tag8021q: core::default::Default::default() }
    }
}

pub struct ComputeInterconnectAttachmentPrivateInterconnectInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeInterconnectAttachmentPrivateInterconnectInfoElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectAttachmentPrivateInterconnectInfoElRef {
        ComputeInterconnectAttachmentPrivateInterconnectInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentPrivateInterconnectInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `tag8021q` after provisioning.\n"]
    pub fn tag8021q(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag8021q", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeInterconnectAttachmentTimeoutsEl {
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

impl ToListMappable for ComputeInterconnectAttachmentTimeoutsEl {
    type O = BlockAssignable<ComputeInterconnectAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentTimeoutsEl {}

impl BuildComputeInterconnectAttachmentTimeoutsEl {
    pub fn build(self) -> ComputeInterconnectAttachmentTimeoutsEl {
        ComputeInterconnectAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeInterconnectAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeInterconnectAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectAttachmentTimeoutsElRef {
        ComputeInterconnectAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentTimeoutsElRef {
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
