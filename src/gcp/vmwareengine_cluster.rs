use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VmwareengineClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_configs: Option<Vec<VmwareengineClusterNodeTypeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineClusterTimeoutsEl>,
    dynamic: VmwareengineClusterDynamic,
}

struct VmwareengineCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineClusterData>,
}

#[derive(Clone)]
pub struct VmwareengineCluster(Rc<VmwareengineCluster_>);

impl VmwareengineCluster {
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

    #[doc= "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(self, v: impl Into<BlockAssignable<VmwareengineClusterNodeTypeConfigsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_type_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_type_configs = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VmwareengineCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VmwareengineCluster { }

impl ToListMappable for VmwareengineCluster {
    type O = ListRef<VmwareengineClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VmwareengineCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVmwareengineCluster {
    pub tf_id: String,
    #[doc= "The ID of the Cluster."]
    pub name: PrimField<String>,
    #[doc= "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}

impl BuildVmwareengineCluster {
    pub fn build(self, stack: &mut Stack) -> VmwareengineCluster {
        let out = VmwareengineCluster(Rc::new(VmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                node_type_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VmwareengineClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareengineClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VmwareengineClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VmwareengineClusterNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    node_count: PrimField<f64>,
    node_type_id: PrimField<String>,
}

impl VmwareengineClusterNodeTypeConfigsEl {
    #[doc= "Set the field `custom_core_count`.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }
}

impl ToListMappable for VmwareengineClusterNodeTypeConfigsEl {
    type O = BlockAssignable<VmwareengineClusterNodeTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineClusterNodeTypeConfigsEl {
    #[doc= "The number of nodes of this type in the cluster."]
    pub node_count: PrimField<f64>,
    #[doc= ""]
    pub node_type_id: PrimField<String>,
}

impl BuildVmwareengineClusterNodeTypeConfigsEl {
    pub fn build(self) -> VmwareengineClusterNodeTypeConfigsEl {
        VmwareengineClusterNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: self.node_count,
            node_type_id: self.node_type_id,
        }
    }
}

pub struct VmwareengineClusterNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareengineClusterNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterNodeTypeConfigsElRef {
        VmwareengineClusterNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineClusterNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_core_count` after provisioning.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes of this type in the cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VmwareengineClusterTimeoutsEl {
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

impl ToListMappable for VmwareengineClusterTimeoutsEl {
    type O = BlockAssignable<VmwareengineClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineClusterTimeoutsEl {}

impl BuildVmwareengineClusterTimeoutsEl {
    pub fn build(self) -> VmwareengineClusterTimeoutsEl {
        VmwareengineClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VmwareengineClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VmwareengineClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineClusterTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct VmwareengineClusterDynamic {
    node_type_configs: Option<DynamicBlock<VmwareengineClusterNodeTypeConfigsEl>>,
}
