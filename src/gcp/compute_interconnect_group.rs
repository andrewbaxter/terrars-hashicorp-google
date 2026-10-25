#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInterconnectGroupData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<ComputeInterconnectGroup_Intent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnects: std::option::Option<std::vec::Vec<ComputeInterconnectGroup_Interconnects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInterconnectGroup_Timeouts>,
    dynamic: ComputeInterconnectGroupDynamic,
}

struct ComputeInterconnectGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInterconnectGroupData_>,
}

#[derive(Clone)]
pub struct ComputeInterconnectGroup(std::rc::Rc<ComputeInterconnectGroup_>);

impl ComputeInterconnectGroup {
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

    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectGroup_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectGroup_Interconnects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interconnects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interconnects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInterconnectGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `configured` after provisioning.\nThe status of the group as configured. This has the same\nstructure as the operational field reported by the OperationalStatus\nmethod, but does not take into account the operational status of each\nresource."]
    pub fn configured(&self) -> terrars::ListRef<ComputeInterconnectGroup_ConfiguredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configured", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `physical_structure` after provisioning.\nAn analysis of the physical layout of Interconnects in this\ngroup. Every Interconnect in the group is shown once in this structure."]
    pub fn physical_structure(&self) -> terrars::ListRef<ComputeInterconnectGroup_PhysicalStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.physical_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<ComputeInterconnectGroup_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectGroup_TimeoutsRef {
        ComputeInterconnectGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeInterconnectGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInterconnectGroup { }

impl terrars::ToListMappable for ComputeInterconnectGroup {
    type O = terrars::ListRef<ComputeInterconnectGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInterconnectGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_interconnect_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInterconnectGroup {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInterconnectGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInterconnectGroup {
        let out = ComputeInterconnectGroup(std::rc::Rc::new(ComputeInterconnectGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInterconnectGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                intent: core::default::Default::default(),
                interconnects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInterconnectGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInterconnectGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `configured` after provisioning.\nThe status of the group as configured. This has the same\nstructure as the operational field reported by the OperationalStatus\nmethod, but does not take into account the operational status of each\nresource."]
    pub fn configured(&self) -> terrars::ListRef<ComputeInterconnectGroup_ConfiguredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configured", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `physical_structure` after provisioning.\nAn analysis of the physical layout of Interconnects in this\ngroup. Every Interconnect in the group is shown once in this structure."]
    pub fn physical_structure(&self) -> terrars::ListRef<ComputeInterconnectGroup_PhysicalStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.physical_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<ComputeInterconnectGroup_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectGroup_TimeoutsRef {
        ComputeInterconnectGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeInterconnectGroupDynamic {
    intent: std::option::Option<terrars::DynamicBlock<ComputeInterconnectGroup_Intent>>,
    interconnects: std::option::Option<terrars::DynamicBlock<ComputeInterconnectGroup_Interconnects>>,
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocker_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explanation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facilities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metros: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
    #[doc = "Set the field `blocker_type`.\n"]
    pub fn set_blocker_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.blocker_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documentation_link`.\n"]
    pub fn set_documentation_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.documentation_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `explanation`.\n"]
    pub fn set_explanation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.explanation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.facilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.interconnects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metros = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
    type O =
        terrars::BlockAssignable<ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {}

impl BuildComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
    pub fn build(self) -> ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
        ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers {
            blocker_type: core::default::Default::default(),
            documentation_link: core::default::Default::default(),
            explanation: core::default::Default::default(),
            facilities: core::default::Default::default(),
            interconnects: core::default::Default::default(),
            metros: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef {
        ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blocker_type` after provisioning.\n"]
    pub fn blocker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocker_type", self.base))
    }

    #[doc = "Get a reference to the value of field `documentation_link` after provisioning.\n"]
    pub fn documentation_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_link", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation` after provisioning.\n"]
    pub fn explanation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explanation", self.base))
    }

    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }

    #[doc = "Get a reference to the value of field `interconnects` after provisioning.\n"]
    pub fn interconnects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interconnects", self.base))
    }

    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Configured_TopologyCapability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intended_capability_blockers: std
    ::option
    ::Option<terrars::ListField<ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_sla: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectGroup_Configured_TopologyCapability {
    #[doc = "Set the field `intended_capability_blockers`.\n"]
    pub fn set_intended_capability_blockers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockers>,
                    >,
    ) -> Self {
        self.intended_capability_blockers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_sla`.\n"]
    pub fn set_supported_sla(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.supported_sla = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_Configured_TopologyCapability {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_Configured_TopologyCapability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Configured_TopologyCapability {}

impl BuildComputeInterconnectGroup_Configured_TopologyCapability {
    pub fn build(self) -> ComputeInterconnectGroup_Configured_TopologyCapability {
        ComputeInterconnectGroup_Configured_TopologyCapability {
            intended_capability_blockers: core::default::Default::default(),
            supported_sla: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_Configured_TopologyCapabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_Configured_TopologyCapabilityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectGroup_Configured_TopologyCapabilityRef {
        ComputeInterconnectGroup_Configured_TopologyCapabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_Configured_TopologyCapabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intended_capability_blockers` after provisioning.\n"]
    pub fn intended_capability_blockers(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectGroup_Configured_TopologyCapability_IntendedCapabilityBlockersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intended_capability_blockers", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_sla` after provisioning.\n"]
    pub fn supported_sla(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supported_sla", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Configured {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_capability: std::option::Option<terrars::ListField<ComputeInterconnectGroup_Configured_TopologyCapability>>,
}

impl ComputeInterconnectGroup_Configured {
    #[doc = "Set the field `topology_capability`.\n"]
    pub fn set_topology_capability(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInterconnectGroup_Configured_TopologyCapability>>,
    ) -> Self {
        self.topology_capability = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_Configured {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_Configured>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Configured {}

impl BuildComputeInterconnectGroup_Configured {
    pub fn build(self) -> ComputeInterconnectGroup_Configured {
        ComputeInterconnectGroup_Configured { topology_capability: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_ConfiguredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_ConfiguredRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectGroup_ConfiguredRef {
        ComputeInterconnectGroup_ConfiguredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_ConfiguredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topology_capability` after provisioning.\n"]
    pub fn topology_capability(&self) -> terrars::ListRef<ComputeInterconnectGroup_Configured_TopologyCapabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology_capability", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.interconnects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {}

impl BuildComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
    pub fn build(self) -> ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
        ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones {
            interconnects: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef {
        ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interconnects` after provisioning.\n"]
    pub fn interconnects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interconnects", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones>>,
}

impl ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
    #[doc = "Set the field `facility`.\n"]
    pub fn set_facility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.facility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_Zones>>,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {}

impl BuildComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
    pub fn build(self) -> ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
        ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities {
            facility: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef {
        ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `facility` after provisioning.\n"]
    pub fn facility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.facility", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> terrars::ListRef<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities_ZonesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_PhysicalStructure_Metros {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facilities: std::option::Option<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metro: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectGroup_PhysicalStructure_Metros {
    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros_Facilities>>,
    ) -> Self {
        self.facilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metro`.\n"]
    pub fn set_metro(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metro = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_PhysicalStructure_Metros {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_PhysicalStructure_Metros>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_PhysicalStructure_Metros {}

impl BuildComputeInterconnectGroup_PhysicalStructure_Metros {
    pub fn build(self) -> ComputeInterconnectGroup_PhysicalStructure_Metros {
        ComputeInterconnectGroup_PhysicalStructure_Metros {
            facilities: core::default::Default::default(),
            metro: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_PhysicalStructure_MetrosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_PhysicalStructure_MetrosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectGroup_PhysicalStructure_MetrosRef {
        ComputeInterconnectGroup_PhysicalStructure_MetrosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_PhysicalStructure_MetrosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(&self) -> terrars::ListRef<ComputeInterconnectGroup_PhysicalStructure_Metros_FacilitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }

    #[doc = "Get a reference to the value of field `metro` after provisioning.\n"]
    pub fn metro(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metro", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_PhysicalStructure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metros: std::option::Option<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros>>,
}

impl ComputeInterconnectGroup_PhysicalStructure {
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInterconnectGroup_PhysicalStructure_Metros>>,
    ) -> Self {
        self.metros = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_PhysicalStructure {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_PhysicalStructure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_PhysicalStructure {}

impl BuildComputeInterconnectGroup_PhysicalStructure {
    pub fn build(self) -> ComputeInterconnectGroup_PhysicalStructure {
        ComputeInterconnectGroup_PhysicalStructure { metros: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_PhysicalStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_PhysicalStructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectGroup_PhysicalStructureRef {
        ComputeInterconnectGroup_PhysicalStructureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_PhysicalStructureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> terrars::ListRef<ComputeInterconnectGroup_PhysicalStructure_MetrosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Intent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_capability: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectGroup_Intent {
    #[doc =
        "Set the field `topology_capability`.\nThe reliability the user intends this group to be capable of, in terms\nof the Interconnect product SLAs. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn set_topology_capability(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.topology_capability = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_Intent {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_Intent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Intent {}

impl BuildComputeInterconnectGroup_Intent {
    pub fn build(self) -> ComputeInterconnectGroup_Intent {
        ComputeInterconnectGroup_Intent { topology_capability: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_IntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_IntentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectGroup_IntentRef {
        ComputeInterconnectGroup_IntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_IntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `topology_capability` after provisioning.\nThe reliability the user intends this group to be capable of, in terms\nof the Interconnect product SLAs. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn topology_capability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topology_capability", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Interconnects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeInterconnectGroup_Interconnects {
    #[doc =
        "Set the field `interconnect`.\nThe URL of an Interconnect in this group. All Interconnects in the group are unique."]
    pub fn set_interconnect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interconnect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectGroup_Interconnects {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_Interconnects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Interconnects {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInterconnectGroup_Interconnects {
    pub fn build(self) -> ComputeInterconnectGroup_Interconnects {
        ComputeInterconnectGroup_Interconnects {
            interconnect: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_InterconnectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_InterconnectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectGroup_InterconnectsRef {
        ComputeInterconnectGroup_InterconnectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_InterconnectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `interconnect` after provisioning.\nThe URL of an Interconnect in this group. All Interconnects in the group are unique."]
    pub fn interconnect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectGroup_Timeouts {
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

impl terrars::ToListMappable for ComputeInterconnectGroup_Timeouts {
    type O = terrars::BlockAssignable<ComputeInterconnectGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectGroup_Timeouts {}

impl BuildComputeInterconnectGroup_Timeouts {
    pub fn build(self) -> ComputeInterconnectGroup_Timeouts {
        ComputeInterconnectGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectGroup_TimeoutsRef {
        ComputeInterconnectGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectGroup_TimeoutsRef {
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
