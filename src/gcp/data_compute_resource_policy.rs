#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeResourcePolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeResourcePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeResourcePolicyData_>,
}

#[derive(Clone)]
pub struct DataComputeResourcePolicy(std::rc::Rc<DataComputeResourcePolicy_>);

impl DataComputeResourcePolicy {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_GroupPlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_InstanceSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\nPolicy for creating snapshots of persistent disks."]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_policy` after provisioning.\nRepresents the workload policy."]
    pub fn workload_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_WorkloadPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_policy", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeResourcePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeResourcePolicy { }

impl terrars::ToListMappable for DataComputeResourcePolicy {
    type O = terrars::ListRef<DataComputeResourcePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeResourcePolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_resource_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeResourcePolicy {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeResourcePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeResourcePolicy {
        let out = DataComputeResourcePolicy(std::rc::Rc::new(DataComputeResourcePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeResourcePolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
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

pub struct DataComputeResourcePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeResourcePolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc =
        "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_GroupPlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_InstanceSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_schedule_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\nPolicy for creating snapshots of persistent disks."]
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_policy` after provisioning.\nRepresents the workload policy."]
    pub fn workload_policy(&self) -> terrars::ListRef<DataComputeResourcePolicy_WorkloadPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_DiskConsistencyGroupPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeResourcePolicy_DiskConsistencyGroupPolicy {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_DiskConsistencyGroupPolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_DiskConsistencyGroupPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_DiskConsistencyGroupPolicy {}

impl BuildDataComputeResourcePolicy_DiskConsistencyGroupPolicy {
    pub fn build(self) -> DataComputeResourcePolicy_DiskConsistencyGroupPolicy {
        DataComputeResourcePolicy_DiskConsistencyGroupPolicy { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
        DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_DiskConsistencyGroupPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_GroupPlacementPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeResourcePolicy_GroupPlacementPolicy {
    #[doc = "Set the field `availability_domain_count`.\n"]
    pub fn set_availability_domain_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_domain_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collocation`.\n"]
    pub fn set_collocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_topology`.\n"]
    pub fn set_gpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gpu_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_GroupPlacementPolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_GroupPlacementPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_GroupPlacementPolicy {}

impl BuildDataComputeResourcePolicy_GroupPlacementPolicy {
    pub fn build(self) -> DataComputeResourcePolicy_GroupPlacementPolicy {
        DataComputeResourcePolicy_GroupPlacementPolicy {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            gpu_topology: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_GroupPlacementPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_GroupPlacementPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_GroupPlacementPolicyRef {
        DataComputeResourcePolicy_GroupPlacementPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_GroupPlacementPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_domain_count` after provisioning.\n"]
    pub fn availability_domain_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_domain_count", self.base))
    }

    #[doc = "Get a reference to the value of field `collocation` after provisioning.\n"]
    pub fn collocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collocation", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_topology` after provisioning.\n"]
    pub fn gpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_topology", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {}

impl BuildDataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
    pub fn build(self) -> DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
        DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule {
            schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
        DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {}

impl BuildDataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
    pub fn build(self) -> DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
        DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule {
            schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
        DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_start_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_stop_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>>,
}

impl DataComputeResourcePolicy_InstanceSchedulePolicy {
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_start_schedule`.\n"]
    pub fn set_vm_start_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartSchedule>>,
    ) -> Self {
        self.vm_start_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_stop_schedule`.\n"]
    pub fn set_vm_stop_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopSchedule>>,
    ) -> Self {
        self.vm_stop_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_InstanceSchedulePolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_InstanceSchedulePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_InstanceSchedulePolicy {}

impl BuildDataComputeResourcePolicy_InstanceSchedulePolicy {
    pub fn build(self) -> DataComputeResourcePolicy_InstanceSchedulePolicy {
        DataComputeResourcePolicy_InstanceSchedulePolicy {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_InstanceSchedulePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_InstanceSchedulePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_InstanceSchedulePolicyRef {
        DataComputeResourcePolicy_InstanceSchedulePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_InstanceSchedulePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_start_schedule` after provisioning.\n"]
    pub fn vm_start_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStartScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_start_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_InstanceSchedulePolicy_VmStopScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_stop_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_source_disk_delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    #[doc = "Set the field `max_retention_days`.\n"]
    pub fn set_max_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_source_disk_delete`.\n"]
    pub fn set_on_source_disk_delete(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_source_disk_delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy {
            max_retention_days: core::default::Default::default(),
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_retention_days` after provisioning.\n"]
    pub fn max_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retention_days", self.base))
    }

    #[doc = "Get a reference to the value of field `on_source_disk_delete` after provisioning.\n"]
    pub fn on_source_disk_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_source_disk_delete", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_in_cycle: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    #[doc = "Set the field `days_in_cycle`.\n"]
    pub fn set_days_in_cycle(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_in_cycle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule {
            days_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_in_cycle` after provisioning.\n"]
    pub fn days_in_cycle(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_in_cycle", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_in_cycle: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    #[doc = "Set the field `hours_in_cycle`.\n"]
    pub fn set_hours_in_cycle(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours_in_cycle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule {
            hours_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours_in_cycle` after provisioning.\n"]
    pub fn hours_in_cycle(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours_in_cycle", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    type O =
        terrars
        ::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_weeks: std
    ::option
    ::Option<terrars::SetField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    #[doc = "Set the field `day_of_weeks`.\n"]
    pub fn set_day_of_weeks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeks,
                        >,
                    >,
    ) -> Self {
        self.day_of_weeks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule {
            day_of_weeks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_weeks` after provisioning.\n"]
    pub fn day_of_weeks(
        &self,
    ) -> terrars::SetRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule_DayOfWeeksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.day_of_weeks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailySchedule>,
                    >,
    ) -> Self {
        self.daily_schedule = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlySchedule>,
                    >,
    ) -> Self {
        self.hourly_schedule = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklySchedule>,
                    >,
    ) -> Self {
        self.weekly_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_DailyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_HourlyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule_WeeklyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    #[doc = "Set the field `chain_name`.\n"]
    pub fn set_chain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.guest_flush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_locations`.\n"]
    pub fn set_storage_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.storage_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chain_name` after provisioning.\n"]
    pub fn chain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\n"]
    pub fn storage_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.storage_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_properties: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>>,
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicy {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicy>>,
    ) -> Self {
        self.retention_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_Schedule>>,
    ) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_properties`.\n"]
    pub fn set_snapshot_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotProperties>>,
    ) -> Self {
        self.snapshot_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_SnapshotSchedulePolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_SnapshotSchedulePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_SnapshotSchedulePolicy {}

impl BuildDataComputeResourcePolicy_SnapshotSchedulePolicy {
    pub fn build(self) -> DataComputeResourcePolicy_SnapshotSchedulePolicy {
        DataComputeResourcePolicy_SnapshotSchedulePolicy {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_SnapshotSchedulePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_SnapshotSchedulePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicy_SnapshotSchedulePolicyRef {
        DataComputeResourcePolicy_SnapshotSchedulePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_SnapshotSchedulePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicy_SnapshotSchedulePolicy_SnapshotPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicy_WorkloadPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_topology_distance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicy_WorkloadPolicy {
    #[doc = "Set the field `accelerator_topology`.\n"]
    pub fn set_accelerator_topology(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.accelerator_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_topology_distance`.\n"]
    pub fn set_max_topology_distance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_topology_distance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicy_WorkloadPolicy {
    type O = terrars::BlockAssignable<DataComputeResourcePolicy_WorkloadPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicy_WorkloadPolicy {}

impl BuildDataComputeResourcePolicy_WorkloadPolicy {
    pub fn build(self) -> DataComputeResourcePolicy_WorkloadPolicy {
        DataComputeResourcePolicy_WorkloadPolicy {
            accelerator_topology: core::default::Default::default(),
            max_topology_distance: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeResourcePolicy_WorkloadPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicy_WorkloadPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeResourcePolicy_WorkloadPolicyRef {
        DataComputeResourcePolicy_WorkloadPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicy_WorkloadPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_topology` after provisioning.\n"]
    pub fn accelerator_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_topology", self.base))
    }

    #[doc = "Get a reference to the value of field `max_topology_distance` after provisioning.\n"]
    pub fn max_topology_distance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_topology_distance", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
