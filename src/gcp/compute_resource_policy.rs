#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeResourcePolicyData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_consistency_group_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_DiskConsistencyGroupPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_placement_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_GroupPlacementPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_schedule_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_InstanceSchedulePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_schedule_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeResourcePolicy_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_WorkloadPolicy>>,
    dynamic: ComputeResourcePolicyDynamic,
}

struct ComputeResourcePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeResourcePolicyData_>,
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

    #[doc = "Set the field `region`.\nRegion where resource policy resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_consistency_group_policy`.\n"]
    pub fn set_disk_consistency_group_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_DiskConsistencyGroupPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_GroupPlacementPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeResourcePolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload_policy`.\n"]
    pub fn set_workload_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_WorkloadPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload_policy = std::option::Option::Some(d);
            },
        }
        self
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
    pub fn disk_consistency_group_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_DiskConsistencyGroupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_GroupPlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_InstanceSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicy_TimeoutsRef {
        ComputeResourcePolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_WorkloadPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_policy", self.extract_ref()))
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
            data: std::cell::RefCell::new(ComputeResourcePolicyData_ {
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
                region: core::default::Default::default(),
                disk_consistency_group_policy: core::default::Default::default(),
                group_placement_policy: core::default::Default::default(),
                instance_schedule_policy: core::default::Default::default(),
                snapshot_schedule_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload_policy: core::default::Default::default(),
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
    pub fn disk_consistency_group_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_DiskConsistencyGroupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_GroupPlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_InstanceSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicy_TimeoutsRef {
        ComputeResourcePolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_WorkloadPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicyDynamic {
    disk_consistency_group_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_DiskConsistencyGroupPolicy>>,
    group_placement_policy: std::option::Option<terrars::DynamicBlock<ComputeResourcePolicy_GroupPlacementPolicy>>,
    instance_schedule_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_InstanceSchedulePolicy>>,
    snapshot_schedule_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy>>,
    workload_policy: std::option::Option<terrars::DynamicBlock<ComputeResourcePolicy_WorkloadPolicy>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_DiskConsistencyGroupPolicy {
    enabled: terrars::PrimField<bool>,
}

impl ComputeResourcePolicy_DiskConsistencyGroupPolicy { }

impl terrars::ToListMappable for ComputeResourcePolicy_DiskConsistencyGroupPolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_DiskConsistencyGroupPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_DiskConsistencyGroupPolicy {
    #[doc = "Enable disk consistency on the resource policy."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildComputeResourcePolicy_DiskConsistencyGroupPolicy {
    pub fn build(self) -> ComputeResourcePolicy_DiskConsistencyGroupPolicy {
        ComputeResourcePolicy_DiskConsistencyGroupPolicy { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
        ComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeResourcePolicy_GroupPlacementPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeResourcePolicy_GroupPlacementPolicy {
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

    #[doc = "Set the field `gpu_topology`.\nSpecifies the shape of the GPU slice, in slice based GPU families eg. A4X."]
    pub fn set_gpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gpu_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_count`.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicy_GroupPlacementPolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_GroupPlacementPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_GroupPlacementPolicy {}

