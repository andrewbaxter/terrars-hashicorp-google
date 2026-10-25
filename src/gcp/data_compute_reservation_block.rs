#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeReservationBlockData_ {
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
    reservation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeReservationBlock_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeReservationBlockData_>,
}

#[derive(Clone)]
pub struct DataComputeReservationBlock(std::rc::Rc<DataComputeReservationBlock_>);

impl DataComputeReservationBlock {
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

    #[doc = "Set the field `project`.\nThe project in which the resource belongs."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe zone where the reservation block resides."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `block_count` after provisioning.\nThe number of resources that are allocated in this reservation block."]
    pub fn block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\nHealth information for the reservation block."]
    pub fn health_info(&self) -> terrars::ListRef<DataComputeReservationBlock_HealthInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `in_use_count` after provisioning.\nThe number of instances that are currently in use on this reservation block."]
    pub fn in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservationBlock for reservation blocks."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation block."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `physical_topology` after provisioning.\nThe physical topology of the reservation block."]
    pub fn physical_topology(&self) -> terrars::ListRef<DataComputeReservationBlock_PhysicalTopologyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.physical_topology", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation` after provisioning.\nThe name of the parent reservation."]
    pub fn reservation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_maintenance` after provisioning.\nMaintenance information for this reservation block."]
    pub fn reservation_maintenance(&self) -> terrars::ListRef<DataComputeReservationBlock_ReservationMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_maintenance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sub_block_count` after provisioning.\nThe number of reservation subBlocks associated with this reservation block."]
    pub fn reservation_sub_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_sub_block_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sub_block_in_use_count` after provisioning.\nThe number of in-use reservation subBlocks associated with this reservation block."]
    pub fn reservation_sub_block_in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_sub_block_in_use_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `resource_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the reservation block."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sub_block_names` after provisioning.\nList of all block sub-block names in the parent block."]
    pub fn sub_block_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_block_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation block resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeReservationBlock {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeReservationBlock { }

impl terrars::ToListMappable for DataComputeReservationBlock {
    type O = terrars::ListRef<DataComputeReservationBlockRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeReservationBlock_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_reservation_block".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeReservationBlock {
    pub tf_id: std::string::String,
    #[doc = "The name of the reservation block."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the parent reservation."]
    pub reservation: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeReservationBlock {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeReservationBlock {
        let out = DataComputeReservationBlock(std::rc::Rc::new(DataComputeReservationBlock_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeReservationBlockData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                reservation: self.reservation,
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeReservationBlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservationBlockRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeReservationBlockRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `block_count` after provisioning.\nThe number of resources that are allocated in this reservation block."]
    pub fn block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\nHealth information for the reservation block."]
    pub fn health_info(&self) -> terrars::ListRef<DataComputeReservationBlock_HealthInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `in_use_count` after provisioning.\nThe number of instances that are currently in use on this reservation block."]
    pub fn in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservationBlock for reservation blocks."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation block."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `physical_topology` after provisioning.\nThe physical topology of the reservation block."]
    pub fn physical_topology(&self) -> terrars::ListRef<DataComputeReservationBlock_PhysicalTopologyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.physical_topology", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation` after provisioning.\nThe name of the parent reservation."]
    pub fn reservation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_maintenance` after provisioning.\nMaintenance information for this reservation block."]
    pub fn reservation_maintenance(&self) -> terrars::ListRef<DataComputeReservationBlock_ReservationMaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_maintenance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sub_block_count` after provisioning.\nThe number of reservation subBlocks associated with this reservation block."]
    pub fn reservation_sub_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_sub_block_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_sub_block_in_use_count` after provisioning.\nThe number of in-use reservation subBlocks associated with this reservation block."]
    pub fn reservation_sub_block_in_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_sub_block_in_use_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `resource_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the reservation block."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sub_block_names` after provisioning.\nList of all block sub-block names in the parent block."]
    pub fn sub_block_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_block_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation block resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservationBlock_HealthInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    degraded_sub_block_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_sub_block_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeReservationBlock_HealthInfo {
    #[doc = "Set the field `degraded_sub_block_count`.\n"]
    pub fn set_degraded_sub_block_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.degraded_sub_block_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_status`.\n"]
    pub fn set_health_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `healthy_sub_block_count`.\n"]
    pub fn set_healthy_sub_block_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_sub_block_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservationBlock_HealthInfo {
    type O = terrars::BlockAssignable<DataComputeReservationBlock_HealthInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservationBlock_HealthInfo {}

impl BuildDataComputeReservationBlock_HealthInfo {
    pub fn build(self) -> DataComputeReservationBlock_HealthInfo {
        DataComputeReservationBlock_HealthInfo {
            degraded_sub_block_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_sub_block_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservationBlock_HealthInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservationBlock_HealthInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeReservationBlock_HealthInfoRef {
        DataComputeReservationBlock_HealthInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservationBlock_HealthInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `degraded_sub_block_count` after provisioning.\n"]
    pub fn degraded_sub_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.degraded_sub_block_count", self.base))
    }

    #[doc = "Get a reference to the value of field `health_status` after provisioning.\n"]
    pub fn health_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_status", self.base))
    }

    #[doc = "Get a reference to the value of field `healthy_sub_block_count` after provisioning.\n"]
    pub fn healthy_sub_block_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_sub_block_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservationBlock_PhysicalTopology {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeReservationBlock_PhysicalTopology {
    #[doc = "Set the field `block`.\n"]
    pub fn set_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservationBlock_PhysicalTopology {
    type O = terrars::BlockAssignable<DataComputeReservationBlock_PhysicalTopology>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservationBlock_PhysicalTopology {}

impl BuildDataComputeReservationBlock_PhysicalTopology {
    pub fn build(self) -> DataComputeReservationBlock_PhysicalTopology {
        DataComputeReservationBlock_PhysicalTopology {
            block: core::default::Default::default(),
            cluster: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservationBlock_PhysicalTopologyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservationBlock_PhysicalTopologyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservationBlock_PhysicalTopologyRef {
        DataComputeReservationBlock_PhysicalTopologyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservationBlock_PhysicalTopologyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeReservationBlock_ReservationMaintenance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subblock_infra_maintenance_ongoing_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subblock_infra_maintenance_pending_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeReservationBlock_ReservationMaintenance {
    #[doc = "Set the field `instance_maintenance_ongoing_count`.\n"]
    pub fn set_instance_maintenance_ongoing_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.instance_maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_maintenance_pending_count`.\n"]
    pub fn set_instance_maintenance_pending_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.instance_maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_ongoing_count`.\n"]
    pub fn set_maintenance_ongoing_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_pending_count`.\n"]
    pub fn set_maintenance_pending_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_type`.\n"]
    pub fn set_scheduling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheduling_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subblock_infra_maintenance_ongoing_count`.\n"]
    pub fn set_subblock_infra_maintenance_ongoing_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_ongoing_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subblock_infra_maintenance_pending_count`.\n"]
    pub fn set_subblock_infra_maintenance_pending_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_pending_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeReservationBlock_ReservationMaintenance {
    type O = terrars::BlockAssignable<DataComputeReservationBlock_ReservationMaintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeReservationBlock_ReservationMaintenance {}

impl BuildDataComputeReservationBlock_ReservationMaintenance {
    pub fn build(self) -> DataComputeReservationBlock_ReservationMaintenance {
        DataComputeReservationBlock_ReservationMaintenance {
            instance_maintenance_ongoing_count: core::default::Default::default(),
            instance_maintenance_pending_count: core::default::Default::default(),
            maintenance_ongoing_count: core::default::Default::default(),
            maintenance_pending_count: core::default::Default::default(),
            scheduling_type: core::default::Default::default(),
            subblock_infra_maintenance_ongoing_count: core::default::Default::default(),
            subblock_infra_maintenance_pending_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeReservationBlock_ReservationMaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeReservationBlock_ReservationMaintenanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeReservationBlock_ReservationMaintenanceRef {
        DataComputeReservationBlock_ReservationMaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeReservationBlock_ReservationMaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_ongoing_count` after provisioning.\n"]
    pub fn instance_maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_maintenance_ongoing_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_pending_count` after provisioning.\n"]
    pub fn instance_maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_maintenance_pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_ongoing_count` after provisioning.\n"]
    pub fn maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_ongoing_count", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_pending_count` after provisioning.\n"]
    pub fn maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_type` after provisioning.\n"]
    pub fn scheduling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_ongoing_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_ongoing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_ongoing_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_pending_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_pending_count", self.base),
        )
    }
}
