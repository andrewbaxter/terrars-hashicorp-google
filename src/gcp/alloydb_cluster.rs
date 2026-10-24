use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AlloydbClusterData {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_backup_policy: std::option::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_backup_config: std::option::Option<std::vec::Vec<AlloydbClusterContinuousBackupConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbClusterEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_user: std::option::Option<std::vec::Vec<AlloydbClusterInitialUserEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<AlloydbClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_source: std::option::Option<std::vec::Vec<AlloydbClusterRestoreBackupSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_continuous_backup_source: std::option::Option<std::vec::Vec<AlloydbClusterRestoreContinuousBackupSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_config: std::option::Option<std::vec::Vec<AlloydbClusterSecondaryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AlloydbClusterTimeoutsEl>,
    dynamic: AlloydbClusterDynamic,
}

struct AlloydbCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlloydbClusterData>,
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
        "Set the field `deletion_policy`.\nPolicy to determine if the cluster should be deleted forcefully.\nDeleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.\nDeleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = \"FORCE\" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance."]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `network`.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automated_backup_policy`.\n"]
    pub fn set_automated_backup_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterContinuousBackupConfigEl>>,
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

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterEncryptionConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterInitialUserEl>>,
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

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterNetworkConfigEl>>,
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

    #[doc = "Set the field `restore_backup_source`.\n"]
    pub fn set_restore_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterRestoreBackupSourceEl>>,
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

    #[doc = "Set the field `restore_continuous_backup_source`.\n"]
    pub fn set_restore_continuous_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterRestoreContinuousBackupSourceEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterSecondaryConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<AlloydbClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> terrars::ListRef<AlloydbClusterBackupSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
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
    pub fn continuous_backup_info(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an output-only field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nPolicy to determine if the cluster should be deleted forcefully.\nDeleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.\nDeleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = \"FORCE\" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbClusterEncryptionInfoElRef> {
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
    pub fn migration_source(&self) -> terrars::ListRef<AlloydbClusterMigrationSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
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
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\n"]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\n"]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbClusterEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(&self) -> terrars::ListRef<AlloydbClusterInitialUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\n"]
    pub fn restore_backup_source(&self) -> terrars::ListRef<AlloydbClusterRestoreBackupSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\n"]
    pub fn restore_continuous_backup_source(&self) -> terrars::ListRef<AlloydbClusterRestoreContinuousBackupSourceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\n"]
    pub fn secondary_config(&self) -> terrars::ListRef<AlloydbClusterSecondaryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbClusterTimeoutsElRef {
        AlloydbClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(AlloydbClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                cluster_id: self.cluster_id,
                cluster_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                automated_backup_policy: core::default::Default::default(),
                continuous_backup_config: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                initial_user: core::default::Default::default(),
                network_config: core::default::Default::default(),
                restore_backup_source: core::default::Default::default(),
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
    pub fn backup_source(&self) -> terrars::ListRef<AlloydbClusterBackupSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_source", self.extract_ref()))
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
    pub fn continuous_backup_info(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an output-only field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation."]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nPolicy to determine if the cluster should be deleted forcefully.\nDeleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.\nDeleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = \"FORCE\" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbClusterEncryptionInfoElRef> {
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
    pub fn migration_source(&self) -> terrars::ListRef<AlloydbClusterMigrationSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.migration_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
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
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\n"]
    pub fn automated_backup_policy(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\n"]
    pub fn continuous_backup_config(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbClusterEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(&self) -> terrars::ListRef<AlloydbClusterInitialUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\n"]
    pub fn restore_backup_source(&self) -> terrars::ListRef<AlloydbClusterRestoreBackupSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\n"]
    pub fn restore_continuous_backup_source(&self) -> terrars::ListRef<AlloydbClusterRestoreContinuousBackupSourceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\n"]
    pub fn secondary_config(&self) -> terrars::ListRef<AlloydbClusterSecondaryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbClusterTimeoutsElRef {
        AlloydbClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterBackupSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterBackupSourceEl {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterBackupSourceEl {
    type O = terrars::BlockAssignable<AlloydbClusterBackupSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterBackupSourceEl {}

impl BuildAlloydbClusterBackupSourceEl {
    pub fn build(self) -> AlloydbClusterBackupSourceEl {
        AlloydbClusterBackupSourceEl { backup_name: core::default::Default::default() }
    }
}

pub struct AlloydbClusterBackupSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterBackupSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterBackupSourceElRef {
        AlloydbClusterBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterBackupSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_name` after provisioning.\n"]
    pub fn backup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
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

