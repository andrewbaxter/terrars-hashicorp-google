use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSqlDatabaseInstanceData {
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
}

struct DataSqlDatabaseInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlDatabaseInstanceData>,
}

#[derive(Clone)]
pub struct DataSqlDatabaseInstance(Rc<DataSqlDatabaseInstance_>);

impl DataSqlDatabaseInstance {
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

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.available_maintenance_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `clone` after provisioning.\nConfiguration for creating a new instance as a clone of another instance."]
    pub fn clone(&self) -> ListRef<DataSqlDatabaseInstanceCloneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server (beta) version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, SQLSERVER_2017_STANDARD, SQLSERVER_2017_ENTERPRISE, SQLSERVER_2017_EXPRESS, SQLSERVER_2017_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\nThe dns name of the instance."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<DataSqlDatabaseInstanceIpAddressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_configuration` after provisioning.\nThe configuration for replication."]
    pub fn replica_configuration(&self) -> ListRef<DataSqlDatabaseInstanceReplicaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> ListRef<DataSqlDatabaseInstanceRestoreBackupContextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> ListRef<DataSqlDatabaseInstanceServerCaCertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_email_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\nThe settings to use for the database. The configuration is detailed below."]
    pub fn settings(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for DataSqlDatabaseInstance {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSqlDatabaseInstance { }

impl ToListMappable for DataSqlDatabaseInstance {
    type O = ListRef<DataSqlDatabaseInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSqlDatabaseInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_database_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSqlDatabaseInstance {
    pub tf_id: String,
    #[doc= "The name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub name: PrimField<String>,
}

impl BuildDataSqlDatabaseInstance {
    pub fn build(self, stack: &mut Stack) -> DataSqlDatabaseInstance {
        let out = DataSqlDatabaseInstance(Rc::new(DataSqlDatabaseInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSqlDatabaseInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSqlDatabaseInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSqlDatabaseInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.available_maintenance_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `clone` after provisioning.\nConfiguration for creating a new instance as a clone of another instance."]
    pub fn clone(&self) -> ListRef<DataSqlDatabaseInstanceCloneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server (beta) version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, SQLSERVER_2017_STANDARD, SQLSERVER_2017_ENTERPRISE, SQLSERVER_2017_EXPRESS, SQLSERVER_2017_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\nThe dns name of the instance."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<DataSqlDatabaseInstanceIpAddressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_configuration` after provisioning.\nThe configuration for replication."]
    pub fn replica_configuration(&self) -> ListRef<DataSqlDatabaseInstanceReplicaConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> ListRef<DataSqlDatabaseInstanceRestoreBackupContextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> ListRef<DataSqlDatabaseInstanceServerCaCertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_email_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\nThe settings to use for the database. The configuration is detailed below."]
    pub fn settings(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceCloneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_name: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceCloneEl {
    #[doc= "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `database_names`.\n"]
    pub fn set_database_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.database_names = Some(v.into());
        self
    }

    #[doc= "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_in_time = Some(v.into());
        self
    }

    #[doc= "Set the field `preferred_zone`.\n"]
    pub fn set_preferred_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `source_instance_name`.\n"]
    pub fn set_source_instance_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_instance_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceCloneEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceCloneEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceCloneEl {}

impl BuildDataSqlDatabaseInstanceCloneEl {
    pub fn build(self) -> DataSqlDatabaseInstanceCloneEl {
        DataSqlDatabaseInstanceCloneEl {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_name: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceCloneElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceCloneElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceCloneElRef {
        DataSqlDatabaseInstanceCloneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceCloneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `database_names` after provisioning.\n"]
    pub fn database_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.database_names", self.base))
    }

    #[doc= "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }

    #[doc= "Get a reference to the value of field `preferred_zone` after provisioning.\n"]
    pub fn preferred_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_zone", self.base))
    }

    #[doc= "Get a reference to the value of field `source_instance_name` after provisioning.\n"]
    pub fn source_instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_instance_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceIpAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_retire: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceIpAddressEl {
    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `time_to_retire`.\n"]
    pub fn set_time_to_retire(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_to_retire = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceIpAddressEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceIpAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceIpAddressEl {}

impl BuildDataSqlDatabaseInstanceIpAddressEl {
    pub fn build(self) -> DataSqlDatabaseInstanceIpAddressEl {
        DataSqlDatabaseInstanceIpAddressEl {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceIpAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceIpAddressElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceIpAddressElRef {
        DataSqlDatabaseInstanceIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceIpAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc= "Get a reference to the value of field `time_to_retire` after provisioning.\n"]
    pub fn time_to_retire(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_to_retire", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceReplicaConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_retry_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_file_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_target: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_heartbeat_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_cipher: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_server_certificate: Option<PrimField<bool>>,
}

impl DataSqlDatabaseInstanceReplicaConfigurationEl {
    #[doc= "Set the field `ca_certificate`.\n"]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `connect_retry_interval`.\n"]
    pub fn set_connect_retry_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connect_retry_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `dump_file_path`.\n"]
    pub fn set_dump_file_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dump_file_path = Some(v.into());
        self
    }

    #[doc= "Set the field `failover_target`.\n"]
    pub fn set_failover_target(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover_target = Some(v.into());
        self
    }

    #[doc= "Set the field `master_heartbeat_period`.\n"]
    pub fn set_master_heartbeat_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.master_heartbeat_period = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_cipher`.\n"]
    pub fn set_ssl_cipher(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_cipher = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }

    #[doc= "Set the field `verify_server_certificate`.\n"]
    pub fn set_verify_server_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verify_server_certificate = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceReplicaConfigurationEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceReplicaConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceReplicaConfigurationEl {}

impl BuildDataSqlDatabaseInstanceReplicaConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstanceReplicaConfigurationEl {
        DataSqlDatabaseInstanceReplicaConfigurationEl {
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

pub struct DataSqlDatabaseInstanceReplicaConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceReplicaConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceReplicaConfigurationElRef {
        DataSqlDatabaseInstanceReplicaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceReplicaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ca_certificate` after provisioning.\n"]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc= "Get a reference to the value of field `connect_retry_interval` after provisioning.\n"]
    pub fn connect_retry_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_retry_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `dump_file_path` after provisioning.\n"]
    pub fn dump_file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dump_file_path", self.base))
    }

    #[doc= "Get a reference to the value of field `failover_target` after provisioning.\n"]
    pub fn failover_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover_target", self.base))
    }

    #[doc= "Get a reference to the value of field `master_heartbeat_period` after provisioning.\n"]
    pub fn master_heartbeat_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_heartbeat_period", self.base))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_cipher` after provisioning.\n"]
    pub fn ssl_cipher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_cipher", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc= "Get a reference to the value of field `verify_server_certificate` after provisioning.\n"]
    pub fn verify_server_certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify_server_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceRestoreBackupContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_run_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceRestoreBackupContextEl {
    #[doc= "Set the field `backup_run_id`.\n"]
    pub fn set_backup_run_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.backup_run_id = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceRestoreBackupContextEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceRestoreBackupContextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceRestoreBackupContextEl {}

impl BuildDataSqlDatabaseInstanceRestoreBackupContextEl {
    pub fn build(self) -> DataSqlDatabaseInstanceRestoreBackupContextEl {
        DataSqlDatabaseInstanceRestoreBackupContextEl {
            backup_run_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceRestoreBackupContextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceRestoreBackupContextElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceRestoreBackupContextElRef {
        DataSqlDatabaseInstanceRestoreBackupContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceRestoreBackupContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backup_run_id` after provisioning.\n"]
    pub fn backup_run_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_run_id", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceServerCaCertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_fingerprint: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceServerCaCertEl {
    #[doc= "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }

    #[doc= "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }

    #[doc= "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1_fingerprint = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceServerCaCertEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceServerCaCertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceServerCaCertEl {}

impl BuildDataSqlDatabaseInstanceServerCaCertEl {
    pub fn build(self) -> DataSqlDatabaseInstanceServerCaCertEl {
        DataSqlDatabaseInstanceServerCaCertEl {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceServerCaCertElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceServerCaCertElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceServerCaCertElRef {
        DataSqlDatabaseInstanceServerCaCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceServerCaCertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc= "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc= "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    #[doc= "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {}

impl BuildDataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
        DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl { domain: core::default::Default::default() }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
        DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[doc= "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {}

impl BuildDataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
        DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
        DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retained_backups: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_unit: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc= "Set the field `retained_backups`.\n"]
    pub fn set_retained_backups(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retained_backups = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_unit`.\n"]
    pub fn set_retention_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {}

impl BuildDataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
        DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
            retained_backups: core::default::Default::default(),
            retention_unit: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
        DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `retained_backups` after provisioning.\n"]
    pub fn retained_backups(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retained_backups", self.base))
    }

    #[doc= "Get a reference to the value of field `retention_unit` after provisioning.\n"]
    pub fn retention_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_settings: Option<
        ListField<DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_log_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_log_retention_days: Option<PrimField<f64>>,
}

impl DataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    #[doc= "Set the field `backup_retention_settings`.\n"]
    pub fn set_backup_retention_settings(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>>,
    ) -> Self {
        self.backup_retention_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `binary_log_enabled`.\n"]
    pub fn set_binary_log_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.binary_log_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc= "Set the field `point_in_time_recovery_enabled`.\n"]
    pub fn set_point_in_time_recovery_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.point_in_time_recovery_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }

    #[doc= "Set the field `transaction_log_retention_days`.\n"]
    pub fn set_transaction_log_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.transaction_log_retention_days = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElBackupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElBackupConfigurationEl {}

impl BuildDataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
        DataSqlDatabaseInstanceSettingsElBackupConfigurationEl {
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

pub struct DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef {
        DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backup_retention_settings` after provisioning.\n"]
    pub fn backup_retention_settings(
        &self,
    ) -> ListRef<DataSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_retention_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `binary_log_enabled` after provisioning.\n"]
    pub fn binary_log_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.binary_log_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery_enabled` after provisioning.\n"]
    pub fn point_in_time_recovery_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc= "Get a reference to the value of field `transaction_log_retention_days` after provisioning.\n"]
    pub fn transaction_log_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transaction_log_retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_enabled: Option<PrimField<bool>>,
}

impl DataSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[doc= "Set the field `data_cache_enabled`.\n"]
    pub fn set_data_cache_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_cache_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElDataCacheConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElDataCacheConfigEl {}

impl BuildDataSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElDataCacheConfigEl {
        DataSqlDatabaseInstanceSettingsElDataCacheConfigEl { data_cache_enabled: core::default::Default::default() }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef {
        DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data_cache_enabled` after provisioning.\n"]
    pub fn data_cache_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_cache_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {}

impl BuildDataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
        DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
        DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    #[doc= "Set the field `end_date`.\n"]
    pub fn set_end_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_date = Some(v.into());
        self
    }

    #[doc= "Set the field `start_date`.\n"]
    pub fn set_start_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_date = Some(v.into());
        self
    }

    #[doc= "Set the field `time`.\n"]
    pub fn set_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {}

impl BuildDataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
        DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
        DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc= "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc= "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElInsightsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_insights_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_plans_per_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_application_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_client_address: Option<PrimField<bool>>,
}

impl DataSqlDatabaseInstanceSettingsElInsightsConfigEl {
    #[doc= "Set the field `query_insights_enabled`.\n"]
    pub fn set_query_insights_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.query_insights_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `query_plans_per_minute`.\n"]
    pub fn set_query_plans_per_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_plans_per_minute = Some(v.into());
        self
    }

    #[doc= "Set the field `query_string_length`.\n"]
    pub fn set_query_string_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_string_length = Some(v.into());
        self
    }

    #[doc= "Set the field `record_application_tags`.\n"]
    pub fn set_record_application_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_application_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `record_client_address`.\n"]
    pub fn set_record_client_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_client_address = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElInsightsConfigEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElInsightsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElInsightsConfigEl {}

impl BuildDataSqlDatabaseInstanceSettingsElInsightsConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElInsightsConfigEl {
        DataSqlDatabaseInstanceSettingsElInsightsConfigEl {
            query_insights_enabled: core::default::Default::default(),
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElInsightsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElInsightsConfigElRef {
        DataSqlDatabaseInstanceSettingsElInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `query_insights_enabled` after provisioning.\n"]
    pub fn query_insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_insights_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `query_plans_per_minute` after provisioning.\n"]
    pub fn query_plans_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc= "Get a reference to the value of field `query_string_length` after provisioning.\n"]
    pub fn query_string_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc= "Get a reference to the value of field `record_application_tags` after provisioning.\n"]
    pub fn record_application_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc= "Get a reference to the value of field `record_client_address` after provisioning.\n"]
    pub fn record_client_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[doc= "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {}

impl BuildDataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
        DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
        DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_consumer_projects: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_enabled: Option<PrimField<bool>>,
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[doc= "Set the field `allowed_consumer_projects`.\n"]
    pub fn set_allowed_consumer_projects(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_consumer_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `psc_enabled`.\n"]
    pub fn set_psc_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.psc_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {}

impl BuildDataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
        DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
        DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\n"]
    pub fn allowed_consumer_projects(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc= "Get a reference to the value of field `psc_enabled` after provisioning.\n"]
    pub fn psc_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_networks: Option<SetField<DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_path_for_google_cloud_services: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<SetField<DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[doc= "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `authorized_networks`.\n"]
    pub fn set_authorized_networks(
        mut self,
        v: impl Into<SetField<DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    ) -> Self {
        self.authorized_networks = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_private_path_for_google_cloud_services`.\n"]
    pub fn set_enable_private_path_for_google_cloud_services(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_path_for_google_cloud_services = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv4_enabled`.\n"]
    pub fn set_ipv4_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv4_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `private_network`.\n"]
    pub fn set_private_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_network = Some(v.into());
        self
    }

    #[doc= "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<SetField<DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    ) -> Self {
        self.psc_config = Some(v.into());
        self
    }

    #[doc= "Set the field `require_ssl`.\n"]
    pub fn set_require_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElIpConfigurationEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElIpConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElIpConfigurationEl {}

impl BuildDataSqlDatabaseInstanceSettingsElIpConfigurationEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElIpConfigurationEl {
        DataSqlDatabaseInstanceSettingsElIpConfigurationEl {
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

pub struct DataSqlDatabaseInstanceSettingsElIpConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElIpConfigurationElRef {
        DataSqlDatabaseInstanceSettingsElIpConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `authorized_networks` after provisioning.\n"]
    pub fn authorized_networks(
        &self,
    ) -> SetRef<DataSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef> {
        SetRef::new(self.shared().clone(), format!("{}.authorized_networks", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_private_path_for_google_cloud_services` after provisioning.\n"]
    pub fn enable_private_path_for_google_cloud_services(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_private_path_for_google_cloud_services", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv4_enabled` after provisioning.\n"]
    pub fn ipv4_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `private_network` after provisioning.\n"]
    pub fn private_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_network", self.base))
    }

    #[doc= "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> SetRef<DataSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef> {
        SetRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }

    #[doc= "Get a reference to the value of field `require_ssl` after provisioning.\n"]
    pub fn require_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_gae_application: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[doc= "Set the field `follow_gae_application`.\n"]
    pub fn set_follow_gae_application(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.follow_gae_application = Some(v.into());
        self
    }

    #[doc= "Set the field `secondary_zone`.\n"]
    pub fn set_secondary_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElLocationPreferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElLocationPreferenceEl {}

impl BuildDataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
        DataSqlDatabaseInstanceSettingsElLocationPreferenceEl {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef {
        DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `follow_gae_application` after provisioning.\n"]
    pub fn follow_gae_application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow_gae_application", self.base))
    }

    #[doc= "Get a reference to the value of field `secondary_zone` after provisioning.\n"]
    pub fn secondary_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secondary_zone", self.base))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_track: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[doc= "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }

    #[doc= "Set the field `hour`.\n"]
    pub fn set_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour = Some(v.into());
        self
    }

    #[doc= "Set the field `update_track`.\n"]
    pub fn set_update_track(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_track = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {}

impl BuildDataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
        DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
        DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `hour` after provisioning.\n"]
    pub fn hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc= "Get a reference to the value of field `update_track` after provisioning.\n"]
    pub fn update_track(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_track", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    complexity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disallow_username_substring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_password_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_change_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reuse_interval: Option<PrimField<f64>>,
}

impl DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[doc= "Set the field `complexity`.\n"]
    pub fn set_complexity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.complexity = Some(v.into());
        self
    }

    #[doc= "Set the field `disallow_username_substring`.\n"]
    pub fn set_disallow_username_substring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disallow_username_substring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_password_policy`.\n"]
    pub fn set_enable_password_policy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_password_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `min_length`.\n"]
    pub fn set_min_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_length = Some(v.into());
        self
    }

    #[doc= "Set the field `password_change_interval`.\n"]
    pub fn set_password_change_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_change_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `reuse_interval`.\n"]
    pub fn set_reuse_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.reuse_interval = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {}

impl BuildDataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
        DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: core::default::Default::default(),
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
        DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `complexity` after provisioning.\n"]
    pub fn complexity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.complexity", self.base))
    }

    #[doc= "Get a reference to the value of field `disallow_username_substring` after provisioning.\n"]
    pub fn disallow_username_substring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disallow_username_substring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_password_policy` after provisioning.\n"]
    pub fn enable_password_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_password_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `min_length` after provisioning.\n"]
    pub fn min_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }

    #[doc= "Get a reference to the value of field `password_change_interval` after provisioning.\n"]
    pub fn password_change_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_change_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `reuse_interval` after provisioning.\n"]
    pub fn reuse_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.reuse_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_interval: Option<PrimField<String>>,
}

impl DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[doc= "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_interval`.\n"]
    pub fn set_retention_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `upload_interval`.\n"]
    pub fn set_upload_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_interval = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {}

impl BuildDataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
        DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}

pub struct DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
        DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc= "Get a reference to the value of field `retention_interval` after provisioning.\n"]
    pub fn retention_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `upload_interval` after provisioning.\n"]
    pub fn upload_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSqlDatabaseInstanceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_config: Option<ListField<DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<ListField<DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_configuration: Option<ListField<DataSqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_enforcement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_config: Option<ListField<DataSqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_flags: Option<SetField<DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_maintenance_period: Option<ListField<DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_autoresize: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_autoresize_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insights_config: Option<ListField<DataSqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configuration: Option<ListField<DataSqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_preference: Option<ListField<DataSqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<ListField<DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_validation_policy: Option<ListField<DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pricing_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_audit_config: Option<ListField<DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
}

impl DataSqlDatabaseInstanceSettingsEl {
    #[doc= "Set the field `activation_policy`.\n"]
    pub fn set_activation_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.activation_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `active_directory_config`.\n"]
    pub fn set_active_directory_config(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    ) -> Self {
        self.active_directory_config = Some(v.into());
        self
    }

    #[doc= "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        self.advanced_machine_features = Some(v.into());
        self
    }

    #[doc= "Set the field `availability_type`.\n"]
    pub fn set_availability_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_type = Some(v.into());
        self
    }

    #[doc= "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    ) -> Self {
        self.backup_configuration = Some(v.into());
        self
    }

    #[doc= "Set the field `collation`.\n"]
    pub fn set_collation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collation = Some(v.into());
        self
    }

    #[doc= "Set the field `connector_enforcement`.\n"]
    pub fn set_connector_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connector_enforcement = Some(v.into());
        self
    }

    #[doc= "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    ) -> Self {
        self.data_cache_config = Some(v.into());
        self
    }

    #[doc= "Set the field `database_flags`.\n"]
    pub fn set_database_flags(
        mut self,
        v: impl Into<SetField<DataSqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    ) -> Self {
        self.database_flags = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    ) -> Self {
        self.deny_maintenance_period = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_autoresize`.\n"]
    pub fn set_disk_autoresize(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disk_autoresize = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_autoresize_limit`.\n"]
    pub fn set_disk_autoresize_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_autoresize_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size`.\n"]
    pub fn set_disk_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `edition`.\n"]
    pub fn set_edition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.edition = Some(v.into());
        self
    }

    #[doc= "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    ) -> Self {
        self.insights_config = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_configuration`.\n"]
    pub fn set_ip_configuration(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    ) -> Self {
        self.ip_configuration = Some(v.into());
        self
    }

    #[doc= "Set the field `location_preference`.\n"]
    pub fn set_location_preference(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    ) -> Self {
        self.location_preference = Some(v.into());
        self
    }

    #[doc= "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    ) -> Self {
        self.maintenance_window = Some(v.into());
        self
    }

    #[doc= "Set the field `password_validation_policy`.\n"]
    pub fn set_password_validation_policy(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    ) -> Self {
        self.password_validation_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `pricing_plan`.\n"]
    pub fn set_pricing_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pricing_plan = Some(v.into());
        self
    }

    #[doc= "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v: impl Into<ListField<DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    ) -> Self {
        self.sql_server_audit_config = Some(v.into());
        self
    }

    #[doc= "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier = Some(v.into());
        self
    }

    #[doc= "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `user_labels`.\n"]
    pub fn set_user_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.user_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataSqlDatabaseInstanceSettingsEl {
    type O = BlockAssignable<DataSqlDatabaseInstanceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSqlDatabaseInstanceSettingsEl {}

impl BuildDataSqlDatabaseInstanceSettingsEl {
    pub fn build(self) -> DataSqlDatabaseInstanceSettingsEl {
        DataSqlDatabaseInstanceSettingsEl {
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

pub struct DataSqlDatabaseInstanceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseInstanceSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabaseInstanceSettingsElRef {
        DataSqlDatabaseInstanceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSqlDatabaseInstanceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `activation_policy` after provisioning.\n"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.active_directory_config", self.base))
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc= "Get a reference to the value of field `availability_type` after provisioning.\n"]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.base))
    }

    #[doc= "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElBackupConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `collation` after provisioning.\n"]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc= "Get a reference to the value of field `connector_enforcement` after provisioning.\n"]
    pub fn connector_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_enforcement", self.base))
    }

    #[doc= "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElDataCacheConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc= "Get a reference to the value of field `database_flags` after provisioning.\n"]
    pub fn database_flags(&self) -> SetRef<DataSqlDatabaseInstanceSettingsElDatabaseFlagsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.database_flags", self.base))
    }

    #[doc= "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_autoresize` after provisioning.\n"]
    pub fn disk_autoresize(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_autoresize_limit` after provisioning.\n"]
    pub fn disk_autoresize_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize_limit", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }

    #[doc= "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElInsightsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.insights_config", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElIpConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElLocationPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.location_preference", self.base))
    }

    #[doc= "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc= "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.password_validation_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `pricing_plan` after provisioning.\n"]
    pub fn pricing_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }

    #[doc= "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(&self) -> ListRef<DataSqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sql_server_audit_config", self.base))
    }

    #[doc= "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc= "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc= "Get a reference to the value of field `user_labels` after provisioning.\n"]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
