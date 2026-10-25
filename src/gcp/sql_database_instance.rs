#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SqlDatabaseInstanceData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backupdr_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    database_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_confidential_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_new_sql_network_architecture: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_replicas_for_major_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_transaction_logs_to_cloud_storage_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Clone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_restore_context: std::option::Option<std::vec::Vec<SqlDatabaseInstance_PointInTimeRestoreContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstance_ReplicaConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_cluster: std::option::Option<std::vec::Vec<SqlDatabaseInstance_ReplicationCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_context: std::option::Option<std::vec::Vec<SqlDatabaseInstance_RestoreBackupContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SqlDatabaseInstance_Timeouts>,
    dynamic: SqlDatabaseInstanceDynamic,
}

struct SqlDatabaseInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SqlDatabaseInstanceData_>,
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

    #[doc = "Set the field `backupdr_backup`.\nThe name of the BackupDR backup to restore from."]
    pub fn set_backupdr_backup(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backupdr_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_confidential_mode`.\nEnables Confidential Mode on Hyperdisk storage for enhanced security. Only supported on Zonal C4A PG and MySQL instances."]
    pub fn set_encryption_confidential_mode(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encryption_confidential_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_name`.\n"]
    pub fn set_encryption_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_new_sql_network_architecture`.\nWhether to enforce the new SQL network architecture."]
    pub fn set_enforce_new_sql_network_architecture(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enforce_new_sql_network_architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `final_backup_description`.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn set_final_backup_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().final_backup_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_replicas_for_major_version_upgrade`.\nWhen this parameter is set to true, Cloud SQL instances can perform in-place major version upgrades of read replicas along with the primary instance when 'database_version' is updated. This is an input-only field that is not persisted in the API and only takes effect during a major version upgrade."]
    pub fn set_include_replicas_for_major_version_upgrade(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().include_replicas_for_major_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_type`.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
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
        "Set the field `node_count`.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn set_node_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `replica_names`.\nThe replicas of the instance."]
    pub fn set_replica_names(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().replica_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password`.\nInitial root password. Required for MS SQL Server."]
    pub fn set_root_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().root_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `root_password_wo`.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_root_password_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().root_password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `root_password_wo_version`.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_root_password_wo_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().root_password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `switch_transaction_logs_to_cloud_storage_enabled`.\nWhen set to true, Cloud SQL instances can switch storing point-in-time recovery transaction logs from a data disk to Cloud Storage, freeing up data disk space and enabling longer retention windows. This is an input-only field that is not persisted in the API."]
    pub fn set_switch_transaction_logs_to_cloud_storage_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().switch_transaction_logs_to_cloud_storage_enabled =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clone`.\n"]
    pub fn set_clone(self, v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Clone>>) -> Self {
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

    #[doc = "Set the field `point_in_time_restore_context`.\n"]
    pub fn set_point_in_time_restore_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_PointInTimeRestoreContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().point_in_time_restore_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.point_in_time_restore_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replica_configuration`.\n"]
    pub fn set_replica_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_ReplicaConfiguration>>,
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

    #[doc = "Set the field `replication_cluster`.\n"]
    pub fn set_replication_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_ReplicationCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_backup_context`.\n"]
    pub fn set_restore_backup_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_RestoreBackupContext>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<SqlDatabaseInstance_Timeouts>) -> Self {
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
        "Get a reference to the value of field `backupdr_backup` after provisioning.\nThe name of the BackupDR backup to restore from."]
    pub fn backupdr_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backupdr_backup", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe instance-level dns name of the instance for PSC instances or public IP CAS instances."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_names` after provisioning.\nThe list of DNS names used by this instance. Different connection types for an instance may have different DNS names. DNS names can apply to an individual instance or a cluster of instances."]
    pub fn dns_names(&self) -> terrars::ListRef<SqlDatabaseInstance_DnsNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_confidential_mode` after provisioning.\nEnables Confidential Mode on Hyperdisk storage for enhanced security. Only supported on Zonal C4A PG and MySQL instances."]
    pub fn encryption_confidential_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_confidential_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforce_new_sql_network_architecture` after provisioning.\nWhether to enforce the new SQL network architecture."]
    pub fn enforce_new_sql_network_architecture(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_new_sql_network_architecture", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `final_backup_description` after provisioning.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn final_backup_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_backup_description", self.extract_ref()))
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
        "Get a reference to the value of field `include_replicas_for_major_version_upgrade` after provisioning.\nWhen this parameter is set to true, Cloud SQL instances can perform in-place major version upgrades of read replicas along with the primary instance when 'database_version' is updated. This is an input-only field that is not persisted in the API and only takes effect during a major version upgrade."]
    pub fn include_replicas_for_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_replicas_for_major_version_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<SqlDatabaseInstance_IpAddressRef> {
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
        "Get a reference to the value of field `node_count` after provisioning.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `replica_names` after provisioning.\nThe replicas of the instance."]
    pub fn replica_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password_wo` after provisioning.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password_wo_version` after provisioning.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<SqlDatabaseInstance_ServerCaCertRef> {
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

    #[doc =
        "Get a reference to the value of field `switch_transaction_logs_to_cloud_storage_enabled` after provisioning.\nWhen set to true, Cloud SQL instances can switch storing point-in-time recovery transaction logs from a data disk to Cloud Storage, freeing up data disk space and enabling longer retention windows. This is an input-only field that is not persisted in the API."]
    pub fn switch_transaction_logs_to_cloud_storage_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.switch_transaction_logs_to_cloud_storage_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<SqlDatabaseInstance_CloneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_restore_context` after provisioning.\n"]
    pub fn point_in_time_restore_context(&self) -> terrars::ListRef<SqlDatabaseInstance_PointInTimeRestoreContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_restore_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> terrars::ListRef<SqlDatabaseInstance_ReplicaConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_cluster` after provisioning.\n"]
    pub fn replication_cluster(&self) -> terrars::ListRef<SqlDatabaseInstance_ReplicationClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> terrars::ListRef<SqlDatabaseInstance_RestoreBackupContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<SqlDatabaseInstance_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstance_TimeoutsRef {
        SqlDatabaseInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
        "The MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub database_version: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> SqlDatabaseInstance {
        let out = SqlDatabaseInstance(std::rc::Rc::new(SqlDatabaseInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SqlDatabaseInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backupdr_backup: core::default::Default::default(),
                database_version: self.database_version,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                encryption_confidential_mode: core::default::Default::default(),
                encryption_key_name: core::default::Default::default(),
                enforce_new_sql_network_architecture: core::default::Default::default(),
                final_backup_description: core::default::Default::default(),
                id: core::default::Default::default(),
                include_replicas_for_major_version_upgrade: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                master_instance_name: core::default::Default::default(),
                name: core::default::Default::default(),
                node_count: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_names: core::default::Default::default(),
                root_password: core::default::Default::default(),
                root_password_wo: core::default::Default::default(),
                root_password_wo_version: core::default::Default::default(),
                switch_transaction_logs_to_cloud_storage_enabled: core::default::Default::default(),
                clone: core::default::Default::default(),
                point_in_time_restore_context: core::default::Default::default(),
                replica_configuration: core::default::Default::default(),
                replication_cluster: core::default::Default::default(),
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
        "Get a reference to the value of field `backupdr_backup` after provisioning.\nThe name of the BackupDR backup to restore from."]
    pub fn backupdr_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backupdr_backup", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_name` after provisioning.\nThe connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy."]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe MySQL, PostgreSQL or SQL Server version to use. Supported values include MYSQL_5_6, MYSQL_5_7, MYSQL_8_0, MYSQL_8_4, POSTGRES_9_6, POSTGRES_10, POSTGRES_11, POSTGRES_12, POSTGRES_13, POSTGRES_14, POSTGRES_15, POSTGRES_16, POSTGRES_17, POSTGRES_18, SQLSERVER_2022_STANDARD, SQLSERVER_2022_ENTERPRISE, SQLSERVER_2022_EXPRESS, SQLSERVER_2022_WEB, SQLSERVER_2025_STANDARD, SQLSERVER_2025_ENTERPRISE, SQLSERVER_2025_EXPRESS, SQLSERVER_2025_WEB. Database Version Policies includes an up-to-date reference of supported versions."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nUsed to block Terraform from deleting a SQL Instance. Defaults to true."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe instance-level dns name of the instance for PSC instances or public IP CAS instances."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_names` after provisioning.\nThe list of DNS names used by this instance. Different connection types for an instance may have different DNS names. DNS names can apply to an individual instance or a cluster of instances."]
    pub fn dns_names(&self) -> terrars::ListRef<SqlDatabaseInstance_DnsNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_confidential_mode` after provisioning.\nEnables Confidential Mode on Hyperdisk storage for enhanced security. Only supported on Zonal C4A PG and MySQL instances."]
    pub fn encryption_confidential_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_confidential_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_name` after provisioning.\n"]
    pub fn encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforce_new_sql_network_architecture` after provisioning.\nWhether to enforce the new SQL network architecture."]
    pub fn enforce_new_sql_network_architecture(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_new_sql_network_architecture", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `final_backup_description` after provisioning.\nThe description of final backup if instance enable create final backup during instance deletion. "]
    pub fn final_backup_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_backup_description", self.extract_ref()))
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
        "Get a reference to the value of field `include_replicas_for_major_version_upgrade` after provisioning.\nWhen this parameter is set to true, Cloud SQL instances can perform in-place major version upgrades of read replicas along with the primary instance when 'database_version' is updated. This is an input-only field that is not persisted in the API and only takes effect during a major version upgrade."]
    pub fn include_replicas_for_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_replicas_for_major_version_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance. See https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/instances#SqlInstanceType for supported values."]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<SqlDatabaseInstance_IpAddressRef> {
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
        "Get a reference to the value of field `node_count` after provisioning.\nFor a read pool instance, the number of nodes in the read pool. For read pools with auto scaling enabled, this field is read only."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `replica_names` after provisioning.\nThe replicas of the instance."]
    pub fn replica_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password` after provisioning.\nInitial root password. Required for MS SQL Server."]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password_wo` after provisioning.\nInitial root password. Required for MS SQL Server.\n\t\t\t\tNote: This property is write-only and will not be read from the API. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_password_wo_version` after provisioning.\nTriggers update of root_password_wo write-only. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn root_password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_ca_cert` after provisioning.\n"]
    pub fn server_ca_cert(&self) -> terrars::ListRef<SqlDatabaseInstance_ServerCaCertRef> {
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

    #[doc =
        "Get a reference to the value of field `switch_transaction_logs_to_cloud_storage_enabled` after provisioning.\nWhen set to true, Cloud SQL instances can switch storing point-in-time recovery transaction logs from a data disk to Cloud Storage, freeing up data disk space and enabling longer retention windows. This is an input-only field that is not persisted in the API."]
    pub fn switch_transaction_logs_to_cloud_storage_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.switch_transaction_logs_to_cloud_storage_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `clone` after provisioning.\n"]
    pub fn clone(&self) -> terrars::ListRef<SqlDatabaseInstance_CloneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_restore_context` after provisioning.\n"]
    pub fn point_in_time_restore_context(&self) -> terrars::ListRef<SqlDatabaseInstance_PointInTimeRestoreContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_restore_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_configuration` after provisioning.\n"]
    pub fn replica_configuration(&self) -> terrars::ListRef<SqlDatabaseInstance_ReplicaConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_cluster` after provisioning.\n"]
    pub fn replication_cluster(&self) -> terrars::ListRef<SqlDatabaseInstance_ReplicationClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_context` after provisioning.\n"]
    pub fn restore_backup_context(&self) -> terrars::ListRef<SqlDatabaseInstance_RestoreBackupContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<SqlDatabaseInstance_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlDatabaseInstance_TimeoutsRef {
        SqlDatabaseInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstanceDynamic {
    clone: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Clone>>,
    point_in_time_restore_context: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_PointInTimeRestoreContext>>,
    replica_configuration: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_ReplicaConfiguration>>,
    replication_cluster: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_ReplicationCluster>>,
    restore_backup_context: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_RestoreBackupContext>>,
    settings: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_DnsNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_DnsNames {
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

impl terrars::ToListMappable for SqlDatabaseInstance_DnsNames {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_DnsNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_DnsNames {}

impl BuildSqlDatabaseInstance_DnsNames {
    pub fn build(self) -> SqlDatabaseInstance_DnsNames {
        SqlDatabaseInstance_DnsNames {
            connection_type: core::default::Default::default(),
            dns_scope: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_DnsNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_DnsNamesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_DnsNamesRef {
        SqlDatabaseInstance_DnsNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_DnsNamesRef {
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
pub struct SqlDatabaseInstance_IpAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_to_retire: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_IpAddress {
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

impl terrars::ToListMappable for SqlDatabaseInstance_IpAddress {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_IpAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_IpAddress {}

impl BuildSqlDatabaseInstance_IpAddress {
    pub fn build(self) -> SqlDatabaseInstance_IpAddress {
        SqlDatabaseInstance_IpAddress {
            ip_address: core::default::Default::default(),
            time_to_retire: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_IpAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_IpAddressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_IpAddressRef {
        SqlDatabaseInstance_IpAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_IpAddressRef {
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
pub struct SqlDatabaseInstance_ServerCaCert {
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

impl SqlDatabaseInstance_ServerCaCert {
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

impl terrars::ToListMappable for SqlDatabaseInstance_ServerCaCert {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_ServerCaCert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_ServerCaCert {}

impl BuildSqlDatabaseInstance_ServerCaCert {
    pub fn build(self) -> SqlDatabaseInstance_ServerCaCert {
        SqlDatabaseInstance_ServerCaCert {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_ServerCaCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_ServerCaCertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_ServerCaCertRef {
        SqlDatabaseInstance_ServerCaCertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_ServerCaCertRef {
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
pub struct SqlDatabaseInstance_Clone {
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
    source_instance_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Clone {
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

    #[doc =
        "Set the field `source_instance_deletion_time`.\nThe timestamp of when the source instance was deleted for a clone from a deleted instance."]
    pub fn set_source_instance_deletion_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_instance_deletion_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_project`.\nThe project ID of the source project"]
    pub fn set_source_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Clone {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Clone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Clone {
    #[doc = "The name of the instance from which the point in time should be restored."]
    pub source_instance_name: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Clone {
    pub fn build(self) -> SqlDatabaseInstance_Clone {
        SqlDatabaseInstance_Clone {
            allocated_ip_range: core::default::Default::default(),
            database_names: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            source_instance_deletion_time: core::default::Default::default(),
            source_instance_name: self.source_instance_name,
            source_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_CloneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_CloneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_CloneRef {
        SqlDatabaseInstance_CloneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_CloneRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `source_instance_deletion_time` after provisioning.\nThe timestamp of when the source instance was deleted for a clone from a deleted instance."]
    pub fn source_instance_deletion_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_deletion_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_instance_name` after provisioning.\nThe name of the instance from which the point in time should be restored."]
    pub fn source_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_project` after provisioning.\nThe project ID of the source project"]
    pub fn source_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_PointInTimeRestoreContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    datasource: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_PointInTimeRestoreContext {
    #[doc =
        "Set the field `allocated_ip_range`.\nThe name of the allocated IP range for the internal IP Cloud SQL instance. For example: \"google-managed-services-default\". If you set this, then Cloud SQL creates the IP address for the cloned instance in the allocated range. This range must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035) standards. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time`.\nThe date and time to which you want to restore the instance."]
    pub fn set_point_in_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_in_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preferred_zone`.\nPoint-in-time recovery of an instance to the specified zone. If no zone is specified, then clone to the same primary zone as the source instance."]
    pub fn set_preferred_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preferred_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the target instance to restore to."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_instance`.\nThe name of the target instance to restore to."]
    pub fn set_target_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_PointInTimeRestoreContext {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_PointInTimeRestoreContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_PointInTimeRestoreContext {
    #[doc =
        "The Google Cloud Backup and Disaster Recovery Datasource URI. For example: \"projects/my-project/locations/us-central1/datasources/my-datasource\"."]
    pub datasource: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_PointInTimeRestoreContext {
    pub fn build(self) -> SqlDatabaseInstance_PointInTimeRestoreContext {
        SqlDatabaseInstance_PointInTimeRestoreContext {
            allocated_ip_range: core::default::Default::default(),
            datasource: self.datasource,
            point_in_time: core::default::Default::default(),
            preferred_zone: core::default::Default::default(),
            region: core::default::Default::default(),
            target_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_PointInTimeRestoreContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_PointInTimeRestoreContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_PointInTimeRestoreContextRef {
        SqlDatabaseInstance_PointInTimeRestoreContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_PointInTimeRestoreContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated IP range for the internal IP Cloud SQL instance. For example: \"google-managed-services-default\". If you set this, then Cloud SQL creates the IP address for the cloned instance in the allocated range. This range must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035) standards. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datasource` after provisioning.\nThe Google Cloud Backup and Disaster Recovery Datasource URI. For example: \"projects/my-project/locations/us-central1/datasources/my-datasource\"."]
    pub fn datasource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datasource", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_in_time` after provisioning.\nThe date and time to which you want to restore the instance."]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preferred_zone` after provisioning.\nPoint-in-time recovery of an instance to the specified zone. If no zone is specified, then clone to the same primary zone as the source instance."]
    pub fn preferred_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the target instance to restore to."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_instance` after provisioning.\nThe name of the target instance to restore to."]
    pub fn target_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_ReplicaConfiguration {
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

impl SqlDatabaseInstance_ReplicaConfiguration {
    #[doc = "Set the field `ca_certificate`.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cascadable_replica`.\nSpecifies if a SQL Server replica is a cascadable replica. A cascadable replica is a SQL Server cross region replica that supports replica(s) under it."]
    pub fn set_cascadable_replica(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cascadable_replica = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for SqlDatabaseInstance_ReplicaConfiguration {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_ReplicaConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_ReplicaConfiguration {}

impl BuildSqlDatabaseInstance_ReplicaConfiguration {
    pub fn build(self) -> SqlDatabaseInstance_ReplicaConfiguration {
        SqlDatabaseInstance_ReplicaConfiguration {
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
pub struct SqlDatabaseInstance_ReplicaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_ReplicaConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_ReplicaConfigurationRef {
        SqlDatabaseInstance_ReplicaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_ReplicaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM representation of the trusted CA's x509 certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cascadable_replica` after provisioning.\nSpecifies if a SQL Server replica is a cascadable replica. A cascadable replica is a SQL Server cross region replica that supports replica(s) under it."]
    pub fn cascadable_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cascadable_replica", self.base))
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
pub struct SqlDatabaseInstance_ReplicationCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_dr_replica_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psa_write_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_ReplicationCluster {
    #[doc =
        "Set the field `failover_dr_replica_name`.\nIf the instance is a primary instance, then this field identifies the disaster recovery (DR) replica. The standard format of this field is \"your-project:your-instance\". You can also set this field to \"your-instance\", but cloud SQL backend will convert it to the aforementioned standard format."]
    pub fn set_failover_dr_replica_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.failover_dr_replica_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psa_write_endpoint`.\nIf set, this field indicates this instance has a private service access (PSA) DNS endpoint that is pointing to the primary instance of the cluster. If this instance is the primary, then the DNS endpoint points to this instance. After a switchover or replica failover operation, this DNS endpoint points to the promoted instance. This is a read-only field, returned to the user as information. This field can exist even if a standalone instance doesn't have a DR replica yet or the DR replica is deleted."]
    pub fn set_psa_write_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.psa_write_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_ReplicationCluster {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_ReplicationCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_ReplicationCluster {}

impl BuildSqlDatabaseInstance_ReplicationCluster {
    pub fn build(self) -> SqlDatabaseInstance_ReplicationCluster {
        SqlDatabaseInstance_ReplicationCluster {
            failover_dr_replica_name: core::default::Default::default(),
            psa_write_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_ReplicationClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_ReplicationClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_ReplicationClusterRef {
        SqlDatabaseInstance_ReplicationClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_ReplicationClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dr_replica` after provisioning.\nRead-only field that indicates whether the replica is a DR replica."]
    pub fn dr_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dr_replica", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failover_dr_replica_name` after provisioning.\nIf the instance is a primary instance, then this field identifies the disaster recovery (DR) replica. The standard format of this field is \"your-project:your-instance\". You can also set this field to \"your-instance\", but cloud SQL backend will convert it to the aforementioned standard format."]
    pub fn failover_dr_replica_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_dr_replica_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psa_write_endpoint` after provisioning.\nIf set, this field indicates this instance has a private service access (PSA) DNS endpoint that is pointing to the primary instance of the cluster. If this instance is the primary, then the DNS endpoint points to this instance. After a switchover or replica failover operation, this DNS endpoint points to the promoted instance. This is a read-only field, returned to the user as information. This field can exist even if a standalone instance doesn't have a DR replica yet or the DR replica is deleted."]
    pub fn psa_write_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psa_write_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_RestoreBackupContext {
    backup_run_id: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_RestoreBackupContext {
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

impl terrars::ToListMappable for SqlDatabaseInstance_RestoreBackupContext {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_RestoreBackupContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_RestoreBackupContext {
    #[doc = "The ID of the backup run to restore from."]
    pub backup_run_id: terrars::PrimField<f64>,
}

impl BuildSqlDatabaseInstance_RestoreBackupContext {
    pub fn build(self) -> SqlDatabaseInstance_RestoreBackupContext {
        SqlDatabaseInstance_RestoreBackupContext {
            backup_run_id: self.backup_run_id,
            instance_id: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_RestoreBackupContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_RestoreBackupContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_RestoreBackupContextRef {
        SqlDatabaseInstance_RestoreBackupContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_RestoreBackupContextRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_ActiveDirectoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_credential_secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_ActiveDirectoryConfig {
    #[doc =
        "Set the field `admin_credential_secret_name`.\nThe secret manager key storing the administrator credential. (e.g., projects/{project}/secrets/{secret})."]
    pub fn set_admin_credential_secret_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.admin_credential_secret_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\nDomain controller IPv4 addresses used to bootstrap Active Directory."]
    pub fn set_dns_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe mode of the Active Directory configuration. Can be MANAGED_ACTIVE_DIRECTORY or CUSTOMER_MANAGED_ACTIVE_DIRECTORY."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organizational_unit`.\nThe organizational unit distinguished name. This is the full hierarchical path to the organizational unit."]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_ActiveDirectoryConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_ActiveDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_ActiveDirectoryConfig {
    #[doc = "Domain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings_ActiveDirectoryConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_ActiveDirectoryConfig {
        SqlDatabaseInstance_Settings_ActiveDirectoryConfig {
            admin_credential_secret_name: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            domain: self.domain,
            mode: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef {
        SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_credential_secret_name` after provisioning.\nThe secret manager key storing the administrator credential. (e.g., projects/{project}/secrets/{secret})."]
    pub fn admin_credential_secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_credential_secret_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_servers` after provisioning.\nDomain controller IPv4 addresses used to bootstrap Active Directory."]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nDomain name of the Active Directory for SQL Server (e.g., mydomain.com)."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of the Active Directory configuration. Can be MANAGED_ACTIVE_DIRECTORY or CUSTOMER_MANAGED_ACTIVE_DIRECTORY."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organizational_unit` after provisioning.\nThe organizational unit distinguished name. This is the full hierarchical path to the organizational unit."]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl SqlDatabaseInstance_Settings_AdvancedMachineFeatures {
    #[doc = "Set the field `threads_per_core`.\nThe number of threads per physical core. Can be 1 or 2."]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_AdvancedMachineFeatures {}

impl BuildSqlDatabaseInstance_Settings_AdvancedMachineFeatures {
    pub fn build(self) -> SqlDatabaseInstance_Settings_AdvancedMachineFeatures {
        SqlDatabaseInstance_Settings_AdvancedMachineFeatures { threads_per_core: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef {
        SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
    retained_backups: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
    #[doc = "Set the field `retention_unit`.\nThe unit that 'retainedBackups' represents. Defaults to COUNT"]
    pub fn set_retention_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
    #[doc = "Number of backups to retain."]
    pub retained_backups: terrars::PrimField<f64>,
}

impl BuildSqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
    pub fn build(self) -> SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
        SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings {
            retained_backups: self.retained_backups,
            retention_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef {
        SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef {
    #[allow(dead_code)]
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
struct SqlDatabaseInstance_Settings_BackupConfigurationDynamic {
    backup_retention_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_BackupConfiguration {
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
    ::Option<std::vec::Vec<SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings>>,
    dynamic: SqlDatabaseInstance_Settings_BackupConfigurationDynamic,
}

impl SqlDatabaseInstance_Settings_BackupConfiguration {
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
                        ::BlockAssignable<SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettings>,
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_BackupConfiguration {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_BackupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_BackupConfiguration {}

impl BuildSqlDatabaseInstance_Settings_BackupConfiguration {
    pub fn build(self) -> SqlDatabaseInstance_Settings_BackupConfiguration {
        SqlDatabaseInstance_Settings_BackupConfiguration {
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

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_BackupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_BackupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_BackupConfigurationRef {
        SqlDatabaseInstance_Settings_BackupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_BackupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_tier` after provisioning.\nBackup tier that manages the backups for the instance."]
    pub fn backup_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_tier", self.base))
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
    ) -> terrars::ListRef<SqlDatabaseInstance_Settings_BackupConfiguration_BackupRetentionSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_retention_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags { }

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
    #[doc = "Name of the flag."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Value of the flag."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
    pub fn build(self) -> SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
        SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_ConnectionPoolConfig_FlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_ConnectionPoolConfig_FlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_ConnectionPoolConfig_FlagsRef {
        SqlDatabaseInstance_Settings_ConnectionPoolConfig_FlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_ConnectionPoolConfig_FlagsRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct SqlDatabaseInstance_Settings_ConnectionPoolConfigDynamic {
    flags: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_ConnectionPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pooling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flags: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags>>,
    dynamic: SqlDatabaseInstance_Settings_ConnectionPoolConfigDynamic,
}

impl SqlDatabaseInstance_Settings_ConnectionPoolConfig {
    #[doc =
        "Set the field `connection_pooling_enabled`.\nWhether Managed Connection Pool is enabled for this instance."]
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
                    ::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_ConnectionPoolConfig_Flags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.flags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.flags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_ConnectionPoolConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_ConnectionPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_ConnectionPoolConfig {}

impl BuildSqlDatabaseInstance_Settings_ConnectionPoolConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_ConnectionPoolConfig {
        SqlDatabaseInstance_Settings_ConnectionPoolConfig {
            connection_pooling_enabled: core::default::Default::default(),
            flags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_ConnectionPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_ConnectionPoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_ConnectionPoolConfigRef {
        SqlDatabaseInstance_Settings_ConnectionPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_ConnectionPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_pooling_enabled` after provisioning.\nWhether Managed Connection Pool is enabled for this instance."]
    pub fn connection_pooling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_pooling_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_DataCacheConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SqlDatabaseInstance_Settings_DataCacheConfig {
    #[doc = "Set the field `data_cache_enabled`.\nWhether data cache is enabled for the instance."]
    pub fn set_data_cache_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_cache_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_DataCacheConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_DataCacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_DataCacheConfig {}

impl BuildSqlDatabaseInstance_Settings_DataCacheConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_DataCacheConfig {
        SqlDatabaseInstance_Settings_DataCacheConfig { data_cache_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_DataCacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_DataCacheConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_DataCacheConfigRef {
        SqlDatabaseInstance_Settings_DataCacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_DataCacheConfigRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_DatabaseFlags {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstance_Settings_DatabaseFlags { }

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_DatabaseFlags {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_DatabaseFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_DatabaseFlags {
    #[doc = "Name of the flag."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Value of the flag."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings_DatabaseFlags {
    pub fn build(self) -> SqlDatabaseInstance_Settings_DatabaseFlags {
        SqlDatabaseInstance_Settings_DatabaseFlags {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_DatabaseFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_DatabaseFlagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_Settings_DatabaseFlagsRef {
        SqlDatabaseInstance_Settings_DatabaseFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_DatabaseFlagsRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_DenyMaintenancePeriod {
    end_date: terrars::PrimField<std::string::String>,
    start_date: terrars::PrimField<std::string::String>,
    time: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstance_Settings_DenyMaintenancePeriod { }

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_DenyMaintenancePeriod {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_DenyMaintenancePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_DenyMaintenancePeriod {
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

impl BuildSqlDatabaseInstance_Settings_DenyMaintenancePeriod {
    pub fn build(self) -> SqlDatabaseInstance_Settings_DenyMaintenancePeriod {
        SqlDatabaseInstance_Settings_DenyMaintenancePeriod {
            end_date: self.end_date,
            start_date: self.start_date,
            time: self.time,
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef {
        SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_EntraidConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_EntraidConfig {
    #[doc = "Set the field `application_id`.\nThe application ID for the Entra ID configuration."]
    pub fn set_application_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nThe tenant ID for the Entra ID configuration."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_EntraidConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_EntraidConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_EntraidConfig {}

impl BuildSqlDatabaseInstance_Settings_EntraidConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_EntraidConfig {
        SqlDatabaseInstance_Settings_EntraidConfig {
            application_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_EntraidConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_EntraidConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_Settings_EntraidConfigRef {
        SqlDatabaseInstance_Settings_EntraidConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_EntraidConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nThe application ID for the Entra ID configuration."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nThe tenant ID for the Entra ID configuration."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_FinalBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_days: std::option::Option<terrars::PrimField<f64>>,
}

impl SqlDatabaseInstance_Settings_FinalBackupConfig {
    #[doc =
        "Set the field `enabled`.\nWhen this parameter is set to true, the final backup is enabled for the instance"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_days`.\nThe number of days to retain the final backup after the instance deletion. The valid range is between 1 and 365. For instances managed by BackupDR, the valid range is between 1 day and 99 years. The final backup will be purged at (time_of_instance_deletion + retention_days)."]
    pub fn set_retention_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_FinalBackupConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_FinalBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_FinalBackupConfig {}

impl BuildSqlDatabaseInstance_Settings_FinalBackupConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_FinalBackupConfig {
        SqlDatabaseInstance_Settings_FinalBackupConfig {
            enabled: core::default::Default::default(),
            retention_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_FinalBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_FinalBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_FinalBackupConfigRef {
        SqlDatabaseInstance_Settings_FinalBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_FinalBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen this parameter is set to true, the final backup is enabled for the instance"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe number of days to retain the final backup after the instance deletion. The valid range is between 1 and 365. For instances managed by BackupDR, the valid range is between 1 day and 99 years. The final backup will be purged at (time_of_instance_deletion + retention_days)."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_InsightsConfig {
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

impl SqlDatabaseInstance_Settings_InsightsConfig {
    #[doc = "Set the field `enhanced_query_insights_enabled`.\nTrue if Enhanced Query Insights feature is enabled."]
    pub fn set_enhanced_query_insights_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enhanced_query_insights_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_insights_enabled`.\nTrue if Query Insights feature is enabled."]
    pub fn set_query_insights_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.query_insights_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5. For Enterprise Plus instances, from 0 to 200."]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_length`.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024. For Enterprise Plus instances, from 1024 to 100000."]
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_InsightsConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_InsightsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_InsightsConfig {}

impl BuildSqlDatabaseInstance_Settings_InsightsConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_InsightsConfig {
        SqlDatabaseInstance_Settings_InsightsConfig {
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
pub struct SqlDatabaseInstance_Settings_InsightsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_InsightsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_InsightsConfigRef {
        SqlDatabaseInstance_Settings_InsightsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_InsightsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enhanced_query_insights_enabled` after provisioning.\nTrue if Enhanced Query Insights feature is enabled."]
    pub fn enhanced_query_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_query_insights_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_insights_enabled` after provisioning.\nTrue if Query Insights feature is enabled."]
    pub fn query_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_insights_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5. For Enterprise Plus instances, from 0 to 200."]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_length` after provisioning.\nMaximum query length stored in bytes. Between 256 and 4500. Default to 1024. For Enterprise Plus instances, from 1024 to 100000."]
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
pub struct SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
    pub fn build(self) -> SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
        SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks {
            expiration_time: core::default::Default::default(),
            name: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworksRef {
        SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworksRef {
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
pub struct SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    consumer_network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_service_project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    #[doc =
        "Set the field `consumer_service_project_id`.\nThe project ID of consumer service project of this consumer endpoint."]
    pub fn set_consumer_service_project_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_service_project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    #[doc =
        "The consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. The consumer host project of this network might be different from the consumer service project."]
    pub consumer_network: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
    pub fn build(self) -> SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
        SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections {
            consumer_network: self.consumer_network,
            consumer_service_project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
        SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nThe consumer network of this consumer endpoint. This must be a resource path that includes both the host project and the network name. The consumer host project of this network might be different from the consumer service project."]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_network_status` after provisioning.\nThe connection policy status of the consumer network."]
    pub fn consumer_network_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_service_project_id` after provisioning.\nThe project ID of consumer service project of this consumer endpoint."]
    pub fn consumer_service_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_service_project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_auto_dns_status` after provisioning.\nThe status of the automated DNS provisioning for the instance."]
    pub fn instance_auto_dns_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_auto_dns_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the consumer endpoint."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_connection_policy` after provisioning.\nThe service connection policy created for the auto connection."]
    pub fn service_connection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_connection_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_connection_policy_creation_result` after provisioning.\nThe result of the service connection policy creation."]
    pub fn service_connection_policy_creation_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_connection_policy_creation_result", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe connection status of the consumer endpoint."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_endpoint_auto_dns_status` after provisioning.\nThe status of the automated DNS provisioning for the write endpoint."]
    pub fn write_endpoint_auto_dns_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_endpoint_auto_dns_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstance_Settings_IpConfiguration_PscConfigDynamic {
    psc_auto_connections: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connection_policy_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_dns_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_write_endpoint_dns_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connections: std
    ::option
    ::Option<std::vec::Vec<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections>>,
    dynamic: SqlDatabaseInstance_Settings_IpConfiguration_PscConfigDynamic,
}

impl SqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
    #[doc =
        "Set the field `allowed_consumer_projects`.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_attachment_uri`.\nName of network attachment resource used to authorize a producer service to connect a PSC interface to the consumer's VPC. For example: \"projects/myProject/regions/myRegion/networkAttachments/myNetworkAttachment\". This is required to enable outbound connection on a PSC instance."]
    pub fn set_network_attachment_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psc_auto_connection_policy_enabled`.\nWhether a service connection policy is created for the auto connections configured for the instance."]
    pub fn set_psc_auto_connection_policy_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.psc_auto_connection_policy_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_dns_enabled`.\nWhether PSC auto DNS is enabled for this instance."]
    pub fn set_psc_auto_dns_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_auto_dns_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_enabled`.\nWhether PSC connectivity is enabled for this instance."]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psc_write_endpoint_dns_enabled`.\nWhether PSC write endpoint DNS is enabled for this instance."]
    pub fn set_psc_write_endpoint_dns_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_write_endpoint_dns_enabled = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnections>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_auto_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_auto_connections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_IpConfiguration_PscConfig {}

impl BuildSqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
        SqlDatabaseInstance_Settings_IpConfiguration_PscConfig {
            allowed_consumer_projects: core::default::Default::default(),
            network_attachment_uri: core::default::Default::default(),
            psc_auto_connection_policy_enabled: core::default::Default::default(),
            psc_auto_dns_enabled: core::default::Default::default(),
            psc_enabled: core::default::Default::default(),
            psc_write_endpoint_dns_enabled: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_IpConfiguration_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_IpConfiguration_PscConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_IpConfiguration_PscConfigRef {
        SqlDatabaseInstance_Settings_IpConfiguration_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_IpConfiguration_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\nList of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric)."]
    pub fn allowed_consumer_projects(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_attachment_uri` after provisioning.\nName of network attachment resource used to authorize a producer service to connect a PSC interface to the consumer's VPC. For example: \"projects/myProject/regions/myRegion/networkAttachments/myNetworkAttachment\". This is required to enable outbound connection on a PSC instance."]
    pub fn network_attachment_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_connection_policy_enabled` after provisioning.\nWhether a service connection policy is created for the auto connections configured for the instance."]
    pub fn psc_auto_connection_policy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_auto_connection_policy_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_dns_enabled` after provisioning.\nWhether PSC auto DNS is enabled for this instance."]
    pub fn psc_auto_dns_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_auto_dns_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_enabled` after provisioning.\nWhether PSC connectivity is enabled for this instance."]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_write_endpoint_dns_enabled` after provisioning.\nWhether PSC write endpoint DNS is enabled for this instance."]
    pub fn psc_write_endpoint_dns_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_write_endpoint_dns_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstance_Settings_IpConfigurationDynamic {
    authorized_networks: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks>>,
    psc_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_IpConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_subject_alternative_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_path_for_google_cloud_services: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate_rotation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_networks: std
    ::option
    ::Option<std::vec::Vec<SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig>>,
    dynamic: SqlDatabaseInstance_Settings_IpConfigurationDynamic,
}

impl SqlDatabaseInstance_Settings_IpConfiguration {
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
        "Set the field `custom_subject_alternative_names`.\nThe custom subject alternative names for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn set_custom_subject_alternative_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_subject_alternative_names = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `server_ca_mode`.\nSpecify how the server certificate's Certificate Authority is hosted."]
    pub fn set_server_ca_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_ca_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn set_server_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_certificate_rotation_mode`.\nSettings for how the server certificate gets rotated."]
    pub fn set_server_certificate_rotation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_certificate_rotation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\nSpecify how SSL connection should be enforced in DB connections."]
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
                    ::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_AuthorizedNetworks>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration_PscConfig>>,
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_IpConfiguration {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_IpConfiguration {}

impl BuildSqlDatabaseInstance_Settings_IpConfiguration {
    pub fn build(self) -> SqlDatabaseInstance_Settings_IpConfiguration {
        SqlDatabaseInstance_Settings_IpConfiguration {
            allocated_ip_range: core::default::Default::default(),
            custom_subject_alternative_names: core::default::Default::default(),
            enable_private_path_for_google_cloud_services: core::default::Default::default(),
            ipv4_enabled: core::default::Default::default(),
            private_network: core::default::Default::default(),
            server_ca_mode: core::default::Default::default(),
            server_ca_pool: core::default::Default::default(),
            server_certificate_rotation_mode: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_IpConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_IpConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_IpConfigurationRef {
        SqlDatabaseInstance_Settings_IpConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_IpConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated ip range for the private ip CloudSQL instance. For example: \"google-managed-services-default\". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_subject_alternative_names` after provisioning.\nThe custom subject alternative names for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn custom_subject_alternative_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_subject_alternative_names", self.base))
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
        "Get a reference to the value of field `server_ca_mode` after provisioning.\nSpecify how the server certificate's Certificate Authority is hosted."]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with \"CUSTOMER_MANAGED_CAS_CA\" as the \"server_ca_mode\"."]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_certificate_rotation_mode` after provisioning.\nSettings for how the server certificate gets rotated."]
    pub fn server_certificate_rotation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_rotation_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSpecify how SSL connection should be enforced in DB connections."]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_LocationPreference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_gae_application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_LocationPreference {
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_LocationPreference {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_LocationPreference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_LocationPreference {}

impl BuildSqlDatabaseInstance_Settings_LocationPreference {
    pub fn build(self) -> SqlDatabaseInstance_Settings_LocationPreference {
        SqlDatabaseInstance_Settings_LocationPreference {
            follow_gae_application: core::default::Default::default(),
            secondary_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_LocationPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_LocationPreferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_LocationPreferenceRef {
        SqlDatabaseInstance_Settings_LocationPreferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_LocationPreferenceRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_track: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_MaintenanceWindow {
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

    #[doc =
        "Set the field `update_track`.\nReceive updates after one week (canary) or after two weeks (stable) or after five weeks (week5) of notification."]
    pub fn set_update_track(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_track = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_MaintenanceWindow {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_MaintenanceWindow {}

impl BuildSqlDatabaseInstance_Settings_MaintenanceWindow {
    pub fn build(self) -> SqlDatabaseInstance_Settings_MaintenanceWindow {
        SqlDatabaseInstance_Settings_MaintenanceWindow {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            update_track: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_MaintenanceWindowRef {
        SqlDatabaseInstance_Settings_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_MaintenanceWindowRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `update_track` after provisioning.\nReceive updates after one week (canary) or after two weeks (stable) or after five weeks (week5) of notification."]
    pub fn update_track(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_track", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_PasswordValidationPolicy {
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

impl SqlDatabaseInstance_Settings_PasswordValidationPolicy {
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_PasswordValidationPolicy {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_PasswordValidationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_PasswordValidationPolicy {
    #[doc = "Whether the password policy is enabled or not."]
    pub enable_password_policy: terrars::PrimField<bool>,
}

impl BuildSqlDatabaseInstance_Settings_PasswordValidationPolicy {
    pub fn build(self) -> SqlDatabaseInstance_Settings_PasswordValidationPolicy {
        SqlDatabaseInstance_Settings_PasswordValidationPolicy {
            complexity: core::default::Default::default(),
            disallow_username_substring: core::default::Default::default(),
            enable_password_policy: self.enable_password_policy,
            min_length: core::default::Default::default(),
            password_change_interval: core::default::Default::default(),
            reuse_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_PasswordValidationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_PasswordValidationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_PasswordValidationPolicyRef {
        SqlDatabaseInstance_Settings_PasswordValidationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_PasswordValidationPolicyRef {
    #[allow(dead_code)]
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
pub struct SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_value: std::option::Option<terrars::PrimField<f64>>,
}

impl SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    #[doc = "Set the field `metric`.\nMetric name for Read Pool Auto Scale."]
    pub fn set_metric(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\nTarget value for Read Pool Auto Scale."]
    pub fn set_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {}

impl BuildSqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
    pub fn build(self) -> SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
        SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics {
            metric: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
        SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\nMetric name for Read Pool Auto Scale."]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_value` after provisioning.\nTarget value for Read Pool Auto Scale."]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigDynamic {
    target_metrics: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
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
    ::Option<std::vec::Vec<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics>>,
    dynamic: SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigDynamic,
}

impl SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
    #[doc = "Set the field `disable_scale_in`.\nTrue if auto scale in is disabled."]
    pub fn set_disable_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nTrue if Read Pool Auto Scale is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_node_count`.\nMaximum number of nodes in the read pool. If set to lower than current node count, node count will be updated."]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_node_count`.\nMinimum number of nodes in the read pool. If set to higher than current node count, node count will be updated."]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_cooldown_seconds`.\nThe cooldown period for scale in operations."]
    pub fn set_scale_in_cooldown_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in_cooldown_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_cooldown_seconds`.\nThe cooldown period for scale out operations."]
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
                        terrars::BlockAssignable<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig_TargetMetrics>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {}

impl BuildSqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
        SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig {
            disable_scale_in: core::default::Default::default(),
            enabled: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            scale_in_cooldown_seconds: core::default::Default::default(),
            scale_out_cooldown_seconds: core::default::Default::default(),
            target_metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef {
        SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_scale_in` after provisioning.\nTrue if auto scale in is disabled."]
    pub fn disable_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nTrue if Read Pool Auto Scale is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes in the read pool. If set to lower than current node count, node count will be updated."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the read pool. If set to higher than current node count, node count will be updated."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_in_cooldown_seconds` after provisioning.\nThe cooldown period for scale in operations."]
    pub fn scale_in_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out_cooldown_seconds` after provisioning.\nThe cooldown period for scale out operations."]
    pub fn scale_out_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings_SqlServerAuditConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Settings_SqlServerAuditConfig {
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings_SqlServerAuditConfig {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings_SqlServerAuditConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings_SqlServerAuditConfig {}

impl BuildSqlDatabaseInstance_Settings_SqlServerAuditConfig {
    pub fn build(self) -> SqlDatabaseInstance_Settings_SqlServerAuditConfig {
        SqlDatabaseInstance_Settings_SqlServerAuditConfig {
            bucket: core::default::Default::default(),
            retention_interval: core::default::Default::default(),
            upload_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_Settings_SqlServerAuditConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_Settings_SqlServerAuditConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SqlDatabaseInstance_Settings_SqlServerAuditConfigRef {
        SqlDatabaseInstance_Settings_SqlServerAuditConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_Settings_SqlServerAuditConfigRef {
    #[allow(dead_code)]
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
struct SqlDatabaseInstance_SettingsDynamic {
    active_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_ActiveDirectoryConfig>>,
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_AdvancedMachineFeatures>>,
    backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_BackupConfiguration>>,
    connection_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_ConnectionPoolConfig>>,
    data_cache_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_DataCacheConfig>>,
    database_flags: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_DatabaseFlags>>,
    deny_maintenance_period: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_DenyMaintenancePeriod>>,
    entraid_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_EntraidConfig>>,
    final_backup_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_FinalBackupConfig>>,
    insights_config: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_InsightsConfig>>,
    ip_configuration: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_IpConfiguration>>,
    location_preference: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_LocationPreference>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_MaintenanceWindow>>,
    password_validation_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_PasswordValidationPolicy>>,
    read_pool_auto_scale_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig>>,
    sql_server_audit_config: std
    ::option
    ::Option<terrars::DynamicBlock<SqlDatabaseInstance_Settings_SqlServerAuditConfig>>,
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_api_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
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
    enable_dataplex_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_google_ml_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pricing_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_lag_max_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_backups_on_delete: std::option::Option<terrars::PrimField<bool>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_ActiveDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_BackupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_ConnectionPoolConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_DataCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_DatabaseFlags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_DenyMaintenancePeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entraid_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_EntraidConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_FinalBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insights_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_InsightsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_IpConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_preference: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_LocationPreference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_validation_policy: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_PasswordValidationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_pool_auto_scale_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_audit_config: std::option::Option<std::vec::Vec<SqlDatabaseInstance_Settings_SqlServerAuditConfig>>,
    dynamic: SqlDatabaseInstance_SettingsDynamic,
}

impl SqlDatabaseInstance_Settings {
    #[doc =
        "Set the field `activation_policy`.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn set_activation_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.activation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_upgrade_enabled`.\nEnables Automatic Version Upgrade feature. Can be used with MySQL only."]
    pub fn set_auto_upgrade_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `availability_type`.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL.\nFor read pool instances, this field is read-only. The availability type is changed by specifying\nthe number of nodes (node_count)."]
    pub fn set_availability_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collation`.\nThe name of server instance collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connector_enforcement`.\nEnables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected."]
    pub fn set_connector_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_enforcement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_api_access`.\nAllows using ExecuteSql API to connect to the instance. Disallowed when unspecified."]
    pub fn set_data_api_access(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_api_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_disk_provisioned_iops`.\nProvisioned number of I/O operations per second for the data disk. This field is only used for HYPERDISK_BALANCED disk types."]
    pub fn set_data_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_disk_provisioned_throughput`.\nProvisioned throughput measured in MiB per second for the data disk. This field is only used for HYPERDISK_BALANCED disk types."]
    pub fn set_data_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_disk_provisioned_throughput = std::option::Option::Some(v.into());
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
        "Set the field `disk_size`.\nThe size of data disk, in GB. The size of a running instance can be increased, or reduced when disk_autoresize is disabled (this triggers an in-place storage shrink, which restarts the instance). The minimum value is 10GB for PD_SSD, PD_HDD and 20GB for HYPERDISK_BALANCED."]
    pub fn set_disk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nThe type of supported data disk is tier dependent and can be PD_SSD or PD_HDD or HYPERDISK_BALANCED."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edition`.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn set_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dataplex_integration`.\nEnables Dataplex Integration."]
    pub fn set_enable_dataplex_integration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_dataplex_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_google_ml_integration`.\nEnables Vertex AI Integration."]
    pub fn set_enable_google_ml_integration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_google_ml_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pricing_plan`.\nPricing plan for this instance, can only be PER_USE."]
    pub fn set_pricing_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pricing_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replication_lag_max_seconds`.\nThe acceptable replication lag, in seconds, after which a read replica recreates itself. The lag must persist for at least five minutes before recreation is triggered. This is a replica level field, and must be between 300 seconds (five minutes) and 31536000 seconds (one year)."]
    pub fn set_replication_lag_max_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replication_lag_max_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retain_backups_on_delete`.\nWhen this parameter is set to true, Cloud SQL retains backups of the instance even after the instance is deleted. The ON_DEMAND backup will be retained until customer deletes the backup or the project. The AUTOMATED backup will be retained based on the backups retention setting."]
    pub fn set_retain_backups_on_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retain_backups_on_delete = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_ActiveDirectoryConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_AdvancedMachineFeatures>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_BackupConfiguration>>,
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

    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_ConnectionPoolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_DataCacheConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_DatabaseFlags>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_DenyMaintenancePeriod>>,
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

    #[doc = "Set the field `entraid_config`.\n"]
    pub fn set_entraid_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_EntraidConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entraid_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entraid_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `final_backup_config`.\n"]
    pub fn set_final_backup_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_FinalBackupConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.final_backup_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.final_backup_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `insights_config`.\n"]
    pub fn set_insights_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_InsightsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_IpConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_LocationPreference>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_MaintenanceWindow>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_PasswordValidationPolicy>>,
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

    #[doc = "Set the field `read_pool_auto_scale_config`.\n"]
    pub fn set_read_pool_auto_scale_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_pool_auto_scale_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_pool_auto_scale_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_audit_config`.\n"]
    pub fn set_sql_server_audit_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlDatabaseInstance_Settings_SqlServerAuditConfig>>,
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

impl terrars::ToListMappable for SqlDatabaseInstance_Settings {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Settings {
    #[doc =
        "The machine type to use. See tiers for more details and supported versions. Postgres supports only shared-core machine types, and custom machine types such as db-custom-2-13312. See the Custom Machine Type Documentation to learn about specifying custom machine types."]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildSqlDatabaseInstance_Settings {
    pub fn build(self) -> SqlDatabaseInstance_Settings {
        SqlDatabaseInstance_Settings {
            activation_policy: core::default::Default::default(),
            auto_upgrade_enabled: core::default::Default::default(),
            availability_type: core::default::Default::default(),
            collation: core::default::Default::default(),
            connector_enforcement: core::default::Default::default(),
            data_api_access: core::default::Default::default(),
            data_disk_provisioned_iops: core::default::Default::default(),
            data_disk_provisioned_throughput: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            disk_autoresize: core::default::Default::default(),
            disk_autoresize_limit: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            edition: core::default::Default::default(),
            enable_dataplex_integration: core::default::Default::default(),
            enable_google_ml_integration: core::default::Default::default(),
            pricing_plan: core::default::Default::default(),
            replication_lag_max_seconds: core::default::Default::default(),
            retain_backups_on_delete: core::default::Default::default(),
            tier: self.tier,
            time_zone: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            active_directory_config: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            backup_configuration: core::default::Default::default(),
            connection_pool_config: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            deny_maintenance_period: core::default::Default::default(),
            entraid_config: core::default::Default::default(),
            final_backup_config: core::default::Default::default(),
            insights_config: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            location_preference: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            password_validation_policy: core::default::Default::default(),
            read_pool_auto_scale_config: core::default::Default::default(),
            sql_server_audit_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_SettingsRef {
        SqlDatabaseInstance_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\nThis specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND."]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_upgrade_enabled` after provisioning.\nEnables Automatic Version Upgrade feature. Can be used with MySQL only."]
    pub fn auto_upgrade_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). For all instances, ensure that\nsettings.backup_configuration.enabled is set to true.\nFor MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.\nFor Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled\nis set to true. Defaults to ZONAL.\nFor read pool instances, this field is read-only. The availability type is changed by specifying\nthe number of nodes (node_count)."]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `collation` after provisioning.\nThe name of server instance collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connector_enforcement` after provisioning.\nEnables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected."]
    pub fn connector_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_enforcement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_api_access` after provisioning.\nAllows using ExecuteSql API to connect to the instance. Disallowed when unspecified."]
    pub fn data_api_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_api_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_disk_provisioned_iops` after provisioning.\nProvisioned number of I/O operations per second for the data disk. This field is only used for HYPERDISK_BALANCED disk types."]
    pub fn data_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_disk_provisioned_throughput` after provisioning.\nProvisioned throughput measured in MiB per second for the data disk. This field is only used for HYPERDISK_BALANCED disk types."]
    pub fn data_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_provisioned_throughput", self.base))
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
        "Get a reference to the value of field `disk_size` after provisioning.\nThe size of data disk, in GB. The size of a running instance can be increased, or reduced when disk_autoresize is disabled (this triggers an in-place storage shrink, which restarts the instance). The minimum value is 10GB for PD_SSD, PD_HDD and 20GB for HYPERDISK_BALANCED."]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nThe type of supported data disk is tier dependent and can be PD_SSD or PD_HDD or HYPERDISK_BALANCED."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edition` after provisioning.\nThe edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS."]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_availability_type` after provisioning.\nThe availability type of the Cloud SQL instance, high availability\n(REGIONAL) or single zone (ZONAL). This field always contains the value that is reported by the\nAPI (for read pools, effective_availability_type may differ from availability_type)."]
    pub fn effective_availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_availability_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_dataplex_integration` after provisioning.\nEnables Dataplex Integration."]
    pub fn enable_dataplex_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dataplex_integration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_google_ml_integration` after provisioning.\nEnables Vertex AI Integration."]
    pub fn enable_google_ml_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_google_ml_integration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pricing_plan` after provisioning.\nPricing plan for this instance, can only be PER_USE."]
    pub fn pricing_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pricing_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replication_lag_max_seconds` after provisioning.\nThe acceptable replication lag, in seconds, after which a read replica recreates itself. The lag must persist for at least five minutes before recreation is triggered. This is a replica level field, and must be between 300 seconds (five minutes) and 31536000 seconds (one year)."]
    pub fn replication_lag_max_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_lag_max_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retain_backups_on_delete` after provisioning.\nWhen this parameter is set to true, Cloud SQL retains backups of the instance even after the instance is deleted. The ON_DEMAND backup will be retained until customer deletes the backup or the project. The AUTOMATED backup will be retained based on the backups retention setting."]
    pub fn retain_backups_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_backups_on_delete", self.base))
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
    pub fn active_directory_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_ActiveDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_BackupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_DataCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_DenyMaintenancePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.base))
    }

    #[doc = "Get a reference to the value of field `entraid_config` after provisioning.\n"]
    pub fn entraid_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_EntraidConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entraid_config", self.base))
    }

    #[doc = "Get a reference to the value of field `final_backup_config` after provisioning.\n"]
    pub fn final_backup_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_FinalBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.final_backup_config", self.base))
    }

    #[doc = "Get a reference to the value of field `insights_config` after provisioning.\n"]
    pub fn insights_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_InsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\n"]
    pub fn ip_configuration(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_IpConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `location_preference` after provisioning.\n"]
    pub fn location_preference(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_LocationPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `password_validation_policy` after provisioning.\n"]
    pub fn password_validation_policy(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstance_Settings_PasswordValidationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_validation_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `read_pool_auto_scale_config` after provisioning.\n"]
    pub fn read_pool_auto_scale_config(
        &self,
    ) -> terrars::ListRef<SqlDatabaseInstance_Settings_ReadPoolAutoScaleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_auto_scale_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_audit_config` after provisioning.\n"]
    pub fn sql_server_audit_config(&self) -> terrars::ListRef<SqlDatabaseInstance_Settings_SqlServerAuditConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_audit_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlDatabaseInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlDatabaseInstance_Timeouts {
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

impl terrars::ToListMappable for SqlDatabaseInstance_Timeouts {
    type O = terrars::BlockAssignable<SqlDatabaseInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlDatabaseInstance_Timeouts {}

impl BuildSqlDatabaseInstance_Timeouts {
    pub fn build(self) -> SqlDatabaseInstance_Timeouts {
        SqlDatabaseInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlDatabaseInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlDatabaseInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlDatabaseInstance_TimeoutsRef {
        SqlDatabaseInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlDatabaseInstance_TimeoutsRef {
    #[allow(dead_code)]
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
