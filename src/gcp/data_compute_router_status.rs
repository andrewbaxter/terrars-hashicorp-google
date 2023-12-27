use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRouterStatusData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataComputeRouterStatus_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRouterStatusData>,
}

#[derive(Clone)]
pub struct DataComputeRouterStatus(Rc<DataComputeRouterStatus_>);

impl DataComputeRouterStatus {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nProject ID of the target router."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nRegion of the target router."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> ListRef<DataComputeRouterStatusBestRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.best_routes_for_router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataComputeRouterStatus {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataComputeRouterStatus { }

impl ToListMappable for DataComputeRouterStatus {
    type O = ListRef<DataComputeRouterStatusRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataComputeRouterStatus_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_router_status".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeRouterStatus {
    pub tf_id: String,
    #[doc= "Name of the router to query."]
    pub name: PrimField<String>,
}

impl BuildDataComputeRouterStatus {
    pub fn build(self, stack: &mut Stack) -> DataComputeRouterStatus {
        let out = DataComputeRouterStatus(Rc::new(DataComputeRouterStatus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRouterStatusData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeRouterStatusRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeRouterStatusRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `best_routes` after provisioning.\nBest routes for this router's network."]
    pub fn best_routes(&self) -> ListRef<DataComputeRouterStatusBestRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.best_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `best_routes_for_router` after provisioning.\nBest routes learned by this router."]
    pub fn best_routes_for_router(&self) -> ListRef<DataComputeRouterStatusBestRoutesForRouterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.best_routes_for_router", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the router to query."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nURI of the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nProject ID of the target router."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion of the target router."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
}

impl DataComputeRouterStatusBestRoutesEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest_range = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ilb = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_network = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_vpn_tunnel = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeRouterStatusBestRoutesEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeRouterStatusBestRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesElRef {
        DataComputeRouterStatusBestRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatusBestRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRouterStatusBestRoutesForRouterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_instance_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_vpn_tunnel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
}

impl DataComputeRouterStatusBestRoutesForRouterEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_range`.\n"]
    pub fn set_dest_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest_range = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_gateway`.\n"]
    pub fn set_next_hop_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ilb`.\n"]
    pub fn set_next_hop_ilb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ilb = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance`.\n"]
    pub fn set_next_hop_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_instance_zone`.\n"]
    pub fn set_next_hop_instance_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_instance_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_ip`.\n"]
    pub fn set_next_hop_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_network`.\n"]
    pub fn set_next_hop_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_network = Some(v.into());
        self
    }

    #[doc= "Set the field `next_hop_vpn_tunnel`.\n"]
    pub fn set_next_hop_vpn_tunnel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_hop_vpn_tunnel = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeRouterStatusBestRoutesForRouterEl {
    type O = BlockAssignable<DataComputeRouterStatusBestRoutesForRouterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeRouterStatusBestRoutesForRouterElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterStatusBestRoutesForRouterElRef {
        DataComputeRouterStatusBestRoutesForRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRouterStatusBestRoutesForRouterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_range` after provisioning.\n"]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_gateway` after provisioning.\n"]
    pub fn next_hop_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_ilb` after provisioning.\n"]
    pub fn next_hop_ilb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ilb", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance` after provisioning.\n"]
    pub fn next_hop_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_instance_zone` after provisioning.\n"]
    pub fn next_hop_instance_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_instance_zone", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_ip` after provisioning.\n"]
    pub fn next_hop_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_network` after provisioning.\n"]
    pub fn next_hop_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_network", self.base))
    }

    #[doc= "Get a reference to the value of field `next_hop_vpn_tunnel` after provisioning.\n"]
    pub fn next_hop_vpn_tunnel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_hop_vpn_tunnel", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
