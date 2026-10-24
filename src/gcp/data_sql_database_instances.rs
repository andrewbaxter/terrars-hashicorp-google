use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSqlDatabaseInstancesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSqlDatabaseInstances_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSqlDatabaseInstancesData>,
}

#[derive(Clone)]
pub struct DataSqlDatabaseInstances(std::rc::Rc<DataSqlDatabaseInstances_>);

impl DataSqlDatabaseInstances {
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
        "Set the field `database_version`.\nTo filter out the database instances which are of the specified database version."]
    pub fn set_database_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nProject ID of the project that contains the instances."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nTo filter out the database instances which are located in this specified region."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nTo filter out the database instances based on the current state of the database instance, valid values include : \"SQL_INSTANCE_STATE_UNSPECIFIED\", \"RUNNABLE\", \"SUSPENDED\", \"PENDING_DELETE\", \"PENDING_CREATE\", \"MAINTENANCE\" and \"FAILED\"."]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\nTo filter out the database instances based on the machine type."]
    pub fn set_tier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nTo filter out the database instances which are located in this specified zone."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nTo filter out the database instances which are of the specified database version."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nProject ID of the project that contains the instances."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nTo filter out the database instances which are located in this specified region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nTo filter out the database instances based on the current state of the database instance, valid values include : \"SQL_INSTANCE_STATE_UNSPECIFIED\", \"RUNNABLE\", \"SUSPENDED\", \"PENDING_DELETE\", \"PENDING_CREATE\", \"MAINTENANCE\" and \"FAILED\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nTo filter out the database instances based on the machine type."]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nTo filter out the database instances which are located in this specified zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataSqlDatabaseInstances {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSqlDatabaseInstances { }

impl terrars::ToListMappable for DataSqlDatabaseInstances {
    type O = terrars::ListRef<DataSqlDatabaseInstancesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSqlDatabaseInstances_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_sql_database_instances".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSqlDatabaseInstances {
    pub tf_id: std::string::String,
}

impl BuildDataSqlDatabaseInstances {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSqlDatabaseInstances {
        let out = DataSqlDatabaseInstances(std::rc::Rc::new(DataSqlDatabaseInstances_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSqlDatabaseInstancesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                database_version: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                tier: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSqlDatabaseInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSqlDatabaseInstancesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nTo filter out the database instances which are of the specified database version."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nProject ID of the project that contains the instances."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nTo filter out the database instances which are located in this specified region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nTo filter out the database instances based on the current state of the database instance, valid values include : \"SQL_INSTANCE_STATE_UNSPECIFIED\", \"RUNNABLE\", \"SUSPENDED\", \"PENDING_DELETE\", \"PENDING_CREATE\", \"MAINTENANCE\" and \"FAILED\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nTo filter out the database instances based on the machine type."]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nTo filter out the database instances which are located in this specified zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElCloneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElCloneEl {
    #[doc = "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_names`.\n"]
    pub fn set_database_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.database_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_in_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_zone`.\n"]
    pub fn set_preferred_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preferred_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_instance_name`.\n"]
    pub fn set_source_instance_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElCloneEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElCloneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElCloneEl {}

impl BuildDataSqlDatabaseInstancesInstancesElCloneEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElCloneEl {
        DataSqlDatabaseInstancesInstancesElCloneEl {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_name: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElCloneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElCloneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSqlDatabaseInstancesInstancesElCloneElRef {
        DataSqlDatabaseInstancesInstancesElCloneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElCloneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `database_names` after provisioning.\n"]
    pub fn database_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_names", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_zone` after provisioning.\n"]
    pub fn preferred_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `source_instance_name` after provisioning.\n"]
    pub fn source_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElIpAddressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_to_retire: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElIpAddressEl {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_to_retire`.\n"]
    pub fn set_time_to_retire(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_to_retire = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElIpAddressEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElIpAddressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElIpAddressEl {}

impl BuildDataSqlDatabaseInstancesInstancesElIpAddressEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElIpAddressEl {
        DataSqlDatabaseInstancesInstancesElIpAddressEl {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElIpAddressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElIpAddressElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElIpAddressElRef {
        DataSqlDatabaseInstancesInstancesElIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElIpAddressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `time_to_retire` after provisioning.\n"]
    pub fn time_to_retire(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_to_retire", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_target: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_heartbeat_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_cipher: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_server_certificate: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
    #[doc = "Set the field `ca_certificate`.\n"]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connect_retry_interval`.\n"]
    pub fn set_connect_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connect_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dump_file_path`.\n"]
    pub fn set_dump_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dump_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failover_target`.\n"]
    pub fn set_failover_target(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.failover_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_heartbeat_period`.\n"]
    pub fn set_master_heartbeat_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.master_heartbeat_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_cipher`.\n"]
    pub fn set_ssl_cipher(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_cipher = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify_server_certificate`.\n"]
    pub fn set_verify_server_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify_server_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {}

impl BuildDataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
        DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            connect_retry_interval: core::default::Default::default(),
            dump_file_path: core::default::Default::default(),
            failover_target: core::default::Default::default(),
            master_heartbeat_period: core::default::Default::default(),
            password: core::default::Default::default(),
            ssl_cipher: core::default::Default::default(),
            username: core::default::Default::default(),
            verify_server_certificate: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef {
        DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\n"]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `connect_retry_interval` after provisioning.\n"]
    pub fn connect_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `dump_file_path` after provisioning.\n"]
    pub fn dump_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_file_path", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_target` after provisioning.\n"]
    pub fn failover_target(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_target", self.base))
    }

    #[doc = "Get a reference to the value of field `master_heartbeat_period` after provisioning.\n"]
    pub fn master_heartbeat_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_heartbeat_period", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_cipher` after provisioning.\n"]
    pub fn ssl_cipher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_cipher", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_server_certificate` after provisioning.\n"]
    pub fn verify_server_certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_server_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_run_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
    #[doc = "Set the field `backup_run_id`.\n"]
    pub fn set_backup_run_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.backup_run_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {}

impl BuildDataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
        DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl {
            backup_run_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef {
        DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_run_id` after provisioning.\n"]
    pub fn backup_run_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_run_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElServerCaCertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha1_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElServerCaCertEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha1_fingerprint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElServerCaCertEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElServerCaCertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElServerCaCertEl {}

impl BuildDataSqlDatabaseInstancesInstancesElServerCaCertEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElServerCaCertEl {
        DataSqlDatabaseInstancesInstancesElServerCaCertEl {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElServerCaCertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElServerCaCertElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElServerCaCertElRef {
        DataSqlDatabaseInstancesInstancesElServerCaCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElServerCaCertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
        DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl {
            domain: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
        DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retained_backups: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc = "Set the field `retained_backups`.\n"]
    pub fn set_retained_backups(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retained_backups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_unit`.\n"]
    pub fn set_retention_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    pub fn build(
        self,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
        DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl {
            retained_backups: core::default::Default::default(),
            retention_unit: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retained_backups` after provisioning.\n"]
    pub fn retained_backups(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retained_backups", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_unit` after provisioning.\n"]
    pub fn retention_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_settings: std
    ::option
    ::Option<
        terrars
        ::ListField<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_log_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transaction_log_retention_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
    #[doc = "Set the field `backup_retention_settings`.\n"]
    pub fn set_backup_retention_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsEl,
                        >,
                    >,
    ) -> Self {
        self.backup_retention_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binary_log_enabled`.\n"]
    pub fn set_binary_log_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.binary_log_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time_recovery_enabled`.\n"]
    pub fn set_point_in_time_recovery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.point_in_time_recovery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transaction_log_retention_days`.\n"]
    pub fn set_transaction_log_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transaction_log_retention_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
        DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl {
            backup_retention_settings: core::default::Default::default(),
            binary_log_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            location: core::default::Default::default(),
            point_in_time_recovery_enabled: core::default::Default::default(),
            start_time: core::default::Default::default(),
            transaction_log_retention_days: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_retention_settings` after provisioning.\n"]
    pub fn backup_retention_settings(
        &self,
    ) -> terrars
    ::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElBackupRetentionSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_retention_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `binary_log_enabled` after provisioning.\n"]
    pub fn binary_log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_log_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery_enabled` after provisioning.\n"]
    pub fn point_in_time_recovery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `transaction_log_retention_days` after provisioning.\n"]
    pub fn transaction_log_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transaction_log_retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
    #[doc = "Set the field `data_cache_enabled`.\n"]
    pub fn set_data_cache_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_cache_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
        DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl {
            data_cache_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_cache_enabled` after provisioning.\n"]
    pub fn data_cache_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_cache_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
        DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
        DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_insights_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_plans_per_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_application_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_client_address: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
    #[doc = "Set the field `query_insights_enabled`.\n"]
    pub fn set_query_insights_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.query_insights_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_plans_per_minute`.\n"]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_length`.\n"]
    pub fn set_query_string_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_string_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_application_tags`.\n"]
    pub fn set_record_application_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_application_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_client_address`.\n"]
    pub fn set_record_client_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_client_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
        DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl {
            query_insights_enabled: core::default::Default::default(),
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_insights_enabled` after provisioning.\n"]
    pub fn query_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_insights_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `query_plans_per_minute` after provisioning.\n"]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_length` after provisioning.\n"]
    pub fn query_string_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc = "Get a reference to the value of field `record_application_tags` after provisioning.\n"]
    pub fn record_application_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `record_client_address` after provisioning.\n"]
    pub fn record_client_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
    type O =
        terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
    #[doc = "Set the field `allowed_consumer_projects`.\n"]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_enabled`.\n"]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\n"]
    pub fn allowed_consumer_projects(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_enabled` after provisioning.\n"]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_networks: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_path_for_google_cloud_services: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
    #[doc = "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorized_networks`.\n"]
    pub fn set_authorized_networks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksEl>,
                    >,
    ) -> Self {
        self.authorized_networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_path_for_google_cloud_services`.\n"]
    pub fn set_enable_private_path_for_google_cloud_services(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_private_path_for_google_cloud_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_enabled`.\n"]
    pub fn set_ipv4_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv4_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_network`.\n"]
    pub fn set_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigEl>,
                    >,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_ssl`.\n"]
    pub fn set_require_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl {
            allocated_ip_range: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            enable_private_path_for_google_cloud_services: core::default::Default::default(),
            ipv4_enabled: core::default::Default::default(),
            private_network: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            require_ssl: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `authorized_networks` after provisioning.\n"]
    pub fn authorized_networks(
        &self,
    ) -> terrars::SetRef<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElAuthorizedNetworksElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.authorized_networks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_path_for_google_cloud_services` after provisioning.\n"]
    pub fn enable_private_path_for_google_cloud_services(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_path_for_google_cloud_services", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv4_enabled` after provisioning.\n"]
    pub fn ipv4_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `private_network` after provisioning.\n"]
    pub fn private_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_network", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(
        &self,
    ) -> terrars::SetRef<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElPscConfigElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }

    #[doc = "Get a reference to the value of field `require_ssl` after provisioning.\n"]
    pub fn require_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_gae_application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
    #[doc = "Set the field `follow_gae_application`.\n"]
    pub fn set_follow_gae_application(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.follow_gae_application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_zone`.\n"]
    pub fn set_secondary_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
        DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `follow_gae_application` after provisioning.\n"]
    pub fn follow_gae_application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow_gae_application", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_zone` after provisioning.\n"]
    pub fn secondary_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_track: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hour`.\n"]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_track`.\n"]
    pub fn set_update_track(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_track = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
        DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `hour` after provisioning.\n"]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc = "Get a reference to the value of field `update_track` after provisioning.\n"]
    pub fn update_track(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_track", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    complexity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallow_username_substring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_password_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_change_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
    #[doc = "Set the field `complexity`.\n"]
    pub fn set_complexity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.complexity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disallow_username_substring`.\n"]
    pub fn set_disallow_username_substring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disallow_username_substring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_password_policy`.\n"]
    pub fn set_enable_password_policy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_password_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_length`.\n"]
    pub fn set_min_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_change_interval`.\n"]
    pub fn set_password_change_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_change_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reuse_interval`.\n"]
    pub fn set_reuse_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.reuse_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
        DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: core::default::Default::default(),
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `complexity` after provisioning.\n"]
    pub fn complexity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complexity", self.base))
    }

    #[doc = "Get a reference to the value of field `disallow_username_substring` after provisioning.\n"]
    pub fn disallow_username_substring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disallow_username_substring", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_password_policy` after provisioning.\n"]
    pub fn enable_password_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_password_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `min_length` after provisioning.\n"]
    pub fn min_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }

    #[doc = "Get a reference to the value of field `password_change_interval` after provisioning.\n"]
    pub fn password_change_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_change_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `reuse_interval` after provisioning.\n"]
    pub fn reuse_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_interval`.\n"]
    pub fn set_retention_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.retention_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_interval`.\n"]
    pub fn set_upload_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
        DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_interval` after provisioning.\n"]
    pub fn retention_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_interval` after provisioning.\n"]
    pub fn upload_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesElSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::SetField<DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_autoresize: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_autoresize_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insights_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_preference: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_validation_policy: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pricing_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_audit_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstancesInstancesElSettingsEl {
    #[doc = "Set the field `activation_policy`.\n"]
    pub fn set_activation_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.activation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `active_directory_config`.\n"]
    pub fn set_active_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigEl>>,
    ) -> Self {
        self.active_directory_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesEl>,
                    >,
    ) -> Self {
        self.advanced_machine_features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_type`.\n"]
    pub fn set_availability_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationEl>>,
    ) -> Self {
        self.backup_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collation`.\n"]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_enforcement`.\n"]
    pub fn set_connector_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_enforcement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigEl>>,
    ) -> Self {
        self.data_cache_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_flags`.\n"]
    pub fn set_database_flags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsEl>>,
    ) -> Self {
        self.database_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodEl>>,
    ) -> Self {
        self.deny_maintenance_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_autoresize`.\n"]
    pub fn set_disk_autoresize(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disk_autoresize = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_autoresize_limit`.\n"]
    pub fn set_disk_autoresize_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_autoresize_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size`.\n"]
    pub fn set_disk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edition`.\n"]
    pub fn set_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigEl>>,
    ) -> Self {
        self.insights_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_configuration`.\n"]
    pub fn set_ip_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationEl>>,
    ) -> Self {
        self.ip_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_preference`.\n"]
    pub fn set_location_preference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceEl>>,
    ) -> Self {
        self.location_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowEl>>,
    ) -> Self {
        self.maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_validation_policy`.\n"]
    pub fn set_password_validation_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyEl>,
                    >,
    ) -> Self {
        self.password_validation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pricing_plan`.\n"]
    pub fn set_pricing_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pricing_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigEl>>,
    ) -> Self {
        self.sql_server_audit_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_labels`.\n"]
    pub fn set_user_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesElSettingsEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesElSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesElSettingsEl {}

impl BuildDataSqlDatabaseInstancesInstancesElSettingsEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesElSettingsEl {
        DataSqlDatabaseInstancesInstancesElSettingsEl {
            activation_policy: core::default::Default::default(),
            active_directory_config: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            availability_type: core::default::Default::default(),
            backup_configuration: core::default::Default::default(),
            collation: core::default::Default::default(),
            connector_enforcement: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            deny_maintenance_period: core::default::Default::default(),
            disk_autoresize: core::default::Default::default(),
            disk_autoresize_limit: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            edition: core::default::Default::default(),
            insights_config: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            location_preference: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            password_validation_policy: core::default::Default::default(),
            pricing_plan: core::default::Default::default(),
            sql_server_audit_config: core::default::Default::default(),
            tier: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstancesInstancesElSettingsElRef {
        DataSqlDatabaseInstancesInstancesElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\n"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElActiveDirectoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElBackupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `collation` after provisioning.\n"]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_enforcement` after provisioning.\n"]
    pub fn connector_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_enforcement", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElDataCacheConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\n"]
    pub fn database_flags(&self) -> terrars::SetRef<DataSqlDatabaseInstancesInstancesElSettingsElDatabaseFlagsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.database_flags", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElDenyMaintenancePeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_autoresize` after provisioning.\n"]
    pub fn disk_autoresize(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_autoresize_limit` after provisioning.\n"]
    pub fn disk_autoresize_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }

    #[doc = "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElInsightsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElIpConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElLocationPreferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElPasswordValidationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_validation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `pricing_plan` after provisioning.\n"]
    pub fn pricing_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElSqlServerAuditConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_audit_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\n"]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstancesInstancesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_maintenance_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone: std::option::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElCloneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElIpAddressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_service_attachment_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_configuration: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_context: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_cert: std::option::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElServerCaCertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsEl>>,
}

impl DataSqlDatabaseInstancesInstancesEl {
    #[doc = "Set the field `available_maintenance_versions`.\n"]
    pub fn set_available_maintenance_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.available_maintenance_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clone`.\n"]
    pub fn set_clone(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElCloneEl>>,
    ) -> Self {
        self.clone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_version`.\n"]
    pub fn set_database_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_name`.\n"]
    pub fn set_encryption_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_ip_address`.\n"]
    pub fn set_first_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElIpAddressEl>>,
    ) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_version`.\n"]
    pub fn set_maintenance_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.maintenance_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_instance_name`.\n"]
    pub fn set_master_instance_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_instance_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_service_attachment_link`.\n"]
    pub fn set_psc_service_attachment_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.psc_service_attachment_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_address`.\n"]
    pub fn set_public_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_configuration`.\n"]
    pub fn set_replica_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElReplicaConfigurationEl>>,
    ) -> Self {
        self.replica_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_backup_context`.\n"]
    pub fn set_restore_backup_context(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElRestoreBackupContextEl>>,
    ) -> Self {
        self.restore_backup_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password`.\n"]
    pub fn set_root_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_ca_cert`.\n"]
    pub fn set_server_ca_cert(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElServerCaCertEl>>,
    ) -> Self {
        self.server_ca_cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email_address`.\n"]
    pub fn set_service_account_email_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstancesInstancesElSettingsEl>>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstancesInstancesEl {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstancesInstancesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstancesInstancesEl {}

impl BuildDataSqlDatabaseInstancesInstancesEl {
    pub fn build(self) -> DataSqlDatabaseInstancesInstancesEl {
        DataSqlDatabaseInstancesInstancesEl {
            available_maintenance_versions: core::default::Default::default(),
            clone: core::default::Default::default(),
            connection_name: core::default::Default::default(),
            database_version: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            dns_name: core::default::Default::default(),
            encryption_key_name: core::default::Default::default(),
            first_ip_address: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            maintenance_version: core::default::Default::default(),
            master_instance_name: core::default::Default::default(),
            name: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            project: core::default::Default::default(),
            psc_service_attachment_link: core::default::Default::default(),
            public_ip_address: core::default::Default::default(),
            region: core::default::Default::default(),
            replica_configuration: core::default::Default::default(),
            restore_backup_context: core::default::Default::default(),
            root_password: core::default::Default::default(),
            self_link: core::default::Default::default(),
            server_ca_cert: core::default::Default::default(),
            service_account_email_address: core::default::Default::default(),
            settings: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstancesInstancesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstancesInstancesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSqlDatabaseInstancesInstancesElRef {
        DataSqlDatabaseInstancesInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstancesInstancesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\n"]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_maintenance_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElCloneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `database_version` after provisioning.\n"]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `first_ip_address` after provisioning.\n"]
    pub fn first_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElIpAddressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\n"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.base))
    }

    #[doc = "Get a reference to the value of field `master_instance_name` after provisioning.\n"]
    pub fn master_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_instance_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\n"]
    pub fn psc_service_attachment_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment_link", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\n"]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElReplicaConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElRestoreBackupContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.base))
    }

    #[doc = "Get a reference to the value of field `root_password` after provisioning.\n"]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElServerCaCertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email_address` after provisioning.\n"]
    pub fn service_account_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DataSqlDatabaseInstancesInstancesElSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}