impl terrars::ToListMappable for AlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    type O = terrars::BlockAssignable<AlloydbClusterContinuousBackupInfoElEncryptionInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {}

impl BuildAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    pub fn build(self) -> AlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
        AlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
        AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
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
pub struct AlloydbClusterContinuousBackupInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    earliest_restorable_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_info: std::option::Option<terrars::ListField<AlloydbClusterContinuousBackupInfoElEncryptionInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbClusterContinuousBackupInfoEl {
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
        v: impl std::convert::Into<terrars::ListField<AlloydbClusterContinuousBackupInfoElEncryptionInfoEl>>,
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

impl terrars::ToListMappable for AlloydbClusterContinuousBackupInfoEl {
    type O = terrars::BlockAssignable<AlloydbClusterContinuousBackupInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterContinuousBackupInfoEl {}

impl BuildAlloydbClusterContinuousBackupInfoEl {
    pub fn build(self) -> AlloydbClusterContinuousBackupInfoEl {
        AlloydbClusterContinuousBackupInfoEl {
            earliest_restorable_time: core::default::Default::default(),
            enabled_time: core::default::Default::default(),
            encryption_info: core::default::Default::default(),
            schedule: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterContinuousBackupInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterContinuousBackupInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterContinuousBackupInfoElRef {
        AlloydbClusterContinuousBackupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterContinuousBackupInfoElRef {
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
    pub fn encryption_info(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupInfoElEncryptionInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterEncryptionInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbClusterEncryptionInfoEl {
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

impl terrars::ToListMappable for AlloydbClusterEncryptionInfoEl {
    type O = terrars::BlockAssignable<AlloydbClusterEncryptionInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterEncryptionInfoEl {}

impl BuildAlloydbClusterEncryptionInfoEl {
    pub fn build(self) -> AlloydbClusterEncryptionInfoEl {
        AlloydbClusterEncryptionInfoEl {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterEncryptionInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterEncryptionInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterEncryptionInfoElRef {
        AlloydbClusterEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterEncryptionInfoElRef {
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
pub struct AlloydbClusterMigrationSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterMigrationSourceEl {
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

impl terrars::ToListMappable for AlloydbClusterMigrationSourceEl {
    type O = terrars::BlockAssignable<AlloydbClusterMigrationSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterMigrationSourceEl {}

impl BuildAlloydbClusterMigrationSourceEl {
    pub fn build(self) -> AlloydbClusterMigrationSourceEl {
        AlloydbClusterMigrationSourceEl {
            host_port: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            source_type: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterMigrationSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterMigrationSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterMigrationSourceElRef {
        AlloydbClusterMigrationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterMigrationSourceElRef {
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
pub struct AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
        AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl { kms_key_name: core::default::Default::default() }
    }
}

pub struct AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
        AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
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
pub struct AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    #[doc = "Set the field `count`.\nThe number of backups to retain."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
        AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl { count: core::default::Default::default() }
    }
}

pub struct AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
        AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of backups to retain."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    #[doc =
        "Set the field `retention_period`.\nThe retention period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
        AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
            retention_period: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
        AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
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
pub struct AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
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

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
        AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
        AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
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
struct AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElDynamic {
    start_times: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>>,
}

#[derive(Serialize)]
pub struct AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_times: std::option::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>>,
    dynamic: AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElDynamic,
}

impl AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
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
                    ::Into<
                        terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>,
                    >,
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

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
        AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
            days_of_week: core::default::Default::default(),
            start_times: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
        AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nThe days of the week to perform a backup. At least one day of the week must be provided. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_times` after provisioning.\n"]
    pub fn start_times(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_times", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbClusterAutomatedBackupPolicyElDynamic {
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>>,
    quantity_based_retention: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>>,
    time_based_retention: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>>,
    weekly_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>>,
}

#[derive(Serialize)]
pub struct AlloydbClusterAutomatedBackupPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity_based_retention: std
    ::option
    ::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_retention: std::option::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<std::vec::Vec<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>>,
    dynamic: AlloydbClusterAutomatedBackupPolicyElDynamic,
}

impl AlloydbClusterAutomatedBackupPolicyEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>>,
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
                    ::Into<terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>>,
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
                    ::Into<terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>>,
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

impl terrars::ToListMappable for AlloydbClusterAutomatedBackupPolicyEl {
    type O = terrars::BlockAssignable<AlloydbClusterAutomatedBackupPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterAutomatedBackupPolicyEl {}

impl BuildAlloydbClusterAutomatedBackupPolicyEl {
    pub fn build(self) -> AlloydbClusterAutomatedBackupPolicyEl {
        AlloydbClusterAutomatedBackupPolicyEl {
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

pub struct AlloydbClusterAutomatedBackupPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterAutomatedBackupPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterAutomatedBackupPolicyElRef {
        AlloydbClusterAutomatedBackupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterAutomatedBackupPolicyElRef {
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
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity_based_retention` after provisioning.\n"]
    pub fn quantity_based_retention(
        &self,
    ) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quantity_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_retention` after provisioning.\n"]
    pub fn time_based_retention(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> terrars::ListRef<AlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterContinuousBackupConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {}

impl BuildAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    pub fn build(self) -> AlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
        AlloydbClusterContinuousBackupConfigElEncryptionConfigEl { kms_key_name: core::default::Default::default() }
    }
}

pub struct AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
        AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbClusterContinuousBackupConfigElDynamic {
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterContinuousBackupConfigElEncryptionConfigEl>>,
}

#[derive(Serialize)]
pub struct AlloydbClusterContinuousBackupConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_window_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<AlloydbClusterContinuousBackupConfigElEncryptionConfigEl>>,
    dynamic: AlloydbClusterContinuousBackupConfigElDynamic,
}

impl AlloydbClusterContinuousBackupConfigEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbClusterContinuousBackupConfigElEncryptionConfigEl>>,
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

impl terrars::ToListMappable for AlloydbClusterContinuousBackupConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterContinuousBackupConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterContinuousBackupConfigEl {}

impl BuildAlloydbClusterContinuousBackupConfigEl {
    pub fn build(self) -> AlloydbClusterContinuousBackupConfigEl {
        AlloydbClusterContinuousBackupConfigEl {
            enabled: core::default::Default::default(),
            recovery_window_days: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterContinuousBackupConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterContinuousBackupConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterContinuousBackupConfigElRef {
        AlloydbClusterContinuousBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterContinuousBackupConfigElRef {
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
    pub fn encryption_config(&self) -> terrars::ListRef<AlloydbClusterContinuousBackupConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterEncryptionConfigEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterEncryptionConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterEncryptionConfigEl {}

impl BuildAlloydbClusterEncryptionConfigEl {
    pub fn build(self) -> AlloydbClusterEncryptionConfigEl {
        AlloydbClusterEncryptionConfigEl { kms_key_name: core::default::Default::default() }
    }
}

pub struct AlloydbClusterEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterEncryptionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterEncryptionConfigElRef {
        AlloydbClusterEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterEncryptionConfigElRef {
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
pub struct AlloydbClusterInitialUserEl {
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterInitialUserEl {
    #[doc = "Set the field `user`.\nThe database username."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbClusterInitialUserEl {
    type O = terrars::BlockAssignable<AlloydbClusterInitialUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterInitialUserEl {
    #[doc = "The initial password for the user."]
    pub password: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbClusterInitialUserEl {
    pub fn build(self) -> AlloydbClusterInitialUserEl {
        AlloydbClusterInitialUserEl {
            password: self.password,
            user: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterInitialUserElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterInitialUserElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterInitialUserElRef {
        AlloydbClusterInitialUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterInitialUserElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe initial password for the user."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nThe database username."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterNetworkConfigEl {
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

impl terrars::ToListMappable for AlloydbClusterNetworkConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterNetworkConfigEl {}

impl BuildAlloydbClusterNetworkConfigEl {
    pub fn build(self) -> AlloydbClusterNetworkConfigEl {
        AlloydbClusterNetworkConfigEl {
            allocated_ip_range: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterNetworkConfigElRef {
        AlloydbClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterNetworkConfigElRef {
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
pub struct AlloydbClusterRestoreBackupSourceEl {
    backup_name: terrars::PrimField<std::string::String>,
}

impl AlloydbClusterRestoreBackupSourceEl { }

impl terrars::ToListMappable for AlloydbClusterRestoreBackupSourceEl {
    type O = terrars::BlockAssignable<AlloydbClusterRestoreBackupSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterRestoreBackupSourceEl {
    #[doc = "The name of the backup that this cluster is restored from."]
    pub backup_name: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbClusterRestoreBackupSourceEl {
    pub fn build(self) -> AlloydbClusterRestoreBackupSourceEl {
        AlloydbClusterRestoreBackupSourceEl { backup_name: self.backup_name }
    }
}

pub struct AlloydbClusterRestoreBackupSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterRestoreBackupSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterRestoreBackupSourceElRef {
        AlloydbClusterRestoreBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterRestoreBackupSourceElRef {
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
pub struct AlloydbClusterRestoreContinuousBackupSourceEl {
    cluster: terrars::PrimField<std::string::String>,
    point_in_time: terrars::PrimField<std::string::String>,
}

impl AlloydbClusterRestoreContinuousBackupSourceEl { }

impl terrars::ToListMappable for AlloydbClusterRestoreContinuousBackupSourceEl {
    type O = terrars::BlockAssignable<AlloydbClusterRestoreContinuousBackupSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterRestoreContinuousBackupSourceEl {
    #[doc = "The name of the source cluster that this cluster is restored from."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The point in time that this cluster is restored to, in RFC 3339 format."]
    pub point_in_time: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbClusterRestoreContinuousBackupSourceEl {
    pub fn build(self) -> AlloydbClusterRestoreContinuousBackupSourceEl {
        AlloydbClusterRestoreContinuousBackupSourceEl {
            cluster: self.cluster,
            point_in_time: self.point_in_time,
        }
    }
}

pub struct AlloydbClusterRestoreContinuousBackupSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterRestoreContinuousBackupSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbClusterRestoreContinuousBackupSourceElRef {
        AlloydbClusterRestoreContinuousBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterRestoreContinuousBackupSourceElRef {
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
pub struct AlloydbClusterSecondaryConfigEl {
    primary_cluster_name: terrars::PrimField<std::string::String>,
}

impl AlloydbClusterSecondaryConfigEl { }

impl terrars::ToListMappable for AlloydbClusterSecondaryConfigEl {
    type O = terrars::BlockAssignable<AlloydbClusterSecondaryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterSecondaryConfigEl {
    #[doc =
        "Name of the primary cluster must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub primary_cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbClusterSecondaryConfigEl {
    pub fn build(self) -> AlloydbClusterSecondaryConfigEl {
        AlloydbClusterSecondaryConfigEl { primary_cluster_name: self.primary_cluster_name }
    }
}

pub struct AlloydbClusterSecondaryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterSecondaryConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterSecondaryConfigElRef {
        AlloydbClusterSecondaryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterSecondaryConfigElRef {
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
pub struct AlloydbClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbClusterTimeoutsEl {
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

impl terrars::ToListMappable for AlloydbClusterTimeoutsEl {
    type O = terrars::BlockAssignable<AlloydbClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbClusterTimeoutsEl {}

impl BuildAlloydbClusterTimeoutsEl {
    pub fn build(self) -> AlloydbClusterTimeoutsEl {
        AlloydbClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AlloydbClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbClusterTimeoutsElRef {
        AlloydbClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbClusterTimeoutsElRef {
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
struct AlloydbClusterDynamic {
    automated_backup_policy: std::option::Option<terrars::DynamicBlock<AlloydbClusterAutomatedBackupPolicyEl>>,
    continuous_backup_config: std::option::Option<terrars::DynamicBlock<AlloydbClusterContinuousBackupConfigEl>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<AlloydbClusterEncryptionConfigEl>>,
    initial_user: std::option::Option<terrars::DynamicBlock<AlloydbClusterInitialUserEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<AlloydbClusterNetworkConfigEl>>,
    restore_backup_source: std::option::Option<terrars::DynamicBlock<AlloydbClusterRestoreBackupSourceEl>>,
    restore_continuous_backup_source: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbClusterRestoreContinuousBackupSourceEl>>,
    secondary_config: std::option::Option<terrars::DynamicBlock<AlloydbClusterSecondaryConfigEl>>,
}
