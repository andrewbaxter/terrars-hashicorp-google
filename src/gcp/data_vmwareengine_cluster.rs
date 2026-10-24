use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareengineClusterData {
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
    parent: terrars::PrimField<std::string::String>,
}

struct DataVmwareengineCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareengineClusterData>,
}

#[derive(Clone)]
pub struct DataVmwareengineCluster(std::rc::Rc<DataVmwareengineCluster_>);

impl DataVmwareengineCluster {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> terrars::SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

impl terrars::Referable for DataVmwareengineCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareengineCluster { }

impl terrars::ToListMappable for DataVmwareengineCluster {
    type O = terrars::ListRef<DataVmwareengineClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareengineCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareengineCluster {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Cluster."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareengineCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareengineCluster {
        let out = DataVmwareengineCluster(std::rc::Rc::new(DataVmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareengineClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareengineClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> terrars::SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineClusterNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineClusterNodeTypeConfigsEl {
    #[doc = "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineClusterNodeTypeConfigsEl {
    type O = terrars::BlockAssignable<DataVmwareengineClusterNodeTypeConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineClusterNodeTypeConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareengineClusterNodeTypeConfigsElRef {
        DataVmwareengineClusterNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineClusterNodeTypeConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}
