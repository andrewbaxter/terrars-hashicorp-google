#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAlloydbClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAlloydbCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAlloydbClusterData_>,
}

#[derive(Clone)]
pub struct DataAlloydbCluster(std::rc::Rc<DataAlloydbCluster_>);

impl DataAlloydbCluster {
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

    #[doc = "Set the field `location`.\nThe location where the alloydb cluster should reside."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automated_backup_policy` after provisioning.\nThe automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default."]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_BackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_BackupdrBackupSourceRef> {
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
        "Get a reference to the value of field `continuous_backup_config` after provisioning.\nThe continuous backup config for this cluster.\n\nIf no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days."]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<DataAlloydbCluster_ContinuousBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> terrars::ListRef<DataAlloydbCluster_ContinuousBackupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataplex_config` after provisioning.\nConfiguration for Dataplex integration. This is an optional field. If not set, Dataplex integration will be enabled by default."]
    pub fn dataplex_config(&self) -> terrars::ListRef<DataAlloydbCluster_DataplexConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplex_config", self.extract_ref()))
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
        "Get a reference to the value of field `encryption_config` after provisioning.\nEncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key)."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionInfoRef> {
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
        "Get a reference to the value of field `initial_user` after provisioning.\nInitial user to setup during cluster creation. If unset for new Clusters, a postgres role with null password is created. You will need to create additional users or set the password in order to log in."]
    pub fn initial_user(&self) -> terrars::ListRef<DataAlloydbCluster_InitialUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_update_policy` after provisioning.\nMaintenanceUpdatePolicy defines the policy for system updates."]
    pub fn maintenance_update_policy(&self) -> terrars::ListRef<DataAlloydbCluster_MaintenanceUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> terrars::ListRef<DataAlloydbCluster_MigrationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nMetadata related to network configuration."]
    pub fn network_config(&self) -> terrars::ListRef<DataAlloydbCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the cluster."]
    pub fn psc_config(&self) -> terrars::ListRef<DataAlloydbCluster_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source',  'restore_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backupdr_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupdrBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\nThe BackupDR source used for point in time recovery. Conflicts with 'restore_backupdr_backup_source', 'restore_continuous_backup_source' and 'restore_backupdr_backup_source', they can't be set togeter."]
    pub fn restore_backupdr_pitr_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupdrPitrSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backupdr_pitr_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\nThe source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_continuous_backup_source(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_RestoreContinuousBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `secondary_config` after provisioning.\nConfiguration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY."]
    pub fn secondary_config(&self) -> terrars::ListRef<DataAlloydbCluster_SecondaryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
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
    pub fn trial_metadata(&self) -> terrars::ListRef<DataAlloydbCluster_TrialMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

impl terrars::Referable for DataAlloydbCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAlloydbCluster { }

impl terrars::ToListMappable for DataAlloydbCluster {
    type O = terrars::ListRef<DataAlloydbClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAlloydbCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_alloydb_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlloydbCluster {
    pub tf_id: std::string::String,
    #[doc = "The ID of the alloydb cluster."]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAlloydbCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAlloydbCluster {
        let out = DataAlloydbCluster(std::rc::Rc::new(DataAlloydbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAlloydbClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlloydbClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAlloydbClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automated_backup_policy` after provisioning.\nThe automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default."]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_BackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_BackupdrBackupSourceRef> {
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
        "Get a reference to the value of field `continuous_backup_config` after provisioning.\nThe continuous backup config for this cluster.\n\nIf no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days."]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<DataAlloydbCluster_ContinuousBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> terrars::ListRef<DataAlloydbCluster_ContinuousBackupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataplex_config` after provisioning.\nConfiguration for Dataplex integration. This is an optional field. If not set, Dataplex integration will be enabled by default."]
    pub fn dataplex_config(&self) -> terrars::ListRef<DataAlloydbCluster_DataplexConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplex_config", self.extract_ref()))
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
        "Get a reference to the value of field `encryption_config` after provisioning.\nEncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key)."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionInfoRef> {
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
        "Get a reference to the value of field `initial_user` after provisioning.\nInitial user to setup during cluster creation. If unset for new Clusters, a postgres role with null password is created. You will need to create additional users or set the password in order to log in."]
    pub fn initial_user(&self) -> terrars::ListRef<DataAlloydbCluster_InitialUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_update_policy` after provisioning.\nMaintenanceUpdatePolicy defines the policy for system updates."]
    pub fn maintenance_update_policy(&self) -> terrars::ListRef<DataAlloydbCluster_MaintenanceUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> terrars::ListRef<DataAlloydbCluster_MigrationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nMetadata related to network configuration."]
    pub fn network_config(&self) -> terrars::ListRef<DataAlloydbCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the cluster."]
    pub fn psc_config(&self) -> terrars::ListRef<DataAlloydbCluster_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source',  'restore_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backupdr_backup_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupdrBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\nThe BackupDR source used for point in time recovery. Conflicts with 'restore_backupdr_backup_source', 'restore_continuous_backup_source' and 'restore_backupdr_backup_source', they can't be set togeter."]
    pub fn restore_backupdr_pitr_source(&self) -> terrars::ListRef<DataAlloydbCluster_RestoreBackupdrPitrSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backupdr_pitr_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\nThe source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_continuous_backup_source(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_RestoreContinuousBackupSourceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `secondary_config` after provisioning.\nConfiguration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY."]
    pub fn secondary_config(&self) -> terrars::ListRef<DataAlloydbCluster_SecondaryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
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
    pub fn trial_metadata(&self) -> terrars::ListRef<DataAlloydbCluster_TrialMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trial_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_EncryptionConfig {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_EncryptionConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_EncryptionConfig {}

impl BuildDataAlloydbCluster_EncryptionConfig {
    pub fn build(self) -> DataAlloydbCluster_EncryptionConfig {
        DataAlloydbCluster_EncryptionConfig { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_EncryptionConfigRef {
        DataAlloydbCluster_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    type O = terrars::BlockAssignable<DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {}

impl BuildDataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
    pub fn build(self) -> DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention {
        DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention { count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
        DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    type O = terrars::BlockAssignable<DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {}

impl BuildDataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
    pub fn build(self) -> DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
        DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention {
            retention_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
        DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    type O = terrars::BlockAssignable<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {}

impl BuildDataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
    pub fn build(self) -> DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
        DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
        DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_times: std
    ::option
    ::Option<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>>,
}

impl DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    #[doc = "Set the field `days_of_week`.\n"]
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
                    ::Into<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimes>>,
    ) -> Self {
        self.start_times = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    type O = terrars::BlockAssignable<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {}

impl BuildDataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
    pub fn build(self) -> DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
        DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule {
            days_of_week: core::default::Default::default(),
            start_times: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
        DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_times` after provisioning.\n"]
    pub fn start_times(&self) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule_StartTimesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_times", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_AutomatedBackupPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<terrars::ListField<DataAlloydbCluster_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity_based_retention: std
    ::option
    ::Option<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_retention: std
    ::option
    ::Option<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>>,
}

impl DataAlloydbCluster_AutomatedBackupPolicy {
    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbCluster_EncryptionConfig>>,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `quantity_based_retention`.\n"]
    pub fn set_quantity_based_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetention>>,
    ) -> Self {
        self.quantity_based_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_based_retention`.\n"]
    pub fn set_time_based_retention(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetention>>,
    ) -> Self {
        self.time_based_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbCluster_AutomatedBackupPolicy_WeeklySchedule>>,
    ) -> Self {
        self.weekly_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_AutomatedBackupPolicy {
    type O = terrars::BlockAssignable<DataAlloydbCluster_AutomatedBackupPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_AutomatedBackupPolicy {}

impl BuildDataAlloydbCluster_AutomatedBackupPolicy {
    pub fn build(self) -> DataAlloydbCluster_AutomatedBackupPolicy {
        DataAlloydbCluster_AutomatedBackupPolicy {
            backup_window: core::default::Default::default(),
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            quantity_based_retention: core::default::Default::default(),
            time_based_retention: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_AutomatedBackupPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_AutomatedBackupPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_AutomatedBackupPolicyRef {
        DataAlloydbCluster_AutomatedBackupPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_AutomatedBackupPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_window", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity_based_retention` after provisioning.\n"]
    pub fn quantity_based_retention(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicy_QuantityBasedRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quantity_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_retention` after provisioning.\n"]
    pub fn time_based_retention(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicy_TimeBasedRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> terrars::ListRef<DataAlloydbCluster_AutomatedBackupPolicy_WeeklyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_BackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_BackupSource {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_BackupSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_BackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_BackupSource {}

impl BuildDataAlloydbCluster_BackupSource {
    pub fn build(self) -> DataAlloydbCluster_BackupSource {
        DataAlloydbCluster_BackupSource { backup_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_BackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_BackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_BackupSourceRef {
        DataAlloydbCluster_BackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_BackupSourceRef {
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
pub struct DataAlloydbCluster_BackupdrBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_BackupdrBackupSource {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_BackupdrBackupSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_BackupdrBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_BackupdrBackupSource {}

impl BuildDataAlloydbCluster_BackupdrBackupSource {
    pub fn build(self) -> DataAlloydbCluster_BackupdrBackupSource {
        DataAlloydbCluster_BackupdrBackupSource { backup: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_BackupdrBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_BackupdrBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_BackupdrBackupSourceRef {
        DataAlloydbCluster_BackupdrBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_BackupdrBackupSourceRef {
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
pub struct DataAlloydbCluster_ContinuousBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<terrars::ListField<DataAlloydbCluster_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_window_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbCluster_ContinuousBackupConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbCluster_EncryptionConfig>>,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_window_days`.\n"]
    pub fn set_recovery_window_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recovery_window_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_ContinuousBackupConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_ContinuousBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_ContinuousBackupConfig {}

impl BuildDataAlloydbCluster_ContinuousBackupConfig {
    pub fn build(self) -> DataAlloydbCluster_ContinuousBackupConfig {
        DataAlloydbCluster_ContinuousBackupConfig {
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            recovery_window_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_ContinuousBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_ContinuousBackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_ContinuousBackupConfigRef {
        DataAlloydbCluster_ContinuousBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_ContinuousBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_window_days` after provisioning.\n"]
    pub fn recovery_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_window_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_EncryptionInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataAlloydbCluster_EncryptionInfo {
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

impl terrars::ToListMappable for DataAlloydbCluster_EncryptionInfo {
    type O = terrars::BlockAssignable<DataAlloydbCluster_EncryptionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_EncryptionInfo {}

impl BuildDataAlloydbCluster_EncryptionInfo {
    pub fn build(self) -> DataAlloydbCluster_EncryptionInfo {
        DataAlloydbCluster_EncryptionInfo {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_EncryptionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_EncryptionInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_EncryptionInfoRef {
        DataAlloydbCluster_EncryptionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_EncryptionInfoRef {
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
pub struct DataAlloydbCluster_ContinuousBackupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    earliest_restorable_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_info: std::option::Option<terrars::ListField<DataAlloydbCluster_EncryptionInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataAlloydbCluster_ContinuousBackupInfo {
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
        v: impl std::convert::Into<terrars::ListField<DataAlloydbCluster_EncryptionInfo>>,
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

impl terrars::ToListMappable for DataAlloydbCluster_ContinuousBackupInfo {
    type O = terrars::BlockAssignable<DataAlloydbCluster_ContinuousBackupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_ContinuousBackupInfo {}

impl BuildDataAlloydbCluster_ContinuousBackupInfo {
    pub fn build(self) -> DataAlloydbCluster_ContinuousBackupInfo {
        DataAlloydbCluster_ContinuousBackupInfo {
            earliest_restorable_time: core::default::Default::default(),
            enabled_time: core::default::Default::default(),
            encryption_info: core::default::Default::default(),
            schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_ContinuousBackupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_ContinuousBackupInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_ContinuousBackupInfoRef {
        DataAlloydbCluster_ContinuousBackupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_ContinuousBackupInfoRef {
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
    pub fn encryption_info(&self) -> terrars::ListRef<DataAlloydbCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_DataplexConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAlloydbCluster_DataplexConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_DataplexConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_DataplexConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_DataplexConfig {}

impl BuildDataAlloydbCluster_DataplexConfig {
    pub fn build(self) -> DataAlloydbCluster_DataplexConfig {
        DataAlloydbCluster_DataplexConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_DataplexConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_DataplexConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_DataplexConfigRef {
        DataAlloydbCluster_DataplexConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_DataplexConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_InitialUser {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_InitialUser {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo`.\n"]
    pub fn set_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo_version`.\n"]
    pub fn set_password_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_InitialUser {
    type O = terrars::BlockAssignable<DataAlloydbCluster_InitialUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_InitialUser {}

impl BuildDataAlloydbCluster_InitialUser {
    pub fn build(self) -> DataAlloydbCluster_InitialUser {
        DataAlloydbCluster_InitialUser {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_InitialUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_InitialUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_InitialUserRef {
        DataAlloydbCluster_InitialUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_InitialUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\n"]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\n"]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    type O = terrars::BlockAssignable<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {}

impl BuildDataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
    pub fn build(self) -> DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
        DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
        DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>>,
}

impl DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    type O = terrars::BlockAssignable<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {}

impl BuildDataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
    pub fn build(self) -> DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
        DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
        DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_windows: std
    ::option
    ::Option<terrars::ListField<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>>,
}

impl DataAlloydbCluster_MaintenanceUpdatePolicy {
    #[doc = "Set the field `maintenance_windows`.\n"]
    pub fn set_maintenance_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindows>>,
    ) -> Self {
        self.maintenance_windows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_MaintenanceUpdatePolicy {
    type O = terrars::BlockAssignable<DataAlloydbCluster_MaintenanceUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_MaintenanceUpdatePolicy {}

impl BuildDataAlloydbCluster_MaintenanceUpdatePolicy {
    pub fn build(self) -> DataAlloydbCluster_MaintenanceUpdatePolicy {
        DataAlloydbCluster_MaintenanceUpdatePolicy { maintenance_windows: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_MaintenanceUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_MaintenanceUpdatePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_MaintenanceUpdatePolicyRef {
        DataAlloydbCluster_MaintenanceUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_MaintenanceUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_windows` after provisioning.\n"]
    pub fn maintenance_windows(
        &self,
    ) -> terrars::ListRef<DataAlloydbCluster_MaintenanceUpdatePolicy_MaintenanceWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_windows", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_MigrationSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_MigrationSource {
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

impl terrars::ToListMappable for DataAlloydbCluster_MigrationSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_MigrationSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_MigrationSource {}

impl BuildDataAlloydbCluster_MigrationSource {
    pub fn build(self) -> DataAlloydbCluster_MigrationSource {
        DataAlloydbCluster_MigrationSource {
            host_port: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            source_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_MigrationSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_MigrationSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_MigrationSourceRef {
        DataAlloydbCluster_MigrationSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_MigrationSourceRef {
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
pub struct DataAlloydbCluster_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_NetworkConfig {
    #[doc = "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_NetworkConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_NetworkConfig {}

impl BuildDataAlloydbCluster_NetworkConfig {
    pub fn build(self) -> DataAlloydbCluster_NetworkConfig {
        DataAlloydbCluster_NetworkConfig {
            allocated_ip_range: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_NetworkConfigRef {
        DataAlloydbCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_owned_project_number: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbCluster_PscConfig {
    #[doc = "Set the field `psc_enabled`.\n"]
    pub fn set_psc_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.psc_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_owned_project_number`.\n"]
    pub fn set_service_owned_project_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.service_owned_project_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_PscConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_PscConfig {}

impl BuildDataAlloydbCluster_PscConfig {
    pub fn build(self) -> DataAlloydbCluster_PscConfig {
        DataAlloydbCluster_PscConfig {
            psc_enabled: core::default::Default::default(),
            service_owned_project_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_PscConfigRef {
        DataAlloydbCluster_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `psc_enabled` after provisioning.\n"]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `service_owned_project_number` after provisioning.\n"]
    pub fn service_owned_project_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_owned_project_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_RestoreBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_RestoreBackupSource {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_RestoreBackupSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_RestoreBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_RestoreBackupSource {}

impl BuildDataAlloydbCluster_RestoreBackupSource {
    pub fn build(self) -> DataAlloydbCluster_RestoreBackupSource {
        DataAlloydbCluster_RestoreBackupSource { backup_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_RestoreBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_RestoreBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_RestoreBackupSourceRef {
        DataAlloydbCluster_RestoreBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_RestoreBackupSourceRef {
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
pub struct DataAlloydbCluster_RestoreBackupdrBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_RestoreBackupdrBackupSource {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_RestoreBackupdrBackupSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_RestoreBackupdrBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_RestoreBackupdrBackupSource {}

impl BuildDataAlloydbCluster_RestoreBackupdrBackupSource {
    pub fn build(self) -> DataAlloydbCluster_RestoreBackupdrBackupSource {
        DataAlloydbCluster_RestoreBackupdrBackupSource { backup: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_RestoreBackupdrBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_RestoreBackupdrBackupSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_RestoreBackupdrBackupSourceRef {
        DataAlloydbCluster_RestoreBackupdrBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_RestoreBackupdrBackupSourceRef {
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
pub struct DataAlloydbCluster_RestoreBackupdrPitrSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_RestoreBackupdrPitrSource {
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_in_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_RestoreBackupdrPitrSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_RestoreBackupdrPitrSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_RestoreBackupdrPitrSource {}

impl BuildDataAlloydbCluster_RestoreBackupdrPitrSource {
    pub fn build(self) -> DataAlloydbCluster_RestoreBackupdrPitrSource {
        DataAlloydbCluster_RestoreBackupdrPitrSource {
            data_source: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_RestoreBackupdrPitrSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_RestoreBackupdrPitrSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_RestoreBackupdrPitrSourceRef {
        DataAlloydbCluster_RestoreBackupdrPitrSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_RestoreBackupdrPitrSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_RestoreContinuousBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_RestoreContinuousBackupSource {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_in_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_RestoreContinuousBackupSource {
    type O = terrars::BlockAssignable<DataAlloydbCluster_RestoreContinuousBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_RestoreContinuousBackupSource {}

impl BuildDataAlloydbCluster_RestoreContinuousBackupSource {
    pub fn build(self) -> DataAlloydbCluster_RestoreContinuousBackupSource {
        DataAlloydbCluster_RestoreContinuousBackupSource {
            cluster: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_RestoreContinuousBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_RestoreContinuousBackupSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbCluster_RestoreContinuousBackupSourceRef {
        DataAlloydbCluster_RestoreContinuousBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_RestoreContinuousBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_SecondaryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_SecondaryConfig {
    #[doc = "Set the field `primary_cluster_name`.\n"]
    pub fn set_primary_cluster_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.primary_cluster_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbCluster_SecondaryConfig {
    type O = terrars::BlockAssignable<DataAlloydbCluster_SecondaryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_SecondaryConfig {}

impl BuildDataAlloydbCluster_SecondaryConfig {
    pub fn build(self) -> DataAlloydbCluster_SecondaryConfig {
        DataAlloydbCluster_SecondaryConfig { primary_cluster_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_SecondaryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_SecondaryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_SecondaryConfigRef {
        DataAlloydbCluster_SecondaryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_SecondaryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary_cluster_name` after provisioning.\n"]
    pub fn primary_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_cluster_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbCluster_TrialMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbCluster_TrialMetadata {
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

impl terrars::ToListMappable for DataAlloydbCluster_TrialMetadata {
    type O = terrars::BlockAssignable<DataAlloydbCluster_TrialMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbCluster_TrialMetadata {}

impl BuildDataAlloydbCluster_TrialMetadata {
    pub fn build(self) -> DataAlloydbCluster_TrialMetadata {
        DataAlloydbCluster_TrialMetadata {
            end_time: core::default::Default::default(),
            grace_end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            upgrade_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbCluster_TrialMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbCluster_TrialMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbCluster_TrialMetadataRef {
        DataAlloydbCluster_TrialMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbCluster_TrialMetadataRef {
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
