#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRouterStatusData_ {
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

struct DataComputeRouterStatus_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeRouterStatusData_>,
}

#[derive(Clone)]
pub struct DataComputeRouterStatus(std::rc::Rc<DataComputeRouterStatus_>);

impl DataComputeRouterStatus {
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

    #[doc = "Set the field `project`.\nProject ID of the target router."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion of the target router."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutesForRouterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes_for_router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeRouterStatus {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeRouterStatus { }

impl terrars::ToListMappable for DataComputeRouterStatus {
    type O = terrars::ListRef<DataComputeRouterStatusRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeRouterStatus_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_router_status".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeRouterStatus {
    pub tf_id: std::string::String,
    #[doc = "Name of the router to query."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeRouterStatus {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeRouterStatus {
        let out = DataComputeRouterStatus(std::rc::Rc::new(DataComputeRouterStatus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeRouterStatusData_ {
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

pub struct DataComputeRouterStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeRouterStatusRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutesForRouterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes_for_router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutes_AsPaths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    as_lists: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_segment_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouterStatus_BestRoutes_AsPaths {
    #[doc = "Set the field `as_lists`.\n"]
    pub fn set_as_lists(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.as_lists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_segment_type`.\n"]
    pub fn set_path_segment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_segment_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutes_AsPaths {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutes_AsPaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutes_AsPaths {}

impl BuildDataComputeRouterStatus_BestRoutes_AsPaths {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutes_AsPaths {
        DataComputeRouterStatus_BestRoutes_AsPaths {
            as_lists: core::default::Default::default(),
            path_segment_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutes_AsPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutes_AsPathsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouterStatus_BestRoutes_AsPathsRef {
        DataComputeRouterStatus_BestRoutes_AsPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutes_AsPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `as_lists` after provisioning.\n"]
    pub fn as_lists(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `path_segment_type` after provisioning.\n"]
    pub fn path_segment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_segment_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutes_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRouterStatus_BestRoutes_Params {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutes_Params {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutes_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutes_Params {}

impl BuildDataComputeRouterStatus_BestRoutes_Params {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutes_Params {
        DataComputeRouterStatus_BestRoutes_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutes_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutes_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouterStatus_BestRoutes_ParamsRef {
        DataComputeRouterStatus_BestRoutes_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutes_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutes_Warnings_Data {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouterStatus_BestRoutes_Warnings_Data {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutes_Warnings_Data {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutes_Warnings_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutes_Warnings_Data {}

impl BuildDataComputeRouterStatus_BestRoutes_Warnings_Data {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutes_Warnings_Data {
        DataComputeRouterStatus_BestRoutes_Warnings_Data {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutes_Warnings_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutes_Warnings_DataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouterStatus_BestRoutes_Warnings_DataRef {
        DataComputeRouterStatus_BestRoutes_Warnings_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutes_Warnings_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutes_Warnings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings_Data>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRouterStatus_BestRoutes_Warnings {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings_Data>>,
    ) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutes_Warnings {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutes_Warnings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutes_Warnings {}

impl BuildDataComputeRouterStatus_BestRoutes_Warnings {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutes_Warnings {
        DataComputeRouterStatus_BestRoutes_Warnings {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutes_WarningsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutes_WarningsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouterStatus_BestRoutes_WarningsRef {
        DataComputeRouterStatus_BestRoutes_WarningsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutes_WarningsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_Warnings_DataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    as_paths: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_AsPaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_hub: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ilb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_inter_region_cost: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_med: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_peering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_vpn_tunnel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warnings: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings>>,
}

impl DataComputeRouterStatus_BestRoutes {
    #[doc = "Set the field `as_paths`.\n"]
    pub fn set_as_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_AsPaths>>,
    ) -> Self {
        self.as_paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.creation_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dest_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_hub`.\n"]
    pub fn set_next_hop_hub(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_hub = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ilb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_instance_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_inter_region_cost`.\n"]
    pub fn set_next_hop_inter_region_cost(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_inter_region_cost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_med`.\n"]
    pub fn set_next_hop_med(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_med = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_origin`.\n"]
    pub fn set_next_hop_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_peering`.\n"]
    pub fn set_next_hop_peering(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_peering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_vpn_tunnel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_Params>>,
    ) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_status`.\n"]
    pub fn set_route_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_type`.\n"]
    pub fn set_route_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warnings`.\n"]
    pub fn set_warnings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings>>,
    ) -> Self {
        self.warnings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutes {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutes {}

impl BuildDataComputeRouterStatus_BestRoutes {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutes {
        DataComputeRouterStatus_BestRoutes {
            as_paths: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_hub: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_inter_region_cost: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_med: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_origin: core::default::Default::default(),
            next_hop_peering: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            params: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            route_status: core::default::Default::default(),
            route_type: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
            warnings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouterStatus_BestRoutesRef {
        DataComputeRouterStatus_BestRoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_AsPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\n"]
    pub fn next_hop_hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_hub", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\n"]
    pub fn next_hop_inter_region_cost(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_inter_region_cost", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\n"]
    pub fn next_hop_med(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_med", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\n"]
    pub fn next_hop_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\n"]
    pub fn next_hop_peering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_peering", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `route_status` after provisioning.\n"]
    pub fn route_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_status", self.base))
    }

    #[doc = "Get a reference to the value of field `route_type` after provisioning.\n"]
    pub fn route_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_type", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `warnings` after provisioning.\n"]
    pub fn warnings(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_WarningsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatus_BestRoutesForRouter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    as_paths: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_AsPaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_hub: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ilb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_inter_region_cost: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_med: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_peering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_vpn_tunnel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warnings: std::option::Option<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings>>,
}

impl DataComputeRouterStatus_BestRoutesForRouter {
    #[doc = "Set the field `as_paths`.\n"]
    pub fn set_as_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_AsPaths>>,
    ) -> Self {
        self.as_paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.creation_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dest_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_hub`.\n"]
    pub fn set_next_hop_hub(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_hub = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ilb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_instance_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_inter_region_cost`.\n"]
    pub fn set_next_hop_inter_region_cost(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_inter_region_cost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_med`.\n"]
    pub fn set_next_hop_med(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_med = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_origin`.\n"]
    pub fn set_next_hop_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_peering`.\n"]
    pub fn set_next_hop_peering(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_peering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_hop_vpn_tunnel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_Params>>,
    ) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_status`.\n"]
    pub fn set_route_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_type`.\n"]
    pub fn set_route_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warnings`.\n"]
    pub fn set_warnings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRouterStatus_BestRoutes_Warnings>>,
    ) -> Self {
        self.warnings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRouterStatus_BestRoutesForRouter {
    type O = terrars::BlockAssignable<DataComputeRouterStatus_BestRoutesForRouter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatus_BestRoutesForRouter {}

impl BuildDataComputeRouterStatus_BestRoutesForRouter {
    pub fn build(self) -> DataComputeRouterStatus_BestRoutesForRouter {
        DataComputeRouterStatus_BestRoutesForRouter {
            as_paths: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_hub: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_inter_region_cost: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_med: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_origin: core::default::Default::default(),
            next_hop_peering: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            params: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            route_status: core::default::Default::default(),
            route_type: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
            warnings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRouterStatus_BestRoutesForRouterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatus_BestRoutesForRouterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouterStatus_BestRoutesForRouterRef {
        DataComputeRouterStatus_BestRoutesForRouterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatus_BestRoutesForRouterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `as_paths` after provisioning.\n"]
    pub fn as_paths(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_AsPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_hub` after provisioning.\n"]
    pub fn next_hop_hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_hub", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_inter_region_cost` after provisioning.\n"]
    pub fn next_hop_inter_region_cost(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_inter_region_cost", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_med` after provisioning.\n"]
    pub fn next_hop_med(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_med", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_origin` after provisioning.\n"]
    pub fn next_hop_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_peering` after provisioning.\n"]
    pub fn next_hop_peering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_peering", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `route_status` after provisioning.\n"]
    pub fn route_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_status", self.base))
    }

    #[doc = "Get a reference to the value of field `route_type` after provisioning.\n"]
    pub fn route_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_type", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `warnings` after provisioning.\n"]
    pub fn warnings(&self) -> terrars::ListRef<DataComputeRouterStatus_BestRoutes_WarningsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}
