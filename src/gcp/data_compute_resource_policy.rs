use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeResourcePolicyData {
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
    data: std::cell::RefCell<DataComputeResourcePolicyData>,
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(&self) -> terrars::ListRef<DataComputeResourcePolicyGroupPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElRef> {
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
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataComputeResourcePolicyData {
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_consistency_group_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(&self) -> terrars::ListRef<DataComputeResourcePolicyGroupPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElRef> {
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
    pub fn snapshot_schedule_policy(&self) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_schedule_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicyDiskConsistencyGroupPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicyDiskConsistencyGroupPolicyEl {}

impl BuildDataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
        DataComputeResourcePolicyDiskConsistencyGroupPolicyEl { enabled: core::default::Default::default() }
    }
}

pub struct DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
        DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicyGroupPlacementPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeResourcePolicyGroupPlacementPolicyEl {
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

    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicyGroupPlacementPolicyEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicyGroupPlacementPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicyGroupPlacementPolicyEl {}

impl BuildDataComputeResourcePolicyGroupPlacementPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyGroupPlacementPolicyEl {
        DataComputeResourcePolicyGroupPlacementPolicyEl {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicyGroupPlacementPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyGroupPlacementPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicyGroupPlacementPolicyElRef {
        DataComputeResourcePolicyGroupPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicyGroupPlacementPolicyElRef {
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

    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {}

impl BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
            schedule: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {}

impl BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
            schedule: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_start_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_stop_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
}

impl DataComputeResourcePolicyInstanceSchedulePolicyEl {
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
                    ::Into<terrars::ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
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
                    ::Into<terrars::ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    ) -> Self {
        self.vm_stop_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyEl {}

impl BuildDataComputeResourcePolicyInstanceSchedulePolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyEl {
        DataComputeResourcePolicyInstanceSchedulePolicyEl {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicyInstanceSchedulePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicyInstanceSchedulePolicyElRef {
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
    ) -> terrars::ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_start_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_stop_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_source_disk_delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
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

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
            max_retention_days: core::default::Default::default(),
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
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
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_in_cycle: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
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

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
            days_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
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
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_in_cycle: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
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

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
            hours_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
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
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
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

impl terrars
::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    type O =
        terrars
        ::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
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
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_weeks: std
    ::option
    ::Option<
        terrars::SetField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>,
    >,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
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
                            DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
                        >,
                    >,
    ) -> Self {
        self.day_of_weeks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
            day_of_weeks: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_weeks` after provisioning.\n"]
    pub fn day_of_weeks(
        &self,
    ) -> terrars::SetRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.day_of_weeks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
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
                        terrars
                        ::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>,
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
                        terrars
                        ::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>,
                    >,
    ) -> Self {
        self.weekly_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_flush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
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

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
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
pub struct DataComputeResourcePolicySnapshotSchedulePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_properties: std
    ::option
    ::Option<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
}

impl DataComputeResourcePolicySnapshotSchedulePolicyEl {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    ) -> Self {
        self.retention_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
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
                    ::Into<terrars::ListField<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
    ) -> Self {
        self.snapshot_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyEl {
    type O = terrars::BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyEl {}

impl BuildDataComputeResourcePolicySnapshotSchedulePolicyEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyEl {
        DataComputeResourcePolicySnapshotSchedulePolicyEl {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
        }
    }
}

pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> terrars::ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_properties", self.base))
    }
}
