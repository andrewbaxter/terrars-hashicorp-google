#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MemorystoreInstanceData_ {
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
    acl_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_configs: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    shard_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_backup_config: std::option::Option<std::vec::Vec<MemorystoreInstance_AutomatedBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_instance_replication_config: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_CrossInstanceReplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_auto_created_endpoints: std::option::Option<std::vec::Vec<MemorystoreInstance_DesiredAutoCreatedEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_psc_auto_connections: std::option::Option<std::vec::Vec<MemorystoreInstance_DesiredPscAutoConnections>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_source: std::option::Option<std::vec::Vec<MemorystoreInstance_GcsSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<MemorystoreInstance_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_backup_source: std::option::Option<std::vec::Vec<MemorystoreInstance_ManagedBackupSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistence_config: std::option::Option<std::vec::Vec<MemorystoreInstance_PersistenceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MemorystoreInstance_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_distribution_config: std::option::Option<std::vec::Vec<MemorystoreInstance_ZoneDistributionConfig>>,
    dynamic: MemorystoreInstanceDynamic,
}

struct MemorystoreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MemorystoreInstanceData_>,
}

#[derive(Clone)]
pub struct MemorystoreInstance(std::rc::Rc<MemorystoreInstance_>);

impl MemorystoreInstance {
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
        "Set the field `acl_policy`.\nThe name of the ACL policy to attach to the instance.\nFormat: projects/{project}/locations/{location}/aclPolicies/{acl_policy}"]
    pub fn set_acl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().acl_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authorization_mode`.\nOptional. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH\nTOKEN_AUTH"]
    pub fn set_authorization_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorization_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection_enabled`.\nOptional. If set to true deletion of the instance will fail."]
    pub fn set_deletion_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_configs`.\nOptional. User-provided engine configurations for the instance."]
    pub fn set_engine_configs(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().engine_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\nOptional. Engine version of the instance."]
    pub fn set_engine_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maintenance_version`.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn set_maintenance_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_type`.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn set_node_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replica_count`.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn set_replica_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_ca_mode`.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn set_server_ca_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_ca_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn set_server_ca_pool(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transit_encryption_mode`.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn set_transit_encryption_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automated_backup_config`.\n"]
    pub fn set_automated_backup_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automated_backup_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automated_backup_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_instance_replication_config`.\n"]
    pub fn set_cross_instance_replication_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_instance_replication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cross_instance_replication_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `desired_auto_created_endpoints`.\n"]
    pub fn set_desired_auto_created_endpoints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_DesiredAutoCreatedEndpoints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_auto_created_endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.desired_auto_created_endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `desired_psc_auto_connections`.\n"]
    pub fn set_desired_psc_auto_connections(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_DesiredPscAutoConnections>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_psc_auto_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.desired_psc_auto_connections = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_source`.\n"]
    pub fn set_gcs_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_GcsSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_MaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_backup_source`.\n"]
    pub fn set_managed_backup_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_ManagedBackupSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_backup_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_backup_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `persistence_config`.\n"]
    pub fn set_persistence_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistence_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistence_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MemorystoreInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_distribution_config`.\n"]
    pub fn set_zone_distribution_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_ZoneDistributionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zone_distribution_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zone_distribution_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `acl_policy` after provisioning.\nThe name of the ACL policy to attach to the instance.\nFormat: projects/{project}/locations/{location}/aclPolicies/{acl_policy}"]
    pub fn acl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH\nTOKEN_AUTH"]
    pub fn authorization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<MemorystoreInstance_DiscoveryEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> terrars::ListRef<MemorystoreInstance_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.engine_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acl_policy_in_sync` after provisioning.\nWhether the ACL policy is in sync with the cluster."]
    pub fn is_acl_policy_in_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acl_policy_in_sync", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<MemorystoreInstance_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<MemorystoreInstance_ManagedServerCaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_server_ca", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> terrars::ListRef<MemorystoreInstance_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> terrars::ListRef<MemorystoreInstance_PscAttachmentDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_attachment_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<MemorystoreInstance_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> terrars::ListRef<MemorystoreInstance_StateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> terrars::ListRef<MemorystoreInstance_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\n"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\n"]
    pub fn desired_auto_created_endpoints(&self) -> terrars::ListRef<MemorystoreInstance_DesiredAutoCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n"]
    pub fn desired_psc_auto_connections(&self) -> terrars::ListRef<MemorystoreInstance_DesiredPscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.desired_psc_auto_connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> terrars::ListRef<MemorystoreInstance_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<MemorystoreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> terrars::ListRef<MemorystoreInstance_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> terrars::ListRef<MemorystoreInstance_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstance_TimeoutsRef {
        MemorystoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<MemorystoreInstance_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

impl terrars::Referable for MemorystoreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MemorystoreInstance { }

impl terrars::ToListMappable for MemorystoreInstance {
    type O = terrars::ListRef<MemorystoreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MemorystoreInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_memorystore_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMemorystoreInstance {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Required. Number of shards for the instance."]
    pub shard_count: terrars::PrimField<f64>,
}

impl BuildMemorystoreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> MemorystoreInstance {
        let out = MemorystoreInstance(std::rc::Rc::new(MemorystoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MemorystoreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl_policy: core::default::Default::default(),
                authorization_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                engine_configs: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                maintenance_version: core::default::Default::default(),
                mode: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                server_ca_mode: core::default::Default::default(),
                server_ca_pool: core::default::Default::default(),
                shard_count: self.shard_count,
                transit_encryption_mode: core::default::Default::default(),
                automated_backup_config: core::default::Default::default(),
                cross_instance_replication_config: core::default::Default::default(),
                desired_auto_created_endpoints: core::default::Default::default(),
                desired_psc_auto_connections: core::default::Default::default(),
                gcs_source: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                managed_backup_source: core::default::Default::default(),
                persistence_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                zone_distribution_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MemorystoreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MemorystoreInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `acl_policy` after provisioning.\nThe name of the ACL policy to attach to the instance.\nFormat: projects/{project}/locations/{location}/aclPolicies/{acl_policy}"]
    pub fn acl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. Authorization mode of the instance. Possible values:\n AUTH_DISABLED\nIAM_AUTH\nTOKEN_AUTH"]
    pub fn authorization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_maintenance_versions` after provisioning.\nThis field is used to determine the available maintenance versions for the self service update."]
    pub fn available_maintenance_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.available_maintenance_versions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `backup_collection` after provisioning.\nThe backup collection full resource name.\nExample: projects/{project}/locations/{location}/backupCollections/{collection}"]
    pub fn backup_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. If set to true deletion of the instance will fail."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<MemorystoreInstance_DiscoveryEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_maintenance_version` after provisioning.\nThis field represents the actual maintenance version of the cluster."]
    pub fn effective_maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_maintenance_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nEndpoints for the instance."]
    pub fn endpoints(&self) -> terrars::ListRef<MemorystoreInstance_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_configs` after provisioning.\nOptional. User-provided engine configurations for the instance."]
    pub fn engine_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.engine_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_version` after provisioning.\nOptional. Engine version of the instance."]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nRequired. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acl_policy_in_sync` after provisioning.\nWhether the ACL policy is in sync with the cluster."]
    pub fn is_acl_policy_in_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acl_policy_in_sync", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to represent user-provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<MemorystoreInstance_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<MemorystoreInstance_ManagedServerCaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_server_ca", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. cluster or cluster-disabled. \n Possible values:\n CLUSTER\n CLUSTER_DISABLED Possible values: [\"CLUSTER\", \"CLUSTER_DISABLED\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Unique name of the instance.\nFormat: projects/{project}/locations/{location}/instances/{instance}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_config` after provisioning.\nRepresents configuration for nodes of the instance."]
    pub fn node_config(&self) -> terrars::ListRef<MemorystoreInstance_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> terrars::ListRef<MemorystoreInstance_PscAttachmentDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_attachment_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<MemorystoreInstance_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. Number of replica nodes per shard. If omitted the default is 0 replicas."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Memorystore instance.\nIf not provided, GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"GOOGLE_MANAGED_PER_INSTANCE_CA\", \"GOOGLE_MANAGED_SHARED_CA\", \"CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the instance."]
    pub fn shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the instance. \n Possible values:\n CREATING\nACTIVE\nUPDATING\nDELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_info` after provisioning.\nAdditional information about the state of the instance."]
    pub fn state_info(&self) -> terrars::ListRef<MemorystoreInstance_StateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. Immutable. In-transit encryption mode of the instance. \n Possible values:\n TRANSIT_ENCRYPTION_DISABLED\nSERVER_AUTHENTICATION"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System assigned, unique identifier for the instance."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Latest update timestamp of the instance."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> terrars::ListRef<MemorystoreInstance_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\n"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\n"]
    pub fn desired_auto_created_endpoints(&self) -> terrars::ListRef<MemorystoreInstance_DesiredAutoCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n"]
    pub fn desired_psc_auto_connections(&self) -> terrars::ListRef<MemorystoreInstance_DesiredPscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.desired_psc_auto_connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> terrars::ListRef<MemorystoreInstance_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<MemorystoreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> terrars::ListRef<MemorystoreInstance_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> terrars::ListRef<MemorystoreInstance_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstance_TimeoutsRef {
        MemorystoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<MemorystoreInstance_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstanceDynamic {
    automated_backup_config: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_AutomatedBackupConfig>>,
    cross_instance_replication_config: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_CrossInstanceReplicationConfig>>,
    desired_auto_created_endpoints: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_DesiredAutoCreatedEndpoints>>,
    desired_psc_auto_connections: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_DesiredPscAutoConnections>>,
    gcs_source: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_GcsSource>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_MaintenancePolicy>>,
    managed_backup_source: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_ManagedBackupSource>>,
    persistence_config: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_PersistenceConfig>>,
    zone_distribution_config: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_ZoneDistributionConfig>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_DiscoveryEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl MemorystoreInstance_DiscoveryEndpoints {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_DiscoveryEndpoints {
    type O = terrars::BlockAssignable<MemorystoreInstance_DiscoveryEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_DiscoveryEndpoints {}

impl BuildMemorystoreInstance_DiscoveryEndpoints {
    pub fn build(self) -> MemorystoreInstance_DiscoveryEndpoints {
        MemorystoreInstance_DiscoveryEndpoints {
            address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_DiscoveryEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_DiscoveryEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_DiscoveryEndpointsRef {
        MemorystoreInstance_DiscoveryEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_DiscoveryEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    type O = terrars::BlockAssignable<MemorystoreInstance_Endpoints_Connections_PscAutoConnection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_Endpoints_Connections_PscAutoConnection {}

impl BuildMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    pub fn build(self) -> MemorystoreInstance_Endpoints_Connections_PscAutoConnection {
        MemorystoreInstance_Endpoints_Connections_PscAutoConnection {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
        MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_Endpoints_Connections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connection: std
    ::option
    ::Option<terrars::ListField<MemorystoreInstance_Endpoints_Connections_PscAutoConnection>>,
}

impl MemorystoreInstance_Endpoints_Connections {
    #[doc = "Set the field `psc_auto_connection`.\n"]
    pub fn set_psc_auto_connection(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MemorystoreInstance_Endpoints_Connections_PscAutoConnection>>,
    ) -> Self {
        self.psc_auto_connection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_Endpoints_Connections {
    type O = terrars::BlockAssignable<MemorystoreInstance_Endpoints_Connections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_Endpoints_Connections {}

impl BuildMemorystoreInstance_Endpoints_Connections {
    pub fn build(self) -> MemorystoreInstance_Endpoints_Connections {
        MemorystoreInstance_Endpoints_Connections { psc_auto_connection: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_Endpoints_ConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_Endpoints_ConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_Endpoints_ConnectionsRef {
        MemorystoreInstance_Endpoints_ConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_Endpoints_ConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `psc_auto_connection` after provisioning.\n"]
    pub fn psc_auto_connection(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connection", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connections: std::option::Option<terrars::ListField<MemorystoreInstance_Endpoints_Connections>>,
}

impl MemorystoreInstance_Endpoints {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MemorystoreInstance_Endpoints_Connections>>,
    ) -> Self {
        self.connections = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_Endpoints {
    type O = terrars::BlockAssignable<MemorystoreInstance_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_Endpoints {}

impl BuildMemorystoreInstance_Endpoints {
    pub fn build(self) -> MemorystoreInstance_Endpoints {
        MemorystoreInstance_Endpoints { connections: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_EndpointsRef {
        MemorystoreInstance_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> terrars::ListRef<MemorystoreInstance_Endpoints_ConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_MaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_MaintenanceSchedule {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_deadline_time`.\n"]
    pub fn set_schedule_deadline_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_deadline_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_MaintenanceSchedule {
    type O = terrars::BlockAssignable<MemorystoreInstance_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_MaintenanceSchedule {}

impl BuildMemorystoreInstance_MaintenanceSchedule {
    pub fn build(self) -> MemorystoreInstance_MaintenanceSchedule {
        MemorystoreInstance_MaintenanceSchedule {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_MaintenanceScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_MaintenanceScheduleRef {
        MemorystoreInstance_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_MaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_deadline_time` after provisioning.\n"]
    pub fn schedule_deadline_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_deadline_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_ManagedServerCa_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MemorystoreInstance_ManagedServerCa_CaCerts {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_ManagedServerCa_CaCerts {
    type O = terrars::BlockAssignable<MemorystoreInstance_ManagedServerCa_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_ManagedServerCa_CaCerts {}

impl BuildMemorystoreInstance_ManagedServerCa_CaCerts {
    pub fn build(self) -> MemorystoreInstance_ManagedServerCa_CaCerts {
        MemorystoreInstance_ManagedServerCa_CaCerts { certificates: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_ManagedServerCa_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_ManagedServerCa_CaCertsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_ManagedServerCa_CaCertsRef {
        MemorystoreInstance_ManagedServerCa_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_ManagedServerCa_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_ManagedServerCa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<MemorystoreInstance_ManagedServerCa_CaCerts>>,
}

impl MemorystoreInstance_ManagedServerCa {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MemorystoreInstance_ManagedServerCa_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_ManagedServerCa {
    type O = terrars::BlockAssignable<MemorystoreInstance_ManagedServerCa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_ManagedServerCa {}

impl BuildMemorystoreInstance_ManagedServerCa {
    pub fn build(self) -> MemorystoreInstance_ManagedServerCa {
        MemorystoreInstance_ManagedServerCa { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_ManagedServerCaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_ManagedServerCaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_ManagedServerCaRef {
        MemorystoreInstance_ManagedServerCaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_ManagedServerCaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<MemorystoreInstance_ManagedServerCa_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl MemorystoreInstance_NodeConfig {
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_NodeConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_NodeConfig {}

impl BuildMemorystoreInstance_NodeConfig {
    pub fn build(self) -> MemorystoreInstance_NodeConfig {
        MemorystoreInstance_NodeConfig { size_gb: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_NodeConfigRef {
        MemorystoreInstance_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_PscAttachmentDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_PscAttachmentDetails {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_PscAttachmentDetails {
    type O = terrars::BlockAssignable<MemorystoreInstance_PscAttachmentDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_PscAttachmentDetails {}

impl BuildMemorystoreInstance_PscAttachmentDetails {
    pub fn build(self) -> MemorystoreInstance_PscAttachmentDetails {
        MemorystoreInstance_PscAttachmentDetails {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_PscAttachmentDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_PscAttachmentDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_PscAttachmentDetailsRef {
        MemorystoreInstance_PscAttachmentDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_PscAttachmentDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_PscAutoConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_PscAutoConnections {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_status`.\n"]
    pub fn set_psc_connection_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.psc_connection_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_PscAutoConnections {
    type O = terrars::BlockAssignable<MemorystoreInstance_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_PscAutoConnections {}

impl BuildMemorystoreInstance_PscAutoConnections {
    pub fn build(self) -> MemorystoreInstance_PscAutoConnections {
        MemorystoreInstance_PscAutoConnections {
            connection_type: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            psc_connection_status: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_PscAutoConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_PscAutoConnectionsRef {
        MemorystoreInstance_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_PscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\n"]
    pub fn psc_connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_status", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_StateInfo_UpdateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_shard_count: std::option::Option<terrars::PrimField<f64>>,
}

impl MemorystoreInstance_StateInfo_UpdateInfo {
    #[doc = "Set the field `target_engine_version`.\n"]
    pub fn set_target_engine_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_node_type`.\n"]
    pub fn set_target_node_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_replica_count`.\n"]
    pub fn set_target_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_shard_count`.\n"]
    pub fn set_target_shard_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_shard_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_StateInfo_UpdateInfo {
    type O = terrars::BlockAssignable<MemorystoreInstance_StateInfo_UpdateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_StateInfo_UpdateInfo {}

impl BuildMemorystoreInstance_StateInfo_UpdateInfo {
    pub fn build(self) -> MemorystoreInstance_StateInfo_UpdateInfo {
        MemorystoreInstance_StateInfo_UpdateInfo {
            target_engine_version: core::default::Default::default(),
            target_node_type: core::default::Default::default(),
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_StateInfo_UpdateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_StateInfo_UpdateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_StateInfo_UpdateInfoRef {
        MemorystoreInstance_StateInfo_UpdateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_StateInfo_UpdateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_engine_version` after provisioning.\n"]
    pub fn target_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `target_node_type` after provisioning.\n"]
    pub fn target_node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_replica_count` after provisioning.\n"]
    pub fn target_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `target_shard_count` after provisioning.\n"]
    pub fn target_shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_shard_count", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_StateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_info: std::option::Option<terrars::ListField<MemorystoreInstance_StateInfo_UpdateInfo>>,
}

impl MemorystoreInstance_StateInfo {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MemorystoreInstance_StateInfo_UpdateInfo>>,
    ) -> Self {
        self.update_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_StateInfo {
    type O = terrars::BlockAssignable<MemorystoreInstance_StateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_StateInfo {}

impl BuildMemorystoreInstance_StateInfo {
    pub fn build(self) -> MemorystoreInstance_StateInfo {
        MemorystoreInstance_StateInfo { update_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_StateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_StateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_StateInfoRef {
        MemorystoreInstance_StateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_StateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> terrars::ListRef<MemorystoreInstance_StateInfo_UpdateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    hours: terrars::PrimField<f64>,
}

impl MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime { }

impl terrars::ToListMappable for MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    type O = terrars::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[doc =
        "Hours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: terrars::PrimField<f64>,
}

impl BuildMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    pub fn build(self) -> MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
        MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime { hours: self.hours }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
        MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
    dynamic: MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleDynamic,
}

impl MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>,
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

impl terrars::ToListMappable for MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    type O = terrars::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {}

impl BuildMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    pub fn build(self) -> MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
        MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
        MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_AutomatedBackupConfigDynamic {
    fixed_frequency_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_AutomatedBackupConfig {
    retention: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_frequency_schedule: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>>,
    dynamic: MemorystoreInstance_AutomatedBackupConfigDynamic,
}

impl MemorystoreInstance_AutomatedBackupConfig {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_frequency_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_frequency_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_AutomatedBackupConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_AutomatedBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_AutomatedBackupConfig {
    #[doc =
        "How long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\". The default_value is \"3024000s\""]
    pub retention: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstance_AutomatedBackupConfig {
    pub fn build(self) -> MemorystoreInstance_AutomatedBackupConfig {
        MemorystoreInstance_AutomatedBackupConfig {
            retention: self.retention,
            fixed_frequency_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_AutomatedBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_AutomatedBackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_AutomatedBackupConfigRef {
        MemorystoreInstance_AutomatedBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_AutomatedBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nHow long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\". The default_value is \"3024000s\""]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_frequency_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
    type O = terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
        MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef {
        MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    type O =
        terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
        MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
        MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_instance: std
    ::option
    ::Option<terrars::ListField<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_instance: std
    ::option
    ::Option<terrars::ListField<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance>>,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstance>,
                    >,
    ) -> Self {
        self.primary_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_instance`.\n"]
    pub fn set_secondary_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance>,
                    >,
    ) -> Self {
        self.secondary_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    type O = terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_Membership>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig_Membership {
        MemorystoreInstance_CrossInstanceReplicationConfig_Membership {
            primary_instance: core::default::Default::default(),
            secondary_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
        MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_PrimaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_instance` after provisioning.\n"]
    pub fn secondary_instance(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    #[doc =
        "Set the field `instance`.\nThe full resource path of the primary instance in the format: projects/{project}/locations/{region}/instances/{instance-id}"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    type O = terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
        MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
            instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
        MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe full resource path of the primary instance in the format: projects/{project}/locations/{region}/instances/{instance-id}"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the primary instance."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    #[doc =
        "Set the field `instance`.\nThe full resource path of the Nth instance in the format: projects/{project}/locations/{region}/instance/{instance-id}"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    type O = terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
        MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
            instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
        MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe full resource path of the Nth instance in the format: projects/{project}/locations/{region}/instance/{instance-id}"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the Nth instance."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_CrossInstanceReplicationConfigDynamic {
    primary_instance: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>>,
    secondary_instances: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_instance: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_instances: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>>,
    dynamic: MemorystoreInstance_CrossInstanceReplicationConfigDynamic,
}

impl MemorystoreInstance_CrossInstanceReplicationConfig {
    #[doc =
        "Set the field `instance_role`.\nThe instance role supports the following values:\n1. 'INSTANCE_ROLE_UNSPECIFIED': This is an independent instance that has never participated in cross instance replication. It allows both reads and writes.\n2. 'NONE': This is an independent instance that previously participated in cross instance replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n3. 'PRIMARY': This instance serves as the replication source for secondary instance that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n4. 'SECONDARY': This instance replicates data from the primary instance. It allows only reads. Possible values: [\"INSTANCE_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn set_instance_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_instance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_instances`.\n"]
    pub fn set_secondary_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secondary_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_instances = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_CrossInstanceReplicationConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_CrossInstanceReplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_CrossInstanceReplicationConfig {}

impl BuildMemorystoreInstance_CrossInstanceReplicationConfig {
    pub fn build(self) -> MemorystoreInstance_CrossInstanceReplicationConfig {
        MemorystoreInstance_CrossInstanceReplicationConfig {
            instance_role: core::default::Default::default(),
            primary_instance: core::default::Default::default(),
            secondary_instances: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_CrossInstanceReplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_CrossInstanceReplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_CrossInstanceReplicationConfigRef {
        MemorystoreInstance_CrossInstanceReplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_CrossInstanceReplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_role` after provisioning.\nThe instance role supports the following values:\n1. 'INSTANCE_ROLE_UNSPECIFIED': This is an independent instance that has never participated in cross instance replication. It allows both reads and writes.\n2. 'NONE': This is an independent instance that previously participated in cross instance replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n3. 'PRIMARY': This instance serves as the replication source for secondary instance that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n4. 'SECONDARY': This instance replicates data from the primary instance. It allows only reads. Possible values: [\"INSTANCE_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn instance_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nAn output only view of all the member instance participating in cross instance replication. This field is populated for all the member clusters irrespective of their cluster role."]
    pub fn membership(&self) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last time cross instance replication config was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_instances` after provisioning.\n"]
    pub fn secondary_instances(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_instances", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_DesiredAutoCreatedEndpoints {
    network: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl MemorystoreInstance_DesiredAutoCreatedEndpoints { }

impl terrars::ToListMappable for MemorystoreInstance_DesiredAutoCreatedEndpoints {
    type O = terrars::BlockAssignable<MemorystoreInstance_DesiredAutoCreatedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_DesiredAutoCreatedEndpoints {
    #[doc =
        "Required. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc = "Required. The consumer project_id where the forwarding rule is created from."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstance_DesiredAutoCreatedEndpoints {
    pub fn build(self) -> MemorystoreInstance_DesiredAutoCreatedEndpoints {
        MemorystoreInstance_DesiredAutoCreatedEndpoints {
            network: self.network,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_DesiredAutoCreatedEndpointsRef {
        MemorystoreInstance_DesiredAutoCreatedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nRequired. The consumer project_id where the forwarding rule is created from."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_DesiredPscAutoConnections {
    network: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl MemorystoreInstance_DesiredPscAutoConnections { }

impl terrars::ToListMappable for MemorystoreInstance_DesiredPscAutoConnections {
    type O = terrars::BlockAssignable<MemorystoreInstance_DesiredPscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_DesiredPscAutoConnections {
    #[doc =
        "Required. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc = "Required. The consumer project_id where the forwarding rule is created from."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstance_DesiredPscAutoConnections {
    pub fn build(self) -> MemorystoreInstance_DesiredPscAutoConnections {
        MemorystoreInstance_DesiredPscAutoConnections {
            network: self.network,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_DesiredPscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_DesiredPscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_DesiredPscAutoConnectionsRef {
        MemorystoreInstance_DesiredPscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_DesiredPscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nRequired. The consumer project_id where the forwarding rule is created from."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_GcsSource {
    uris: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_GcsSource { }

impl terrars::ToListMappable for MemorystoreInstance_GcsSource {
    type O = terrars::BlockAssignable<MemorystoreInstance_GcsSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_GcsSource {
    #[doc = "URIs of the GCS objects to import.\nExample: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub uris: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMemorystoreInstance_GcsSource {
    pub fn build(self) -> MemorystoreInstance_GcsSource {
        MemorystoreInstance_GcsSource { uris: self.uris }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_GcsSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_GcsSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_GcsSourceRef {
        MemorystoreInstance_GcsSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_GcsSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uris` after provisioning.\nURIs of the GCS objects to import.\nExample: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub fn uris(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    #[doc =
        "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    type O = terrars::BlockAssignable<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {}

impl BuildMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    pub fn build(self) -> MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
        MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
        MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    day: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
    dynamic: MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowDynamic,
}

impl MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>,
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

impl terrars::ToListMappable for MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    type O = terrars::BlockAssignable<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[doc =
        "The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    pub fn build(self) -> MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
        MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
        MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nThe day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nDuration of the maintenance window.\nThe current window is fixed at 1 hour.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_MaintenancePolicyDynamic {
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>>,
    dynamic: MemorystoreInstance_MaintenancePolicyDynamic,
}

impl MemorystoreInstance_MaintenancePolicy {
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_MaintenancePolicy {
    type O = terrars::BlockAssignable<MemorystoreInstance_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_MaintenancePolicy {}

impl BuildMemorystoreInstance_MaintenancePolicy {
    pub fn build(self) -> MemorystoreInstance_MaintenancePolicy {
        MemorystoreInstance_MaintenancePolicy {
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_MaintenancePolicyRef {
        MemorystoreInstance_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the policy was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> terrars::ListRef<MemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_ManagedBackupSource {
    backup: terrars::PrimField<std::string::String>,
}

impl MemorystoreInstance_ManagedBackupSource { }

impl terrars::ToListMappable for MemorystoreInstance_ManagedBackupSource {
    type O = terrars::BlockAssignable<MemorystoreInstance_ManagedBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_ManagedBackupSource {
    #[doc = "Example: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub backup: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstance_ManagedBackupSource {
    pub fn build(self) -> MemorystoreInstance_ManagedBackupSource {
        MemorystoreInstance_ManagedBackupSource { backup: self.backup }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_ManagedBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_ManagedBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_ManagedBackupSourceRef {
        MemorystoreInstance_ManagedBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_ManagedBackupSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup` after provisioning.\nExample: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub fn backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_PersistenceConfig_AofConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_fsync: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_PersistenceConfig_AofConfig {
    #[doc = "Set the field `append_fsync`.\nOptional. The fsync mode. \n Possible values:\n NEVER\nEVERY_SEC\nALWAYS"]
    pub fn set_append_fsync(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.append_fsync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_PersistenceConfig_AofConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig_AofConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_PersistenceConfig_AofConfig {}

impl BuildMemorystoreInstance_PersistenceConfig_AofConfig {
    pub fn build(self) -> MemorystoreInstance_PersistenceConfig_AofConfig {
        MemorystoreInstance_PersistenceConfig_AofConfig { append_fsync: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_PersistenceConfig_AofConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_PersistenceConfig_AofConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_PersistenceConfig_AofConfigRef {
        MemorystoreInstance_PersistenceConfig_AofConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_PersistenceConfig_AofConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `append_fsync` after provisioning.\nOptional. The fsync mode. \n Possible values:\n NEVER\nEVERY_SEC\nALWAYS"]
    pub fn append_fsync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_PersistenceConfig_RdbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_PersistenceConfig_RdbConfig {
    #[doc =
        "Set the field `rdb_snapshot_period`.\nOptional. Period between RDB snapshots. \n Possible values:\n ONE_HOUR\nSIX_HOURS\nTWELVE_HOURS\nTWENTY_FOUR_HOURS"]
    pub fn set_rdb_snapshot_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rdb_snapshot_start_time`.\nOptional. Time that the first snapshot was/will be attempted, and to which future\nsnapshots will be aligned. If not provided, the current time will be\nused."]
    pub fn set_rdb_snapshot_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_PersistenceConfig_RdbConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig_RdbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_PersistenceConfig_RdbConfig {}

impl BuildMemorystoreInstance_PersistenceConfig_RdbConfig {
    pub fn build(self) -> MemorystoreInstance_PersistenceConfig_RdbConfig {
        MemorystoreInstance_PersistenceConfig_RdbConfig {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_PersistenceConfig_RdbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_PersistenceConfig_RdbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstance_PersistenceConfig_RdbConfigRef {
        MemorystoreInstance_PersistenceConfig_RdbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_PersistenceConfig_RdbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\nOptional. Period between RDB snapshots. \n Possible values:\n ONE_HOUR\nSIX_HOURS\nTWELVE_HOURS\nTWENTY_FOUR_HOURS"]
    pub fn rdb_snapshot_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\nOptional. Time that the first snapshot was/will be attempted, and to which future\nsnapshots will be aligned. If not provided, the current time will be\nused."]
    pub fn rdb_snapshot_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstance_PersistenceConfigDynamic {
    aof_config: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_PersistenceConfig_AofConfig>>,
    rdb_config: std::option::Option<terrars::DynamicBlock<MemorystoreInstance_PersistenceConfig_RdbConfig>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstance_PersistenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aof_config: std::option::Option<std::vec::Vec<MemorystoreInstance_PersistenceConfig_AofConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_config: std::option::Option<std::vec::Vec<MemorystoreInstance_PersistenceConfig_RdbConfig>>,
    dynamic: MemorystoreInstance_PersistenceConfigDynamic,
}

impl MemorystoreInstance_PersistenceConfig {
    #[doc =
        "Set the field `mode`.\nOptional. Current persistence mode. \n Possible values:\nDISABLED\nRDB\nAOF Possible values: [\"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig_AofConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aof_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aof_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rdb_config`.\n"]
    pub fn set_rdb_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig_RdbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rdb_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rdb_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_PersistenceConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_PersistenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_PersistenceConfig {}

impl BuildMemorystoreInstance_PersistenceConfig {
    pub fn build(self) -> MemorystoreInstance_PersistenceConfig {
        MemorystoreInstance_PersistenceConfig {
            mode: core::default::Default::default(),
            aof_config: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_PersistenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_PersistenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_PersistenceConfigRef {
        MemorystoreInstance_PersistenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_PersistenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. Current persistence mode. \n Possible values:\nDISABLED\nRDB\nAOF Possible values: [\"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> terrars::ListRef<MemorystoreInstance_PersistenceConfig_AofConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> terrars::ListRef<MemorystoreInstance_PersistenceConfig_RdbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_Timeouts {
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

impl terrars::ToListMappable for MemorystoreInstance_Timeouts {
    type O = terrars::BlockAssignable<MemorystoreInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_Timeouts {}

impl BuildMemorystoreInstance_Timeouts {
    pub fn build(self) -> MemorystoreInstance_Timeouts {
        MemorystoreInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_TimeoutsRef {
        MemorystoreInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_TimeoutsRef {
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

#[derive(Serialize)]
pub struct MemorystoreInstance_ZoneDistributionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstance_ZoneDistributionConfig {
    #[doc =
        "Set the field `mode`.\nOptional. Current zone distribution mode. Defaults to MULTI_ZONE. \n Possible values:\n MULTI_ZONE\nSINGLE_ZONE Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nOptional. Defines zone where all resources will be allocated with SINGLE_ZONE mode.\nIgnored for MULTI_ZONE mode."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstance_ZoneDistributionConfig {
    type O = terrars::BlockAssignable<MemorystoreInstance_ZoneDistributionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstance_ZoneDistributionConfig {}

impl BuildMemorystoreInstance_ZoneDistributionConfig {
    pub fn build(self) -> MemorystoreInstance_ZoneDistributionConfig {
        MemorystoreInstance_ZoneDistributionConfig {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstance_ZoneDistributionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstance_ZoneDistributionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MemorystoreInstance_ZoneDistributionConfigRef {
        MemorystoreInstance_ZoneDistributionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstance_ZoneDistributionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. Current zone distribution mode. Defaults to MULTI_ZONE. \n Possible values:\n MULTI_ZONE\nSINGLE_ZONE Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nOptional. Defines zone where all resources will be allocated with SINGLE_ZONE mode.\nIgnored for MULTI_ZONE mode."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
