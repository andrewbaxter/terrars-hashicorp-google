use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputePacketMirroringData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collector_ilb: std::option::Option<std::vec::Vec<ComputePacketMirroringCollectorIlbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<ComputePacketMirroringFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirrored_resources: std::option::Option<std::vec::Vec<ComputePacketMirroringMirroredResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<ComputePacketMirroringNetworkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputePacketMirroringTimeoutsEl>,
    dynamic: ComputePacketMirroringDynamic,
}

struct ComputePacketMirroring_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputePacketMirroringData>,
}

#[derive(Clone)]
pub struct ComputePacketMirroring(std::rc::Rc<ComputePacketMirroring_>);

impl ComputePacketMirroring {
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

    #[doc = "Set the field `description`.\nA human-readable description of the rule."]
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
        "Set the field `priority`.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collector_ilb`.\n"]
    pub fn set_collector_ilb(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringCollectorIlbEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().collector_ilb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.collector_ilb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mirrored_resources`.\n"]
    pub fn set_mirrored_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mirrored_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mirrored_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringNetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputePacketMirroringTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the packet mirroring rule"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collector_ilb` after provisioning.\n"]
    pub fn collector_ilb(&self) -> terrars::ListRef<ComputePacketMirroringCollectorIlbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collector_ilb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ComputePacketMirroringFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> terrars::ListRef<ComputePacketMirroringMirroredResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mirrored_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<ComputePacketMirroringNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputePacketMirroring {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputePacketMirroring { }

impl terrars::ToListMappable for ComputePacketMirroring {
    type O = terrars::ListRef<ComputePacketMirroringRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputePacketMirroring_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_packet_mirroring".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputePacketMirroring {
    pub tf_id: std::string::String,
    #[doc = "The name of the packet mirroring rule"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroring {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputePacketMirroring {
        let out = ComputePacketMirroring(std::rc::Rc::new(ComputePacketMirroring_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputePacketMirroringData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                collector_ilb: core::default::Default::default(),
                filter: core::default::Default::default(),
                mirrored_resources: core::default::Default::default(),
                network: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputePacketMirroringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputePacketMirroringRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the packet mirroring rule"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collector_ilb` after provisioning.\n"]
    pub fn collector_ilb(&self) -> terrars::ListRef<ComputePacketMirroringCollectorIlbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collector_ilb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ComputePacketMirroringFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> terrars::ListRef<ComputePacketMirroringMirroredResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mirrored_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<ComputePacketMirroringNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringCollectorIlbEl {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroringCollectorIlbEl { }

impl terrars::ToListMappable for ComputePacketMirroringCollectorIlbEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringCollectorIlbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringCollectorIlbEl {
    #[doc = "The URL of the forwarding rule."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroringCollectorIlbEl {
    pub fn build(self) -> ComputePacketMirroringCollectorIlbEl {
        ComputePacketMirroringCollectorIlbEl { url: self.url }
    }
}

pub struct ComputePacketMirroringCollectorIlbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringCollectorIlbElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroringCollectorIlbElRef {
        ComputePacketMirroringCollectorIlbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringCollectorIlbElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the forwarding rule."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputePacketMirroringFilterEl {
    #[doc =
        "Set the field `cidr_ranges`.\nIP CIDR ranges that apply as a filter on the source (ingress) or\ndestination (egress) IP in the IP header. Only IPv4 is supported."]
    pub fn set_cidr_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidr_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `direction`.\nDirection of traffic to mirror. Default value: \"BOTH\" Possible values: [\"INGRESS\", \"EGRESS\", \"BOTH\"]"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_protocols`.\nPossible IP protocols including tcp, udp, icmp and esp"]
    pub fn set_ip_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_protocols = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputePacketMirroringFilterEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringFilterEl {}

impl BuildComputePacketMirroringFilterEl {
    pub fn build(self) -> ComputePacketMirroringFilterEl {
        ComputePacketMirroringFilterEl {
            cidr_ranges: core::default::Default::default(),
            direction: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
        }
    }
}

pub struct ComputePacketMirroringFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroringFilterElRef {
        ComputePacketMirroringFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr_ranges` after provisioning.\nIP CIDR ranges that apply as a filter on the source (ingress) or\ndestination (egress) IP in the IP header. Only IPv4 is supported."]
    pub fn cidr_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection of traffic to mirror. Default value: \"BOTH\" Possible values: [\"INGRESS\", \"EGRESS\", \"BOTH\"]"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocols` after provisioning.\nPossible IP protocols including tcp, udp, icmp and esp"]
    pub fn ip_protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesElInstancesEl {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroringMirroredResourcesElInstancesEl { }

impl terrars::ToListMappable for ComputePacketMirroringMirroredResourcesElInstancesEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesElInstancesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringMirroredResourcesElInstancesEl {
    #[doc = "The URL of the instances where this rule should be active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroringMirroredResourcesElInstancesEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesElInstancesEl {
        ComputePacketMirroringMirroredResourcesElInstancesEl { url: self.url }
    }
}

pub struct ComputePacketMirroringMirroredResourcesElInstancesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringMirroredResourcesElInstancesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePacketMirroringMirroredResourcesElInstancesElRef {
        ComputePacketMirroringMirroredResourcesElInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringMirroredResourcesElInstancesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL of the instances where this rule should be active."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesElSubnetworksEl {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroringMirroredResourcesElSubnetworksEl { }

impl terrars::ToListMappable for ComputePacketMirroringMirroredResourcesElSubnetworksEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesElSubnetworksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringMirroredResourcesElSubnetworksEl {
    #[doc = "The URL of the subnetwork where this rule should be active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroringMirroredResourcesElSubnetworksEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesElSubnetworksEl {
        ComputePacketMirroringMirroredResourcesElSubnetworksEl { url: self.url }
    }
}

pub struct ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
        ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL of the subnetwork where this rule should be active."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePacketMirroringMirroredResourcesElDynamic {
    instances: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
    subnetworks: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
}

#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<std::vec::Vec<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetworks: std::option::Option<std::vec::Vec<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
    dynamic: ComputePacketMirroringMirroredResourcesElDynamic,
}

impl ComputePacketMirroringMirroredResourcesEl {
    #[doc = "Set the field `tags`.\nAll instances with these tags will be mirrored."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instances = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subnetworks`.\n"]
    pub fn set_subnetworks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subnetworks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subnetworks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputePacketMirroringMirroredResourcesEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringMirroredResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringMirroredResourcesEl {}

impl BuildComputePacketMirroringMirroredResourcesEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesEl {
        ComputePacketMirroringMirroredResourcesEl {
            tags: core::default::Default::default(),
            instances: core::default::Default::default(),
            subnetworks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputePacketMirroringMirroredResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringMirroredResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroringMirroredResourcesElRef {
        ComputePacketMirroringMirroredResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringMirroredResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nAll instances with these tags will be mirrored."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<ComputePacketMirroringMirroredResourcesElInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetworks` after provisioning.\n"]
    pub fn subnetworks(&self) -> terrars::ListRef<ComputePacketMirroringMirroredResourcesElSubnetworksElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnetworks", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringNetworkEl {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroringNetworkEl { }

impl terrars::ToListMappable for ComputePacketMirroringNetworkEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringNetworkEl {
    #[doc = "The full self_link URL of the network where this rule is active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroringNetworkEl {
    pub fn build(self) -> ComputePacketMirroringNetworkEl {
        ComputePacketMirroringNetworkEl { url: self.url }
    }
}

pub struct ComputePacketMirroringNetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringNetworkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroringNetworkElRef {
        ComputePacketMirroringNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe full self_link URL of the network where this rule is active."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroringTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputePacketMirroringTimeoutsEl {
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

impl terrars::ToListMappable for ComputePacketMirroringTimeoutsEl {
    type O = terrars::BlockAssignable<ComputePacketMirroringTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroringTimeoutsEl {}

impl BuildComputePacketMirroringTimeoutsEl {
    pub fn build(self) -> ComputePacketMirroringTimeoutsEl {
        ComputePacketMirroringTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputePacketMirroringTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroringTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroringTimeoutsElRef {
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
struct ComputePacketMirroringDynamic {
    collector_ilb: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringCollectorIlbEl>>,
    filter: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringFilterEl>>,
    mirrored_resources: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringMirroredResourcesEl>>,
    network: std::option::Option<terrars::DynamicBlock<ComputePacketMirroringNetworkEl>>,
}
