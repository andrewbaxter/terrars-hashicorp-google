#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputePacketMirroringData_ {
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
    enable: std::option::Option<terrars::PrimField<std::string::String>>,
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
    collector_ilb: std::option::Option<std::vec::Vec<ComputePacketMirroring_CollectorIlb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<ComputePacketMirroring_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirrored_resources: std::option::Option<std::vec::Vec<ComputePacketMirroring_MirroredResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<ComputePacketMirroring_Network>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputePacketMirroring_Timeouts>,
    dynamic: ComputePacketMirroringDynamic,
}

struct ComputePacketMirroring_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputePacketMirroringData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human-readable description of the rule."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable`.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn set_enable(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().enable = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_CollectorIlb>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_Filter>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_MirroredResources>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_Network>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputePacketMirroring_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn enable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
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
    pub fn collector_ilb(&self) -> terrars::ListRef<ComputePacketMirroring_CollectorIlbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collector_ilb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ComputePacketMirroring_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> terrars::ListRef<ComputePacketMirroring_MirroredResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mirrored_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<ComputePacketMirroring_NetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroring_TimeoutsRef {
        ComputePacketMirroring_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(ComputePacketMirroringData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn enable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
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
    pub fn collector_ilb(&self) -> terrars::ListRef<ComputePacketMirroring_CollectorIlbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collector_ilb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ComputePacketMirroring_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> terrars::ListRef<ComputePacketMirroring_MirroredResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mirrored_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<ComputePacketMirroring_NetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroring_TimeoutsRef {
        ComputePacketMirroring_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputePacketMirroringDynamic {
    collector_ilb: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_CollectorIlb>>,
    filter: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_Filter>>,
    mirrored_resources: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_MirroredResources>>,
    network: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_Network>>,
}

#[derive(Serialize)]
pub struct ComputePacketMirroring_CollectorIlb {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroring_CollectorIlb { }

impl terrars::ToListMappable for ComputePacketMirroring_CollectorIlb {
    type O = terrars::BlockAssignable<ComputePacketMirroring_CollectorIlb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_CollectorIlb {
    #[doc = "The URL of the forwarding rule."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroring_CollectorIlb {
    pub fn build(self) -> ComputePacketMirroring_CollectorIlb {
        ComputePacketMirroring_CollectorIlb { url: self.url }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_CollectorIlbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_CollectorIlbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroring_CollectorIlbRef {
        ComputePacketMirroring_CollectorIlbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_CollectorIlbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the forwarding rule."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroring_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputePacketMirroring_Filter {
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

impl terrars::ToListMappable for ComputePacketMirroring_Filter {
    type O = terrars::BlockAssignable<ComputePacketMirroring_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_Filter {}

impl BuildComputePacketMirroring_Filter {
    pub fn build(self) -> ComputePacketMirroring_Filter {
        ComputePacketMirroring_Filter {
            cidr_ranges: core::default::Default::default(),
            direction: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroring_FilterRef {
        ComputePacketMirroring_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_FilterRef {
    #[allow(dead_code)]
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
pub struct ComputePacketMirroring_MirroredResources_Instances {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroring_MirroredResources_Instances { }

impl terrars::ToListMappable for ComputePacketMirroring_MirroredResources_Instances {
    type O = terrars::BlockAssignable<ComputePacketMirroring_MirroredResources_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_MirroredResources_Instances {
    #[doc = "The URL of the instances where this rule should be active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroring_MirroredResources_Instances {
    pub fn build(self) -> ComputePacketMirroring_MirroredResources_Instances {
        ComputePacketMirroring_MirroredResources_Instances { url: self.url }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_MirroredResources_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_MirroredResources_InstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePacketMirroring_MirroredResources_InstancesRef {
        ComputePacketMirroring_MirroredResources_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_MirroredResources_InstancesRef {
    #[allow(dead_code)]
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
pub struct ComputePacketMirroring_MirroredResources_Subnetworks {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroring_MirroredResources_Subnetworks { }

impl terrars::ToListMappable for ComputePacketMirroring_MirroredResources_Subnetworks {
    type O = terrars::BlockAssignable<ComputePacketMirroring_MirroredResources_Subnetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_MirroredResources_Subnetworks {
    #[doc = "The URL of the subnetwork where this rule should be active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroring_MirroredResources_Subnetworks {
    pub fn build(self) -> ComputePacketMirroring_MirroredResources_Subnetworks {
        ComputePacketMirroring_MirroredResources_Subnetworks { url: self.url }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_MirroredResources_SubnetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_MirroredResources_SubnetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePacketMirroring_MirroredResources_SubnetworksRef {
        ComputePacketMirroring_MirroredResources_SubnetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_MirroredResources_SubnetworksRef {
    #[allow(dead_code)]
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
struct ComputePacketMirroring_MirroredResourcesDynamic {
    instances: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_MirroredResources_Instances>>,
    subnetworks: std::option::Option<terrars::DynamicBlock<ComputePacketMirroring_MirroredResources_Subnetworks>>,
}

#[derive(Serialize)]
pub struct ComputePacketMirroring_MirroredResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<std::vec::Vec<ComputePacketMirroring_MirroredResources_Instances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetworks: std::option::Option<std::vec::Vec<ComputePacketMirroring_MirroredResources_Subnetworks>>,
    dynamic: ComputePacketMirroring_MirroredResourcesDynamic,
}

impl ComputePacketMirroring_MirroredResources {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_MirroredResources_Instances>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePacketMirroring_MirroredResources_Subnetworks>>,
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

impl terrars::ToListMappable for ComputePacketMirroring_MirroredResources {
    type O = terrars::BlockAssignable<ComputePacketMirroring_MirroredResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_MirroredResources {}

impl BuildComputePacketMirroring_MirroredResources {
    pub fn build(self) -> ComputePacketMirroring_MirroredResources {
        ComputePacketMirroring_MirroredResources {
            tags: core::default::Default::default(),
            instances: core::default::Default::default(),
            subnetworks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_MirroredResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_MirroredResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroring_MirroredResourcesRef {
        ComputePacketMirroring_MirroredResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_MirroredResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nAll instances with these tags will be mirrored."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePacketMirroring_Network {
    url: terrars::PrimField<std::string::String>,
}

impl ComputePacketMirroring_Network { }

impl terrars::ToListMappable for ComputePacketMirroring_Network {
    type O = terrars::BlockAssignable<ComputePacketMirroring_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_Network {
    #[doc = "The full self_link URL of the network where this rule is active."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildComputePacketMirroring_Network {
    pub fn build(self) -> ComputePacketMirroring_Network {
        ComputePacketMirroring_Network { url: self.url }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_NetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroring_NetworkRef {
        ComputePacketMirroring_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_NetworkRef {
    #[allow(dead_code)]
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
pub struct ComputePacketMirroring_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputePacketMirroring_Timeouts {
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

impl terrars::ToListMappable for ComputePacketMirroring_Timeouts {
    type O = terrars::BlockAssignable<ComputePacketMirroring_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePacketMirroring_Timeouts {}

impl BuildComputePacketMirroring_Timeouts {
    pub fn build(self) -> ComputePacketMirroring_Timeouts {
        ComputePacketMirroring_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePacketMirroring_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePacketMirroring_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePacketMirroring_TimeoutsRef {
        ComputePacketMirroring_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePacketMirroring_TimeoutsRef {
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
