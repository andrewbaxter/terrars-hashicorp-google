use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeResourcePolicyData {
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
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_consistency_group_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_placement_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_schedule_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_schedule_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeResourcePolicyTimeoutsEl>,
    dynamic: ComputeResourcePolicyDynamic,
}

struct ComputeResourcePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeResourcePolicyData>,
}

#[derive(Clone)]
pub struct ComputeResourcePolicy(std::rc::Rc<ComputeResourcePolicy_>);

impl ComputeResourcePolicy {
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

    #[doc = "Set the field `region`.\nRegion where resource policy resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_consistency_group_policy`.\n"]
    pub fn set_disk_consistency_group_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_consistency_group_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_consistency_group_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `group_placement_policy`.\n"]
    pub fn set_group_placement_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group_placement_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group_placement_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_schedule_policy`.\n"]
    pub fn set_instance_schedule_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_schedule_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_schedule_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_schedule_policy`.\n"]
    pub fn set_snapshot_schedule_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_schedule_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_schedule_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeResourcePolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
        "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where resource policy resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\n"]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(&self) -> terrars::ListRef<ComputeResourcePolicyGroupPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicyInstanceSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeResourcePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeResourcePolicy { }

impl terrars::ToListMappable for ComputeResourcePolicy {
    type O = terrars::ListRef<ComputeResourcePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeResourcePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_resource_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeResourcePolicy {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeResourcePolicy {
        let out = ComputeResourcePolicy(std::rc::Rc::new(ComputeResourcePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeResourcePolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                disk_consistency_group_policy: core::default::Default::default(),
                group_placement_policy: core::default::Default::default(),
                instance_schedule_policy: core::default::Default::default(),
                snapshot_schedule_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeResourcePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeResourcePolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where resource policy resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\n"]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(&self) -> terrars::ListRef<ComputeResourcePolicyGroupPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicyInstanceSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    enabled: terrars::PrimField<bool>,
}

impl ComputeResourcePolicyDiskConsistencyGroupPolicyEl { }

impl terrars::ToListMappable for ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[doc = "Enable disk consistency on the resource policy."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    pub fn build(self) -> ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
        ComputeResourcePolicyDiskConsistencyGroupPolicyEl { enabled: self.enabled }
    }
}

pub struct ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
        ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable disk consistency on the resource policy."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyGroupPlacementPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeResourcePolicyGroupPlacementPolicyEl {
    #[doc =
        "Set the field `availability_domain_count`.\nThe number of availability domains instances will be spread across. If two instances are in different\navailability domain, they will not be put in the same low latency network"]
    pub fn set_availability_domain_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_domain_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `collocation`.\nCollocation specifies whether to place VMs inside the same availability domain on the same low-latency network.\nSpecify 'COLLOCATED' to enable collocation. Can only be specified with 'vm_count'. If compute instances are created\nwith a COLLOCATED policy, then exactly 'vm_count' instances must be created at the same time with the resource policy\nattached. Possible values: [\"COLLOCATED\"]"]
    pub fn set_collocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collocation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_count`.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicyGroupPlacementPolicyEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyGroupPlacementPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyGroupPlacementPolicyEl {}

impl BuildComputeResourcePolicyGroupPlacementPolicyEl {
    pub fn build(self) -> ComputeResourcePolicyGroupPlacementPolicyEl {
        ComputeResourcePolicyGroupPlacementPolicyEl {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicyGroupPlacementPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyGroupPlacementPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicyGroupPlacementPolicyElRef {
        ComputeResourcePolicyGroupPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyGroupPlacementPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `availability_domain_count` after provisioning.\nThe number of availability domains instances will be spread across. If two instances are in different\navailability domain, they will not be put in the same low latency network"]
    pub fn availability_domain_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_domain_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `collocation` after provisioning.\nCollocation specifies whether to place VMs inside the same availability domain on the same low-latency network.\nSpecify 'COLLOCATED' to enable collocation. Can only be specified with 'vm_count'. If compute instances are created\nwith a COLLOCATED policy, then exactly 'vm_count' instances must be created at the same time with the resource policy\nattached. Possible values: [\"COLLOCATED\"]"]
    pub fn collocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collocation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_count` after provisioning.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    schedule: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl { }

impl terrars::ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl { schedule: self.schedule }
    }
}

pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nSpecifies the frequency for the operation, using the unix-cron format."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    schedule: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl { }

impl terrars::ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl { schedule: self.schedule }
    }
}

pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nSpecifies the frequency for the operation, using the unix-cron format."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicyInstanceSchedulePolicyElDynamic {
    vm_start_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    vm_stop_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    time_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_start_schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_stop_schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    dynamic: ComputeResourcePolicyInstanceSchedulePolicyElDynamic,
}

