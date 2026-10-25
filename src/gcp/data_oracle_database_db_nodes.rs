#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseDbNodesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_vm_cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseDbNodes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseDbNodesData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseDbNodes(std::rc::Rc<DataOracleDatabaseDbNodes_>);

impl DataOracleDatabaseDbNodes {
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

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloud_vm_cluster` after provisioning.\nvmcluster"]
    pub fn cloud_vm_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_nodes` after provisioning.\n"]
    pub fn db_nodes(&self) -> terrars::ListRef<DataOracleDatabaseDbNodes_DbNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseDbNodes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseDbNodes { }

impl terrars::ToListMappable for DataOracleDatabaseDbNodes {
    type O = terrars::ListRef<DataOracleDatabaseDbNodesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseDbNodes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_db_nodes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseDbNodes {
    pub tf_id: std::string::String,
    #[doc = "vmcluster"]
    pub cloud_vm_cluster: terrars::PrimField<std::string::String>,
    #[doc = "location"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseDbNodes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseDbNodes {
        let out = DataOracleDatabaseDbNodes(std::rc::Rc::new(DataOracleDatabaseDbNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOracleDatabaseDbNodesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cloud_vm_cluster: self.cloud_vm_cluster,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseDbNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseDbNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseDbNodesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloud_vm_cluster` after provisioning.\nvmcluster"]
    pub fn cloud_vm_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_nodes` after provisioning.\n"]
    pub fn db_nodes(&self) -> terrars::ListRef<DataOracleDatabaseDbNodes_DbNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseDbNodes_DbNodes_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOracleDatabaseDbNodes_DbNodes_Properties {
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_ocid`.\n"]
    pub fn set_db_server_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_server_ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocpu_count`.\n"]
    pub fn set_ocpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ocpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_cpu_core_count`.\n"]
    pub fn set_total_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_cpu_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseDbNodes_DbNodes_Properties {
    type O = terrars::BlockAssignable<DataOracleDatabaseDbNodes_DbNodes_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseDbNodes_DbNodes_Properties {}

impl BuildDataOracleDatabaseDbNodes_DbNodes_Properties {
    pub fn build(self) -> DataOracleDatabaseDbNodes_DbNodes_Properties {
        DataOracleDatabaseDbNodes_DbNodes_Properties {
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocid: core::default::Default::default(),
            hostname: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            state: core::default::Default::default(),
            total_cpu_core_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseDbNodes_DbNodes_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseDbNodes_DbNodes_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseDbNodes_DbNodes_PropertiesRef {
        DataOracleDatabaseDbNodes_DbNodes_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseDbNodes_DbNodes_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_ocid` after provisioning.\n"]
    pub fn db_server_ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\n"]
    pub fn ocpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `total_cpu_core_count` after provisioning.\n"]
    pub fn total_cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cpu_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseDbNodes_DbNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<DataOracleDatabaseDbNodes_DbNodes_Properties>>,
}

impl DataOracleDatabaseDbNodes_DbNodes {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOracleDatabaseDbNodes_DbNodes_Properties>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseDbNodes_DbNodes {
    type O = terrars::BlockAssignable<DataOracleDatabaseDbNodes_DbNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseDbNodes_DbNodes {}

impl BuildDataOracleDatabaseDbNodes_DbNodes {
    pub fn build(self) -> DataOracleDatabaseDbNodes_DbNodes {
        DataOracleDatabaseDbNodes_DbNodes {
            name: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseDbNodes_DbNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseDbNodes_DbNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOracleDatabaseDbNodes_DbNodesRef {
        DataOracleDatabaseDbNodes_DbNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseDbNodes_DbNodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseDbNodes_DbNodes_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
