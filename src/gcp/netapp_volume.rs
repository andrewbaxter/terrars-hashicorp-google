#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetappVolumeData_ {
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
    capacity_gib: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_capacity: std::option::Option<terrars::PrimField<bool>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiple_endpoints: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    protocols: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_settings: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_directory: std::option::Option<terrars::PrimField<bool>>,
    storage_pool: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_mibps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unix_permissions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_config: std::option::Option<std::vec::Vec<NetappVolume_BackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_devices: std::option::Option<std::vec::Vec<NetappVolume_BlockDevices>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_parameters: std::option::Option<std::vec::Vec<NetappVolume_CacheParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_policy: std::option::Option<std::vec::Vec<NetappVolume_ExportPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hybrid_replication_parameters: std::option::Option<std::vec::Vec<NetappVolume_HybridReplicationParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_capacity_config: std::option::Option<std::vec::Vec<NetappVolume_LargeCapacityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_parameters: std::option::Option<std::vec::Vec<NetappVolume_RestoreParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_policy: std::option::Option<std::vec::Vec<NetappVolume_SnapshotPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiering_policy: std::option::Option<std::vec::Vec<NetappVolume_TieringPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetappVolume_Timeouts>,
    dynamic: NetappVolumeDynamic,
}

struct NetappVolume_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetappVolumeData_>,
}

#[derive(Clone)]
pub struct NetappVolume(std::rc::Rc<NetappVolume_>);

