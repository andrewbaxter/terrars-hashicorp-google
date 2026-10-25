#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataNetworkManagementConnectivityTestsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataNetworkManagementConnectivityTests_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkManagementConnectivityTestsData_>,
}

#[derive(Clone)]
pub struct DataNetworkManagementConnectivityTests(std::rc::Rc<DataNetworkManagementConnectivityTests_>);

impl DataNetworkManagementConnectivityTests {
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

    #[doc =
        "Set the field `filter`.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `connectivity_tests` after provisioning.\n"]
    pub fn connectivity_tests(&self) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_tests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkManagementConnectivityTests {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkManagementConnectivityTests { }

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests {
    type O = terrars::ListRef<DataNetworkManagementConnectivityTestsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkManagementConnectivityTests_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_network_management_connectivity_tests".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkManagementConnectivityTests {
    pub tf_id: std::string::String,
}

impl BuildDataNetworkManagementConnectivityTests {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkManagementConnectivityTests {
        let out = DataNetworkManagementConnectivityTests(std::rc::Rc::new(DataNetworkManagementConnectivityTests_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkManagementConnectivityTestsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkManagementConnectivityTestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkManagementConnectivityTestsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `connectivity_tests` after provisioning.\n"]
    pub fn connectivity_tests(&self) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_tests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nLists the ConnectivityTests that match the filter expression. A filter expression filters the resources listed in the response."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_master_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_pod: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fqdn`.\n"]
    pub fn set_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_master_cluster`.\n"]
    pub fn set_gke_master_cluster(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gke_master_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_pod`.\n"]
    pub fn set_gke_pod(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gke_pod = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redis_cluster`.\n"]
    pub fn set_redis_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redis_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redis_instance`.\n"]
    pub fn set_redis_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redis_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Destination {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Destination {
            cloud_sql_instance: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            fqdn: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            gke_pod: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            redis_cluster: core::default::Default::default(),
            redis_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef {
        DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\n"]
    pub fn gke_master_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_master_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_pod` after provisioning.\n"]
    pub fn gke_pod(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_pod", self.base))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_cluster` after provisioning.\n"]
    pub fn redis_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_instance` after provisioning.\n"]
    pub fn redis_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
    type O =
        terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
    type O =
        terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision {
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_version: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_function: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run_revision: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_master_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_Source {
    #[doc = "Set the field `app_engine_version`.\n"]
    pub fn set_app_engine_version(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersion>,
                    >,
    ) -> Self {
        self.app_engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunction>,
                    >,
    ) -> Self {
        self.cloud_function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_run_revision`.\n"]
    pub fn set_cloud_run_revision(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevision>,
                    >,
    ) -> Self {
        self.cloud_run_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_sql_instance`.\n"]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_master_cluster`.\n"]
    pub fn set_gke_master_cluster(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gke_master_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests_Source {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests_Source {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests_Source {
        DataNetworkManagementConnectivityTests_ConnectivityTests_Source {
            app_engine_version: core::default::Default::default(),
            cloud_function: core::default::Default::default(),
            cloud_run_revision: core::default::Default::default(),
            cloud_sql_instance: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef {
        DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_engine_version` after provisioning.\n"]
    pub fn app_engine_version(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_AppEngineVersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_run_revision` after provisioning.\n"]
    pub fn cloud_run_revision(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTests_Source_CloudRunRevisionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run_revision", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\n"]
    pub fn cloud_sql_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\n"]
    pub fn gke_master_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_master_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_firewall_checks: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    round_trip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terraform_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkManagementConnectivityTests_ConnectivityTests {
    #[doc = "Set the field `bypass_firewall_checks`.\n"]
    pub fn set_bypass_firewall_checks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bypass_firewall_checks = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Destination>>,
    ) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.effective_labels = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `related_projects`.\n"]
    pub fn set_related_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.related_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `round_trip`.\n"]
    pub fn set_round_trip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.round_trip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkManagementConnectivityTests_ConnectivityTests_Source>>,
    ) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.terraform_labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkManagementConnectivityTests_ConnectivityTests {
    type O = terrars::BlockAssignable<DataNetworkManagementConnectivityTests_ConnectivityTests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkManagementConnectivityTests_ConnectivityTests {}

impl BuildDataNetworkManagementConnectivityTests_ConnectivityTests {
    pub fn build(self) -> DataNetworkManagementConnectivityTests_ConnectivityTests {
        DataNetworkManagementConnectivityTests_ConnectivityTests {
            bypass_firewall_checks: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            destination: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            protocol: core::default::Default::default(),
            related_projects: core::default::Default::default(),
            round_trip: core::default::Default::default(),
            source: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkManagementConnectivityTests_ConnectivityTestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkManagementConnectivityTests_ConnectivityTestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkManagementConnectivityTests_ConnectivityTestsRef {
        DataNetworkManagementConnectivityTests_ConnectivityTestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkManagementConnectivityTests_ConnectivityTestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\n"]
    pub fn bypass_firewall_checks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_firewall_checks", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTests_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `related_projects` after provisioning.\n"]
    pub fn related_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_projects", self.base))
    }

    #[doc = "Get a reference to the value of field `round_trip` after provisioning.\n"]
    pub fn round_trip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.round_trip", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataNetworkManagementConnectivityTests_ConnectivityTests_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }
}
