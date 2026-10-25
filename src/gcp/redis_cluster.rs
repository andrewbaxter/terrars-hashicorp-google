#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct RedisClusterData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_configs: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
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
    automated_backup_config: std::option::Option<std::vec::Vec<RedisCluster_AutomatedBackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_cluster_replication_config: std::option::Option<std::vec::Vec<RedisCluster_CrossClusterReplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_source: std::option::Option<std::vec::Vec<RedisCluster_GcsSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<RedisCluster_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_backup_source: std::option::Option<std::vec::Vec<RedisCluster_ManagedBackupSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistence_config: std::option::Option<std::vec::Vec<RedisCluster_PersistenceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_configs: std::option::Option<std::vec::Vec<RedisCluster_PscConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RedisCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_distribution_config: std::option::Option<std::vec::Vec<RedisCluster_ZoneDistributionConfig>>,
    dynamic: RedisClusterDynamic,
}

struct RedisCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedisClusterData_>,
}

#[derive(Clone)]
pub struct RedisCluster(std::rc::Rc<RedisCluster_>);

impl RedisCluster {
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

    #[doc = "Set the field `acl_policy`.\nOptional. The name of the ACL policy to attach to the cluster."]
    pub fn set_acl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().acl_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authorization_mode`.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\", \"AUTH_MODE_TOKEN_AUTH\"]"]
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
        "Set the field `deletion_protection_enabled`.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn set_deletion_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Set the field `name`.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_type`.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
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
        "Set the field `redis_configs`.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn set_redis_configs(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().redis_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the region of the Redis cluster."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nOptional. The number of replica nodes per shard."]
    pub fn set_replica_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_ca_mode`.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn set_server_ca_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_ca_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_ca_pool`.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn set_server_ca_pool(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transit_encryption_mode`.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig>>,
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

    #[doc = "Set the field `cross_cluster_replication_config`.\n"]
    pub fn set_cross_cluster_replication_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_cluster_replication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cross_cluster_replication_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_source`.\n"]
    pub fn set_gcs_source(self, v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_GcsSource>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_MaintenancePolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_ManagedBackupSource>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_PersistenceConfig>>,
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

    #[doc = "Set the field `psc_configs`.\n"]
    pub fn set_psc_configs(self, v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_PscConfigs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RedisCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_distribution_config`.\n"]
    pub fn set_zone_distribution_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_ZoneDistributionConfig>>,
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
        "Get a reference to the value of field `acl_policy` after provisioning.\nOptional. The name of the ACL policy to attach to the cluster."]
    pub fn acl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\", \"AUTH_MODE_TOKEN_AUTH\"]"]
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<RedisCluster_DiscoveryEndpointsRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acl_policy_in_sync` after provisioning.\nOptional. Whether the ACL policy is in sync with the cluster."]
    pub fn is_acl_policy_in_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acl_policy_in_sync", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<RedisCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<RedisCluster_ManagedServerCaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_server_ca", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precise_size_gb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<RedisCluster_PscConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> terrars::ListRef<RedisCluster_PscServiceAttachmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_service_attachments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.redis_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> terrars::ListRef<RedisCluster_StateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> terrars::ListRef<RedisCluster_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\n"]
    pub fn cross_cluster_replication_config(&self) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> terrars::ListRef<RedisCluster_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<RedisCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> terrars::ListRef<RedisCluster_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> terrars::ListRef<RedisCluster_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> terrars::ListRef<RedisCluster_PscConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisCluster_TimeoutsRef {
        RedisCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<RedisCluster_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

impl terrars::Referable for RedisCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedisCluster { }

impl terrars::ToListMappable for RedisCluster {
    type O = terrars::ListRef<RedisClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedisCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_redis_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedisCluster {
    pub tf_id: std::string::String,
    #[doc = "Required. Number of shards for the Redis cluster."]
    pub shard_count: terrars::PrimField<f64>,
}

impl BuildRedisCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> RedisCluster {
        let out = RedisCluster(std::rc::Rc::new(RedisCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedisClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl_policy: core::default::Default::default(),
                authorization_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                name: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                redis_configs: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                server_ca_mode: core::default::Default::default(),
                server_ca_pool: core::default::Default::default(),
                shard_count: self.shard_count,
                transit_encryption_mode: core::default::Default::default(),
                automated_backup_config: core::default::Default::default(),
                cross_cluster_replication_config: core::default::Default::default(),
                gcs_source: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                managed_backup_source: core::default::Default::default(),
                persistence_config: core::default::Default::default(),
                psc_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                zone_distribution_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedisClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedisClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `acl_policy` after provisioning.\nOptional. The name of the ACL policy to attach to the cluster."]
    pub fn acl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\", \"AUTH_MODE_TOKEN_AUTH\"]"]
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nOptional. Indicates if the cluster is deletion protected or not.\nIf the value if set to true, any delete cluster operation will fail.\nDefault value is true."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<RedisCluster_DiscoveryEndpointsRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acl_policy_in_sync` after provisioning.\nOptional. Whether the ACL policy is in sync with the cluster."]
    pub fn is_acl_policy_in_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acl_policy_in_sync", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the at-rest data of the cluster."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<RedisCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<RedisCluster_ManagedServerCaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_server_ca", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nThe nodeType for the Redis cluster.\nIf not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values: [\"REDIS_SHARED_CORE_NANO\", \"REDIS_HIGHMEM_MEDIUM\", \"REDIS_HIGHCPU_MEDIUM\", \"REDIS_STANDARD_LARGE\", \"REDIS_HIGHMEM_XLARGE\", \"REDIS_HIGHMEM_2XLARGE\", \"REDIS_STANDARD_SMALL\"]"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precise_size_gb` after provisioning.\nOutput only. Redis memory precise size in GB for the entire cluster."]
    pub fn precise_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precise_size_gb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<RedisCluster_PscConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> terrars::ListRef<RedisCluster_PscServiceAttachmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_service_attachments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_configs` after provisioning.\nConfigure Redis Cluster behavior using a subset of native Redis configuration parameters.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations"]
    pub fn redis_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.redis_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_mode` after provisioning.\nThe serverCaMode for the TLS enabled Redis cluster.\nIf not provided, SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA will be used as default Possible values: [\"SERVER_CA_MODE_GOOGLE_MANAGED_PER_INSTANCE_CA\", \"SERVER_CA_MODE_GOOGLE_MANAGED_SHARED_CA\", \"SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\", \"SERVER_CA_MODE_UNSPECIFIED\"]"]
    pub fn server_ca_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_pool` after provisioning.\nThe resource name of the server CA pool for an instance with SERVER_CA_MODE_CUSTOMER_MANAGED_CAS_CA\nas the server_ca_mode.\nFormat: projects/{project}/locations/{region}/caPools/{caPoolId}"]
    pub fn server_ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_ca_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> terrars::ListRef<RedisCluster_StateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_backup_config` after provisioning.\n"]
    pub fn automated_backup_config(&self) -> terrars::ListRef<RedisCluster_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\n"]
    pub fn cross_cluster_replication_config(&self) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\n"]
    pub fn gcs_source(&self) -> terrars::ListRef<RedisCluster_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<RedisCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_backup_source` after provisioning.\n"]
    pub fn managed_backup_source(&self) -> terrars::ListRef<RedisCluster_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> terrars::ListRef<RedisCluster_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> terrars::ListRef<RedisCluster_PscConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisCluster_TimeoutsRef {
        RedisCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_distribution_config` after provisioning.\n"]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<RedisCluster_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RedisClusterDynamic {
    automated_backup_config: std::option::Option<terrars::DynamicBlock<RedisCluster_AutomatedBackupConfig>>,
    cross_cluster_replication_config: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_CrossClusterReplicationConfig>>,
    gcs_source: std::option::Option<terrars::DynamicBlock<RedisCluster_GcsSource>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<RedisCluster_MaintenancePolicy>>,
    managed_backup_source: std::option::Option<terrars::DynamicBlock<RedisCluster_ManagedBackupSource>>,
    persistence_config: std::option::Option<terrars::DynamicBlock<RedisCluster_PersistenceConfig>>,
    psc_configs: std::option::Option<terrars::DynamicBlock<RedisCluster_PscConfigs>>,
    zone_distribution_config: std::option::Option<terrars::DynamicBlock<RedisCluster_ZoneDistributionConfig>>,
}

#[derive(Serialize)]
pub struct RedisCluster_DiscoveryEndpoints_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_DiscoveryEndpoints_PscConfig {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_DiscoveryEndpoints_PscConfig {
    type O = terrars::BlockAssignable<RedisCluster_DiscoveryEndpoints_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_DiscoveryEndpoints_PscConfig {}

impl BuildRedisCluster_DiscoveryEndpoints_PscConfig {
    pub fn build(self) -> RedisCluster_DiscoveryEndpoints_PscConfig {
        RedisCluster_DiscoveryEndpoints_PscConfig { network: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_DiscoveryEndpoints_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_DiscoveryEndpoints_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_DiscoveryEndpoints_PscConfigRef {
        RedisCluster_DiscoveryEndpoints_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_DiscoveryEndpoints_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_DiscoveryEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<terrars::ListField<RedisCluster_DiscoveryEndpoints_PscConfig>>,
}

impl RedisCluster_DiscoveryEndpoints {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedisCluster_DiscoveryEndpoints_PscConfig>>,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_DiscoveryEndpoints {
    type O = terrars::BlockAssignable<RedisCluster_DiscoveryEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_DiscoveryEndpoints {}

impl BuildRedisCluster_DiscoveryEndpoints {
    pub fn build(self) -> RedisCluster_DiscoveryEndpoints {
        RedisCluster_DiscoveryEndpoints {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            psc_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_DiscoveryEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_DiscoveryEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_DiscoveryEndpointsRef {
        RedisCluster_DiscoveryEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_DiscoveryEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<RedisCluster_DiscoveryEndpoints_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_MaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_MaintenanceSchedule {
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

impl terrars::ToListMappable for RedisCluster_MaintenanceSchedule {
    type O = terrars::BlockAssignable<RedisCluster_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_MaintenanceSchedule {}

impl BuildRedisCluster_MaintenanceSchedule {
    pub fn build(self) -> RedisCluster_MaintenanceSchedule {
        RedisCluster_MaintenanceSchedule {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_MaintenanceScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_MaintenanceScheduleRef {
        RedisCluster_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_MaintenanceScheduleRef {
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
pub struct RedisCluster_ManagedServerCa_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl RedisCluster_ManagedServerCa_CaCerts {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_ManagedServerCa_CaCerts {
    type O = terrars::BlockAssignable<RedisCluster_ManagedServerCa_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_ManagedServerCa_CaCerts {}

impl BuildRedisCluster_ManagedServerCa_CaCerts {
    pub fn build(self) -> RedisCluster_ManagedServerCa_CaCerts {
        RedisCluster_ManagedServerCa_CaCerts { certificates: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_ManagedServerCa_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_ManagedServerCa_CaCertsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_ManagedServerCa_CaCertsRef {
        RedisCluster_ManagedServerCa_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_ManagedServerCa_CaCertsRef {
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
pub struct RedisCluster_ManagedServerCa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<RedisCluster_ManagedServerCa_CaCerts>>,
}

impl RedisCluster_ManagedServerCa {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedisCluster_ManagedServerCa_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_ManagedServerCa {
    type O = terrars::BlockAssignable<RedisCluster_ManagedServerCa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_ManagedServerCa {}

impl BuildRedisCluster_ManagedServerCa {
    pub fn build(self) -> RedisCluster_ManagedServerCa {
        RedisCluster_ManagedServerCa { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_ManagedServerCaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_ManagedServerCaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_ManagedServerCaRef {
        RedisCluster_ManagedServerCaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_ManagedServerCaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<RedisCluster_ManagedServerCa_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_PscConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_PscConnections {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for RedisCluster_PscConnections {
    type O = terrars::BlockAssignable<RedisCluster_PscConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PscConnections {}

impl BuildRedisCluster_PscConnections {
    pub fn build(self) -> RedisCluster_PscConnections {
        RedisCluster_PscConnections {
            address: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PscConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PscConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PscConnectionsRef {
        RedisCluster_PscConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PscConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_PscServiceAttachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_PscServiceAttachments {
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

impl terrars::ToListMappable for RedisCluster_PscServiceAttachments {
    type O = terrars::BlockAssignable<RedisCluster_PscServiceAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PscServiceAttachments {}

impl BuildRedisCluster_PscServiceAttachments {
    pub fn build(self) -> RedisCluster_PscServiceAttachments {
        RedisCluster_PscServiceAttachments {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PscServiceAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PscServiceAttachmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PscServiceAttachmentsRef {
        RedisCluster_PscServiceAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PscServiceAttachmentsRef {
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
pub struct RedisCluster_StateInfo_UpdateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_shard_count: std::option::Option<terrars::PrimField<f64>>,
}

impl RedisCluster_StateInfo_UpdateInfo {
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

impl terrars::ToListMappable for RedisCluster_StateInfo_UpdateInfo {
    type O = terrars::BlockAssignable<RedisCluster_StateInfo_UpdateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_StateInfo_UpdateInfo {}

impl BuildRedisCluster_StateInfo_UpdateInfo {
    pub fn build(self) -> RedisCluster_StateInfo_UpdateInfo {
        RedisCluster_StateInfo_UpdateInfo {
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_StateInfo_UpdateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_StateInfo_UpdateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_StateInfo_UpdateInfoRef {
        RedisCluster_StateInfo_UpdateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_StateInfo_UpdateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct RedisCluster_StateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_info: std::option::Option<terrars::ListField<RedisCluster_StateInfo_UpdateInfo>>,
}

impl RedisCluster_StateInfo {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedisCluster_StateInfo_UpdateInfo>>,
    ) -> Self {
        self.update_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_StateInfo {
    type O = terrars::BlockAssignable<RedisCluster_StateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_StateInfo {}

impl BuildRedisCluster_StateInfo {
    pub fn build(self) -> RedisCluster_StateInfo {
        RedisCluster_StateInfo { update_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_StateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_StateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_StateInfoRef {
        RedisCluster_StateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_StateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> terrars::ListRef<RedisCluster_StateInfo_UpdateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    hours: terrars::PrimField<f64>,
}

impl RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime { }

impl terrars::ToListMappable for RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    type O = terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[doc =
        "Hours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: terrars::PrimField<f64>,
}

impl BuildRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    pub fn build(self) -> RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
        RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime { hours: self.hours }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
        RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
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
struct RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
}

#[derive(Serialize)]
pub struct RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
    dynamic: RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleDynamic,
}

impl RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>,
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

impl terrars::ToListMappable for RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    type O = terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {}

impl BuildRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    pub fn build(self) -> RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
        RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
        RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedisCluster_AutomatedBackupConfigDynamic {
    fixed_frequency_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>>,
}

#[derive(Serialize)]
pub struct RedisCluster_AutomatedBackupConfig {
    retention: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_frequency_schedule: std
    ::option
    ::Option<std::vec::Vec<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>>,
    dynamic: RedisCluster_AutomatedBackupConfigDynamic,
}

impl RedisCluster_AutomatedBackupConfig {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>>,
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

impl terrars::ToListMappable for RedisCluster_AutomatedBackupConfig {
    type O = terrars::BlockAssignable<RedisCluster_AutomatedBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_AutomatedBackupConfig {
    #[doc =
        "How long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub retention: terrars::PrimField<std::string::String>,
}

impl BuildRedisCluster_AutomatedBackupConfig {
    pub fn build(self) -> RedisCluster_AutomatedBackupConfig {
        RedisCluster_AutomatedBackupConfig {
            retention: self.retention,
            fixed_frequency_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_AutomatedBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_AutomatedBackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_AutomatedBackupConfigRef {
        RedisCluster_AutomatedBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_AutomatedBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nHow long to keep automated backups before the backups are deleted.\nThe value should be between 1 day and 365 days. If not specified, the default value is 35 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> terrars::ListRef<RedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_frequency_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {}

impl BuildRedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
        RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef {
        RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {}

impl BuildRedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
        RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef {
        RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig_Membership {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_cluster: std
    ::option
    ::Option<terrars::ListField<RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_clusters: std
    ::option
    ::Option<terrars::ListField<RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters>>,
}

impl RedisCluster_CrossClusterReplicationConfig_Membership {
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryCluster>>,
    ) -> Self {
        self.primary_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClusters>,
                    >,
    ) -> Self {
        self.secondary_clusters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig_Membership {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_Membership>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig_Membership {}

impl BuildRedisCluster_CrossClusterReplicationConfig_Membership {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig_Membership {
        RedisCluster_CrossClusterReplicationConfig_Membership {
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfig_MembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfig_MembershipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_CrossClusterReplicationConfig_MembershipRef {
        RedisCluster_CrossClusterReplicationConfig_MembershipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfig_MembershipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(
        &self,
    ) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfig_Membership_PrimaryClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfig_Membership_SecondaryClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_clusters", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    #[doc =
        "Set the field `cluster`.\nThe full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_PrimaryCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {}

impl BuildRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
        RedisCluster_CrossClusterReplicationConfig_PrimaryCluster { cluster: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
        RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the primary cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    #[doc =
        "Set the field `cluster`.\nThe full resource path of the secondary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_SecondaryClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {}

impl BuildRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
        RedisCluster_CrossClusterReplicationConfig_SecondaryClusters { cluster: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
        RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe full resource path of the secondary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique id of the secondary cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedisCluster_CrossClusterReplicationConfigDynamic {
    primary_cluster: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
    secondary_clusters: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
}

#[derive(Serialize)]
pub struct RedisCluster_CrossClusterReplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_cluster: std::option::Option<std::vec::Vec<RedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_clusters: std::option::Option<std::vec::Vec<RedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
    dynamic: RedisCluster_CrossClusterReplicationConfigDynamic,
}

impl RedisCluster_CrossClusterReplicationConfig {
    #[doc =
        "Set the field `cluster_role`.\nThe role of the cluster in cross cluster replication. Supported values are:\n\n1. 'CLUSTER_ROLE_UNSPECIFIED': This is an independent cluster that has never participated in cross cluster replication. It allows both reads and writes.\n\n1. 'NONE': This is an independent cluster that previously participated in cross cluster replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n\n1. 'PRIMARY': This cluster serves as the replication source for secondary clusters that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n\n1. 'SECONDARY': This cluster replicates data from the primary cluster. It allows only reads. Possible values: [\"CLUSTER_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn set_cluster_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_clusters`.\n"]
    pub fn set_secondary_clusters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secondary_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_clusters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedisCluster_CrossClusterReplicationConfig {
    type O = terrars::BlockAssignable<RedisCluster_CrossClusterReplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_CrossClusterReplicationConfig {}

impl BuildRedisCluster_CrossClusterReplicationConfig {
    pub fn build(self) -> RedisCluster_CrossClusterReplicationConfig {
        RedisCluster_CrossClusterReplicationConfig {
            cluster_role: core::default::Default::default(),
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_CrossClusterReplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_CrossClusterReplicationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_CrossClusterReplicationConfigRef {
        RedisCluster_CrossClusterReplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_CrossClusterReplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_role` after provisioning.\nThe role of the cluster in cross cluster replication. Supported values are:\n\n1. 'CLUSTER_ROLE_UNSPECIFIED': This is an independent cluster that has never participated in cross cluster replication. It allows both reads and writes.\n\n1. 'NONE': This is an independent cluster that previously participated in cross cluster replication(either as a 'PRIMARY' or 'SECONDARY' cluster). It allows both reads and writes.\n\n1. 'PRIMARY': This cluster serves as the replication source for secondary clusters that are replicating from it. Any data written to it is automatically replicated to its secondary clusters. It allows both reads and writes.\n\n1. 'SECONDARY': This cluster replicates data from the primary cluster. It allows only reads. Possible values: [\"CLUSTER_ROLE_UNSPECIFIED\", \"NONE\", \"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nAn output only view of all the member clusters participating in cross cluster replication. This field is populated for all the member clusters irrespective of their cluster role."]
    pub fn membership(&self) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfig_MembershipRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last time cross cluster replication config was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(&self) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> terrars::ListRef<RedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_clusters", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_GcsSource {
    uris: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_GcsSource { }

impl terrars::ToListMappable for RedisCluster_GcsSource {
    type O = terrars::BlockAssignable<RedisCluster_GcsSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_GcsSource {
    #[doc = "URIs of the GCS objects to import. Example: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub uris: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildRedisCluster_GcsSource {
    pub fn build(self) -> RedisCluster_GcsSource {
        RedisCluster_GcsSource { uris: self.uris }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_GcsSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_GcsSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_GcsSourceRef {
        RedisCluster_GcsSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_GcsSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uris` after provisioning.\nURIs of the GCS objects to import. Example: gs://bucket1/object1, gs://bucket2/folder2/object2"]
    pub fn uris(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
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

impl terrars::ToListMappable for RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    type O = terrars::BlockAssignable<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {}

impl BuildRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    pub fn build(self) -> RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
        RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
        RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
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
struct RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
}

#[derive(Serialize)]
pub struct RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    day: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
    dynamic: RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowDynamic,
}

impl RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>,
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

impl terrars::ToListMappable for RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    type O = terrars::BlockAssignable<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[doc =
        "Required. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: terrars::PrimField<std::string::String>,
}

impl BuildRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    pub fn build(self) -> RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
        RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
        RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nRequired. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nOutput only. Duration of the maintenance window.\nThe current window is fixed at 1 hour.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedisCluster_MaintenancePolicyDynamic {
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct RedisCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std::option::Option<std::vec::Vec<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>>,
    dynamic: RedisCluster_MaintenancePolicyDynamic,
}

impl RedisCluster_MaintenancePolicy {
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>>,
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

impl terrars::ToListMappable for RedisCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<RedisCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_MaintenancePolicy {}

impl BuildRedisCluster_MaintenancePolicy {
    pub fn build(self) -> RedisCluster_MaintenancePolicy {
        RedisCluster_MaintenancePolicy {
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_MaintenancePolicyRef {
        RedisCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> terrars::ListRef<RedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_ManagedBackupSource {
    backup: terrars::PrimField<std::string::String>,
}

impl RedisCluster_ManagedBackupSource { }

impl terrars::ToListMappable for RedisCluster_ManagedBackupSource {
    type O = terrars::BlockAssignable<RedisCluster_ManagedBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_ManagedBackupSource {
    #[doc = "Example: 'projects/{project}/locations/{location}/backupCollections/{collection}/backups/{backup}'."]
    pub backup: terrars::PrimField<std::string::String>,
}

impl BuildRedisCluster_ManagedBackupSource {
    pub fn build(self) -> RedisCluster_ManagedBackupSource {
        RedisCluster_ManagedBackupSource { backup: self.backup }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_ManagedBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_ManagedBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_ManagedBackupSourceRef {
        RedisCluster_ManagedBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_ManagedBackupSourceRef {
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
pub struct RedisCluster_PersistenceConfig_AofConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_fsync: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_PersistenceConfig_AofConfig {
    #[doc =
        "Set the field `append_fsync`.\nOptional. Available fsync modes.\n\n- NO - Do not explicitly call fsync(). Rely on OS defaults.\n- EVERYSEC - Call fsync() once per second in a background thread. A balance between performance and durability.\n- ALWAYS - Call fsync() for earch write command. Possible values: [\"APPEND_FSYNC_UNSPECIFIED\", \"NO\", \"EVERYSEC\", \"ALWAYS\"]"]
    pub fn set_append_fsync(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.append_fsync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_PersistenceConfig_AofConfig {
    type O = terrars::BlockAssignable<RedisCluster_PersistenceConfig_AofConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PersistenceConfig_AofConfig {}

impl BuildRedisCluster_PersistenceConfig_AofConfig {
    pub fn build(self) -> RedisCluster_PersistenceConfig_AofConfig {
        RedisCluster_PersistenceConfig_AofConfig { append_fsync: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PersistenceConfig_AofConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PersistenceConfig_AofConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PersistenceConfig_AofConfigRef {
        RedisCluster_PersistenceConfig_AofConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PersistenceConfig_AofConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `append_fsync` after provisioning.\nOptional. Available fsync modes.\n\n- NO - Do not explicitly call fsync(). Rely on OS defaults.\n- EVERYSEC - Call fsync() once per second in a background thread. A balance between performance and durability.\n- ALWAYS - Call fsync() for earch write command. Possible values: [\"APPEND_FSYNC_UNSPECIFIED\", \"NO\", \"EVERYSEC\", \"ALWAYS\"]"]
    pub fn append_fsync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_PersistenceConfig_RdbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_PersistenceConfig_RdbConfig {
    #[doc =
        "Set the field `rdb_snapshot_period`.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"SNAPSHOT_PERIOD_UNSPECIFIED\", \"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn set_rdb_snapshot_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rdb_snapshot_start_time`.\nThe time that the first snapshot was/will be attempted, and to which\nfuture snapshots will be aligned.\nIf not provided, the current time will be used."]
    pub fn set_rdb_snapshot_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_PersistenceConfig_RdbConfig {
    type O = terrars::BlockAssignable<RedisCluster_PersistenceConfig_RdbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PersistenceConfig_RdbConfig {}

impl BuildRedisCluster_PersistenceConfig_RdbConfig {
    pub fn build(self) -> RedisCluster_PersistenceConfig_RdbConfig {
        RedisCluster_PersistenceConfig_RdbConfig {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PersistenceConfig_RdbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PersistenceConfig_RdbConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PersistenceConfig_RdbConfigRef {
        RedisCluster_PersistenceConfig_RdbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PersistenceConfig_RdbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"SNAPSHOT_PERIOD_UNSPECIFIED\", \"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn rdb_snapshot_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\nThe time that the first snapshot was/will be attempted, and to which\nfuture snapshots will be aligned.\nIf not provided, the current time will be used."]
    pub fn rdb_snapshot_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedisCluster_PersistenceConfigDynamic {
    aof_config: std::option::Option<terrars::DynamicBlock<RedisCluster_PersistenceConfig_AofConfig>>,
    rdb_config: std::option::Option<terrars::DynamicBlock<RedisCluster_PersistenceConfig_RdbConfig>>,
}

#[derive(Serialize)]
pub struct RedisCluster_PersistenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aof_config: std::option::Option<std::vec::Vec<RedisCluster_PersistenceConfig_AofConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_config: std::option::Option<std::vec::Vec<RedisCluster_PersistenceConfig_RdbConfig>>,
    dynamic: RedisCluster_PersistenceConfigDynamic,
}

impl RedisCluster_PersistenceConfig {
    #[doc =
        "Set the field `mode`.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence (both backup and restore) is disabled for the cluster.\n- RDB: RDB based Persistence is enabled.\n- AOF: AOF based Persistence is enabled. Possible values: [\"PERSISTENCE_MODE_UNSPECIFIED\", \"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_PersistenceConfig_AofConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedisCluster_PersistenceConfig_RdbConfig>>,
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

impl terrars::ToListMappable for RedisCluster_PersistenceConfig {
    type O = terrars::BlockAssignable<RedisCluster_PersistenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PersistenceConfig {}

impl BuildRedisCluster_PersistenceConfig {
    pub fn build(self) -> RedisCluster_PersistenceConfig {
        RedisCluster_PersistenceConfig {
            mode: core::default::Default::default(),
            aof_config: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PersistenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PersistenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PersistenceConfigRef {
        RedisCluster_PersistenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PersistenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence (both backup and restore) is disabled for the cluster.\n- RDB: RDB based Persistence is enabled.\n- AOF: AOF based Persistence is enabled. Possible values: [\"PERSISTENCE_MODE_UNSPECIFIED\", \"DISABLED\", \"RDB\", \"AOF\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> terrars::ListRef<RedisCluster_PersistenceConfig_AofConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> terrars::ListRef<RedisCluster_PersistenceConfig_RdbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_PscConfigs {
    network: terrars::PrimField<std::string::String>,
}

impl RedisCluster_PscConfigs { }

impl terrars::ToListMappable for RedisCluster_PscConfigs {
    type O = terrars::BlockAssignable<RedisCluster_PscConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_PscConfigs {
    #[doc =
        "Required. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildRedisCluster_PscConfigs {
    pub fn build(self) -> RedisCluster_PscConfigs {
        RedisCluster_PscConfigs { network: self.network }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_PscConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_PscConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_PscConfigsRef {
        RedisCluster_PscConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_PscConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_Timeouts {
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

impl terrars::ToListMappable for RedisCluster_Timeouts {
    type O = terrars::BlockAssignable<RedisCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_Timeouts {}

impl BuildRedisCluster_Timeouts {
    pub fn build(self) -> RedisCluster_Timeouts {
        RedisCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_TimeoutsRef {
        RedisCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_TimeoutsRef {
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
pub struct RedisCluster_ZoneDistributionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisCluster_ZoneDistributionConfig {
    #[doc =
        "Set the field `mode`.\nImmutable. The mode for zone distribution for Memorystore Redis cluster.\nIf not provided, MULTI_ZONE will be used as default Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nImmutable. The zone for single zone Memorystore Redis cluster."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisCluster_ZoneDistributionConfig {
    type O = terrars::BlockAssignable<RedisCluster_ZoneDistributionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisCluster_ZoneDistributionConfig {}

impl BuildRedisCluster_ZoneDistributionConfig {
    pub fn build(self) -> RedisCluster_ZoneDistributionConfig {
        RedisCluster_ZoneDistributionConfig {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedisCluster_ZoneDistributionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisCluster_ZoneDistributionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisCluster_ZoneDistributionConfigRef {
        RedisCluster_ZoneDistributionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisCluster_ZoneDistributionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nImmutable. The mode for zone distribution for Memorystore Redis cluster.\nIf not provided, MULTI_ZONE will be used as default Possible values: [\"MULTI_ZONE\", \"SINGLE_ZONE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nImmutable. The zone for single zone Memorystore Redis cluster."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
