#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AlloydbClusterData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_await_major_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_backup_policy: std::option::Option<std::vec::Vec<AlloydbCluster_AutomatedBackupPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_backup_config: std::option::Option<std::vec::Vec<AlloydbCluster_ContinuousBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataplex_config: std::option::Option<std::vec::Vec<AlloydbCluster_DataplexConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbCluster_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_user: std::option::Option<std::vec::Vec<AlloydbCluster_InitialUser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_update_policy: std::option::Option<std::vec::Vec<AlloydbCluster_MaintenanceUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<AlloydbCluster_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<AlloydbCluster_PscConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_source: std::option::Option<std::vec::Vec<AlloydbCluster_RestoreBackupSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backupdr_backup_source: std::option::Option<std::vec::Vec<AlloydbCluster_RestoreBackupdrBackupSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backupdr_pitr_source: std::option::Option<std::vec::Vec<AlloydbCluster_RestoreBackupdrPitrSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_continuous_backup_source: std::option::Option<std::vec::Vec<AlloydbCluster_RestoreContinuousBackupSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_config: std::option::Option<std::vec::Vec<AlloydbCluster_SecondaryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AlloydbCluster_Timeouts>,
    dynamic: AlloydbClusterDynamic,
}

struct AlloydbCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlloydbClusterData_>,
}

#[derive(Clone)]
pub struct AlloydbCluster(std::rc::Rc<AlloydbCluster_>);

