use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRouterStatusData {
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
    data: std::cell::RefCell<DataComputeRouterStatusData>,
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
    pub fn best_routes(&self) -> terrars::ListRef<DataComputeRouterStatusBestRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> terrars::ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
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
            data: std::cell::RefCell::new(DataComputeRouterStatusData {
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
    pub fn best_routes(&self) -> terrars::ListRef<DataComputeRouterStatusBestRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> terrars::ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
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
pub struct DataComputeRouterStatusBestRoutesEl {
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
    next_hop_ilb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_vpn_tunnel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRouterStatusBestRoutesEl {
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

    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_network = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataComputeRouterStatusBestRoutesEl {
    type O = terrars::BlockAssignable<DataComputeRouterStatusBestRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatusBestRoutesEl {}

impl BuildDataComputeRouterStatusBestRoutesEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesEl {
        DataComputeRouterStatusBestRoutesEl {
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataComputeRouterStatusBestRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatusBestRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRouterStatusBestRoutesElRef {
        DataComputeRouterStatusBestRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatusBestRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterEl {
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
    next_hop_ilb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_instance_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_hop_vpn_tunnel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRouterStatusBestRoutesForRouterEl {
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

    #[doc = "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.next_hop_network = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataComputeRouterStatusBestRoutesForRouterEl {
    type O = terrars::BlockAssignable<DataComputeRouterStatusBestRoutesForRouterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRouterStatusBestRoutesForRouterEl {}

impl BuildDataComputeRouterStatusBestRoutesForRouterEl {
    pub fn build(self) -> DataComputeRouterStatusBestRoutesForRouterEl {
        DataComputeRouterStatusBestRoutesForRouterEl {
            description: core::default::Default::default(),
            dest_range: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            next_hop_gateway: core::default::Default::default(),
            next_hop_ilb: core::default::Default::default(),
            next_hop_instance: core::default::Default::default(),
            next_hop_instance_zone: core::default::Default::default(),
            next_hop_ip: core::default::Default::default(),
            next_hop_network: core::default::Default::default(),
            next_hop_vpn_tunnel: core::default::Default::default(),
            priority: core::default::Default::default(),
            project: core::default::Default::default(),
            self_link: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataComputeRouterStatusBestRoutesForRouterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRouterStatusBestRoutesForRouterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRouterStatusBestRoutesForRouterElRef {
        DataComputeRouterStatusBestRoutesForRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatusBestRoutesForRouterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
