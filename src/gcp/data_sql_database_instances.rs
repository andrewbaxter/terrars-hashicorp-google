#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSqlDatabaseInstancesData_ {
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
    data: std::cell::RefCell<DataSqlDatabaseInstancesData_>,
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
    pub fn instances(&self) -> terrars::ListRef<DataSqlDatabaseInstances_InstancesRef> {
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
            data: std::cell::RefCell::new(DataSqlDatabaseInstancesData_ {
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
    pub fn instances(&self) -> terrars::ListRef<DataSqlDatabaseInstances_InstancesRef> {
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
pub struct DataSqlDatabaseInstances_Instances_Clone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_deletion_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Clone {
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

    #[doc = "Set the field `source_instance_deletion_time`.\n"]
    pub fn set_source_instance_deletion_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_deletion_time = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `source_project`.\n"]
    pub fn set_source_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Clone {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Clone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Clone {}

impl BuildDataSqlDatabaseInstances_Instances_Clone {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Clone {
        DataSqlDatabaseInstances_Instances_Clone {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_deletion_time: core::default::Default::default(),
            source_instance_name: core::default::Default::default(),
            source_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_CloneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_CloneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSqlDatabaseInstances_Instances_CloneRef {
        DataSqlDatabaseInstances_Instances_CloneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_CloneRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `source_instance_deletion_time` after provisioning.\n"]
    pub fn source_instance_deletion_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_deletion_time", self.base))
    }

    #[doc = "Get a reference to the value of field `source_instance_name` after provisioning.\n"]
    pub fn source_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_project` after provisioning.\n"]
    pub fn source_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_DnsNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_DnsNames {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_scope`.\n"]
    pub fn set_dns_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_DnsNames {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_DnsNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_DnsNames {}

impl BuildDataSqlDatabaseInstances_Instances_DnsNames {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_DnsNames {
        DataSqlDatabaseInstances_Instances_DnsNames {
            connection_type: core::default::Default::default(),
            dns_scope: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_DnsNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_DnsNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_DnsNamesRef {
        DataSqlDatabaseInstances_Instances_DnsNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_DnsNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_scope` after provisioning.\n"]
    pub fn dns_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_IpAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_to_retire: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_IpAddress {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_IpAddress {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_IpAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_IpAddress {}

impl BuildDataSqlDatabaseInstances_Instances_IpAddress {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_IpAddress {
        DataSqlDatabaseInstances_Instances_IpAddress {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_IpAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_IpAddressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_IpAddressRef {
        DataSqlDatabaseInstances_Instances_IpAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_IpAddressRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
    #[doc = "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datasource`.\n"]
    pub fn set_datasource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datasource = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_instance`.\n"]
    pub fn set_target_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {}

impl BuildDataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
        DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext {
            allocated_ip_range: core::default::Default::default(),
            datasource: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            region: core::default::Default::default(),
            target_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef {
        DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `datasource` after provisioning.\n"]
    pub fn datasource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datasource", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_zone` after provisioning.\n"]
    pub fn preferred_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `target_instance` after provisioning.\n"]
    pub fn target_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_ReplicaConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cascadable_replica: std::option::Option<terrars::PrimField<bool>>,
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

impl DataSqlDatabaseInstances_Instances_ReplicaConfiguration {
    #[doc = "Set the field `ca_certificate`.\n"]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cascadable_replica`.\n"]
    pub fn set_cascadable_replica(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cascadable_replica = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_ReplicaConfiguration {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_ReplicaConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_ReplicaConfiguration {}

impl BuildDataSqlDatabaseInstances_Instances_ReplicaConfiguration {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_ReplicaConfiguration {
        DataSqlDatabaseInstances_Instances_ReplicaConfiguration {
            ca_certificate: core::default::Default::default(),
            cascadable_replica: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef {
        DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\n"]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cascadable_replica` after provisioning.\n"]
    pub fn cascadable_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cascadable_replica", self.base))
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
pub struct DataSqlDatabaseInstances_Instances_ReplicationCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dr_replica: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_dr_replica_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psa_write_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_ReplicationCluster {
    #[doc = "Set the field `dr_replica`.\n"]
    pub fn set_dr_replica(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dr_replica = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failover_dr_replica_name`.\n"]
    pub fn set_failover_dr_replica_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.failover_dr_replica_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psa_write_endpoint`.\n"]
    pub fn set_psa_write_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.psa_write_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_ReplicationCluster {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_ReplicationCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_ReplicationCluster {}

impl BuildDataSqlDatabaseInstances_Instances_ReplicationCluster {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_ReplicationCluster {
        DataSqlDatabaseInstances_Instances_ReplicationCluster {
            dr_replica: core::default::Default::default(),
            failover_dr_replica_name: core::default::Default::default(),
            psa_write_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_ReplicationClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_ReplicationClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_ReplicationClusterRef {
        DataSqlDatabaseInstances_Instances_ReplicationClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_ReplicationClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dr_replica` after provisioning.\n"]
    pub fn dr_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dr_replica", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_dr_replica_name` after provisioning.\n"]
    pub fn failover_dr_replica_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_dr_replica_name", self.base))
    }

    #[doc = "Get a reference to the value of field `psa_write_endpoint` after provisioning.\n"]
    pub fn psa_write_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psa_write_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_RestoreBackupContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_run_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_RestoreBackupContext {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_RestoreBackupContext {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_RestoreBackupContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_RestoreBackupContext {}

impl BuildDataSqlDatabaseInstances_Instances_RestoreBackupContext {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_RestoreBackupContext {
        DataSqlDatabaseInstances_Instances_RestoreBackupContext {
            backup_run_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_RestoreBackupContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_RestoreBackupContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_RestoreBackupContextRef {
        DataSqlDatabaseInstances_Instances_RestoreBackupContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_RestoreBackupContextRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_ServerCaCert {
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

impl DataSqlDatabaseInstances_Instances_ServerCaCert {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_ServerCaCert {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_ServerCaCert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_ServerCaCert {}

impl BuildDataSqlDatabaseInstances_Instances_ServerCaCert {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_ServerCaCert {
        DataSqlDatabaseInstances_Instances_ServerCaCert {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_ServerCaCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_ServerCaCertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_ServerCaCertRef {
        DataSqlDatabaseInstances_Instances_ServerCaCertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_ServerCaCertRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_credential_secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
    #[doc = "Set the field `admin_credential_secret_name`.\n"]
    pub fn set_admin_credential_secret_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.admin_credential_secret_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\n"]
    pub fn set_dns_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
        DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig {
            admin_credential_secret_name: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            domain: core::default::Default::default(),
            mode: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_credential_secret_name` after provisioning.\n"]
    pub fn admin_credential_secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_credential_secret_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
        DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures {
            threads_per_core: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef {
        DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retained_backups: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
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
::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
    type O =
        terrars
        ::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
        DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings {
            retained_backups: core::default::Default::default(),
            retention_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef {
        DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_settings: std
    ::option
    ::Option<
        terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_tier: std::option::Option<terrars::PrimField<std::string::String>>,
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

impl DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
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
                            DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettings,
                        >,
                    >,
    ) -> Self {
        self.backup_retention_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_tier`.\n"]
    pub fn set_backup_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_tier = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
        DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration {
            backup_retention_settings: core::default::Default::default(),
            backup_tier: core::default::Default::default(),
            binary_log_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            location: core::default::Default::default(),
            point_in_time_recovery_enabled: core::default::Default::default(),
            start_time: core::default::Default::default(),
            transaction_log_retention_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef {
        DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_retention_settings` after provisioning.\n"]
    pub fn backup_retention_settings(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration_BackupRetentionSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_retention_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_tier` after provisioning.\n"]
    pub fn backup_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_tier", self.base))
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
pub struct DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
        DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef {
        DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pooling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flags: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
    #[doc = "Set the field `connection_pooling_enabled`.\n"]
    pub fn set_connection_pooling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.connection_pooling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flags`.\n"]
    pub fn set_flags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_Flags>>,
    ) -> Self {
        self.flags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
        DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig {
            connection_pooling_enabled: core::default::Default::default(),
            flags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pooling_enabled` after provisioning.\n"]
    pub fn connection_pooling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_pooling_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig_FlagsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
    #[doc = "Set the field `data_cache_enabled`.\n"]
    pub fn set_data_cache_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_cache_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
        DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig {
            data_cache_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_cache_enabled` after provisioning.\n"]
    pub fn data_cache_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_cache_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
        DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef {
        DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
        DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef {
        DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
    #[doc = "Set the field `application_id`.\n"]
    pub fn set_application_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_EntraidConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_EntraidConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
        DataSqlDatabaseInstances_Instances_Settings_EntraidConfig {
            application_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_days`.\n"]
    pub fn set_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
        DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig {
            enabled: core::default::Default::default(),
            retention_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_days` after provisioning.\n"]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_query_insights_enabled: std::option::Option<terrars::PrimField<bool>>,
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

impl DataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
    #[doc = "Set the field `enhanced_query_insights_enabled`.\n"]
    pub fn set_enhanced_query_insights_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enhanced_query_insights_enabled = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_InsightsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_InsightsConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
        DataSqlDatabaseInstances_Instances_Settings_InsightsConfig {
            enhanced_query_insights_enabled: core::default::Default::default(),
            query_insights_enabled: core::default::Default::default(),
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enhanced_query_insights_enabled` after provisioning.\n"]
    pub fn enhanced_query_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_query_insights_enabled", self.base))
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
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
    type O =
        terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_service_project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_auto_dns_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connection_policy_creation_result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_endpoint_auto_dns_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_network_status`.\n"]
    pub fn set_consumer_network_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_network_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_service_project_id`.\n"]
    pub fn set_consumer_service_project_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_service_project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_auto_dns_status`.\n"]
    pub fn set_instance_auto_dns_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_auto_dns_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_connection_policy`.\n"]
    pub fn set_service_connection_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_connection_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_connection_policy_creation_result`.\n"]
    pub fn set_service_connection_policy_creation_result(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_connection_policy_creation_result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_endpoint_auto_dns_status`.\n"]
    pub fn set_write_endpoint_auto_dns_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_endpoint_auto_dns_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    type O =
        terrars
        ::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections {
            consumer_network: core::default::Default::default(),
            consumer_network_status: core::default::Default::default(),
            consumer_service_project_id: core::default::Default::default(),
            instance_auto_dns_status: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            service_connection_policy: core::default::Default::default(),
            service_connection_policy_creation_result: core::default::Default::default(),
            status: core::default::Default::default(),
            write_endpoint_auto_dns_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_network_status` after provisioning.\n"]
    pub fn consumer_network_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network_status", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_service_project_id` after provisioning.\n"]
    pub fn consumer_service_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_service_project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_auto_dns_status` after provisioning.\n"]
    pub fn instance_auto_dns_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_auto_dns_status", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `service_connection_policy` after provisioning.\n"]
    pub fn service_connection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_connection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `service_connection_policy_creation_result` after provisioning.\n"]
    pub fn service_connection_policy_creation_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_connection_policy_creation_result", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `write_endpoint_auto_dns_status` after provisioning.\n"]
    pub fn write_endpoint_auto_dns_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_endpoint_auto_dns_status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connection_policy_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connections: std
    ::option
    ::Option<
        terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_dns_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_write_endpoint_dns_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
    #[doc = "Set the field `allowed_consumer_projects`.\n"]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_attachment_uri`.\n"]
    pub fn set_network_attachment_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_connection_policy_enabled`.\n"]
    pub fn set_psc_auto_connection_policy_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.psc_auto_connection_policy_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnections,
                        >,
                    >,
    ) -> Self {
        self.psc_auto_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_dns_enabled`.\n"]
    pub fn set_psc_auto_dns_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_auto_dns_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_enabled`.\n"]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_write_endpoint_dns_enabled`.\n"]
    pub fn set_psc_write_endpoint_dns_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_write_endpoint_dns_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig {
            allowed_consumer_projects: core::default::Default::default(),
            network_attachment_uri: core::default::Default::default(),
            psc_auto_connection_policy_enabled: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            psc_auto_dns_enabled: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
            psc_write_endpoint_dns_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\n"]
    pub fn allowed_consumer_projects(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc = "Get a reference to the value of field `network_attachment_uri` after provisioning.\n"]
    pub fn network_attachment_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_connection_policy_enabled` after provisioning.\n"]
    pub fn psc_auto_connection_policy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_auto_connection_policy_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(
        &self,
    ) -> terrars
    ::ListRef<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_dns_enabled` after provisioning.\n"]
    pub fn psc_auto_dns_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_auto_dns_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_enabled` after provisioning.\n"]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_write_endpoint_dns_enabled` after provisioning.\n"]
    pub fn psc_write_endpoint_dns_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_write_endpoint_dns_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_networks: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_subject_alternative_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_path_for_google_cloud_services: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate_rotation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
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
                        ::SetField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworks>,
                    >,
    ) -> Self {
        self.authorized_networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_subject_alternative_names`.\n"]
    pub fn set_custom_subject_alternative_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_subject_alternative_names = std::option::Option::Some(v.into());
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
                    ::Into<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfig>>,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_ca_mode`.\n"]
    pub fn set_server_ca_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_ca_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_ca_pool`.\n"]
    pub fn set_server_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_certificate_rotation_mode`.\n"]
    pub fn set_server_certificate_rotation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_certificate_rotation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
        DataSqlDatabaseInstances_Instances_Settings_IpConfiguration {
            allocated_ip_range: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            custom_subject_alternative_names: core::default::Default::default(),
            enable_private_path_for_google_cloud_services: core::default::Default::default(),
            ipv4_enabled: core::default::Default::default(),
            private_network: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            server_ca_mode: core::default::Default::default(),
            server_ca_pool: core::default::Default::default(),
            server_certificate_rotation_mode: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef {
        DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef {
    #[allow(dead_code)]
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
    ) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_AuthorizedNetworksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.authorized_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_subject_alternative_names` after provisioning.\n"]
    pub fn custom_subject_alternative_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_subject_alternative_names", self.base))
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
    ) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration_PscConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }

    #[doc = "Get a reference to the value of field `server_ca_mode` after provisioning.\n"]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `server_ca_pool` after provisioning.\n"]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `server_certificate_rotation_mode` after provisioning.\n"]
    pub fn server_certificate_rotation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_rotation_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_LocationPreference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_gae_application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_LocationPreference {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_LocationPreference {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_LocationPreference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_LocationPreference {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_LocationPreference {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_LocationPreference {
        DataSqlDatabaseInstances_Instances_Settings_LocationPreference {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef {
        DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_track: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
        DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef {
        DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
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

impl DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
        DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: core::default::Default::default(),
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef {
        DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    type O =
        terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
        DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
            metric: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
        DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_scale_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_cooldown_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_cooldown_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_metrics: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_node_count`.\n"]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_count`.\n"]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_cooldown_seconds`.\n"]
    pub fn set_scale_in_cooldown_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in_cooldown_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_cooldown_seconds`.\n"]
    pub fn set_scale_out_cooldown_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out_cooldown_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_metrics`.\n"]
    pub fn set_target_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetrics>,
                    >,
    ) -> Self {
        self.target_metrics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
        DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig {
            disable_scale_in: core::default::Default::default(),
            enabled: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            scale_in_cooldown_seconds: core::default::Default::default(),
            scale_out_cooldown_seconds: core::default::Default::default(),
            target_metrics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown_seconds` after provisioning.\n"]
    pub fn scale_in_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown_seconds` after provisioning.\n"]
    pub fn scale_out_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `target_metrics` after provisioning.\n"]
    pub fn target_metrics(
        &self,
    ) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {}

impl BuildDataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
        DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef {
        DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef {
    #[allow(dead_code)]
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
pub struct DataSqlDatabaseInstances_Instances_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool_config: std
    ::option
    ::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_api_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod>>,
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
    effective_availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dataplex_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_google_ml_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entraid_config: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_EntraidConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insights_config: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_InsightsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_preference: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_LocationPreference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_validation_policy: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pricing_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_pool_auto_scale_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_lag_max_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_backups_on_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_audit_config: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSqlDatabaseInstances_Instances_Settings {
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfig>>,
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeatures>>,
    ) -> Self {
        self.advanced_machine_features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_upgrade_enabled`.\n"]
    pub fn set_auto_upgrade_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade_enabled = std::option::Option::Some(v.into());
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_BackupConfiguration>>,
    ) -> Self {
        self.backup_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collation`.\n"]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfig>>,
    ) -> Self {
        self.connection_pool_config = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_api_access`.\n"]
    pub fn set_data_api_access(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_api_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_DataCacheConfig>>,
    ) -> Self {
        self.data_cache_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_disk_provisioned_iops`.\n"]
    pub fn set_data_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_disk_provisioned_throughput`.\n"]
    pub fn set_data_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_flags`.\n"]
    pub fn set_database_flags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataSqlDatabaseInstances_Instances_Settings_DatabaseFlags>>,
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriod>>,
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

    #[doc = "Set the field `effective_availability_type`.\n"]
    pub fn set_effective_availability_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.effective_availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dataplex_integration`.\n"]
    pub fn set_enable_dataplex_integration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_dataplex_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_google_ml_integration`.\n"]
    pub fn set_enable_google_ml_integration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_google_ml_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entraid_config`.\n"]
    pub fn set_entraid_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_EntraidConfig>>,
    ) -> Self {
        self.entraid_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_backup_config`.\n"]
    pub fn set_final_backup_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfig>>,
    ) -> Self {
        self.final_backup_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_InsightsConfig>>,
    ) -> Self {
        self.insights_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_configuration`.\n"]
    pub fn set_ip_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_IpConfiguration>>,
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_LocationPreference>>,
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindow>>,
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
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicy>>,
    ) -> Self {
        self.password_validation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pricing_plan`.\n"]
    pub fn set_pricing_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pricing_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_pool_auto_scale_config`.\n"]
    pub fn set_read_pool_auto_scale_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfig>>,
    ) -> Self {
        self.read_pool_auto_scale_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_lag_max_seconds`.\n"]
    pub fn set_replication_lag_max_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replication_lag_max_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retain_backups_on_delete`.\n"]
    pub fn set_retain_backups_on_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retain_backups_on_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfig>>,
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

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances_Settings {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances_Settings {}

impl BuildDataSqlDatabaseInstances_Instances_Settings {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances_Settings {
        DataSqlDatabaseInstances_Instances_Settings {
            activation_policy: core::default::Default::default(),
            active_directory_config: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            auto_upgrade_enabled: core::default::Default::default(),
            availability_type: core::default::Default::default(),
            backup_configuration: core::default::Default::default(),
            collation: core::default::Default::default(),
            connection_pool_config: core::default::Default::default(),
            connector_enforcement: core::default::Default::default(),
            data_api_access: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            data_disk_provisioned_iops: core::default::Default::default(),
            data_disk_provisioned_throughput: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            deny_maintenance_period: core::default::Default::default(),
            disk_autoresize: core::default::Default::default(),
            disk_autoresize_limit: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            edition: core::default::Default::default(),
            effective_availability_type: core::default::Default::default(),
            enable_dataplex_integration: core::default::Default::default(),
            enable_google_ml_integration: core::default::Default::default(),
            entraid_config: core::default::Default::default(),
            final_backup_config: core::default::Default::default(),
            insights_config: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            location_preference: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            password_validation_policy: core::default::Default::default(),
            pricing_plan: core::default::Default::default(),
            read_pool_auto_scale_config: core::default::Default::default(),
            replication_lag_max_seconds: core::default::Default::default(),
            retain_backups_on_delete: core::default::Default::default(),
            sql_server_audit_config: core::default::Default::default(),
            tier: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_Instances_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_Instances_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSqlDatabaseInstances_Instances_SettingsRef {
        DataSqlDatabaseInstances_Instances_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_Instances_SettingsRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_ActiveDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_upgrade_enabled` after provisioning.\n"]
    pub fn auto_upgrade_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_BackupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `collation` after provisioning.\n"]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(
        &self,
    ) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_ConnectionPoolConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_enforcement` after provisioning.\n"]
    pub fn connector_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_enforcement", self.base))
    }

    #[doc = "Get a reference to the value of field `data_api_access` after provisioning.\n"]
    pub fn data_api_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_api_access", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_DataCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `data_disk_provisioned_iops` after provisioning.\n"]
    pub fn data_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_provisioned_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `data_disk_provisioned_throughput` after provisioning.\n"]
    pub fn data_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_provisioned_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\n"]
    pub fn database_flags(&self) -> terrars::SetRef<DataSqlDatabaseInstances_Instances_Settings_DatabaseFlagsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.database_flags", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_DenyMaintenancePeriodRef> {
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

    #[doc = "Get a reference to the value of field `effective_availability_type` after provisioning.\n"]
    pub fn effective_availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_availability_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_dataplex_integration` after provisioning.\n"]
    pub fn enable_dataplex_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dataplex_integration", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_google_ml_integration` after provisioning.\n"]
    pub fn enable_google_ml_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_google_ml_integration", self.base))
    }

    #[doc = "Get a reference to the value of field `entraid_config` after provisioning.\n"]
    pub fn entraid_config(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_EntraidConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entraid_config", self.base))
    }

    #[doc = "Get a reference to the value of field `final_backup_config` after provisioning.\n"]
    pub fn final_backup_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_FinalBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.final_backup_config", self.base))
    }

    #[doc = "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_InsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_IpConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_LocationPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_PasswordValidationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_validation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `pricing_plan` after provisioning.\n"]
    pub fn pricing_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `read_pool_auto_scale_config` after provisioning.\n"]
    pub fn read_pool_auto_scale_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_ReadPoolAutoScaleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_auto_scale_config", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_lag_max_seconds` after provisioning.\n"]
    pub fn replication_lag_max_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_lag_max_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_backups_on_delete` after provisioning.\n"]
    pub fn retain_backups_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_backups_on_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_Settings_SqlServerAuditConfigRef> {
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
pub struct DataSqlDatabaseInstances_Instances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_maintenance_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backupdr_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Clone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_DnsNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_confidential_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_new_sql_network_architecture: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_replicas_for_major_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_IpAddress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_restore_context: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext>>,
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
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_ReplicaConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_cluster: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_ReplicationCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_context: std
    ::option
    ::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_RestoreBackupContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_cert: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_ServerCaCert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_transaction_logs_to_cloud_storage_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSqlDatabaseInstances_Instances {
    #[doc = "Set the field `available_maintenance_versions`.\n"]
    pub fn set_available_maintenance_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.available_maintenance_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backupdr_backup`.\n"]
    pub fn set_backupdr_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backupdr_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clone`.\n"]
    pub fn set_clone(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Clone>>,
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

    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_names`.\n"]
    pub fn set_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_DnsNames>>,
    ) -> Self {
        self.dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_confidential_mode`.\n"]
    pub fn set_encryption_confidential_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encryption_confidential_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `enforce_new_sql_network_architecture`.\n"]
    pub fn set_enforce_new_sql_network_architecture(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enforce_new_sql_network_architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_backup_description`.\n"]
    pub fn set_final_backup_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.final_backup_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_ip_address`.\n"]
    pub fn set_first_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_replicas_for_major_version_upgrade`.\n"]
    pub fn set_include_replicas_for_major_version_upgrade(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.include_replicas_for_major_version_upgrade = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_IpAddress>>,
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

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time_restore_context`.\n"]
    pub fn set_point_in_time_restore_context(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_PointInTimeRestoreContext>>,
    ) -> Self {
        self.point_in_time_restore_context = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_ReplicaConfiguration>>,
    ) -> Self {
        self.replica_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_names`.\n"]
    pub fn set_replica_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replica_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_cluster`.\n"]
    pub fn set_replication_cluster(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_ReplicationCluster>>,
    ) -> Self {
        self.replication_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_backup_context`.\n"]
    pub fn set_restore_backup_context(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_RestoreBackupContext>>,
    ) -> Self {
        self.restore_backup_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password`.\n"]
    pub fn set_root_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password_wo`.\n"]
    pub fn set_root_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password_wo_version`.\n"]
    pub fn set_root_password_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.root_password_wo_version = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_ServerCaCert>>,
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
        v: impl std::convert::Into<terrars::ListField<DataSqlDatabaseInstances_Instances_Settings>>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `switch_transaction_logs_to_cloud_storage_enabled`.\n"]
    pub fn set_switch_transaction_logs_to_cloud_storage_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.switch_transaction_logs_to_cloud_storage_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSqlDatabaseInstances_Instances {
    type O = terrars::BlockAssignable<DataSqlDatabaseInstances_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstances_Instances {}

impl BuildDataSqlDatabaseInstances_Instances {
    pub fn build(self) -> DataSqlDatabaseInstances_Instances {
        DataSqlDatabaseInstances_Instances {
            available_maintenance_versions: core::default::Default::default(),
            backupdr_backup: core::default::Default::default(),
            clone: core::default::Default::default(),
            connection_name: core::default::Default::default(),
            database_version: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            dns_name: core::default::Default::default(),
            dns_names: core::default::Default::default(),
            encryption_confidential_mode: core::default::Default::default(),
            encryption_key_name: core::default::Default::default(),
            enforce_new_sql_network_architecture: core::default::Default::default(),
            final_backup_description: core::default::Default::default(),
            first_ip_address: core::default::Default::default(),
            include_replicas_for_major_version_upgrade: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            maintenance_version: core::default::Default::default(),
            master_instance_name: core::default::Default::default(),
            name: core::default::Default::default(),
            node_count: core::default::Default::default(),
            point_in_time_restore_context: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            project: core::default::Default::default(),
            psc_service_attachment_link: core::default::Default::default(),
            public_ip_address: core::default::Default::default(),
            region: core::default::Default::default(),
            replica_configuration: core::default::Default::default(),
            replica_names: core::default::Default::default(),
            replication_cluster: core::default::Default::default(),
            restore_backup_context: core::default::Default::default(),
            root_password: core::default::Default::default(),
            root_password_wo: core::default::Default::default(),
            root_password_wo_version: core::default::Default::default(),
            self_link: core::default::Default::default(),
            server_ca_cert: core::default::Default::default(),
            service_account_email_address: core::default::Default::default(),
            settings: core::default::Default::default(),
            switch_transaction_logs_to_cloud_storage_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSqlDatabaseInstances_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSqlDatabaseInstances_InstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSqlDatabaseInstances_InstancesRef {
        DataSqlDatabaseInstances_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstances_InstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_maintenance_versions` after provisioning.\n"]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_maintenance_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `backupdr_backup` after provisioning.\n"]
    pub fn backupdr_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backupdr_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_CloneRef> {
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

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\n"]
    pub fn dns_names(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_DnsNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_confidential_mode` after provisioning.\n"]
    pub fn encryption_confidential_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_confidential_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_new_sql_network_architecture` after provisioning.\n"]
    pub fn enforce_new_sql_network_architecture(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_new_sql_network_architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `final_backup_description` after provisioning.\n"]
    pub fn final_backup_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_backup_description", self.base))
    }

    #[doc = "Get a reference to the value of field `first_ip_address` after provisioning.\n"]
    pub fn first_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `include_replicas_for_major_version_upgrade` after provisioning.\n"]
    pub fn include_replicas_for_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_replicas_for_major_version_upgrade", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_IpAddressRef> {
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

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time_restore_context` after provisioning.\n"]
    pub fn point_in_time_restore_context(
        &self,
    ) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_PointInTimeRestoreContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_restore_context", self.base))
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
    pub fn replica_configuration(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_ReplicaConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_names` after provisioning.\n"]
    pub fn replica_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_names", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_cluster` after provisioning.\n"]
    pub fn replication_cluster(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_ReplicationClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_RestoreBackupContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.base))
    }

    #[doc = "Get a reference to the value of field `root_password` after provisioning.\n"]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.base))
    }

    #[doc = "Get a reference to the value of field `root_password_wo` after provisioning.\n"]
    pub fn root_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `root_password_wo_version` after provisioning.\n"]
    pub fn root_password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_ServerCaCertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email_address` after provisioning.\n"]
    pub fn service_account_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DataSqlDatabaseInstances_Instances_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `switch_transaction_logs_to_cloud_storage_enabled` after provisioning.\n"]
    pub fn switch_transaction_logs_to_cloud_storage_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.switch_transaction_logs_to_cloud_storage_enabled", self.base),
        )
    }
}
