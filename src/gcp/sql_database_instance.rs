use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SqlDatabaseInstanceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    database_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone: std::option::Option<std::vec::Vec<SqlDatabaseInstanceCloneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstanceReplicaConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_context: std::option::Option<std::vec::Vec<SqlDatabaseInstanceRestoreBackupContextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SqlDatabaseInstanceTimeoutsEl>,
    dynamic: SqlDatabaseInstanceDynamic,
}

struct SqlDatabaseInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SqlDatabaseInstanceData>,
}

#[derive(Clone)]
pub struct SqlDatabaseInstance(std::rc::Rc<SqlDatabaseInstance_>);

impl SqlDatabaseInstance {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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
        "Set the field `deletion_protection`.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_name`.\n"]
    pub fn set_encryption_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_type`.\nThe type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'."]
    pub fn set_instance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_version`.\nMaintenance version."]
    pub fn set_maintenance_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `master_instance_name`.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn set_master_instance_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().master_instance_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password`.\nInitial root password. Required for MS SQL Server."]
    pub fn set_root_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().root_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clone`.\n"]
    pub fn set_clone(self, v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceCloneEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().clone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.clone = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replica_configuration`.\n"]
    pub fn set_replica_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceReplicaConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replica_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replica_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_backup_context`.\n"]
    pub fn set_restore_backup_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceRestoreBackupContextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_backup_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_backup_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SqlDatabaseInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server (beta) version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, SQLSERVER_2017_STANDARD, SQLSERVER_2017_ENTERPRISE, SQLSERVER_2017_EXPRESS, SQLSERVER_2017_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe dns name of the instance."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<SqlDatabaseInstanceIpAddressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<SqlDatabaseInstanceServerCaCertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<SqlDatabaseInstanceCloneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> terrars::ListRef<SqlDatabaseInstanceReplicaConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> terrars::ListRef<SqlDatabaseInstanceRestoreBackupContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SqlDatabaseInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SqlDatabaseInstance { }

impl terrars::ToListMappable for SqlDatabaseInstance {
    type O = terrars::ListRef<SqlDatabaseInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SqlDatabaseInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_sql_database_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSqlDatabaseInstance {
    pub tf_id: std::string::String,
    #[doc =
        "The MySQL, PostgreSQL or SQL Server (beta) version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, SQLSERVER_2017_STANDARD, SQLSERVER_2017_ENTERPRISE, SQLSERVER_2017_EXPRESS, SQLSERVER_2017_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub database_version: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> SqlDatabaseInstance {
        let out = SqlDatabaseInstance(std::rc::Rc::new(SqlDatabaseInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SqlDatabaseInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database_version: self.database_version,
                deletion_protection: core::default::Default::default(),
                encryption_key_name: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                master_instance_name: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                root_password: core::default::Default::default(),
                clone: core::default::Default::default(),
                replica_configuration: core::default::Default::default(),
                restore_backup_context: core::default::Default::default(),
                settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SqlDatabaseInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SqlDatabaseInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nAvailable Maintenance versions."]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server (beta) version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, SQLSERVER_2017_STANDARD, SQLSERVER_2017_ENTERPRISE, SQLSERVER_2017_EXPRESS, SQLSERVER_2017_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe dns name of the instance."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `first_ip_address` after provisioning.\nThe first IPv4 address of any type assigned. This is to support accessing the first address in the list in a terraform output when the resource is configured with a count."]
    pub fn first_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<SqlDatabaseInstanceIpAddressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nMaintenance version."]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_instance_name` after provisioning.\nThe name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups."]
    pub fn master_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. If the name is left blank, Terraform will randomly generate one when the instance is first created. This is done because after a name is used, it cannot be reused for up to one week."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment_link` after provisioning.\nThe link to service attachment of PSC instance."]
    pub fn psc_service_attachment_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nIPv4 address assigned. This is a workaround for an issue fixed in Terraform 0.12 but also provides a convenient way to access an IP of a specific type without performing filtering in a Terraform config."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region the instance will sit in. Note, Cloud SQL is not available in all regions. A valid region must be provided to use this resource. If a region is not provided in the resource definition, the provider region will be used instead, but this will be an apply-time error for instances if the provider region is not supported with Cloud SQL. If you choose not to provide the region argument for this resource, make sure you understand this."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<SqlDatabaseInstanceServerCaCertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email_address` after provisioning.\nThe service account email address assigned to the instance."]
    pub fn service_account_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<SqlDatabaseInstanceCloneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> terrars::ListRef<SqlDatabaseInstanceReplicaConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> terrars::ListRef<SqlDatabaseInstanceRestoreBackupContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceIpAddressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_to_retire: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceIpAddressEl {
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

impl terrars::ToListMappable for SqlDatabaseInstanceIpAddressEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceIpAddressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceIpAddressEl {}

impl BuildSqlDatabaseInstanceIpAddressEl {
    pub fn build(self) -> SqlDatabaseInstanceIpAddressEl {
        SqlDatabaseInstanceIpAddressEl {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceIpAddressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceIpAddressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceIpAddressElRef {
        SqlDatabaseInstanceIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceIpAddressElRef {
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
pub struct SqlDatabaseInstanceServerCaCertEl {
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

impl SqlDatabaseInstanceServerCaCertEl {
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

impl terrars::ToListMappable for SqlDatabaseInstanceServerCaCertEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceServerCaCertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceServerCaCertEl {}

impl BuildSqlDatabaseInstanceServerCaCertEl {
    pub fn build(self) -> SqlDatabaseInstanceServerCaCertEl {
        SqlDatabaseInstanceServerCaCertEl {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceServerCaCertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceServerCaCertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceServerCaCertElRef {
        SqlDatabaseInstanceServerCaCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceServerCaCertElRef {
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
pub struct SqlDatabaseInstanceCloneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    source_instance_name: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstanceCloneEl {
    #[doc =
        "Set the field `allocated_ip_range`.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_names`.\n(SQL Server only, use with point_in_time) clone only the specified databases from the source instance. Clone all databases if empty."]
    pub fn set_database_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.database_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time`.\nThe timestamp of the point in time that should be restored."]
    pub fn set_point_in_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_in_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preferred_zone`.\n(Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance."]
    pub fn set_preferred_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preferred_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceCloneEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceCloneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceCloneEl {
    #[doc = "The name of the instance from which the point in time should be restored."]
    pub source_instance_name: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceCloneEl {
    pub fn build(self) -> SqlDatabaseInstanceCloneEl {
        SqlDatabaseInstanceCloneEl {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_name: self.source_instance_name,
        }
    }
}

pub struct SqlDatabaseInstanceCloneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceCloneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceCloneElRef {
        SqlDatabaseInstanceCloneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceCloneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_names` after provisioning.\n(SQL Server only, use with point_in_time) clone only the specified databases from the source instance. Clone all databases if empty."]
    pub fn database_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_in_time` after provisioning.\nThe timestamp of the point in time that should be restored."]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preferred_zone` after provisioning.\n(Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance."]
    pub fn preferred_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_instance_name` after provisioning.\nThe name of the instance from which the point in time should be restored."]
    pub fn source_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceReplicaConfigurationEl {
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

impl SqlDatabaseInstanceReplicaConfigurationEl {
    #[doc = "Set the field `ca_certificate`.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate`.\nPEM representation of the replica's x509 certificate."]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_key`.\nPEM representation of the replica's private key. The corresponding public key in encoded in the client_certificate."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connect_retry_interval`.\nThe number of seconds between connect retries. MySQL's default is 60 seconds."]
    pub fn set_connect_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connect_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dump_file_path`.\nPath to a SQL file in Google Cloud Storage from which replica instances are created. Format is gs://bucket/filename."]
    pub fn set_dump_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dump_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failover_target`.\nSpecifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. If the master instance fails, the replica instance will be promoted as the new master instance. Not supported for Postgres"]
    pub fn set_failover_target(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.failover_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_heartbeat_period`.\nTime in ms between replication heartbeats."]
    pub fn set_master_heartbeat_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.master_heartbeat_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nPassword for the replication connection."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_cipher`.\nPermissible ciphers for use in SSL encryption."]
    pub fn set_ssl_cipher(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_cipher = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nUsername for replication connection."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `verify_server_certificate`.\nTrue if the master's common name value is checked during the SSL handshake."]
    pub fn set_verify_server_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify_server_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceReplicaConfigurationEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceReplicaConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceReplicaConfigurationEl {}

impl BuildSqlDatabaseInstanceReplicaConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceReplicaConfigurationEl {
        SqlDatabaseInstanceReplicaConfigurationEl {
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

pub struct SqlDatabaseInstanceReplicaConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceReplicaConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceReplicaConfigurationElRef {
        SqlDatabaseInstanceReplicaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceReplicaConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nPEM representation of the replica's x509 certificate."]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nPEM representation of the replica's private key. The corresponding public key in encoded in the client_certificate."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connect_retry_interval` after provisioning.\nThe number of seconds between connect retries. MySQL's default is 60 seconds."]
    pub fn connect_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_retry_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dump_file_path` after provisioning.\nPath to a SQL file in Google Cloud Storage from which replica instances are created. Format is gs://bucket/filename."]
    pub fn dump_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_file_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failover_target` after provisioning.\nSpecifies if the replica is the failover target. If the field is set to true the replica will be designated as a failover replica. If the master instance fails, the replica instance will be promoted as the new master instance. Not supported for Postgres"]
    pub fn failover_target(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `master_heartbeat_period` after provisioning.\nTime in ms between replication heartbeats."]
    pub fn master_heartbeat_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_heartbeat_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the replication connection."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_cipher` after provisioning.\nPermissible ciphers for use in SSL encryption."]
    pub fn ssl_cipher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_cipher", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for replication connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verify_server_certificate` after provisioning.\nTrue if the master's common name value is checked during the SSL handshake."]
    pub fn verify_server_certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_server_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceRestoreBackupContextEl {
    backup_run_id: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceRestoreBackupContextEl {
    #[doc = "Set the field `instance_id`.\nThe ID of the instance that the backup was taken from."]
    pub fn set_instance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe full project ID of the source instance."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceRestoreBackupContextEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceRestoreBackupContextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceRestoreBackupContextEl {
    #[doc = "The ID of the backup run to restore from."]
    pub backup_run_id: terrars::PrimField<f64>,
}

impl BuildSqlDatabaseInstanceRestoreBackupContextEl {
    pub fn build(self) -> SqlDatabaseInstanceRestoreBackupContextEl {
        SqlDatabaseInstanceRestoreBackupContextEl {
            backup_run_id: self.backup_run_id,
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceRestoreBackupContextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceRestoreBackupContextElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceRestoreBackupContextElRef {
        SqlDatabaseInstanceRestoreBackupContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceRestoreBackupContextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_run_id` after provisioning.\nThe ID of the backup run to restore from."]
    pub fn backup_run_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_run_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the instance that the backup was taken from."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe full project ID of the source instance."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    domain: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl { }

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    #[doc = "Domain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl {
        SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl { domain: self.domain }
    }
}

pub struct SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
        SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nDomain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `threads_per_core`.\nThe number of threads per physical core. Can be 1 or 2."]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {}

impl BuildSqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
        SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
        SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. Can be 1 or 2."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    retained_backups: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc = "Set the field `retention_unit`.\nThe unit that 'retainedBackups' represents. Defaults to COUNT"]
    pub fn set_retention_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    #[doc = "Number of backups to retain."]
    pub retained_backups: terrars::PrimField<f64>,
}

impl BuildSqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
        SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl {
            retained_backups: self.retained_backups,
            retention_unit: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
        SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retained_backups` after provisioning.\nNumber of backups to retain."]
    pub fn retained_backups(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retained_backups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_unit` after provisioning.\nThe unit that 'retainedBackups' represents. Defaults to COUNT"]
    pub fn retention_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElBackupConfigurationElDynamic {
    backup_retention_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElBackupConfigurationEl {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_settings: std
    ::option
    ::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>>,
    dynamic: SqlDatabaseInstanceSettingsElBackupConfigurationElDynamic,
}

impl SqlDatabaseInstanceSettingsElBackupConfigurationEl {
    #[doc =
        "Set the field `binary_log_enabled`.\nTrue if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL."]
    pub fn set_binary_log_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.binary_log_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nTrue if backup configuration is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nLocation of the backup configuration."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time_recovery_enabled`.\nTrue if Point-in-time recovery is enabled."]
    pub fn set_point_in_time_recovery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.point_in_time_recovery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\nHH:MM format time indicating when backup configuration starts."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transaction_log_retention_days`.\nThe number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)"]
    pub fn set_transaction_log_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transaction_log_retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_settings`.\n"]
    pub fn set_backup_retention_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_retention_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_retention_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElBackupConfigurationEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElBackupConfigurationEl {}

impl BuildSqlDatabaseInstanceSettingsElBackupConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElBackupConfigurationEl {
        SqlDatabaseInstanceSettingsElBackupConfigurationEl {
            binary_log_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            location: core::default::Default::default(),
            point_in_time_recovery_enabled: core::default::Default::default(),
            start_time: core::default::Default::default(),
            transaction_log_retention_days: core::default::Default::default(),
            backup_retention_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
        SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElBackupConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `binary_log_enabled` after provisioning.\nTrue if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL."]
    pub fn binary_log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_log_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nTrue if backup configuration is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the backup configuration."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_in_time_recovery_enabled` after provisioning.\nTrue if Point-in-time recovery is enabled."]
    pub fn point_in_time_recovery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nHH:MM format time indicating when backup configuration starts."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transaction_log_retention_days` after provisioning.\nThe number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)"]
    pub fn transaction_log_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transaction_log_retention_days", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_retention_settings` after provisioning.\n"]
    pub fn backup_retention_settings(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstanceSettingsElBackupConfigurationElBackupRetentionSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_retention_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    #[doc = "Set the field `data_cache_enabled`.\nWhether data cache is enabled for the instance."]
    pub fn set_data_cache_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_cache_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElDataCacheConfigEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDataCacheConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElDataCacheConfigEl {}

impl BuildSqlDatabaseInstanceSettingsElDataCacheConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDataCacheConfigEl {
        SqlDatabaseInstanceSettingsElDataCacheConfigEl { data_cache_enabled: core::default::Default::default() }
    }
}

pub struct SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
        SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElDataCacheConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_cache_enabled` after provisioning.\nWhether data cache is enabled for the instance."]
    pub fn data_cache_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_cache_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstanceSettingsElDatabaseFlagsEl { }

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    #[doc = "Name of the flag."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Value of the flag."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceSettingsElDatabaseFlagsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
        SqlDatabaseInstanceSettingsElDatabaseFlagsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
        SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElDatabaseFlagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the flag."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the flag."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    end_date: terrars::PrimField<std::string::String>,
    start_date: terrars::PrimField<std::string::String>,
    time: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl { }

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    #[doc =
        "End date before which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub end_date: terrars::PrimField<std::string::String>,
    #[doc =
        "Start date after which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub start_date: terrars::PrimField<std::string::String>,
    #[doc =
        "Time in UTC when the \"deny maintenance period\" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00"]
    pub time: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
        SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl {
            end_date: self.end_date,
            start_date: self.start_date,
            time: self.time,
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
        SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_date` after provisioning.\nEnd date before which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_date` after provisioning.\nStart date after which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time` after provisioning.\nTime in UTC when the \"deny maintenance period\" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00"]
    pub fn time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElInsightsConfigEl {
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

impl SqlDatabaseInstanceSettingsElInsightsConfigEl {
    #[doc = "Set the field `query_insights_enabled`.\nTrue if Query Insights feature is enabled."]
    pub fn set_query_insights_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.query_insights_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5."]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_length`.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024."]
    pub fn set_query_string_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_string_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_application_tags`.\nTrue if Query Insights will record application tags from query when enabled."]
    pub fn set_record_application_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_application_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_client_address`.\nTrue if Query Insights will record client address when enabled."]
    pub fn set_record_client_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_client_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElInsightsConfigEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElInsightsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElInsightsConfigEl {}

impl BuildSqlDatabaseInstanceSettingsElInsightsConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElInsightsConfigEl {
        SqlDatabaseInstanceSettingsElInsightsConfigEl {
            query_insights_enabled: core::default::Default::default(),
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElInsightsConfigElRef {
        SqlDatabaseInstanceSettingsElInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElInsightsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query_insights_enabled` after provisioning.\nTrue if Query Insights feature is enabled."]
    pub fn query_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_insights_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5."]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_length` after provisioning.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024."]
    pub fn query_string_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_application_tags` after provisioning.\nTrue if Query Insights will record application tags from query when enabled."]
    pub fn record_application_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_client_address` after provisioning.\nTrue if Query Insights will record client address when enabled."]
    pub fn record_client_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
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
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
        SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksElRef {
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
pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    #[doc =
        "Set the field `allowed_consumer_projects`.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_enabled`.\nWhether PSC connectivity is enabled for this instance."]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {}

impl BuildSqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn allowed_consumer_projects(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_enabled` after provisioning.\nWhether PSC connectivity is enabled for this instance."]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElIpConfigurationElDynamic {
    authorized_networks: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    psc_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_path_for_google_cloud_services: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_networks: std
    ::option
    ::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    dynamic: SqlDatabaseInstanceSettingsElIpConfigurationElDynamic,
}

impl SqlDatabaseInstanceSettingsElIpConfigurationEl {
    #[doc =
        "Set the field `allocated_ip_range`.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_path_for_google_cloud_services`.\nWhether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported."]
    pub fn set_enable_private_path_for_google_cloud_services(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_private_path_for_google_cloud_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_enabled`.\nWhether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured."]
    pub fn set_ipv4_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv4_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_network`.\nThe VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set."]
    pub fn set_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_ssl`.\nWhether SSL connections over IP are enforced or not. To change this field, also set the corresponding value in ssl_mode if it has been set too."]
    pub fn set_require_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_mode`.\nSpecify how SSL connection should be enforced in DB connections. This field provides more SSL enforcment options compared to require_ssl. To change this field, also set the correspoding value in require_ssl."]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElAuthorizedNetworksEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorized_networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_networks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationElPscConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElIpConfigurationEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElIpConfigurationEl {}

impl BuildSqlDatabaseInstanceSettingsElIpConfigurationEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElIpConfigurationEl {
        SqlDatabaseInstanceSettingsElIpConfigurationEl {
            allocated_ip_range: core::default::Default::default(),
            enable_private_path_for_google_cloud_services: core::default::Default::default(),
            ipv4_enabled: core::default::Default::default(),
            private_network: core::default::Default::default(),
            require_ssl: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElIpConfigurationElRef {
        SqlDatabaseInstanceSettingsElIpConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElIpConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_path_for_google_cloud_services` after provisioning.\nWhether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported."]
    pub fn enable_private_path_for_google_cloud_services(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_path_for_google_cloud_services", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ipv4_enabled` after provisioning.\nWhether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured."]
    pub fn ipv4_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_network` after provisioning.\nThe VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set."]
    pub fn private_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_ssl` after provisioning.\nWhether SSL connections over IP are enforced or not. To change this field, also set the corresponding value in ssl_mode if it has been set too."]
    pub fn require_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSpecify how SSL connection should be enforced in DB connections. This field provides more SSL enforcment options compared to require_ssl. To change this field, also set the correspoding value in require_ssl."]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_gae_application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    #[doc =
        "Set the field `follow_gae_application`.\nA Google App Engine application whose zone to remain in. Must be in the same region as this instance."]
    pub fn set_follow_gae_application(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.follow_gae_application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_zone`.\nThe preferred Compute Engine zone for the secondary/failover"]
    pub fn set_secondary_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe preferred compute engine zone."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElLocationPreferenceEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElLocationPreferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElLocationPreferenceEl {}

impl BuildSqlDatabaseInstanceSettingsElLocationPreferenceEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElLocationPreferenceEl {
        SqlDatabaseInstanceSettingsElLocationPreferenceEl {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
        SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElLocationPreferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `follow_gae_application` after provisioning.\nA Google App Engine application whose zone to remain in. Must be in the same region as this instance."]
    pub fn follow_gae_application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow_gae_application", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_zone` after provisioning.\nThe preferred Compute Engine zone for the secondary/failover"]
    pub fn secondary_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe preferred compute engine zone."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_track: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    #[doc = "Set the field `day`.\nDay of week (1-7), starting on Monday"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hour`.\nHour of day (0-23), ignored if day not set"]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_track`.\nReceive updates earlier (canary) or later (stable)"]
    pub fn set_update_track(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_track = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElMaintenanceWindowEl {}

impl BuildSqlDatabaseInstanceSettingsElMaintenanceWindowEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
        SqlDatabaseInstanceSettingsElMaintenanceWindowEl {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
        SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of week (1-7), starting on Monday"]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hour` after provisioning.\nHour of day (0-23), ignored if day not set"]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_track` after provisioning.\nReceive updates earlier (canary) or later (stable)"]
    pub fn update_track(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_track", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    complexity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallow_username_substring: std::option::Option<terrars::PrimField<bool>>,
    enable_password_policy: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_change_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[doc = "Set the field `complexity`.\nPassword complexity."]
    pub fn set_complexity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.complexity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disallow_username_substring`.\nDisallow username as a part of the password."]
    pub fn set_disallow_username_substring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disallow_username_substring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_length`.\nMinimum number of characters allowed."]
    pub fn set_min_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_change_interval`.\nMinimum interval after which the password can be changed. This flag is only supported for PostgresSQL."]
    pub fn set_password_change_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_change_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reuse_interval`.\nNumber of previous passwords that cannot be reused."]
    pub fn set_reuse_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.reuse_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    #[doc = "Whether the password policy is enabled or not."]
    pub enable_password_policy: terrars::PrimField<bool>,
}

impl BuildSqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
        SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: self.enable_password_policy,
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
        SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `complexity` after provisioning.\nPassword complexity."]
    pub fn complexity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complexity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disallow_username_substring` after provisioning.\nDisallow username as a part of the password."]
    pub fn disallow_username_substring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disallow_username_substring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_password_policy` after provisioning.\nWhether the password policy is enabled or not."]
    pub fn enable_password_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_password_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_length` after provisioning.\nMinimum number of characters allowed."]
    pub fn min_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_change_interval` after provisioning.\nMinimum interval after which the password can be changed. This flag is only supported for PostgresSQL."]
    pub fn password_change_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_change_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reuse_interval` after provisioning.\nNumber of previous passwords that cannot be reused."]
    pub fn reuse_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    #[doc = "Set the field `bucket`.\nThe name of the destination bucket (e.g., gs://mybucket)."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_interval`.\nHow long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".."]
    pub fn set_retention_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.retention_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `upload_interval`.\nHow often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_upload_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {}

impl BuildSqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
        SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
        SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nThe name of the destination bucket (e.g., gs://mybucket)."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_interval` after provisioning.\nHow long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".."]
    pub fn retention_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload_interval` after provisioning.\nHow often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn upload_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstanceSettingsElDynamic {
    active_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    data_cache_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    database_flags: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    deny_maintenance_period: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    insights_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    ip_configuration: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    location_preference: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    password_validation_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    sql_server_audit_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
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
    pricing_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_config: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insights_config: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_preference: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_validation_policy: std
    ::option
    ::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_audit_config: std::option::Option<std::vec::Vec<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    dynamic: SqlDatabaseInstanceSettingsElDynamic,
}

impl SqlDatabaseInstanceSettingsEl {
    #[doc =
        "Set the field `activation_policy`.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn set_activation_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.activation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `availability_type`.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL."]
    pub fn set_availability_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collation`.\nThe name of server instance collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_enforcement`.\nSpecifies if connections must use Cloud SQL connectors."]
    pub fn set_connector_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_enforcement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection_enabled`.\nConfiguration to protect against accidental instance deletion."]
    pub fn set_deletion_protection_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_autoresize`.\nEnables auto-resizing of the storage size. Defaults to true."]
    pub fn set_disk_autoresize(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disk_autoresize = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_autoresize_limit`.\nThe maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn set_disk_autoresize_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_autoresize_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size`.\nThe size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB."]
    pub fn set_disk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nThe type of data disk: PD_SSD or PD_HDD. Defaults to PD_SSD."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edition`.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn set_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pricing_plan`.\nPricing plan for this instance, can only be PER_USE."]
    pub fn set_pricing_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pricing_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nThe time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_labels`.\nA set of key/value user label pairs to assign to the instance."]
    pub fn set_user_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `active_directory_config`.\n"]
    pub fn set_active_directory_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElActiveDirectoryConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.active_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.active_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_machine_features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElBackupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDataCacheConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cache_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_flags`.\n"]
    pub fn set_database_flags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDatabaseFlagsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_flags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_flags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deny_maintenance_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deny_maintenance_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElInsightsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.insights_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.insights_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_configuration`.\n"]
    pub fn set_ip_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElIpConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `location_preference`.\n"]
    pub fn set_location_preference(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElLocationPreferenceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location_preference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location_preference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_validation_policy`.\n"]
    pub fn set_password_validation_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElPasswordValidationPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_validation_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_validation_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstanceSettingsElSqlServerAuditConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_server_audit_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_audit_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceSettingsEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceSettingsEl {
    #[doc =
        "The machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types."]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstanceSettingsEl {
    pub fn build(self) -> SqlDatabaseInstanceSettingsEl {
        SqlDatabaseInstanceSettingsEl {
            activation_policy: core::default::Default::default(),
            availability_type: core::default::Default::default(),
            collation: core::default::Default::default(),
            connector_enforcement: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            disk_autoresize: core::default::Default::default(),
            disk_autoresize_limit: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            edition: core::default::Default::default(),
            pricing_plan: core::default::Default::default(),
            tier: self.tier,
            time_zone: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            active_directory_config: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            backup_configuration: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            deny_maintenance_period: core::default::Default::default(),
            insights_config: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            location_preference: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            password_validation_policy: core::default::Default::default(),
            sql_server_audit_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceSettingsElRef {
        SqlDatabaseInstanceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL."]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `collation` after provisioning.\nThe name of server instance collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connector_enforcement` after provisioning.\nSpecifies if connections must use Cloud SQL connectors."]
    pub fn connector_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_enforcement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nConfiguration to protect against accidental instance deletion."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_autoresize` after provisioning.\nEnables auto-resizing of the storage size. Defaults to true."]
    pub fn disk_autoresize(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_autoresize_limit` after provisioning.\nThe maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn disk_autoresize_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_autoresize_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size` after provisioning.\nThe size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB."]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nThe type of data disk: PD_SSD or PD_HDD. Defaults to PD_SSD."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edition` after provisioning.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pricing_plan` after provisioning.\nPricing plan for this instance, can only be PER_USE."]
    pub fn pricing_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types."]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nA set of key/value user label pairs to assign to the instance."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nUsed to make sure changes to the settings block are atomic."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElActiveDirectoryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstanceSettingsElAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElBackupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElDataCacheConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElDenyMaintenancePeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.base))
    }

    #[doc = "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElInsightsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElIpConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElLocationPreferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstanceSettingsElPasswordValidationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_validation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(&self) -> terrars::ListRef<SqlDatabaseInstanceSettingsElSqlServerAuditConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_audit_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstanceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<SqlDatabaseInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstanceTimeoutsEl {}

impl BuildSqlDatabaseInstanceTimeoutsEl {
    pub fn build(self) -> SqlDatabaseInstanceTimeoutsEl {
        SqlDatabaseInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SqlDatabaseInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstanceTimeoutsElRef {
        SqlDatabaseInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstanceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstanceDynamic {
    clone: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceCloneEl>>,
    replica_configuration: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceReplicaConfigurationEl>>,
    restore_backup_context: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceRestoreBackupContextEl>>,
    settings: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstanceSettingsEl>>,
}