impl AlloydbCluster {
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
        "Set the field `annotations`.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_type`.\nThe type of cluster. If not set, defaults to PRIMARY. Default value: \"PRIMARY\" Possible values: [\"PRIMARY\", \"SECONDARY\"]"]
    pub fn set_cluster_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_version`.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn set_database_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/alloydb_cluster.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the cluster.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the cluster will fail.\nWhen the field is set to false, deleting the cluster is allowed."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-settable and human-readable display name for the Cluster."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the alloydb cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_await_major_version_upgrade`.\nSet to true to skip awaiting on the major version upgrade of the cluster.\nPossible values: true, false\nDefault value: \"true\""]
    pub fn set_skip_await_major_version_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_await_major_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subscription_type`.\nThe subscrition type of cluster. Possible values: [\"TRIAL\", \"STANDARD\"]"]
    pub fn set_subscription_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subscription_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automated_backup_policy`.\n"]
    pub fn set_automated_backup_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automated_backup_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automated_backup_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `continuous_backup_config`.\n"]
    pub fn set_continuous_backup_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_ContinuousBackupConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().continuous_backup_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.continuous_backup_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataplex_config`.\n"]
    pub fn set_dataplex_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_DataplexConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataplex_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataplex_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_user`.\n"]
    pub fn set_initial_user(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_InitialUser>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_user = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_update_policy`.\n"]
    pub fn set_maintenance_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_PscConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_backup_source`.\n"]
    pub fn set_restore_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_RestoreBackupSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_backup_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_backup_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_backupdr_backup_source`.\n"]
    pub fn set_restore_backupdr_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_RestoreBackupdrBackupSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_backupdr_backup_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_backupdr_backup_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_backupdr_pitr_source`.\n"]
    pub fn set_restore_backupdr_pitr_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_RestoreBackupdrPitrSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_backupdr_pitr_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_backupdr_pitr_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_continuous_backup_source`.\n"]
    pub fn set_restore_continuous_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_RestoreContinuousBackupSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_continuous_backup_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_continuous_backup_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_config`.\n"]
    pub fn set_secondary_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_SecondaryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AlloydbCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> terrars::ListRef<AlloydbCluster_BackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> terrars::ListRef<AlloydbCluster_BackupdrBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backupdr_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of cluster. If not set, defaults to PRIMARY. Default value: \"PRIMARY\" Possible values: [\"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> terrars::ListRef<AlloydbCluster_ContinuousBackupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/alloydb_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the cluster will fail.\nWhen the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Cluster."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the alloydb cluster should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> terrars::ListRef<AlloydbCluster_MigrationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_await_major_version_upgrade` after provisioning.\nSet to true to skip awaiting on the major version upgrade of the cluster.\nPossible values: true, false\nDefault value: \"true\""]
    pub fn skip_await_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_major_version_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nThe subscrition type of cluster. Possible values: [\"TRIAL\", \"STANDARD\"]"]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_metadata` after provisioning.\nContains information and all metadata related to TRIAL clusters."]
    pub fn trial_metadata(&self) -> terrars::ListRef<AlloydbCluster_TrialMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\n"]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\n"]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<AlloydbCluster_ContinuousBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplex_config` after provisioning.\n"]
    pub fn dataplex_config(&self) -> terrars::ListRef<AlloydbCluster_DataplexConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplex_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(&self) -> terrars::ListRef<AlloydbCluster_InitialUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_update_policy` after provisioning.\n"]
    pub fn maintenance_update_policy(&self) -> terrars::ListRef<AlloydbCluster_MaintenanceUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<AlloydbCluster_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\n"]
    pub fn restore_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\n"]
    pub fn restore_backupdr_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupdrBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\n"]
    pub fn restore_backupdr_pitr_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupdrPitrSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backupdr_pitr_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\n"]
    pub fn restore_continuous_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreContinuousBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\n"]
    pub fn secondary_config(&self) -> terrars::ListRef<AlloydbCluster_SecondaryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbCluster_TimeoutsRef {
        AlloydbCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AlloydbCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AlloydbCluster { }

impl terrars::ToListMappable for AlloydbCluster {
    type O = terrars::ListRef<AlloydbClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AlloydbCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_alloydb_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlloydbCluster {
    pub tf_id: std::string::String,
    #[doc = "The ID of the alloydb cluster."]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = "The location where the alloydb cluster should reside."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> AlloydbCluster {
        let out = AlloydbCluster(std::rc::Rc::new(AlloydbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlloydbClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                cluster_id: self.cluster_id,
                cluster_type: core::default::Default::default(),
                database_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                skip_await_major_version_upgrade: core::default::Default::default(),
                subscription_type: core::default::Default::default(),
                automated_backup_policy: core::default::Default::default(),
                continuous_backup_config: core::default::Default::default(),
                dataplex_config: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                initial_user: core::default::Default::default(),
                maintenance_update_policy: core::default::Default::default(),
                network_config: core::default::Default::default(),
                psc_config: core::default::Default::default(),
                restore_backup_source: core::default::Default::default(),
                restore_backupdr_backup_source: core::default::Default::default(),
                restore_backupdr_pitr_source: core::default::Default::default(),
                restore_continuous_backup_source: core::default::Default::default(),
                secondary_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlloydbClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlloydbClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> terrars::ListRef<AlloydbCluster_BackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> terrars::ListRef<AlloydbCluster_BackupdrBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backupdr_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of cluster. If not set, defaults to PRIMARY. Default value: \"PRIMARY\" Possible values: [\"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> terrars::ListRef<AlloydbCluster_ContinuousBackupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/alloydb_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the cluster will fail.\nWhen the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Cluster."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the alloydb cluster should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> terrars::ListRef<AlloydbCluster_MigrationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_await_major_version_upgrade` after provisioning.\nSet to true to skip awaiting on the major version upgrade of the cluster.\nPossible values: true, false\nDefault value: \"true\""]
    pub fn skip_await_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_major_version_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nThe subscrition type of cluster. Possible values: [\"TRIAL\", \"STANDARD\"]"]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_metadata` after provisioning.\nContains information and all metadata related to TRIAL clusters."]
    pub fn trial_metadata(&self) -> terrars::ListRef<AlloydbCluster_TrialMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\n"]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\n"]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<AlloydbCluster_ContinuousBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplex_config` after provisioning.\n"]
    pub fn dataplex_config(&self) -> terrars::ListRef<AlloydbCluster_DataplexConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplex_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(&self) -> terrars::ListRef<AlloydbCluster_InitialUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_update_policy` after provisioning.\n"]
    pub fn maintenance_update_policy(&self) -> terrars::ListRef<AlloydbCluster_MaintenanceUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<AlloydbCluster_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\n"]
    pub fn restore_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\n"]
    pub fn restore_backupdr_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupdrBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\n"]
    pub fn restore_backupdr_pitr_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreBackupdrPitrSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backupdr_pitr_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\n"]
    pub fn restore_continuous_backup_source(&self) -> terrars::ListRef<AlloydbCluster_RestoreContinuousBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\n"]
    pub fn secondary_config(&self) -> terrars::ListRef<AlloydbCluster_SecondaryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbCluster_TimeoutsRef {
        AlloydbCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AlloydbClusterDynamic {
    automated_backup_policy: std::option::Option<terrars::DynamicBlock<AlloydbCluster_AutomatedBackupPolicy>>,
    continuous_backup_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_ContinuousBackupConfig>>,
    dataplex_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_DataplexConfig>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_EncryptionConfig>>,
    initial_user: std::option::Option<terrars::DynamicBlock<AlloydbCluster_InitialUser>>,
    maintenance_update_policy: std::option::Option<terrars::DynamicBlock<AlloydbCluster_MaintenanceUpdatePolicy>>,
    network_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_NetworkConfig>>,
    psc_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_PscConfig>>,
    restore_backup_source: std::option::Option<terrars::DynamicBlock<AlloydbCluster_RestoreBackupSource>>,
    restore_backupdr_backup_source: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_RestoreBackupdrBackupSource>>,
    restore_backupdr_pitr_source: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_RestoreBackupdrPitrSource>>,
    restore_continuous_backup_source: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_RestoreContinuousBackupSource>>,
    secondary_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_SecondaryConfig>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_BackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_BackupSource {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_BackupSource {
    type O = terrars::BlockAssignable<AlloydbCluster_BackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_BackupSource {}

impl BuildAlloydbCluster_BackupSource {
    pub fn build(self) -> AlloydbCluster_BackupSource {
        AlloydbCluster_BackupSource { backup_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_BackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_BackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_BackupSourceRef {
        AlloydbCluster_BackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_BackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_name` after provisioning.\n"]
    pub fn backup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_BackupdrBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_BackupdrBackupSource {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_BackupdrBackupSource {
    type O = terrars::BlockAssignable<AlloydbCluster_BackupdrBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_BackupdrBackupSource {}

impl BuildAlloydbCluster_BackupdrBackupSource {
    pub fn build(self) -> AlloydbCluster_BackupdrBackupSource {
        AlloydbCluster_BackupdrBackupSource { backup: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_BackupdrBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_BackupdrBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_BackupdrBackupSourceRef {
        AlloydbCluster_BackupdrBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_BackupdrBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup` after provisioning.\n"]
    pub fn backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_EncryptionInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbCluster_EncryptionInfo {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_versions`.\n"]
    pub fn set_kms_key_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.kms_key_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_EncryptionInfo {
    type O = terrars::BlockAssignable<AlloydbCluster_EncryptionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_EncryptionInfo {}

impl BuildAlloydbCluster_EncryptionInfo {
    pub fn build(self) -> AlloydbCluster_EncryptionInfo {
        AlloydbCluster_EncryptionInfo {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_EncryptionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_EncryptionInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_EncryptionInfoRef {
        AlloydbCluster_EncryptionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_EncryptionInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_versions` after provisioning.\n"]
    pub fn kms_key_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_versions", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_ContinuousBackupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    earliest_restorable_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_info: std::option::Option<terrars::ListField<AlloydbCluster_EncryptionInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbCluster_ContinuousBackupInfo {
    #[doc = "Set the field `earliest_restorable_time`.\n"]
    pub fn set_earliest_restorable_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.earliest_restorable_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_time`.\n"]
    pub fn set_enabled_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enabled_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_info`.\n"]
    pub fn set_encryption_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<AlloydbCluster_EncryptionInfo>>,
    ) -> Self {
        self.encryption_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_ContinuousBackupInfo {
    type O = terrars::BlockAssignable<AlloydbCluster_ContinuousBackupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_ContinuousBackupInfo {}

impl BuildAlloydbCluster_ContinuousBackupInfo {
    pub fn build(self) -> AlloydbCluster_ContinuousBackupInfo {
        AlloydbCluster_ContinuousBackupInfo {
            earliest_restorable_time: core::default::Default::default(),
            enabled_time: core::default::Default::default(),
            encryption_info: core::default::Default::default(),
            schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_ContinuousBackupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_ContinuousBackupInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_ContinuousBackupInfoRef {
        AlloydbCluster_ContinuousBackupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_ContinuousBackupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `earliest_restorable_time` after provisioning.\n"]
    pub fn earliest_restorable_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.earliest_restorable_time", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled_time` after provisioning.\n"]
    pub fn enabled_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_time", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_MigrationSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_MigrationSource {
    #[doc = "Set the field `host_port`.\n"]
    pub fn set_host_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reference_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_MigrationSource {
    type O = terrars::BlockAssignable<AlloydbCluster_MigrationSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_MigrationSource {}

impl BuildAlloydbCluster_MigrationSource {
    pub fn build(self) -> AlloydbCluster_MigrationSource {
        AlloydbCluster_MigrationSource {
            host_port: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            source_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_MigrationSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_MigrationSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_MigrationSourceRef {
        AlloydbCluster_MigrationSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_MigrationSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_port` after provisioning.\n"]
    pub fn host_port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_port", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_TrialMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_TrialMetadata {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grace_end_time`.\n"]
    pub fn set_grace_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grace_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_time`.\n"]
    pub fn set_upgrade_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upgrade_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_TrialMetadata {
    type O = terrars::BlockAssignable<AlloydbCluster_TrialMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_TrialMetadata {}

impl BuildAlloydbCluster_TrialMetadata {
    pub fn build(self) -> AlloydbCluster_TrialMetadata {
        AlloydbCluster_TrialMetadata {
            end_time: core::default::Default::default(),
            grace_end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            upgrade_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_TrialMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_TrialMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_TrialMetadataRef {
        AlloydbCluster_TrialMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_TrialMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `grace_end_time` after provisioning.\n"]
    pub fn grace_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_time` after provisioning.\n"]
    pub fn upgrade_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upgrade_time", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_EncryptionConfig {
    #[doc =
        "Set the field `kms_key_name`.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_EncryptionConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_EncryptionConfig {}

impl BuildAlloydbCluster_EncryptionConfig {
    pub fn build(self) -> AlloydbCluster_EncryptionConfig {
        AlloydbCluster_EncryptionConfig { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_EncryptionConfigRef {
        AlloydbCluster_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    #[doc = "Set the field `count`.\nThe number of backups to retain."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    type O = terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {}

impl BuildAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    pub fn build(self) -> AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
        AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention { count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
        AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of backups to retain."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    #[doc =
        "Set the field `retention_period`.\nThe retention period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    type O = terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {}

impl BuildAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    pub fn build(self) -> AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
        AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
            retention_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
        AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retention_period` after provisioning.\nThe retention period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    #[doc =
        "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Currently, only the value 0 is supported."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Currently, only the value 0 is supported."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Currently, only the value 0 is supported."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    type O = terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {}

impl BuildAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    pub fn build(self) -> AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
        AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
        AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Currently, only the value 0 is supported."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Currently, only the value 0 is supported."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Currently, only the value 0 is supported."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleDynamic {
    start_times: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_times: std::option::Option<std::vec::Vec<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>>,
    dynamic: AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleDynamic,
}

impl AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    #[doc =
        "Set the field `days_of_week`.\nThe days of the week to perform a backup. At least one day of the week must be provided. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_times`.\n"]
    pub fn set_start_times(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_times = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_times = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    type O = terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {}

impl BuildAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    pub fn build(self) -> AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
        AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
            days_of_week: core::default::Default::default(),
            start_times: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
        AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nThe days of the week to perform a backup. At least one day of the week must be provided. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_times` after provisioning.\n"]
    pub fn start_times(&self) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_times", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbCluster_AutomatedBackupPolicyDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_EncryptionConfig>>,
    quantity_based_retention: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>>,
    time_based_retention: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>>,
    weekly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_AutomatedBackupPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbCluster_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity_based_retention: std
    ::option
    ::Option<std::vec::Vec<AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_retention: std::option::Option<std::vec::Vec<AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<std::vec::Vec<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>>,
    dynamic: AlloydbCluster_AutomatedBackupPolicyDynamic,
}

impl AlloydbCluster_AutomatedBackupPolicy {
    #[doc =
        "Set the field `backup_window`.\nThe length of the time window during which a backup can be taken. If a backup does not succeed within this time window, it will be canceled and considered failed.\n\nThe backup window must be at least 5 minutes long. There is no upper bound on the window. If not set, it will default to 1 hour.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_backup_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether automated backups are enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nLabels to apply to backups created using this configuration."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe location where the backup will be stored. Currently, the only supported option is to store the backup in the same region as the cluster."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quantity_based_retention`.\n"]
    pub fn set_quantity_based_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.quantity_based_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.quantity_based_retention = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_based_retention`.\n"]
    pub fn set_time_based_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_based_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_retention = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_AutomatedBackupPolicy {
    type O = terrars::BlockAssignable<AlloydbCluster_AutomatedBackupPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_AutomatedBackupPolicy {}

impl BuildAlloydbCluster_AutomatedBackupPolicy {
    pub fn build(self) -> AlloydbCluster_AutomatedBackupPolicy {
        AlloydbCluster_AutomatedBackupPolicy {
            backup_window: core::default::Default::default(),
            enabled: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            quantity_based_retention: core::default::Default::default(),
            time_based_retention: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_AutomatedBackupPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_AutomatedBackupPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_AutomatedBackupPolicyRef {
        AlloydbCluster_AutomatedBackupPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_AutomatedBackupPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_window` after provisioning.\nThe length of the time window during which a backup can be taken. If a backup does not succeed within this time window, it will be canceled and considered failed.\n\nThe backup window must be at least 5 minutes long. There is no upper bound on the window. If not set, it will default to 1 hour.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_window", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether automated backups are enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to backups created using this configuration."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the backup will be stored. Currently, the only supported option is to store the backup in the same region as the cluster."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity_based_retention` after provisioning.\n"]
    pub fn quantity_based_retention(
        &self,
    ) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quantity_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_retention` after provisioning.\n"]
    pub fn time_based_retention(&self) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> terrars::ListRef<AlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbCluster_ContinuousBackupConfigDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<AlloydbCluster_EncryptionConfig>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_ContinuousBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_window_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbCluster_EncryptionConfig>>,
    dynamic: AlloydbCluster_ContinuousBackupConfigDynamic,
}

impl AlloydbCluster_ContinuousBackupConfig {
    #[doc = "Set the field `enabled`.\nWhether continuous backup recovery is enabled. If not set, defaults to true."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recovery_window_days`.\nThe numbers of days that are eligible to restore from using PITR. To support the entire recovery window, backups and logs are retained for one day more than the recovery window.\n\nIf not set, defaults to 14 days."]
    pub fn set_recovery_window_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recovery_window_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbCluster_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_ContinuousBackupConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_ContinuousBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_ContinuousBackupConfig {}

impl BuildAlloydbCluster_ContinuousBackupConfig {
    pub fn build(self) -> AlloydbCluster_ContinuousBackupConfig {
        AlloydbCluster_ContinuousBackupConfig {
            enabled: core::default::Default::default(),
            recovery_window_days: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_ContinuousBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_ContinuousBackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_ContinuousBackupConfigRef {
        AlloydbCluster_ContinuousBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_ContinuousBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether continuous backup recovery is enabled. If not set, defaults to true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recovery_window_days` after provisioning.\nThe numbers of days that are eligible to restore from using PITR. To support the entire recovery window, backups and logs are retained for one day more than the recovery window.\n\nIf not set, defaults to 14 days."]
    pub fn recovery_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_window_days", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_DataplexConfig {
    enabled: terrars::PrimField<bool>,
}

impl AlloydbCluster_DataplexConfig { }

impl terrars::ToListMappable for AlloydbCluster_DataplexConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_DataplexConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_DataplexConfig {
    #[doc = "Indicates whether Dataplex integration is enabled for the cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAlloydbCluster_DataplexConfig {
    pub fn build(self) -> AlloydbCluster_DataplexConfig {
        AlloydbCluster_DataplexConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_DataplexConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_DataplexConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_DataplexConfigRef {
        AlloydbCluster_DataplexConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_DataplexConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether Dataplex integration is enabled for the cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_InitialUser {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_InitialUser {
    #[doc = "Set the field `password`.\nThe initial password for the user."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo`.\nThe initial password for the user."]
    pub fn set_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_wo_version`.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_password_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nThe database username."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_InitialUser {
    type O = terrars::BlockAssignable<AlloydbCluster_InitialUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_InitialUser {}

impl BuildAlloydbCluster_InitialUser {
    pub fn build(self) -> AlloydbCluster_InitialUser {
        AlloydbCluster_InitialUser {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_InitialUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_InitialUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_InitialUserRef {
        AlloydbCluster_InitialUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_InitialUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe initial password for the user."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_wo` after provisioning.\nThe initial password for the user."]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_wo_version` after provisioning.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nThe database username."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    hours: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Currently, only the value 0 is supported."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Currently, only the value 0 is supported."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Currently, only the value 0 is supported."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    type O = terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23."]
    pub hours: terrars::PrimField<f64>,
}

impl BuildAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    pub fn build(self) -> AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
        AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
            hours: self.hours,
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
        AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Currently, only the value 0 is supported."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Currently, only the value 0 is supported."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Currently, only the value 0 is supported."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    day: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>>,
    dynamic: AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsDynamic,
}

impl AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    type O = terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    #[doc =
        "Preferred day of the week for maintenance, e.g. MONDAY, TUESDAY, etc. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    pub fn build(self) -> AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
        AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
        AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nPreferred day of the week for maintenance, e.g. MONDAY, TUESDAY, etc. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbCluster_MaintenanceUpdatePolicyDynamic {
    maintenance_windows: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>>,
}

#[derive(Serialize)]
pub struct AlloydbCluster_MaintenanceUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_windows: std::option::Option<std::vec::Vec<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>>,
    dynamic: AlloydbCluster_MaintenanceUpdatePolicyDynamic,
}

impl AlloydbCluster_MaintenanceUpdatePolicy {
    #[doc = "Set the field `maintenance_windows`.\n"]
    pub fn set_maintenance_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_windows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_MaintenanceUpdatePolicy {
    type O = terrars::BlockAssignable<AlloydbCluster_MaintenanceUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_MaintenanceUpdatePolicy {}

impl BuildAlloydbCluster_MaintenanceUpdatePolicy {
    pub fn build(self) -> AlloydbCluster_MaintenanceUpdatePolicy {
        AlloydbCluster_MaintenanceUpdatePolicy {
            maintenance_windows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_MaintenanceUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_MaintenanceUpdatePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_MaintenanceUpdatePolicyRef {
        AlloydbCluster_MaintenanceUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_MaintenanceUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_windows` after provisioning.\n"]
    pub fn maintenance_windows(&self) -> terrars::ListRef<AlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_windows", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_NetworkConfig {
    #[doc =
        "Set the field `allocated_ip_range`.\nThe name of the allocated IP range for the private IP AlloyDB cluster. For example: \"google-managed-services-default\".\nIf set, the instance IPs for this cluster will be created in the allocated range."]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: \"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_NetworkConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_NetworkConfig {}

impl BuildAlloydbCluster_NetworkConfig {
    pub fn build(self) -> AlloydbCluster_NetworkConfig {
        AlloydbCluster_NetworkConfig {
            allocated_ip_range: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_NetworkConfigRef {
        AlloydbCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range` after provisioning.\nThe name of the allocated IP range for the private IP AlloyDB cluster. For example: \"google-managed-services-default\".\nIf set, the instance IPs for this cluster will be created in the allocated range."]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: \"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AlloydbCluster_PscConfig {
    #[doc =
        "Set the field `psc_enabled`.\nCreate an instance that allows connections from Private Service Connect endpoints to the instance."]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbCluster_PscConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_PscConfig {}

impl BuildAlloydbCluster_PscConfig {
    pub fn build(self) -> AlloydbCluster_PscConfig {
        AlloydbCluster_PscConfig { psc_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_PscConfigRef {
        AlloydbCluster_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `psc_enabled` after provisioning.\nCreate an instance that allows connections from Private Service Connect endpoints to the instance."]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_owned_project_number` after provisioning.\nThe project number that needs to be allowlisted on the network attachment to enable outbound connectivity, if the network attachment is configured to ACCEPT_MANUAL connections.\nIn case the network attachment is configured to ACCEPT_AUTOMATIC, this project number does not need to be allowlisted explicitly."]
    pub fn service_owned_project_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_owned_project_number", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_RestoreBackupSource {
    backup_name: terrars::PrimField<std::string::String>,
}

impl AlloydbCluster_RestoreBackupSource { }

impl terrars::ToListMappable for AlloydbCluster_RestoreBackupSource {
    type O = terrars::BlockAssignable<AlloydbCluster_RestoreBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_RestoreBackupSource {
    #[doc = "The name of the backup that this cluster is restored from."]
    pub backup_name: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_RestoreBackupSource {
    pub fn build(self) -> AlloydbCluster_RestoreBackupSource {
        AlloydbCluster_RestoreBackupSource { backup_name: self.backup_name }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_RestoreBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_RestoreBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_RestoreBackupSourceRef {
        AlloydbCluster_RestoreBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_RestoreBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_name` after provisioning.\nThe name of the backup that this cluster is restored from."]
    pub fn backup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_RestoreBackupdrBackupSource {
    backup: terrars::PrimField<std::string::String>,
}

impl AlloydbCluster_RestoreBackupdrBackupSource { }

impl terrars::ToListMappable for AlloydbCluster_RestoreBackupdrBackupSource {
    type O = terrars::BlockAssignable<AlloydbCluster_RestoreBackupdrBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_RestoreBackupdrBackupSource {
    #[doc =
        "The name of the BackupDR backup that this cluster is restored from. It must be of the format \"projects/[PROJECT]/locations/[LOCATION]/backupVaults/[VAULT_ID]/dataSources/[DATASOURCE_ID]/backups/[BACKUP_ID]\""]
    pub backup: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_RestoreBackupdrBackupSource {
    pub fn build(self) -> AlloydbCluster_RestoreBackupdrBackupSource {
        AlloydbCluster_RestoreBackupdrBackupSource { backup: self.backup }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_RestoreBackupdrBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_RestoreBackupdrBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_RestoreBackupdrBackupSourceRef {
        AlloydbCluster_RestoreBackupdrBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_RestoreBackupdrBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup` after provisioning.\nThe name of the BackupDR backup that this cluster is restored from. It must be of the format \"projects/[PROJECT]/locations/[LOCATION]/backupVaults/[VAULT_ID]/dataSources/[DATASOURCE_ID]/backups/[BACKUP_ID]\""]
    pub fn backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_RestoreBackupdrPitrSource {
    data_source: terrars::PrimField<std::string::String>,
    point_in_time: terrars::PrimField<std::string::String>,
}

impl AlloydbCluster_RestoreBackupdrPitrSource { }

impl terrars::ToListMappable for AlloydbCluster_RestoreBackupdrPitrSource {
    type O = terrars::BlockAssignable<AlloydbCluster_RestoreBackupdrPitrSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_RestoreBackupdrPitrSource {
    #[doc =
        "The name of the BackupDR data source that this cluster is restore from. It must be of the format \"projects/[PROJECT]/locations/[LOCATION]/backupVaults/[VAULT_ID]/dataSources/[DATASOURCE_ID]\""]
    pub data_source: terrars::PrimField<std::string::String>,
    #[doc = "The point in time that this cluster is restored to, in RFC 3339 format."]
    pub point_in_time: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_RestoreBackupdrPitrSource {
    pub fn build(self) -> AlloydbCluster_RestoreBackupdrPitrSource {
        AlloydbCluster_RestoreBackupdrPitrSource {
            data_source: self.data_source,
            point_in_time: self.point_in_time,
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_RestoreBackupdrPitrSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_RestoreBackupdrPitrSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_RestoreBackupdrPitrSourceRef {
        AlloydbCluster_RestoreBackupdrPitrSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_RestoreBackupdrPitrSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nThe name of the BackupDR data source that this cluster is restore from. It must be of the format \"projects/[PROJECT]/locations/[LOCATION]/backupVaults/[VAULT_ID]/dataSources/[DATASOURCE_ID]\""]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_in_time` after provisioning.\nThe point in time that this cluster is restored to, in RFC 3339 format."]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_RestoreContinuousBackupSource {
    cluster: terrars::PrimField<std::string::String>,
    point_in_time: terrars::PrimField<std::string::String>,
}

impl AlloydbCluster_RestoreContinuousBackupSource { }

impl terrars::ToListMappable for AlloydbCluster_RestoreContinuousBackupSource {
    type O = terrars::BlockAssignable<AlloydbCluster_RestoreContinuousBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_RestoreContinuousBackupSource {
    #[doc = "The name of the source cluster that this cluster is restored from."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The point in time that this cluster is restored to, in RFC 3339 format."]
    pub point_in_time: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_RestoreContinuousBackupSource {
    pub fn build(self) -> AlloydbCluster_RestoreContinuousBackupSource {
        AlloydbCluster_RestoreContinuousBackupSource {
            cluster: self.cluster,
            point_in_time: self.point_in_time,
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_RestoreContinuousBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_RestoreContinuousBackupSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbCluster_RestoreContinuousBackupSourceRef {
        AlloydbCluster_RestoreContinuousBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_RestoreContinuousBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe name of the source cluster that this cluster is restored from."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_in_time` after provisioning.\nThe point in time that this cluster is restored to, in RFC 3339 format."]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_SecondaryConfig {
    primary_cluster_name: terrars::PrimField<std::string::String>,
}

impl AlloydbCluster_SecondaryConfig { }

impl terrars::ToListMappable for AlloydbCluster_SecondaryConfig {
    type O = terrars::BlockAssignable<AlloydbCluster_SecondaryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_SecondaryConfig {
    #[doc =
        "Name of the primary cluster must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub primary_cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbCluster_SecondaryConfig {
    pub fn build(self) -> AlloydbCluster_SecondaryConfig {
        AlloydbCluster_SecondaryConfig { primary_cluster_name: self.primary_cluster_name }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_SecondaryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_SecondaryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_SecondaryConfigRef {
        AlloydbCluster_SecondaryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_SecondaryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `primary_cluster_name` after provisioning.\nName of the primary cluster must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn primary_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_cluster_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbCluster_Timeouts {
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

impl terrars::ToListMappable for AlloydbCluster_Timeouts {
    type O = terrars::BlockAssignable<AlloydbCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbCluster_Timeouts {}

impl BuildAlloydbCluster_Timeouts {
    pub fn build(self) -> AlloydbCluster_Timeouts {
        AlloydbCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbCluster_TimeoutsRef {
        AlloydbCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbCluster_TimeoutsRef {
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
