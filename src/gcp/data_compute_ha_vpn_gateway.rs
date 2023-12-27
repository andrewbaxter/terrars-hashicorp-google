use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeHaVpnGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataComputeHaVpnGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeHaVpnGatewayData>,
}

#[derive(Clone)]
pub struct DataComputeHaVpnGateway(Rc<DataComputeHaVpnGateway_>);

impl DataComputeHaVpnGateway {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe region this gateway should sit in."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network this VPN gateway is accepting traffic for."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region this gateway should sit in."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this VPN gateway to identify the IP protocols that are enabled.\nIf not specified, IPV4_ONLY will be used. Default value: \"IPV4_ONLY\" Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_interfaces` after provisioning.\nA list of interfaces on this VPN gateway."]
    pub fn vpn_interfaces(&self) -> ListRef<DataComputeHaVpnGatewayVpnInterfacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_interfaces", self.extract_ref()))
    }
}

impl Referable for DataComputeHaVpnGateway {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataComputeHaVpnGateway { }

impl ToListMappable for DataComputeHaVpnGateway {
    type O = ListRef<DataComputeHaVpnGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataComputeHaVpnGateway_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_ha_vpn_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeHaVpnGateway {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}

impl BuildDataComputeHaVpnGateway {
    pub fn build(self, stack: &mut Stack) -> DataComputeHaVpnGateway {
        let out = DataComputeHaVpnGateway(Rc::new(DataComputeHaVpnGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeHaVpnGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeHaVpnGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeHaVpnGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeHaVpnGatewayRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe network this VPN gateway is accepting traffic for."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region this gateway should sit in."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this VPN gateway to identify the IP protocols that are enabled.\nIf not specified, IPV4_ONLY will be used. Default value: \"IPV4_ONLY\" Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_interfaces` after provisioning.\nA list of interfaces on this VPN gateway."]
    pub fn vpn_interfaces(&self) -> ListRef<DataComputeHaVpnGatewayVpnInterfacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_interfaces", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeHaVpnGatewayVpnInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
}

impl DataComputeHaVpnGatewayVpnInterfacesEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `interconnect_attachment`.\n"]
    pub fn set_interconnect_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interconnect_attachment = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeHaVpnGatewayVpnInterfacesEl {
    type O = BlockAssignable<DataComputeHaVpnGatewayVpnInterfacesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHaVpnGatewayVpnInterfacesEl {}

impl BuildDataComputeHaVpnGatewayVpnInterfacesEl {
    pub fn build(self) -> DataComputeHaVpnGatewayVpnInterfacesEl {
        DataComputeHaVpnGatewayVpnInterfacesEl {
            id: core::default::Default::default(),
            interconnect_attachment: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHaVpnGatewayVpnInterfacesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeHaVpnGatewayVpnInterfacesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeHaVpnGatewayVpnInterfacesElRef {
        DataComputeHaVpnGatewayVpnInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHaVpnGatewayVpnInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `interconnect_attachment` after provisioning.\n"]
    pub fn interconnect_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect_attachment", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}