impl BuildComputeResourcePolicy_GroupPlacementPolicy {
    pub fn build(self) -> ComputeResourcePolicy_GroupPlacementPolicy {
        ComputeResourcePolicy_GroupPlacementPolicy {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            gpu_topology: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_GroupPlacementPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_GroupPlacementPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResourcePolicy_GroupPlacementPolicyRef {
        ComputeResourcePolicy_GroupPlacementPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_GroupPlacementPolicyRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `gpu_topology` after provisioning.\nSpecifies the shape of the GPU slice, in slice based GPU families eg. A4X."]
    pub fn gpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_topology", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_count` after provisioning.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    schedule: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule { }

impl terrars::ToListMappable for ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    pub fn build(self) -> ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
        ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule { schedule: self.schedule }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
        ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    #[allow(dead_code)]
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
pub struct ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    schedule: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule { }

impl terrars::ToListMappable for ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    pub fn build(self) -> ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
        ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule { schedule: self.schedule }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
        ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    #[allow(dead_code)]
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
struct ComputeResourcePolicy_InstanceSchedulePolicyDynamic {
    vm_start_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>>,
    vm_stop_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_InstanceSchedulePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    time_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_start_schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_stop_schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>>,
    dynamic: ComputeResourcePolicy_InstanceSchedulePolicyDynamic,
}

impl ComputeResourcePolicy_InstanceSchedulePolicy {
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
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>>,
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
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>>,
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

impl terrars::ToListMappable for ComputeResourcePolicy_InstanceSchedulePolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_InstanceSchedulePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_InstanceSchedulePolicy {
    #[doc =
        "Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name\nfrom the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_InstanceSchedulePolicy {
    pub fn build(self) -> ComputeResourcePolicy_InstanceSchedulePolicy {
        ComputeResourcePolicy_InstanceSchedulePolicy {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: self.time_zone,
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_InstanceSchedulePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_InstanceSchedulePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_InstanceSchedulePolicyRef {
        ComputeResourcePolicy_InstanceSchedulePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_InstanceSchedulePolicyRef {
    #[allow(dead_code)]
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
    pub fn vm_start_schedule(&self) -> terrars::ListRef<ComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_start_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(&self) -> terrars::ListRef<ComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_stop_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    max_retention_days: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_source_disk_delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
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

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    #[doc = "Maximum age of the snapshot that is allowed to be kept."]
    pub max_retention_days: terrars::PrimField<f64>,
}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
        ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
            max_retention_days: self.max_retention_days,
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    days_in_cycle: terrars::PrimField<f64>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule { }

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    #[doc =
        "Defines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1."]
    pub days_in_cycle: terrars::PrimField<f64>,
    #[doc =
        "This must be in UTC format that resolves to one of\n00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,\nboth 13:00-5 and 08:00 are valid."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
            days_in_cycle: self.days_in_cycle,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    #[allow(dead_code)]
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
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    hours_in_cycle: terrars::PrimField<f64>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule { }

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    #[doc = "The number of hours between snapshots."]
    pub hours_in_cycle: terrars::PrimField<f64>,
    #[doc =
        "Time within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT. eg: 21:00"]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
            hours_in_cycle: self.hours_in_cycle,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours_in_cycle` after provisioning.\nThe number of hours between snapshots."]
    pub fn hours_in_cycle(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours_in_cycle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nTime within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT. eg: 21:00"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    day: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks { }

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    type O =
        terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    #[doc =
        "The day of the week to create the snapshot. e.g. MONDAY Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
    #[doc =
        "Time within the window to start the operations.\nIt must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
            day: self.day,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    #[allow(dead_code)]
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
struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleDynamic {
    day_of_weeks: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_weeks: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks>>,
    dynamic: ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleDynamic,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
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
                            ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks,
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

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
            day_of_weeks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleDynamic {
    daily_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>>,
    hourly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>>,
    weekly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>>,
    dynamic: ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleDynamic,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>,
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
                        terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>,
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
                        terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>,
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

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
        ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
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

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
        ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
        ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    #[allow(dead_code)]
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
struct ComputeResourcePolicy_SnapshotSchedulePolicyDynamic {
    retention_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>>,
    schedule: std::option::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>>,
    snapshot_properties: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>>,
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_properties: std
    ::option
    ::Option<std::vec::Vec<ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>>,
    dynamic: ComputeResourcePolicy_SnapshotSchedulePolicyDynamic,
}

impl ComputeResourcePolicy_SnapshotSchedulePolicy {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>>,
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
                    ::Into<terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>>,
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

impl terrars::ToListMappable for ComputeResourcePolicy_SnapshotSchedulePolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_SnapshotSchedulePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_SnapshotSchedulePolicy {}

impl BuildComputeResourcePolicy_SnapshotSchedulePolicy {
    pub fn build(self) -> ComputeResourcePolicy_SnapshotSchedulePolicy {
        ComputeResourcePolicy_SnapshotSchedulePolicy {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_SnapshotSchedulePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_SnapshotSchedulePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResourcePolicy_SnapshotSchedulePolicyRef {
        ComputeResourcePolicy_SnapshotSchedulePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_SnapshotSchedulePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> terrars::ListRef<ComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResourcePolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResourcePolicy_Timeouts {
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

impl terrars::ToListMappable for ComputeResourcePolicy_Timeouts {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_Timeouts {}

impl BuildComputeResourcePolicy_Timeouts {
    pub fn build(self) -> ComputeResourcePolicy_Timeouts {
        ComputeResourcePolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResourcePolicy_TimeoutsRef {
        ComputeResourcePolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ComputeResourcePolicy_WorkloadPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_topology_distance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeResourcePolicy_WorkloadPolicy {
    #[doc =
        "Set the field `accelerator_topology`.\nThe accelerator topology. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if max topology distance is set."]
    pub fn set_accelerator_topology(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.accelerator_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_topology_distance`.\nThe maximum topology distance. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if accelerator topology or accelerator topology mode is set. Possible values: [\"BLOCK\", \"CLUSTER\", \"SUBBLOCK\"]"]
    pub fn set_max_topology_distance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_topology_distance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResourcePolicy_WorkloadPolicy {
    type O = terrars::BlockAssignable<ComputeResourcePolicy_WorkloadPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResourcePolicy_WorkloadPolicy {
    #[doc = "The type of workload policy. Possible values: [\"HIGH_AVAILABILITY\", \"HIGH_THROUGHPUT\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeResourcePolicy_WorkloadPolicy {
    pub fn build(self) -> ComputeResourcePolicy_WorkloadPolicy {
        ComputeResourcePolicy_WorkloadPolicy {
            accelerator_topology: core::default::Default::default(),
            max_topology_distance: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResourcePolicy_WorkloadPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResourcePolicy_WorkloadPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResourcePolicy_WorkloadPolicyRef {
        ComputeResourcePolicy_WorkloadPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResourcePolicy_WorkloadPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_topology` after provisioning.\nThe accelerator topology. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if max topology distance is set."]
    pub fn accelerator_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_topology", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_topology_distance` after provisioning.\nThe maximum topology distance. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if accelerator topology or accelerator topology mode is set. Possible values: [\"BLOCK\", \"CLUSTER\", \"SUBBLOCK\"]"]
    pub fn max_topology_distance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_topology_distance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of workload policy. Possible values: [\"HIGH_AVAILABILITY\", \"HIGH_THROUGHPUT\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
