use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNetworkPeeringRoutesConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    export_custom_routes: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    import_custom_routes: PrimField<bool>,
    network: PrimField<String>,
    peering: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeNetworkPeeringRoutesConfigTimeoutsEl>,
}

struct ComputeNetworkPeeringRoutesConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeNetworkPeeringRoutesConfigData>,
}

#[derive(Clone)]
pub struct ComputeNetworkPeeringRoutesConfig(Rc<ComputeNetworkPeeringRoutesConfig_>);

impl ComputeNetworkPeeringRoutesConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeNetworkPeeringRoutesConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `export_custom_routes` after provisioning.\nWhether to export the custom routes to the peer network."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_custom_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_custom_routes` after provisioning.\nWhether to import the custom routes to the peer network."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_custom_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name of the primary network for the peering."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peering` after provisioning.\nName of the peering."]
    pub fn peering(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
        ComputeNetworkPeeringRoutesConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeNetworkPeeringRoutesConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeNetworkPeeringRoutesConfig { }

impl ToListMappable for ComputeNetworkPeeringRoutesConfig {
    type O = ListRef<ComputeNetworkPeeringRoutesConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeNetworkPeeringRoutesConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_network_peering_routes_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNetworkPeeringRoutesConfig {
    pub tf_id: String,
    #[doc= "Whether to export the custom routes to the peer network."]
    pub export_custom_routes: PrimField<bool>,
    #[doc= "Whether to import the custom routes to the peer network."]
    pub import_custom_routes: PrimField<bool>,
    #[doc= "The name of the primary network for the peering."]
    pub network: PrimField<String>,
    #[doc= "Name of the peering."]
    pub peering: PrimField<String>,
}

impl BuildComputeNetworkPeeringRoutesConfig {
    pub fn build(self, stack: &mut Stack) -> ComputeNetworkPeeringRoutesConfig {
        let out = ComputeNetworkPeeringRoutesConfig(Rc::new(ComputeNetworkPeeringRoutesConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeNetworkPeeringRoutesConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                export_custom_routes: self.export_custom_routes,
                id: core::default::Default::default(),
                import_custom_routes: self.import_custom_routes,
                network: self.network,
                peering: self.peering,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNetworkPeeringRoutesConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeNetworkPeeringRoutesConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNetworkPeeringRoutesConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `export_custom_routes` after provisioning.\nWhether to export the custom routes to the peer network."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_custom_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_custom_routes` after provisioning.\nWhether to import the custom routes to the peer network."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_custom_routes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name of the primary network for the peering."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peering` after provisioning.\nName of the peering."]
    pub fn peering(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
        ComputeNetworkPeeringRoutesConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkPeeringRoutesConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeNetworkPeeringRoutesConfigTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeNetworkPeeringRoutesConfigTimeoutsEl {
    type O = BlockAssignable<ComputeNetworkPeeringRoutesConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkPeeringRoutesConfigTimeoutsEl {}

impl BuildComputeNetworkPeeringRoutesConfigTimeoutsEl {
    pub fn build(self) -> ComputeNetworkPeeringRoutesConfigTimeoutsEl {
        ComputeNetworkPeeringRoutesConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
        ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkPeeringRoutesConfigTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