impl NetappVolume {
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
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos_enabled`.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn set_kerberos_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().kerberos_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `large_capacity`.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn set_large_capacity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().large_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multiple_endpoints`.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn set_multiple_endpoints(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multiple_endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_actions`.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn set_restricted_actions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().restricted_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_style`.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn set_security_style(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_style = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `share_name`.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn set_share_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().share_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `smb_settings`.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn set_smb_settings(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().smb_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `snapshot_directory`.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn set_snapshot_directory(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().snapshot_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput_mibps`.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn set_throughput_mibps(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_mibps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unix_permissions`.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn set_unix_permissions(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().unix_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_config`.\n"]
    pub fn set_backup_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_BackupConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `block_devices`.\n"]
    pub fn set_block_devices(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_BlockDevices>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_devices = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_devices = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_parameters`.\n"]
    pub fn set_cache_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_CacheParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `export_policy`.\n"]
    pub fn set_export_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_ExportPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hybrid_replication_parameters`.\n"]
    pub fn set_hybrid_replication_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_HybridReplicationParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hybrid_replication_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hybrid_replication_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `large_capacity_config`.\n"]
    pub fn set_large_capacity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_LargeCapacityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().large_capacity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.large_capacity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_parameters`.\n"]
    pub fn set_restore_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_RestoreParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_policy`.\n"]
    pub fn set_snapshot_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_SnapshotPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_TieringPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetappVolume_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `active_directory` after provisioning.\nReports the resource name of the Active Directory policy being used. Inherited from storage pool."]
    pub fn active_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the volume (in GiB)."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cold_tier_size_gib` after provisioning.\nOutput only. Size of the volume cold tier data in GiB."]
    pub fn cold_tier_size_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cold_tier_size_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the volume. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_type` after provisioning.\nReports the data-at-rest encryption type of the volume. Inherited from storage pool."]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_replication` after provisioning.\nIndicates whether the volume is part of a volume replication relationship."]
    pub fn has_replication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the volume. This field is only used for flex Service Level"]
    pub fn hot_tier_size_used_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hot_tier_size_used_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kerberos_enabled` after provisioning.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn kerberos_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_config` after provisioning.\nReports the CMEK policy resurce name being used for volume encryption. Inherited from storage pool."]
    pub fn kms_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `large_capacity` after provisioning.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn large_capacity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.large_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ldap_enabled` after provisioning.\nFlag indicating if the volume is NFS LDAP enabled or not. Inherited from storage pool."]
    pub fn ldap_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ldap_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nName of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mount_options` after provisioning.\nReports mount instructions for this volume."]
    pub fn mount_options(&self) -> terrars::ListRef<NetappVolume_MountOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multiple_endpoints` after provisioning.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn multiple_endpoints(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiple_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the volume. Needs to be unique per location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'. Inherited from storage pool."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocols` after provisioning.\nThe protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psa_range` after provisioning.\nName of the Private Service Access allocated range. Inherited from storage pool."]
    pub fn psa_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psa_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional volume."]
    pub fn replica_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restricted_actions` after provisioning.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn restricted_actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_style` after provisioning.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_level` after provisioning.\nService level of the volume. Inherited from storage pool. Supported values are : PREMIUM, EXTREME, STANDARD, FLEX."]
    pub fn service_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `share_name` after provisioning.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `smb_settings` after provisioning.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn smb_settings(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_directory` after provisioning.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn snapshot_directory(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the volume."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the volume."]
    pub fn state_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool` after provisioning.\nName of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
    pub fn storage_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn throughput_mibps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mibps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unix_permissions` after provisioning.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn unix_permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unix_permissions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `used_gib` after provisioning.\nUsed capacity of the volume (in GiB). This is computed periodically and it does not represent the realtime usage."]
    pub fn used_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional volume."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> terrars::ListRef<NetappVolume_BackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_devices` after provisioning.\n"]
    pub fn block_devices(&self) -> terrars::ListRef<NetappVolume_BlockDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_parameters` after provisioning.\n"]
    pub fn cache_parameters(&self) -> terrars::ListRef<NetappVolume_CacheParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_policy` after provisioning.\n"]
    pub fn export_policy(&self) -> terrars::ListRef<NetappVolume_ExportPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hybrid_replication_parameters` after provisioning.\n"]
    pub fn hybrid_replication_parameters(&self) -> terrars::ListRef<NetappVolume_HybridReplicationParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_replication_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_capacity_config` after provisioning.\n"]
    pub fn large_capacity_config(&self) -> terrars::ListRef<NetappVolume_LargeCapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_parameters` after provisioning.\n"]
    pub fn restore_parameters(&self) -> terrars::ListRef<NetappVolume_RestoreParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> terrars::ListRef<NetappVolume_TieringPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolume_TimeoutsRef {
        NetappVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetappVolume {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetappVolume { }

impl terrars::ToListMappable for NetappVolume {
    type O = terrars::ListRef<NetappVolumeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetappVolume_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_netapp_volume".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetappVolume {
    pub tf_id: std::string::String,
    #[doc = "Capacity of the volume (in GiB)."]
    pub capacity_gib: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the volume. Needs to be unique per location."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub protocols: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Name of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
    pub storage_pool: terrars::PrimField<std::string::String>,
}

impl BuildNetappVolume {
    pub fn build(self, stack: &mut terrars::Stack) -> NetappVolume {
        let out = NetappVolume(std::rc::Rc::new(NetappVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetappVolumeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kerberos_enabled: core::default::Default::default(),
                labels: core::default::Default::default(),
                large_capacity: core::default::Default::default(),
                location: self.location,
                multiple_endpoints: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocols: self.protocols,
                restricted_actions: core::default::Default::default(),
                security_style: core::default::Default::default(),
                share_name: core::default::Default::default(),
                smb_settings: core::default::Default::default(),
                snapshot_directory: core::default::Default::default(),
                storage_pool: self.storage_pool,
                throughput_mibps: core::default::Default::default(),
                unix_permissions: core::default::Default::default(),
                backup_config: core::default::Default::default(),
                block_devices: core::default::Default::default(),
                cache_parameters: core::default::Default::default(),
                export_policy: core::default::Default::default(),
                hybrid_replication_parameters: core::default::Default::default(),
                large_capacity_config: core::default::Default::default(),
                restore_parameters: core::default::Default::default(),
                snapshot_policy: core::default::Default::default(),
                tiering_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetappVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetappVolumeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active_directory` after provisioning.\nReports the resource name of the Active Directory policy being used. Inherited from storage pool."]
    pub fn active_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the volume (in GiB)."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cold_tier_size_gib` after provisioning.\nOutput only. Size of the volume cold tier data in GiB."]
    pub fn cold_tier_size_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cold_tier_size_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the volume. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_type` after provisioning.\nReports the data-at-rest encryption type of the volume. Inherited from storage pool."]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_replication` after provisioning.\nIndicates whether the volume is part of a volume replication relationship."]
    pub fn has_replication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the volume. This field is only used for flex Service Level"]
    pub fn hot_tier_size_used_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hot_tier_size_used_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kerberos_enabled` after provisioning.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn kerberos_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_config` after provisioning.\nReports the CMEK policy resurce name being used for volume encryption. Inherited from storage pool."]
    pub fn kms_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `large_capacity` after provisioning.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn large_capacity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.large_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ldap_enabled` after provisioning.\nFlag indicating if the volume is NFS LDAP enabled or not. Inherited from storage pool."]
    pub fn ldap_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ldap_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nName of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mount_options` after provisioning.\nReports mount instructions for this volume."]
    pub fn mount_options(&self) -> terrars::ListRef<NetappVolume_MountOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multiple_endpoints` after provisioning.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn multiple_endpoints(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiple_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the volume. Needs to be unique per location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'. Inherited from storage pool."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocols` after provisioning.\nThe protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psa_range` after provisioning.\nName of the Private Service Access allocated range. Inherited from storage pool."]
    pub fn psa_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psa_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional volume."]
    pub fn replica_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restricted_actions` after provisioning.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn restricted_actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_actions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_style` after provisioning.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_level` after provisioning.\nService level of the volume. Inherited from storage pool. Supported values are : PREMIUM, EXTREME, STANDARD, FLEX."]
    pub fn service_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `share_name` after provisioning.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `smb_settings` after provisioning.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn smb_settings(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_directory` after provisioning.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn snapshot_directory(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the volume."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the volume."]
    pub fn state_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool` after provisioning.\nName of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
    pub fn storage_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn throughput_mibps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mibps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unix_permissions` after provisioning.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn unix_permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unix_permissions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `used_gib` after provisioning.\nUsed capacity of the volume (in GiB). This is computed periodically and it does not represent the realtime usage."]
    pub fn used_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional volume."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> terrars::ListRef<NetappVolume_BackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_devices` after provisioning.\n"]
    pub fn block_devices(&self) -> terrars::ListRef<NetappVolume_BlockDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_parameters` after provisioning.\n"]
    pub fn cache_parameters(&self) -> terrars::ListRef<NetappVolume_CacheParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_policy` after provisioning.\n"]
    pub fn export_policy(&self) -> terrars::ListRef<NetappVolume_ExportPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hybrid_replication_parameters` after provisioning.\n"]
    pub fn hybrid_replication_parameters(&self) -> terrars::ListRef<NetappVolume_HybridReplicationParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hybrid_replication_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_capacity_config` after provisioning.\n"]
    pub fn large_capacity_config(&self) -> terrars::ListRef<NetappVolume_LargeCapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_parameters` after provisioning.\n"]
    pub fn restore_parameters(&self) -> terrars::ListRef<NetappVolume_RestoreParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> terrars::ListRef<NetappVolume_TieringPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolume_TimeoutsRef {
        NetappVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetappVolumeDynamic {
    backup_config: std::option::Option<terrars::DynamicBlock<NetappVolume_BackupConfig>>,
    block_devices: std::option::Option<terrars::DynamicBlock<NetappVolume_BlockDevices>>,
    cache_parameters: std::option::Option<terrars::DynamicBlock<NetappVolume_CacheParameters>>,
    export_policy: std::option::Option<terrars::DynamicBlock<NetappVolume_ExportPolicy>>,
    hybrid_replication_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<NetappVolume_HybridReplicationParameters>>,
    large_capacity_config: std::option::Option<terrars::DynamicBlock<NetappVolume_LargeCapacityConfig>>,
    restore_parameters: std::option::Option<terrars::DynamicBlock<NetappVolume_RestoreParameters>>,
    snapshot_policy: std::option::Option<terrars::DynamicBlock<NetappVolume_SnapshotPolicy>>,
    tiering_policy: std::option::Option<terrars::DynamicBlock<NetappVolume_TieringPolicy>>,
}

#[derive(Serialize)]
pub struct NetappVolume_MountOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_full: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instructions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_MountOptions {
    #[doc = "Set the field `export`.\n"]
    pub fn set_export(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_full`.\n"]
    pub fn set_export_full(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_full = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instructions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_MountOptions {
    type O = terrars::BlockAssignable<NetappVolume_MountOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_MountOptions {}

impl BuildNetappVolume_MountOptions {
    pub fn build(self) -> NetappVolume_MountOptions {
        NetappVolume_MountOptions {
            export: core::default::Default::default(),
            export_full: core::default::Default::default(),
            instructions: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_MountOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_MountOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_MountOptionsRef {
        NetappVolume_MountOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_MountOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.base))
    }

    #[doc = "Get a reference to the value of field `export_full` after provisioning.\n"]
    pub fn export_full(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_full", self.base))
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_BackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_vault: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_backup_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl NetappVolume_BackupConfig {
    #[doc =
        "Set the field `backup_policies`.\nSpecify a single backup policy ID for scheduled backups. Format: 'projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}'"]
    pub fn set_backup_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.backup_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `backup_vault`.\nID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.\nFormat: 'projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}'"]
    pub fn set_backup_vault(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_vault = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheduled_backup_enabled`.\nWhen set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified."]
    pub fn set_scheduled_backup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scheduled_backup_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_BackupConfig {
    type O = terrars::BlockAssignable<NetappVolume_BackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_BackupConfig {}

impl BuildNetappVolume_BackupConfig {
    pub fn build(self) -> NetappVolume_BackupConfig {
        NetappVolume_BackupConfig {
            backup_policies: core::default::Default::default(),
            backup_vault: core::default::Default::default(),
            scheduled_backup_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_BackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_BackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_BackupConfigRef {
        NetappVolume_BackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_BackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_policies` after provisioning.\nSpecify a single backup policy ID for scheduled backups. Format: 'projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}'"]
    pub fn backup_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_policies", self.base))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault` after provisioning.\nID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.\nFormat: 'projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}'"]
    pub fn backup_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_backup_enabled` after provisioning.\nWhen set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified."]
    pub fn scheduled_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_backup_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_BlockDevices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    os_type: terrars::PrimField<std::string::String>,
}

impl NetappVolume_BlockDevices {
    #[doc =
        "Set the field `host_groups`.\nA list of host groups that identify hosts that can mount the block volume.\nFormat:\n'projects/{project_id}/locations/{location}/hostGroups/{host_group_id}'\nThis field can be updated after the block device is created."]
    pub fn set_host_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.host_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nUser-defined name for the block device, unique within the Volume. In case\nno user input is provided, name will be autogenerated in the backend.\nThe name must meet the following requirements:\n*   Be between 1 and 255 characters long.\n*   Contain only uppercase or lowercase letters (A-Z, a-z), numbers (0-9),\n    and the following special characters: \"-\", \"_\", \"}\", \"{\", \".\".\n*   Spaces are not allowed."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_BlockDevices {
    type O = terrars::BlockAssignable<NetappVolume_BlockDevices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_BlockDevices {
    #[doc =
        "The OS type of the volume.\nThis field can't be changed after the block device is created. Possible values: [\"LINUX\", \"WINDOWS\", \"ESXI\"]"]
    pub os_type: terrars::PrimField<std::string::String>,
}

impl BuildNetappVolume_BlockDevices {
    pub fn build(self) -> NetappVolume_BlockDevices {
        NetappVolume_BlockDevices {
            host_groups: core::default::Default::default(),
            name: core::default::Default::default(),
            os_type: self.os_type,
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_BlockDevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_BlockDevicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_BlockDevicesRef {
        NetappVolume_BlockDevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_BlockDevicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_groups` after provisioning.\nA list of host groups that identify hosts that can mount the block volume.\nFormat:\n'projects/{project_id}/locations/{location}/hostGroups/{host_group_id}'\nThis field can be updated after the block device is created."]
    pub fn host_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nDevice identifier of the Block volume. This represents lun_serial_number\nfor ISCSI volumes"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUser-defined name for the block device, unique within the Volume. In case\nno user input is provided, name will be autogenerated in the backend.\nThe name must meet the following requirements:\n*   Be between 1 and 255 characters long.\n*   Contain only uppercase or lowercase letters (A-Z, a-z), numbers (0-9),\n    and the following special characters: \"-\", \"_\", \"}\", \"{\", \".\".\n*   Spaces are not allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_type` after provisioning.\nThe OS type of the volume.\nThis field can't be changed after the block device is created. Possible values: [\"LINUX\", \"WINDOWS\", \"ESXI\"]"]
    pub fn os_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gib` after provisioning.\nThe size of the block device in GiB.\nAny value provided in this field during Volume creation is IGNORED.\nThe block device's size is system-managed and will be set to match\nthe parent Volume's 'capacity_gib'."]
    pub fn size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_CacheParameters_CacheConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cifs_change_notify_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl NetappVolume_CacheParameters_CacheConfig {
    #[doc =
        "Set the field `cifs_change_notify_enabled`.\nOptional. Flag indicating whether a CIFS change notification is enabled for the FlexCache volume."]
    pub fn set_cifs_change_notify_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cifs_change_notify_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_CacheParameters_CacheConfig {
    type O = terrars::BlockAssignable<NetappVolume_CacheParameters_CacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_CacheParameters_CacheConfig {}

impl BuildNetappVolume_CacheParameters_CacheConfig {
    pub fn build(self) -> NetappVolume_CacheParameters_CacheConfig {
        NetappVolume_CacheParameters_CacheConfig { cifs_change_notify_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_CacheParameters_CacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_CacheParameters_CacheConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_CacheParameters_CacheConfigRef {
        NetappVolume_CacheParameters_CacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_CacheParameters_CacheConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cifs_change_notify_enabled` after provisioning.\nOptional. Flag indicating whether a CIFS change notification is enabled for the FlexCache volume."]
    pub fn cifs_change_notify_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cifs_change_notify_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetappVolume_CacheParametersDynamic {
    cache_config: std::option::Option<terrars::DynamicBlock<NetappVolume_CacheParameters_CacheConfig>>,
}

#[derive(Serialize)]
pub struct NetappVolume_CacheParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_global_file_lock: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_svm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peering_command_expiry_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_config: std::option::Option<std::vec::Vec<NetappVolume_CacheParameters_CacheConfig>>,
    dynamic: NetappVolume_CacheParametersDynamic,
}

impl NetappVolume_CacheParameters {
    #[doc =
        "Set the field `enable_global_file_lock`.\nOptional. Field indicating whether cache volume as global file lock enabled."]
    pub fn set_enable_global_file_lock(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_global_file_lock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_cluster_name`.\nRequired. Name of the origin volume's ONTAP cluster."]
    pub fn set_peer_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_ip_addresses`.\nRequired. List of IC LIF addresses of the origin volume's ONTAP cluster."]
    pub fn set_peer_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.peer_ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_svm_name`.\nRequired. Name of the origin volume's SVM."]
    pub fn set_peer_svm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_svm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_volume_name`.\nRequired. Name of the origin volume for the cache volume."]
    pub fn set_peer_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_volume_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peering_command_expiry_time`.\nOptional. Expiration time for the peering command to be executed on user's ONTAP. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn set_peering_command_expiry_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.peering_command_expiry_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_config`.\n"]
    pub fn set_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_CacheParameters_CacheConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetappVolume_CacheParameters {
    type O = terrars::BlockAssignable<NetappVolume_CacheParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_CacheParameters {}

impl BuildNetappVolume_CacheParameters {
    pub fn build(self) -> NetappVolume_CacheParameters {
        NetappVolume_CacheParameters {
            enable_global_file_lock: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_ip_addresses: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            peering_command_expiry_time: core::default::Default::default(),
            cache_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_CacheParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_CacheParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_CacheParametersRef {
        NetappVolume_CacheParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_CacheParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_state` after provisioning.\nState of the cache volume indicating the peering status."]
    pub fn cache_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nCopy-paste-able commands to be used on user's ONTAP to accept peering requests."]
    pub fn command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_global_file_lock` after provisioning.\nOptional. Field indicating whether cache volume as global file lock enabled."]
    pub fn enable_global_file_lock(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_global_file_lock", self.base))
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\nTemporary passphrase generated to accept cluster peering command."]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_cluster_name` after provisioning.\nRequired. Name of the origin volume's ONTAP cluster."]
    pub fn peer_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip_addresses` after provisioning.\nRequired. List of IC LIF addresses of the origin volume's ONTAP cluster."]
    pub fn peer_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_ip_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_svm_name` after provisioning.\nRequired. Name of the origin volume's SVM."]
    pub fn peer_svm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_svm_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_volume_name` after provisioning.\nRequired. Name of the origin volume for the cache volume."]
    pub fn peer_volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_volume_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peering_command_expiry_time` after provisioning.\nOptional. Expiration time for the peering command to be executed on user's ONTAP. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn peering_command_expiry_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peering_command_expiry_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state_details` after provisioning.\nDetailed description of the current cache state."]
    pub fn state_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_details", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> terrars::ListRef<NetappVolume_CacheParameters_CacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_config", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_ExportPolicy_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clients: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anon_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_root_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5_read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5_read_write: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5i_read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5i_read_write: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5p_read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos5p_read_write: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfsv3: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfsv4: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_ExportPolicy_Rules {
    #[doc =
        "Set the field `access_type`.\nDefines the access type for clients matching the 'allowedClients' specification. Possible values: [\"READ_ONLY\", \"READ_WRITE\", \"READ_NONE\"]"]
    pub fn set_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_clients`.\nDefines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses."]
    pub fn set_allowed_clients(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.allowed_clients = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `anon_uid`.\nAn integer representing the anonymous user ID. Range is 0 to 4294967295. Required when 'squash_mode' is 'ALL_SQUASH'."]
    pub fn set_anon_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anon_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `has_root_access`.\nIf enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive."]
    pub fn set_has_root_access(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.has_root_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode."]
    pub fn set_kerberos5_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5_read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5_read_write(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5_read_write = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5i_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode."]
    pub fn set_kerberos5i_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5i_read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5i_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5i_read_write(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5i_read_write = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5p_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode."]
    pub fn set_kerberos5p_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5p_read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kerberos5p_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5p_read_write(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.kerberos5p_read_write = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfsv3`.\nEnable to apply the export rule to NFSV3 clients."]
    pub fn set_nfsv3(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nfsv3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfsv4`.\nEnable to apply the export rule to NFSV4.1 clients."]
    pub fn set_nfsv4(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nfsv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `squash_mode`.\nSquashMode defines how remote user privileges are restricted when accessing an NFS export. It controls how the user identities (like root) are mapped to anonymous users to limit access and enforce security.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive. Possible values: [\"SQUASH_MODE_UNSPECIFIED\", \"NO_ROOT_SQUASH\", \"ROOT_SQUASH\", \"ALL_SQUASH\"]"]
    pub fn set_squash_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.squash_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_ExportPolicy_Rules {
    type O = terrars::BlockAssignable<NetappVolume_ExportPolicy_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_ExportPolicy_Rules {}

impl BuildNetappVolume_ExportPolicy_Rules {
    pub fn build(self) -> NetappVolume_ExportPolicy_Rules {
        NetappVolume_ExportPolicy_Rules {
            access_type: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            has_root_access: core::default::Default::default(),
            kerberos5_read_only: core::default::Default::default(),
            kerberos5_read_write: core::default::Default::default(),
            kerberos5i_read_only: core::default::Default::default(),
            kerberos5i_read_write: core::default::Default::default(),
            kerberos5p_read_only: core::default::Default::default(),
            kerberos5p_read_write: core::default::Default::default(),
            nfsv3: core::default::Default::default(),
            nfsv4: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_ExportPolicy_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_ExportPolicy_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_ExportPolicy_RulesRef {
        NetappVolume_ExportPolicy_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_ExportPolicy_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_type` after provisioning.\nDefines the access type for clients matching the 'allowedClients' specification. Possible values: [\"READ_ONLY\", \"READ_WRITE\", \"READ_NONE\"]"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_clients` after provisioning.\nDefines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses."]
    pub fn allowed_clients(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `anon_uid` after provisioning.\nAn integer representing the anonymous user ID. Range is 0 to 4294967295. Required when 'squash_mode' is 'ALL_SQUASH'."]
    pub fn anon_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `has_root_access` after provisioning.\nIf enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive."]
    pub fn has_root_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_root_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode."]
    pub fn kerberos5_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5_read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5_read_write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5_read_write", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5i_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode."]
    pub fn kerberos5i_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5i_read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5i_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5i_read_write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5i_read_write", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5p_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode."]
    pub fn kerberos5p_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5p_read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kerberos5p_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5p_read_write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos5p_read_write", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nfsv3` after provisioning.\nEnable to apply the export rule to NFSV3 clients."]
    pub fn nfsv3(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nfsv3", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nfsv4` after provisioning.\nEnable to apply the export rule to NFSV4.1 clients."]
    pub fn nfsv4(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nfsv4", self.base))
    }

    #[doc =
        "Get a reference to the value of field `squash_mode` after provisioning.\nSquashMode defines how remote user privileges are restricted when accessing an NFS export. It controls how the user identities (like root) are mapped to anonymous users to limit access and enforce security.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive. Possible values: [\"SQUASH_MODE_UNSPECIFIED\", \"NO_ROOT_SQUASH\", \"ROOT_SQUASH\", \"ALL_SQUASH\"]"]
    pub fn squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetappVolume_ExportPolicyDynamic {
    rules: std::option::Option<terrars::DynamicBlock<NetappVolume_ExportPolicy_Rules>>,
}

#[derive(Serialize)]
pub struct NetappVolume_ExportPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<NetappVolume_ExportPolicy_Rules>>,
    dynamic: NetappVolume_ExportPolicyDynamic,
}

impl NetappVolume_ExportPolicy {
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_ExportPolicy_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetappVolume_ExportPolicy {
    type O = terrars::BlockAssignable<NetappVolume_ExportPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_ExportPolicy {}

impl BuildNetappVolume_ExportPolicy {
    pub fn build(self) -> NetappVolume_ExportPolicy {
        NetappVolume_ExportPolicy {
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_ExportPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_ExportPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_ExportPolicyRef {
        NetappVolume_ExportPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_ExportPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<NetappVolume_ExportPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_HybridReplicationParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hybrid_replication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_volume_constituent_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_svm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_HybridReplicationParameters {
    #[doc =
        "Set the field `cluster_location`.\nOptional. Name of source cluster location associated with the replication. This is a free-form field\nfor display purposes only."]
    pub fn set_cluster_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. Description of the replication."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hybrid_replication_type`.\nOptional. Type of the hybrid replication. Use 'MIGRATION' to create a volume migration\nand 'ONPREM_REPLICATION' to create an external replication.\nOther values are read-only. 'REVERSE_ONPREM_REPLICATION' is used to represent an external\nreplication which got reversed. Default is 'MIGRATION'. Possible values: [\"MIGRATION\", \"CONTINUOUS_REPLICATION\", \"ONPREM_REPLICATION\", \"REVERSE_ONPREM_REPLICATION\"]"]
    pub fn set_hybrid_replication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hybrid_replication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels to be added to the replication as the key value pairs.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `large_volume_constituent_count`.\nOptional. If the source is a FlexGroup volume, this field needs to match the number of constituents in the FlexGroup."]
    pub fn set_large_volume_constituent_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.large_volume_constituent_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_cluster_name`.\nRequired. Name of the ONTAP source cluster to be peered with NetApp Volumes."]
    pub fn set_peer_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_ip_addresses`.\nRequired. List of all intercluster LIF IP addresses of the ONTAP source cluster."]
    pub fn set_peer_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.peer_ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_svm_name`.\nRequired. Name of the ONTAP source vserver SVM to be peered with NetApp Volumes."]
    pub fn set_peer_svm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_svm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `peer_volume_name`.\nRequired. Name of the ONTAP source volume to be replicated to NetApp Volumes destination volume."]
    pub fn set_peer_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_volume_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication`.\nRequired. Desired name for the replication of this volume."]
    pub fn set_replication(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replication = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replication_schedule`.\nOptional. Replication Schedule for the replication created. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn set_replication_schedule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replication_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_HybridReplicationParameters {
    type O = terrars::BlockAssignable<NetappVolume_HybridReplicationParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_HybridReplicationParameters {}

impl BuildNetappVolume_HybridReplicationParameters {
    pub fn build(self) -> NetappVolume_HybridReplicationParameters {
        NetappVolume_HybridReplicationParameters {
            cluster_location: core::default::Default::default(),
            description: core::default::Default::default(),
            hybrid_replication_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            large_volume_constituent_count: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_ip_addresses: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            replication: core::default::Default::default(),
            replication_schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_HybridReplicationParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_HybridReplicationParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_HybridReplicationParametersRef {
        NetappVolume_HybridReplicationParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_HybridReplicationParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_location` after provisioning.\nOptional. Name of source cluster location associated with the replication. This is a free-form field\nfor display purposes only."]
    pub fn cluster_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the replication."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nOptional. Type of the hybrid replication. Use 'MIGRATION' to create a volume migration\nand 'ONPREM_REPLICATION' to create an external replication.\nOther values are read-only. 'REVERSE_ONPREM_REPLICATION' is used to represent an external\nreplication which got reversed. Default is 'MIGRATION'. Possible values: [\"MIGRATION\", \"CONTINUOUS_REPLICATION\", \"ONPREM_REPLICATION\", \"REVERSE_ONPREM_REPLICATION\"]"]
    pub fn hybrid_replication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hybrid_replication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to be added to the replication as the key value pairs.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `large_volume_constituent_count` after provisioning.\nOptional. If the source is a FlexGroup volume, this field needs to match the number of constituents in the FlexGroup."]
    pub fn large_volume_constituent_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.large_volume_constituent_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_cluster_name` after provisioning.\nRequired. Name of the ONTAP source cluster to be peered with NetApp Volumes."]
    pub fn peer_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_ip_addresses` after provisioning.\nRequired. List of all intercluster LIF IP addresses of the ONTAP source cluster."]
    pub fn peer_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_ip_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_svm_name` after provisioning.\nRequired. Name of the ONTAP source vserver SVM to be peered with NetApp Volumes."]
    pub fn peer_svm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_svm_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_volume_name` after provisioning.\nRequired. Name of the ONTAP source volume to be replicated to NetApp Volumes destination volume."]
    pub fn peer_volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_volume_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replication` after provisioning.\nRequired. Desired name for the replication of this volume."]
    pub fn replication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replication_schedule` after provisioning.\nOptional. Replication Schedule for the replication created. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_LargeCapacityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constituent_count: std::option::Option<terrars::PrimField<f64>>,
}

impl NetappVolume_LargeCapacityConfig {
    #[doc =
        "Set the field `constituent_count`.\nThe number of internal constituents (e.g., FlexVols) for this large volume.\nThe minimum number of constituents is 2."]
    pub fn set_constituent_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constituent_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_LargeCapacityConfig {
    type O = terrars::BlockAssignable<NetappVolume_LargeCapacityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_LargeCapacityConfig {}

impl BuildNetappVolume_LargeCapacityConfig {
    pub fn build(self) -> NetappVolume_LargeCapacityConfig {
        NetappVolume_LargeCapacityConfig { constituent_count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_LargeCapacityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_LargeCapacityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_LargeCapacityConfigRef {
        NetappVolume_LargeCapacityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_LargeCapacityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `constituent_count` after provisioning.\nThe number of internal constituents (e.g., FlexVols) for this large volume.\nThe minimum number of constituents is 2."]
    pub fn constituent_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constituent_count", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_RestoreParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_RestoreParameters {
    #[doc =
        "Set the field `source_backup`.\nFull name of the backup to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/backupVaults/{{backupVaultId}}/backups/{{backup}}'."]
    pub fn set_source_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_snapshot`.\nFull name of the snapshot to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/volumes/{{volume}}/snapshots/{{snapshot}}'."]
    pub fn set_source_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_snapshot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_RestoreParameters {
    type O = terrars::BlockAssignable<NetappVolume_RestoreParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_RestoreParameters {}

impl BuildNetappVolume_RestoreParameters {
    pub fn build(self) -> NetappVolume_RestoreParameters {
        NetappVolume_RestoreParameters {
            source_backup: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_RestoreParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_RestoreParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_RestoreParametersRef {
        NetappVolume_RestoreParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_RestoreParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_backup` after provisioning.\nFull name of the backup to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/backupVaults/{{backupVaultId}}/backups/{{backup}}'."]
    pub fn source_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backup", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nFull name of the snapshot to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/volumes/{{volume}}/snapshots/{{snapshot}}'."]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_SnapshotPolicy_DailySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute: std::option::Option<terrars::PrimField<f64>>,
    snapshots_to_keep: terrars::PrimField<f64>,
}

impl NetappVolume_SnapshotPolicy_DailySchedule {
    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_SnapshotPolicy_DailySchedule {
    type O = terrars::BlockAssignable<NetappVolume_SnapshotPolicy_DailySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_SnapshotPolicy_DailySchedule {
    #[doc = "The maximum number of snapshots to keep for the daily schedule."]
    pub snapshots_to_keep: terrars::PrimField<f64>,
}

impl BuildNetappVolume_SnapshotPolicy_DailySchedule {
    pub fn build(self) -> NetappVolume_SnapshotPolicy_DailySchedule {
        NetappVolume_SnapshotPolicy_DailySchedule {
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_SnapshotPolicy_DailyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_SnapshotPolicy_DailyScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_SnapshotPolicy_DailyScheduleRef {
        NetappVolume_SnapshotPolicy_DailyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_SnapshotPolicy_DailyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the daily schedule."]
    pub fn snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_to_keep", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_SnapshotPolicy_HourlySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute: std::option::Option<terrars::PrimField<f64>>,
    snapshots_to_keep: terrars::PrimField<f64>,
}

impl NetappVolume_SnapshotPolicy_HourlySchedule {
    #[doc =
        "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_SnapshotPolicy_HourlySchedule {
    type O = terrars::BlockAssignable<NetappVolume_SnapshotPolicy_HourlySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_SnapshotPolicy_HourlySchedule {
    #[doc = "The maximum number of snapshots to keep for the hourly schedule."]
    pub snapshots_to_keep: terrars::PrimField<f64>,
}

impl BuildNetappVolume_SnapshotPolicy_HourlySchedule {
    pub fn build(self) -> NetappVolume_SnapshotPolicy_HourlySchedule {
        NetappVolume_SnapshotPolicy_HourlySchedule {
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_SnapshotPolicy_HourlyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_SnapshotPolicy_HourlyScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_SnapshotPolicy_HourlyScheduleRef {
        NetappVolume_SnapshotPolicy_HourlyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_SnapshotPolicy_HourlyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the hourly schedule."]
    pub fn snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_to_keep", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_SnapshotPolicy_MonthlySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_month: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute: std::option::Option<terrars::PrimField<f64>>,
    snapshots_to_keep: terrars::PrimField<f64>,
}

impl NetappVolume_SnapshotPolicy_MonthlySchedule {
    #[doc =
        "Set the field `days_of_month`.\nSet the day or days of the month to make a snapshot (1-31). Accepts a comma separated number of days. Defaults to '1'."]
    pub fn set_days_of_month(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.days_of_month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_SnapshotPolicy_MonthlySchedule {
    type O = terrars::BlockAssignable<NetappVolume_SnapshotPolicy_MonthlySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_SnapshotPolicy_MonthlySchedule {
    #[doc = "The maximum number of snapshots to keep for the monthly schedule"]
    pub snapshots_to_keep: terrars::PrimField<f64>,
}

impl BuildNetappVolume_SnapshotPolicy_MonthlySchedule {
    pub fn build(self) -> NetappVolume_SnapshotPolicy_MonthlySchedule {
        NetappVolume_SnapshotPolicy_MonthlySchedule {
            days_of_month: core::default::Default::default(),
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_SnapshotPolicy_MonthlyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_SnapshotPolicy_MonthlyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetappVolume_SnapshotPolicy_MonthlyScheduleRef {
        NetappVolume_SnapshotPolicy_MonthlyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_SnapshotPolicy_MonthlyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_month` after provisioning.\nSet the day or days of the month to make a snapshot (1-31). Accepts a comma separated number of days. Defaults to '1'."]
    pub fn days_of_month(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_of_month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the monthly schedule"]
    pub fn snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_to_keep", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_SnapshotPolicy_WeeklySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute: std::option::Option<terrars::PrimField<f64>>,
    snapshots_to_keep: terrars::PrimField<f64>,
}

impl NetappVolume_SnapshotPolicy_WeeklySchedule {
    #[doc =
        "Set the field `day`.\nSet the day or days of the week to make a snapshot. Accepts a comma separated days of the week. Defaults to 'Sunday'."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_SnapshotPolicy_WeeklySchedule {
    type O = terrars::BlockAssignable<NetappVolume_SnapshotPolicy_WeeklySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_SnapshotPolicy_WeeklySchedule {
    #[doc = "The maximum number of snapshots to keep for the weekly schedule."]
    pub snapshots_to_keep: terrars::PrimField<f64>,
}

impl BuildNetappVolume_SnapshotPolicy_WeeklySchedule {
    pub fn build(self) -> NetappVolume_SnapshotPolicy_WeeklySchedule {
        NetappVolume_SnapshotPolicy_WeeklySchedule {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_SnapshotPolicy_WeeklyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_SnapshotPolicy_WeeklyScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_SnapshotPolicy_WeeklyScheduleRef {
        NetappVolume_SnapshotPolicy_WeeklyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_SnapshotPolicy_WeeklyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nSet the day or days of the week to make a snapshot. Accepts a comma separated days of the week. Defaults to 'Sunday'."]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the weekly schedule."]
    pub fn snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_to_keep", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetappVolume_SnapshotPolicyDynamic {
    daily_schedule: std::option::Option<terrars::DynamicBlock<NetappVolume_SnapshotPolicy_DailySchedule>>,
    hourly_schedule: std::option::Option<terrars::DynamicBlock<NetappVolume_SnapshotPolicy_HourlySchedule>>,
    monthly_schedule: std::option::Option<terrars::DynamicBlock<NetappVolume_SnapshotPolicy_MonthlySchedule>>,
    weekly_schedule: std::option::Option<terrars::DynamicBlock<NetappVolume_SnapshotPolicy_WeeklySchedule>>,
}

#[derive(Serialize)]
pub struct NetappVolume_SnapshotPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std::option::Option<std::vec::Vec<NetappVolume_SnapshotPolicy_DailySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_schedule: std::option::Option<std::vec::Vec<NetappVolume_SnapshotPolicy_HourlySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_schedule: std::option::Option<std::vec::Vec<NetappVolume_SnapshotPolicy_MonthlySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<std::vec::Vec<NetappVolume_SnapshotPolicy_WeeklySchedule>>,
    dynamic: NetappVolume_SnapshotPolicyDynamic,
}

impl NetappVolume_SnapshotPolicy {
    #[doc =
        "Set the field `enabled`.\nEnables automated snapshot creation according to defined schedule. Default is false.\nTo disable automatic snapshot creation you have to remove the whole snapshot_policy block."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_SnapshotPolicy_DailySchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hourly_schedule`.\n"]
    pub fn set_hourly_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_SnapshotPolicy_HourlySchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hourly_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monthly_schedule`.\n"]
    pub fn set_monthly_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_SnapshotPolicy_MonthlySchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monthly_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetappVolume_SnapshotPolicy_WeeklySchedule>>,
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

impl terrars::ToListMappable for NetappVolume_SnapshotPolicy {
    type O = terrars::BlockAssignable<NetappVolume_SnapshotPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_SnapshotPolicy {}

impl BuildNetappVolume_SnapshotPolicy {
    pub fn build(self) -> NetappVolume_SnapshotPolicy {
        NetappVolume_SnapshotPolicy {
            enabled: core::default::Default::default(),
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            monthly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_SnapshotPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_SnapshotPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_SnapshotPolicyRef {
        NetappVolume_SnapshotPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_SnapshotPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables automated snapshot creation according to defined schedule. Default is false.\nTo disable automatic snapshot creation you have to remove the whole snapshot_policy block."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicy_DailyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicy_HourlyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_schedule` after provisioning.\n"]
    pub fn monthly_schedule(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicy_MonthlyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> terrars::ListRef<NetappVolume_SnapshotPolicy_WeeklyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_TieringPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooling_threshold_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hot_tier_bypass_mode_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_TieringPolicy {
    #[doc =
        "Set the field `cooling_threshold_days`.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn set_cooling_threshold_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooling_threshold_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hot_tier_bypass_mode_enabled`.\nOptional. Flag indicating that the hot tier bypass mode is enabled. Default is false.\nOnly applicable to Flex service level."]
    pub fn set_hot_tier_bypass_mode_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hot_tier_bypass_mode_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tier_action`.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn set_tier_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetappVolume_TieringPolicy {
    type O = terrars::BlockAssignable<NetappVolume_TieringPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_TieringPolicy {}

impl BuildNetappVolume_TieringPolicy {
    pub fn build(self) -> NetappVolume_TieringPolicy {
        NetappVolume_TieringPolicy {
            cooling_threshold_days: core::default::Default::default(),
            hot_tier_bypass_mode_enabled: core::default::Default::default(),
            tier_action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_TieringPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_TieringPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_TieringPolicyRef {
        NetappVolume_TieringPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_TieringPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cooling_threshold_days` after provisioning.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn cooling_threshold_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooling_threshold_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hot_tier_bypass_mode_enabled` after provisioning.\nOptional. Flag indicating that the hot tier bypass mode is enabled. Default is false.\nOnly applicable to Flex service level."]
    pub fn hot_tier_bypass_mode_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hot_tier_bypass_mode_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tier_action` after provisioning.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn tier_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier_action", self.base))
    }
}

#[derive(Serialize)]
pub struct NetappVolume_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetappVolume_Timeouts {
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

impl terrars::ToListMappable for NetappVolume_Timeouts {
    type O = terrars::BlockAssignable<NetappVolume_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappVolume_Timeouts {}

impl BuildNetappVolume_Timeouts {
    pub fn build(self) -> NetappVolume_Timeouts {
        NetappVolume_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetappVolume_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetappVolume_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetappVolume_TimeoutsRef {
        NetappVolume_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappVolume_TimeoutsRef {
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
