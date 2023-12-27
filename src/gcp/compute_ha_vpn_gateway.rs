use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeHaVpnGatewayData {
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
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeHaVpnGatewayTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_interfaces: Option<Vec<ComputeHaVpnGatewayVpnInterfacesEl>>,
    dynamic: ComputeHaVpnGatewayDynamic,
}

struct ComputeHaVpnGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeHaVpnGatewayData>,
}

#[derive(Clone)]
pub struct ComputeHaVpnGateway(Rc<ComputeHaVpnGateway_>);

impl ComputeHaVpnGateway {
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

    #[doc= "Set the field `stack_type`.\nThe stack type for this VPN gateway to identify the IP protocols that are enabled.\nIf not specified, IPV4_ONLY will be used. Default value: \"IPV4_ONLY\" Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn set_stack_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stack_type = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeHaVpnGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `vpn_interfaces`.\n"]
    pub fn set_vpn_interfaces(self, v: impl Into<BlockAssignable<ComputeHaVpnGatewayVpnInterfacesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpn_interfaces = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpn_interfaces = Some(d);
            },
        }
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

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeHaVpnGatewayTimeoutsElRef {
        ComputeHaVpnGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_interfaces` after provisioning.\n"]
    pub fn vpn_interfaces(&self) -> ListRef<ComputeHaVpnGatewayVpnInterfacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_interfaces", self.extract_ref()))
    }
}

impl Referable for ComputeHaVpnGateway {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeHaVpnGateway { }

impl ToListMappable for ComputeHaVpnGateway {
    type O = ListRef<ComputeHaVpnGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeHaVpnGateway_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_ha_vpn_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeHaVpnGateway {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "The network this VPN gateway is accepting traffic for."]
    pub network: PrimField<String>,
}

impl BuildComputeHaVpnGateway {
    pub fn build(self, stack: &mut Stack) -> ComputeHaVpnGateway {
        let out = ComputeHaVpnGateway(Rc::new(ComputeHaVpnGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeHaVpnGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                stack_type: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpn_interfaces: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeHaVpnGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeHaVpnGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeHaVpnGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeHaVpnGatewayTimeoutsElRef {
        ComputeHaVpnGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpn_interfaces` after provisioning.\n"]
    pub fn vpn_interfaces(&self) -> ListRef<ComputeHaVpnGatewayVpnInterfacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_interfaces", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeHaVpnGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ComputeHaVpnGatewayTimeoutsEl {
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

impl ToListMappable for ComputeHaVpnGatewayTimeoutsEl {
    type O = BlockAssignable<ComputeHaVpnGatewayTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeHaVpnGatewayTimeoutsEl {}

impl BuildComputeHaVpnGatewayTimeoutsEl {
    pub fn build(self) -> ComputeHaVpnGatewayTimeoutsEl {
        ComputeHaVpnGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeHaVpnGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeHaVpnGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeHaVpnGatewayTimeoutsElRef {
        ComputeHaVpnGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeHaVpnGatewayTimeoutsElRef {
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

#[derive(Serialize)]
pub struct ComputeHaVpnGatewayVpnInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect_attachment: Option<PrimField<String>>,
}

impl ComputeHaVpnGatewayVpnInterfacesEl {
    #[doc= "Set the field `id`.\nThe numeric ID of this VPN gateway interface."]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `interconnect_attachment`.\nURL of the interconnect attachment resource. When the value\nof this field is present, the VPN Gateway will be used for\nIPsec-encrypted Cloud Interconnect; all Egress or Ingress\ntraffic for this VPN Gateway interface will go through the\nspecified interconnect attachment resource.\n\nNot currently available publicly."]
    pub fn set_interconnect_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interconnect_attachment = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeHaVpnGatewayVpnInterfacesEl {
    type O = BlockAssignable<ComputeHaVpnGatewayVpnInterfacesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeHaVpnGatewayVpnInterfacesEl {}

impl BuildComputeHaVpnGatewayVpnInterfacesEl {
    pub fn build(self) -> ComputeHaVpnGatewayVpnInterfacesEl {
        ComputeHaVpnGatewayVpnInterfacesEl {
            id: core::default::Default::default(),
            interconnect_attachment: core::default::Default::default(),
        }
    }
}

pub struct ComputeHaVpnGatewayVpnInterfacesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeHaVpnGatewayVpnInterfacesElRef {
    fn new(shared: StackShared, base: String) -> ComputeHaVpnGatewayVpnInterfacesElRef {
        ComputeHaVpnGatewayVpnInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeHaVpnGatewayVpnInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe numeric ID of this VPN gateway interface."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `interconnect_attachment` after provisioning.\nURL of the interconnect attachment resource. When the value\nof this field is present, the VPN Gateway will be used for\nIPsec-encrypted Cloud Interconnect; all Egress or Ingress\ntraffic for this VPN Gateway interface will go through the\nspecified interconnect attachment resource.\n\nNot currently available publicly."]
    pub fn interconnect_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect_attachment", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\nThe external IP address for this VPN gateway interface."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeHaVpnGatewayDynamic {
    vpn_interfaces: Option<DynamicBlock<ComputeHaVpnGatewayVpnInterfacesEl>>,
}