impl ComputeResourcePolicyInstanceSchedulePolicyEl {
    #[doc =
        "Set the field `expiration_time`.\nThe expiration time of the schedule. The timestamp is an RFC3339 string."]
    pub fn set_expiration_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\nThe start time of the schedule. The timestamp is an RFC3339 string."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_start_schedule`.\n"]
    pub fn set_vm_start_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vm_start_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_start_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vm_stop_schedule`.\n"]
    pub fn set_vm_stop_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vm_stop_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_stop_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyInstanceSchedulePolicyEl {
    #[doc =
        "Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name\nfrom the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicyInstanceSchedulePolicyEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyEl {
        ComputeResourcePolicyInstanceSchedulePolicyEl {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: self.time_zone,
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicyInstanceSchedulePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicyInstanceSchedulePolicyElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expiration_time` after provisioning.\nThe expiration time of the schedule. The timestamp is an RFC3339 string."]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe start time of the schedule. The timestamp is an RFC3339 string."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name\nfrom the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_start_schedule` after provisioning.\n"]
    pub fn vm_start_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_start_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(&self) -> terrars::ListRef<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_stop_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    max_retention_days: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_source_disk_delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[doc =
        "Set the field `on_source_disk_delete`.\nSpecifies the behavior to apply to scheduled snapshots when\nthe source disk is deleted. Default value: \"KEEP_AUTO_SNAPSHOTS\" Possible values: [\"KEEP_AUTO_SNAPSHOTS\", \"APPLY_RETENTION_POLICY\"]"]
    pub fn set_on_source_disk_delete(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_source_disk_delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[doc = "Maximum age of the snapshot that is allowed to be kept."]
    pub max_retention_days: terrars::PrimField<f64>,
}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
        ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
            max_retention_days: self.max_retention_days,
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_retention_days` after provisioning.\nMaximum age of the snapshot that is allowed to be kept."]
    pub fn max_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retention_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_source_disk_delete` after provisioning.\nSpecifies the behavior to apply to scheduled snapshots when\nthe source disk is deleted. Default value: \"KEEP_AUTO_SNAPSHOTS\" Possible values: [\"KEEP_AUTO_SNAPSHOTS\", \"APPLY_RETENTION_POLICY\"]"]
    pub fn on_source_disk_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_source_disk_delete", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    days_in_cycle: terrars::PrimField<f64>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl { }

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    #[doc =
        "Defines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1."]
    pub days_in_cycle: terrars::PrimField<f64>,
    #[doc =
        "This must be in UTC format that resolves to one of\n00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,\nboth 13:00-5 and 08:00 are valid."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
            days_in_cycle: self.days_in_cycle,
            start_time: self.start_time,
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_in_cycle` after provisioning.\nDefines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1."]
    pub fn days_in_cycle(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_in_cycle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThis must be in UTC format that resolves to one of\n00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,\nboth 13:00-5 and 08:00 are valid."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    hours_in_cycle: terrars::PrimField<f64>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl { }

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    #[doc = "The number of hours between snapshots."]
    pub hours_in_cycle: terrars::PrimField<f64>,
    #[doc =
        "Time within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT.\neg: 21:00"]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
            hours_in_cycle: self.hours_in_cycle,
            start_time: self.start_time,
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours_in_cycle` after provisioning.\nThe number of hours between snapshots."]
    pub fn hours_in_cycle(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours_in_cycle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nTime within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT.\neg: 21:00"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    day: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl { }

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    type O =
        terrars
        ::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    #[doc =
        "The day of the week to create the snapshot. e.g. MONDAY Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
    #[doc =
        "Time within the window to start the operations.\nIt must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
            day: self.day,
            start_time: self.start_time,
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nThe day of the week to create the snapshot. e.g. MONDAY Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nTime within the window to start the operations.\nIt must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDynamic {
    day_of_weeks: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_weeks: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>>,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDynamic,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[doc = "Set the field `day_of_weeks`.\n"]
    pub fn set_day_of_weeks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.day_of_weeks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.day_of_weeks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
            day_of_weeks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDynamic {
    daily_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>>,
    hourly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>>,
    weekly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>>,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDynamic,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hourly_schedule`.\n"]
    pub fn set_hourly_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hourly_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[doc =
        "Set the field `chain_name`.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and comply\nwith RFC1035."]
    pub fn set_chain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_flush`.\nWhether to perform a 'guest aware' snapshot."]
    pub fn set_guest_flush(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.guest_flush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nA set of key-value pairs."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_locations`.\nCloud Storage bucket location to store the auto snapshot\n(regional or multi-regional)"]
    pub fn set_storage_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.storage_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {}

impl BuildComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
        ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and comply\nwith RFC1035."]
    pub fn chain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guest_flush` after provisioning.\nWhether to perform a 'guest aware' snapshot."]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key-value pairs."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket location to store the auto snapshot\n(regional or multi-regional)"]
    pub fn storage_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.storage_locations", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElDynamic {
    retention_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    snapshot_properties: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_properties: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElDynamic,
}

impl ComputeResourcePolicySnapshotSchedulePolicyEl {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retention_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_properties`.\n"]
    pub fn set_snapshot_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snapshot_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicySnapshotSchedulePolicyEl {}

impl BuildComputeResourcePolicySnapshotSchedulePolicyEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyEl {
        ComputeResourcePolicySnapshotSchedulePolicyEl {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicySnapshotSchedulePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResourcePolicyTimeoutsEl {
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
}

impl terrars::ToListMappable for ComputeResourcePolicyTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeResourcePolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicyTimeoutsEl {}

impl BuildComputeResourcePolicyTimeoutsEl {
    pub fn build(self) -> ComputeResourcePolicyTimeoutsEl {
        ComputeResourcePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeResourcePolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicyTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicyDynamic {
    disk_consistency_group_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    group_placement_policy: std::option::Option<terrars::DynamicBlock<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    instance_schedule_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    snapshot_schedule_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
}
