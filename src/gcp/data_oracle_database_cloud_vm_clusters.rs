#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseCloudVmClustersData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseCloudVmClusters_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseCloudVmClustersData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseCloudVmClusters(std::rc::Rc<DataOracleDatabaseCloudVmClusters_>);

impl DataOracleDatabaseCloudVmClusters {
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

    #[doc = "Get a reference to the value of field `cloud_vm_clusters` after provisioning.\n"]
    pub fn cloud_vm_clusters(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_vm_clusters", self.extract_ref()))
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

impl terrars::Referable for DataOracleDatabaseCloudVmClusters {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseCloudVmClusters { }

impl terrars::ToListMappable for DataOracleDatabaseCloudVmClusters {
    type O = terrars::ListRef<DataOracleDatabaseCloudVmClustersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseCloudVmClusters_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_cloud_vm_clusters".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters {
    pub tf_id: std::string::String,
    #[doc = "location"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseCloudVmClusters {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseCloudVmClusters {
        let out = DataOracleDatabaseCloudVmClusters(std::rc::Rc::new(DataOracleDatabaseCloudVmClusters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOracleDatabaseCloudVmClustersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseCloudVmClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClustersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseCloudVmClustersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloud_vm_clusters` after provisioning.\n"]
    pub fn cloud_vm_clusters(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_vm_clusters", self.extract_ref()))
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
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
    #[doc = "Set the field `connection_state`.\n"]
    pub fn set_connection_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_agent_email`.\n"]
    pub fn set_service_agent_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_agent_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {}

impl BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector {
            connection_state: core::default::Default::default(),
            service_agent_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_state` after provisioning.\n"]
    pub fn connection_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_state", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_email` after provisioning.\n"]
    pub fn service_agent_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent_email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_events_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_monitoring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
    #[doc = "Set the field `diagnostics_events_enabled`.\n"]
    pub fn set_diagnostics_events_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.diagnostics_events_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_monitoring_enabled`.\n"]
    pub fn set_health_monitoring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.health_monitoring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incident_logs_enabled`.\n"]
    pub fn set_incident_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.incident_logs_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
    type O =
        terrars
        ::BlockAssignable<
            DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {}

impl BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions {
            diagnostics_events_enabled: core::default::Default::default(),
            health_monitoring_enabled: core::default::Default::default(),
            incident_logs_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `diagnostics_events_enabled` after provisioning.\n"]
    pub fn diagnostics_events_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostics_events_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `health_monitoring_enabled` after provisioning.\n"]
    pub fn health_monitoring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_monitoring_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `incident_logs_enabled` after provisioning.\n"]
    pub fn incident_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {}

impl BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compartment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_ocids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_data_collection_options: std
    ::option
    ::Option<
        terrars
        ::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_redundancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_listener_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gi_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_backup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_dns: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_dns_record_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_ip_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp_ssl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse_diskgroup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_public_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_management_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_backup_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_file_system_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
    #[doc = "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compartment_id`.\n"]
    pub fn set_compartment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compartment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_storage_size_tb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_ocids`.\n"]
    pub fn set_db_server_ocids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.db_server_ocids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `diagnostics_data_collection_options`.\n"]
    pub fn set_diagnostics_data_collection_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptions,
                        >,
                    >,
    ) -> Self {
        self.diagnostics_data_collection_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_redundancy`.\n"]
    pub fn set_disk_redundancy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_redundancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_listener_ip`.\n"]
    pub fn set_dns_listener_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_listener_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gi_version`.\n"]
    pub fn set_gi_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gi_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_prefix`.\n"]
    pub fn set_hostname_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license_type`.\n"]
    pub fn set_license_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_backup_enabled`.\n"]
    pub fn set_local_backup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.local_backup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oci_url = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `scan_dns`.\n"]
    pub fn set_scan_dns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scan_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_dns_record_id`.\n"]
    pub fn set_scan_dns_record_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scan_dns_record_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_ip_ids`.\n"]
    pub fn set_scan_ip_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scan_ip_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp`.\n"]
    pub fn set_scan_listener_port_tcp(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp_ssl`.\n"]
    pub fn set_scan_listener_port_tcp_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sparse_diskgroup_enabled`.\n"]
    pub fn set_sparse_diskgroup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sparse_diskgroup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_public_keys`.\n"]
    pub fn set_ssh_public_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssh_public_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_management_type`.\n"]
    pub fn set_storage_management_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_management_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_size_gb`.\n"]
    pub fn set_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_version`.\n"]
    pub fn set_system_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZone>>,
    ) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_backup_storage_type`.\n"]
    pub fn set_vm_backup_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_backup_storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_file_system_storage_type`.\n"]
    pub fn set_vm_file_system_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_file_system_storage_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {}

impl BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties {
            cluster_name: core::default::Default::default(),
            compartment_id: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocids: core::default::Default::default(),
            diagnostics_data_collection_options: core::default::Default::default(),
            disk_redundancy: core::default::Default::default(),
            dns_listener_ip: core::default::Default::default(),
            domain: core::default::Default::default(),
            gi_version: core::default::Default::default(),
            hostname: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            license_type: core::default::Default::default(),
            local_backup_enabled: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            scan_dns: core::default::Default::default(),
            scan_dns_record_id: core::default::Default::default(),
            scan_ip_ids: core::default::Default::default(),
            scan_listener_port_tcp: core::default::Default::default(),
            scan_listener_port_tcp_ssl: core::default::Default::default(),
            shape: core::default::Default::default(),
            sparse_diskgroup_enabled: core::default::Default::default(),
            ssh_public_keys: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_management_type: core::default::Default::default(),
            storage_size_gb: core::default::Default::default(),
            system_version: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_backup_storage_type: core::default::Default::default(),
            vm_file_system_storage_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\n"]
    pub fn compartment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compartment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_ocids` after provisioning.\n"]
    pub fn db_server_ocids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_ocids", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostics_data_collection_options` after provisioning.\n"]
    pub fn diagnostics_data_collection_options(
        &self,
    ) -> terrars
    ::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_DiagnosticsDataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.diagnostics_data_collection_options", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\n"]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_listener_ip` after provisioning.\n"]
    pub fn dns_listener_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_listener_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\n"]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\n"]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `license_type` after provisioning.\n"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc = "Get a reference to the value of field `local_backup_enabled` after provisioning.\n"]
    pub fn local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_backup_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\n"]
    pub fn ocpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_dns` after provisioning.\n"]
    pub fn scan_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\n"]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\n"]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\n"]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp_ssl` after provisioning.\n"]
    pub fn scan_listener_port_tcp_ssl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `sparse_diskgroup_enabled` after provisioning.\n"]
    pub fn sparse_diskgroup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sparse_diskgroup_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\n"]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_management_type` after provisioning.\n"]
    pub fn storage_management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_management_type", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_size_gb` after provisioning.\n"]
    pub fn storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `system_version` after provisioning.\n"]
    pub fn system_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties_TimeZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_backup_storage_type` after provisioning.\n"]
    pub fn vm_backup_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_backup_storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_file_system_storage_type` after provisioning.\n"]
    pub fn vm_file_system_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_file_system_storage_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_odb_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_subnet_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_vm_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exadata_infrastructure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exascale_db_storage_vault: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_oracle_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_connector: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terraform_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClusters {
    #[doc = "Set the field `backup_odb_subnet`.\n"]
    pub fn set_backup_odb_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_odb_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_subnet_cidr`.\n"]
    pub fn set_backup_subnet_cidr(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_subnet_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_vm_cluster_id`.\n"]
    pub fn set_cloud_vm_cluster_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_vm_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `exadata_infrastructure`.\n"]
    pub fn set_exadata_infrastructure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.exadata_infrastructure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exascale_db_storage_vault`.\n"]
    pub fn set_exascale_db_storage_vault(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.exascale_db_storage_vault = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_oracle_zone`.\n"]
    pub fn set_gcp_oracle_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcp_oracle_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_connector`.\n"]
    pub fn set_identity_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnector>>,
    ) -> Self {
        self.identity_connector = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `odb_network`.\n"]
    pub fn set_odb_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.odb_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `odb_subnet`.\n"]
    pub fn set_odb_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.odb_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOracleDatabaseCloudVmClusters_CloudVmClusters_Properties>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataOracleDatabaseCloudVmClusters_CloudVmClusters {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmClusters_CloudVmClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters {}

impl BuildDataOracleDatabaseCloudVmClusters_CloudVmClusters {
    pub fn build(self) -> DataOracleDatabaseCloudVmClusters_CloudVmClusters {
        DataOracleDatabaseCloudVmClusters_CloudVmClusters {
            backup_odb_subnet: core::default::Default::default(),
            backup_subnet_cidr: core::default::Default::default(),
            cidr: core::default::Default::default(),
            cloud_vm_cluster_id: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            display_name: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            exadata_infrastructure: core::default::Default::default(),
            exascale_db_storage_vault: core::default::Default::default(),
            gcp_oracle_zone: core::default::Default::default(),
            identity_connector: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            odb_network: core::default::Default::default(),
            odb_subnet: core::default::Default::default(),
            project: core::default::Default::default(),
            properties: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmClusters_CloudVmClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClusters_CloudVmClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmClusters_CloudVmClustersRef {
        DataOracleDatabaseCloudVmClusters_CloudVmClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmClusters_CloudVmClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_odb_subnet` after provisioning.\n"]
    pub fn backup_odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_odb_subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\n"]
    pub fn backup_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\n"]
    pub fn cloud_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `exadata_infrastructure` after provisioning.\n"]
    pub fn exadata_infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure", self.base))
    }

    #[doc = "Get a reference to the value of field `exascale_db_storage_vault` after provisioning.\n"]
    pub fn exascale_db_storage_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\n"]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_connector` after provisioning.\n"]
    pub fn identity_connector(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClusters_IdentityConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\n"]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.base))
    }

    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\n"]
    pub fn odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmClusters_CloudVmClusters_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }
}
