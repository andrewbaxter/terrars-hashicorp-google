use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareengineClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
}

struct DataVmwareengineCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineClusterData>,
}

#[derive(Clone)]
pub struct DataVmwareengineCluster(Rc<DataVmwareengineCluster_>);

impl DataVmwareengineCluster {
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

    #[doc= "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
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
}

impl Referable for DataVmwareengineCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVmwareengineCluster { }

impl ToListMappable for DataVmwareengineCluster {
    type O = ListRef<DataVmwareengineClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVmwareengineCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareengineCluster {
    pub tf_id: String,
    #[doc= "The ID of the Cluster."]
    pub name: PrimField<String>,
    #[doc= "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}

impl BuildDataVmwareengineCluster {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineCluster {
        let out = DataVmwareengineCluster(Rc::new(DataVmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareengineClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareengineClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareengineClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc= "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataVmwareengineClusterNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_id: Option<PrimField<String>>,
}

impl DataVmwareengineClusterNodeTypeConfigsEl {
    #[doc= "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataVmwareengineClusterNodeTypeConfigsEl {
    type O = BlockAssignable<DataVmwareengineClusterNodeTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineClusterNodeTypeConfigsEl {}

impl BuildDataVmwareengineClusterNodeTypeConfigsEl {
    pub fn build(self) -> DataVmwareengineClusterNodeTypeConfigsEl {
        DataVmwareengineClusterNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
        }
    }
}

pub struct DataVmwareengineClusterNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVmwareengineClusterNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineClusterNodeTypeConfigsElRef {
        DataVmwareengineClusterNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineClusterNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}
