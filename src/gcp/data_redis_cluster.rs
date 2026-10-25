#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataRedisClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRedisCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRedisClusterData_>,
}

#[derive(Clone)]
pub struct DataRedisCluster(std::rc::Rc<DataRedisCluster_>);

impl DataRedisCluster {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the region of the Redis cluster."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> terrars::ListRef<DataRedisCluster_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
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
        "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\nCross cluster replication config"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
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
    pub fn discovery_endpoints(&self) -> terrars::ListRef<DataRedisCluster_DiscoveryEndpointsRef> {
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

    #[doc =
        "Get a reference to the value of field `gcs_source` after provisioning.\nBackups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters."]
    pub fn gcs_source(&self) -> terrars::ListRef<DataRedisCluster_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataRedisCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataRedisCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_backup_source` after provisioning.\nBackups that generated and managed by memorystore."]
    pub fn managed_backup_source(&self) -> terrars::ListRef<DataRedisCluster_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<DataRedisCluster_ManagedServerCaRef> {
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
        "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence config (RDB, AOF) for the cluster."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataRedisCluster_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
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
        "Get a reference to the value of field `psc_configs` after provisioning.\nRequired. Each PscConfig configures the consumer network where two\nnetwork addresses will be designated to the cluster for client access.\nCurrently, only one PscConfig is supported."]
    pub fn psc_configs(&self) -> terrars::ListRef<DataRedisCluster_PscConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<DataRedisCluster_PscConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> terrars::ListRef<DataRedisCluster_PscServiceAttachmentsRef> {
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
    pub fn state_info(&self) -> terrars::ListRef<DataRedisCluster_StateInfoRef> {
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

    #[doc =
        "Get a reference to the value of field `zone_distribution_config` after provisioning.\nImmutable. Zone distribution config for Memorystore Redis cluster."]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<DataRedisCluster_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataRedisCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRedisCluster { }

impl terrars::ToListMappable for DataRedisCluster {
    type O = terrars::ListRef<DataRedisClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRedisCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_redis_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedisCluster {
    pub tf_id: std::string::String,
    #[doc =
        "Unique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataRedisCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRedisCluster {
        let out = DataRedisCluster(std::rc::Rc::new(DataRedisCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRedisClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedisClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRedisClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> terrars::ListRef<DataRedisCluster_AutomatedBackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_backup_config", self.extract_ref()))
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
        "Get a reference to the value of field `cross_cluster_replication_config` after provisioning.\nCross cluster replication config"]
    pub fn cross_cluster_replication_config(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_cluster_replication_config", self.extract_ref()),
        )
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
    pub fn discovery_endpoints(&self) -> terrars::ListRef<DataRedisCluster_DiscoveryEndpointsRef> {
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

    #[doc =
        "Get a reference to the value of field `gcs_source` after provisioning.\nBackups stored in Cloud Storage buckets. The Cloud Storage buckets need to be the same region as the clusters."]
    pub fn gcs_source(&self) -> terrars::ListRef<DataRedisCluster_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataRedisCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataRedisCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_backup_source` after provisioning.\nBackups that generated and managed by memorystore."]
    pub fn managed_backup_source(&self) -> terrars::ListRef<DataRedisCluster_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nCluster's Certificate Authority. This field will only be populated if Redis Cluster's transit_encryption_mode is TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<DataRedisCluster_ManagedServerCaRef> {
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
        "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence config (RDB, AOF) for the cluster."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataRedisCluster_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
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
        "Get a reference to the value of field `psc_configs` after provisioning.\nRequired. Each PscConfig configures the consumer network where two\nnetwork addresses will be designated to the cluster for client access.\nCurrently, only one PscConfig is supported."]
    pub fn psc_configs(&self) -> terrars::ListRef<DataRedisCluster_PscConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<DataRedisCluster_PscConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachments` after provisioning.\nService attachment details to configure Psc connections."]
    pub fn psc_service_attachments(&self) -> terrars::ListRef<DataRedisCluster_PscServiceAttachmentsRef> {
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
    pub fn state_info(&self) -> terrars::ListRef<DataRedisCluster_StateInfoRef> {
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

    #[doc =
        "Get a reference to the value of field `zone_distribution_config` after provisioning.\nImmutable. Zone distribution config for Memorystore Redis cluster."]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<DataRedisCluster_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    type O = terrars::BlockAssignable<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {}

impl BuildDataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    pub fn build(self) -> DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
        DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
            hours: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
        DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
}

impl DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    type O = terrars::BlockAssignable<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {}

impl BuildDataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
    pub fn build(self) -> DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
        DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule {
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
        DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_AutomatedBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_frequency_schedule: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_AutomatedBackupConfig {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRedisCluster_AutomatedBackupConfig_FixedFrequencySchedule>>,
    ) -> Self {
        self.fixed_frequency_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_AutomatedBackupConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_AutomatedBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_AutomatedBackupConfig {}

impl BuildDataRedisCluster_AutomatedBackupConfig {
    pub fn build(self) -> DataRedisCluster_AutomatedBackupConfig {
        DataRedisCluster_AutomatedBackupConfig {
            fixed_frequency_schedule: core::default::Default::default(),
            retention: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_AutomatedBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_AutomatedBackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_AutomatedBackupConfigRef {
        DataRedisCluster_AutomatedBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_AutomatedBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_AutomatedBackupConfig_FixedFrequencyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_frequency_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
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

impl terrars::ToListMappable for DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    type O = terrars::BlockAssignable<DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {}

impl BuildDataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
    pub fn build(self) -> DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
        DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
        DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef {
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
pub struct DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
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

impl terrars::ToListMappable for DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    type O = terrars::BlockAssignable<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {}

impl BuildDataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
    pub fn build(self) -> DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
        DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters {
            cluster: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
        DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef {
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
pub struct DataRedisCluster_CrossClusterReplicationConfig_Membership {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_cluster: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_clusters: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
}

impl DataRedisCluster_CrossClusterReplicationConfig_Membership {
    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
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
                    ::Into<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
    ) -> Self {
        self.secondary_clusters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_CrossClusterReplicationConfig_Membership {
    type O = terrars::BlockAssignable<DataRedisCluster_CrossClusterReplicationConfig_Membership>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_CrossClusterReplicationConfig_Membership {}

impl BuildDataRedisCluster_CrossClusterReplicationConfig_Membership {
    pub fn build(self) -> DataRedisCluster_CrossClusterReplicationConfig_Membership {
        DataRedisCluster_CrossClusterReplicationConfig_Membership {
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_CrossClusterReplicationConfig_MembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_CrossClusterReplicationConfig_MembershipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_CrossClusterReplicationConfig_MembershipRef {
        DataRedisCluster_CrossClusterReplicationConfig_MembershipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_CrossClusterReplicationConfig_MembershipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(&self) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_clusters", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_CrossClusterReplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_Membership>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_cluster: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_clusters: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_CrossClusterReplicationConfig {
    #[doc = "Set the field `cluster_role`.\n"]
    pub fn set_cluster_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_Membership>>,
    ) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_cluster`.\n"]
    pub fn set_primary_cluster(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_PrimaryCluster>>,
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
                    ::Into<terrars::ListField<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClusters>>,
    ) -> Self {
        self.secondary_clusters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_CrossClusterReplicationConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_CrossClusterReplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_CrossClusterReplicationConfig {}

impl BuildDataRedisCluster_CrossClusterReplicationConfig {
    pub fn build(self) -> DataRedisCluster_CrossClusterReplicationConfig {
        DataRedisCluster_CrossClusterReplicationConfig {
            cluster_role: core::default::Default::default(),
            membership: core::default::Default::default(),
            primary_cluster: core::default::Default::default(),
            secondary_clusters: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_CrossClusterReplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_CrossClusterReplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_CrossClusterReplicationConfigRef {
        DataRedisCluster_CrossClusterReplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_CrossClusterReplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_role` after provisioning.\n"]
    pub fn cluster_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_role", self.base))
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfig_MembershipRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_cluster` after provisioning.\n"]
    pub fn primary_cluster(&self) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfig_PrimaryClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_clusters` after provisioning.\n"]
    pub fn secondary_clusters(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_CrossClusterReplicationConfig_SecondaryClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_clusters", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_DiscoveryEndpoints_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_DiscoveryEndpoints_PscConfig {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_DiscoveryEndpoints_PscConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_DiscoveryEndpoints_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_DiscoveryEndpoints_PscConfig {}

impl BuildDataRedisCluster_DiscoveryEndpoints_PscConfig {
    pub fn build(self) -> DataRedisCluster_DiscoveryEndpoints_PscConfig {
        DataRedisCluster_DiscoveryEndpoints_PscConfig { network: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_DiscoveryEndpoints_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_DiscoveryEndpoints_PscConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_DiscoveryEndpoints_PscConfigRef {
        DataRedisCluster_DiscoveryEndpoints_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_DiscoveryEndpoints_PscConfigRef {
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
pub struct DataRedisCluster_DiscoveryEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<terrars::ListField<DataRedisCluster_DiscoveryEndpoints_PscConfig>>,
}

impl DataRedisCluster_DiscoveryEndpoints {
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
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_DiscoveryEndpoints_PscConfig>>,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_DiscoveryEndpoints {
    type O = terrars::BlockAssignable<DataRedisCluster_DiscoveryEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_DiscoveryEndpoints {}

impl BuildDataRedisCluster_DiscoveryEndpoints {
    pub fn build(self) -> DataRedisCluster_DiscoveryEndpoints {
        DataRedisCluster_DiscoveryEndpoints {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            psc_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_DiscoveryEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_DiscoveryEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_DiscoveryEndpointsRef {
        DataRedisCluster_DiscoveryEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_DiscoveryEndpointsRef {
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
    pub fn psc_config(&self) -> terrars::ListRef<DataRedisCluster_DiscoveryEndpoints_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_GcsSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uris: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataRedisCluster_GcsSource {
    #[doc = "Set the field `uris`.\n"]
    pub fn set_uris(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_GcsSource {
    type O = terrars::BlockAssignable<DataRedisCluster_GcsSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_GcsSource {}

impl BuildDataRedisCluster_GcsSource {
    pub fn build(self) -> DataRedisCluster_GcsSource {
        DataRedisCluster_GcsSource { uris: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_GcsSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_GcsSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_GcsSourceRef {
        DataRedisCluster_GcsSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_GcsSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
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

impl terrars::ToListMappable for DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    type O = terrars::BlockAssignable<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {}

impl BuildDataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    pub fn build(self) -> DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
        DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
        DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
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
pub struct DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
}

impl DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    type O = terrars::BlockAssignable<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {}

impl BuildDataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
    pub fn build(self) -> DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
        DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
        DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>>,
}

impl DataRedisCluster_MaintenancePolicy {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindow>>,
    ) -> Self {
        self.weekly_maintenance_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataRedisCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_MaintenancePolicy {}

impl BuildDataRedisCluster_MaintenancePolicy {
    pub fn build(self) -> DataRedisCluster_MaintenancePolicy {
        DataRedisCluster_MaintenancePolicy {
            create_time: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_MaintenancePolicyRef {
        DataRedisCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> terrars::ListRef<DataRedisCluster_MaintenancePolicy_WeeklyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_MaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_MaintenanceSchedule {
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

impl terrars::ToListMappable for DataRedisCluster_MaintenanceSchedule {
    type O = terrars::BlockAssignable<DataRedisCluster_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_MaintenanceSchedule {}

impl BuildDataRedisCluster_MaintenanceSchedule {
    pub fn build(self) -> DataRedisCluster_MaintenanceSchedule {
        DataRedisCluster_MaintenanceSchedule {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_MaintenanceScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_MaintenanceScheduleRef {
        DataRedisCluster_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_MaintenanceScheduleRef {
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
pub struct DataRedisCluster_ManagedBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_ManagedBackupSource {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_ManagedBackupSource {
    type O = terrars::BlockAssignable<DataRedisCluster_ManagedBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_ManagedBackupSource {}

impl BuildDataRedisCluster_ManagedBackupSource {
    pub fn build(self) -> DataRedisCluster_ManagedBackupSource {
        DataRedisCluster_ManagedBackupSource { backup: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_ManagedBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_ManagedBackupSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_ManagedBackupSourceRef {
        DataRedisCluster_ManagedBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_ManagedBackupSourceRef {
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
pub struct DataRedisCluster_ManagedServerCa_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataRedisCluster_ManagedServerCa_CaCerts {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_ManagedServerCa_CaCerts {
    type O = terrars::BlockAssignable<DataRedisCluster_ManagedServerCa_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_ManagedServerCa_CaCerts {}

impl BuildDataRedisCluster_ManagedServerCa_CaCerts {
    pub fn build(self) -> DataRedisCluster_ManagedServerCa_CaCerts {
        DataRedisCluster_ManagedServerCa_CaCerts { certificates: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_ManagedServerCa_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_ManagedServerCa_CaCertsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_ManagedServerCa_CaCertsRef {
        DataRedisCluster_ManagedServerCa_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_ManagedServerCa_CaCertsRef {
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
pub struct DataRedisCluster_ManagedServerCa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<DataRedisCluster_ManagedServerCa_CaCerts>>,
}

impl DataRedisCluster_ManagedServerCa {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_ManagedServerCa_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_ManagedServerCa {
    type O = terrars::BlockAssignable<DataRedisCluster_ManagedServerCa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_ManagedServerCa {}

impl BuildDataRedisCluster_ManagedServerCa {
    pub fn build(self) -> DataRedisCluster_ManagedServerCa {
        DataRedisCluster_ManagedServerCa { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_ManagedServerCaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_ManagedServerCaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_ManagedServerCaRef {
        DataRedisCluster_ManagedServerCaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_ManagedServerCaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<DataRedisCluster_ManagedServerCa_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_PersistenceConfig_AofConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_fsync: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_PersistenceConfig_AofConfig {
    #[doc = "Set the field `append_fsync`.\n"]
    pub fn set_append_fsync(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.append_fsync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_PersistenceConfig_AofConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_PersistenceConfig_AofConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PersistenceConfig_AofConfig {}

impl BuildDataRedisCluster_PersistenceConfig_AofConfig {
    pub fn build(self) -> DataRedisCluster_PersistenceConfig_AofConfig {
        DataRedisCluster_PersistenceConfig_AofConfig { append_fsync: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PersistenceConfig_AofConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PersistenceConfig_AofConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_PersistenceConfig_AofConfigRef {
        DataRedisCluster_PersistenceConfig_AofConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PersistenceConfig_AofConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_fsync` after provisioning.\n"]
    pub fn append_fsync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_fsync", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_PersistenceConfig_RdbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_PersistenceConfig_RdbConfig {
    #[doc = "Set the field `rdb_snapshot_period`.\n"]
    pub fn set_rdb_snapshot_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rdb_snapshot_start_time`.\n"]
    pub fn set_rdb_snapshot_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_snapshot_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_PersistenceConfig_RdbConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_PersistenceConfig_RdbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PersistenceConfig_RdbConfig {}

impl BuildDataRedisCluster_PersistenceConfig_RdbConfig {
    pub fn build(self) -> DataRedisCluster_PersistenceConfig_RdbConfig {
        DataRedisCluster_PersistenceConfig_RdbConfig {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PersistenceConfig_RdbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PersistenceConfig_RdbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisCluster_PersistenceConfig_RdbConfigRef {
        DataRedisCluster_PersistenceConfig_RdbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PersistenceConfig_RdbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\n"]
    pub fn rdb_snapshot_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_period", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\n"]
    pub fn rdb_snapshot_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_snapshot_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_PersistenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aof_config: std::option::Option<terrars::ListField<DataRedisCluster_PersistenceConfig_AofConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_config: std::option::Option<terrars::ListField<DataRedisCluster_PersistenceConfig_RdbConfig>>,
}

impl DataRedisCluster_PersistenceConfig {
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_PersistenceConfig_AofConfig>>,
    ) -> Self {
        self.aof_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rdb_config`.\n"]
    pub fn set_rdb_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_PersistenceConfig_RdbConfig>>,
    ) -> Self {
        self.rdb_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_PersistenceConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_PersistenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PersistenceConfig {}

impl BuildDataRedisCluster_PersistenceConfig {
    pub fn build(self) -> DataRedisCluster_PersistenceConfig {
        DataRedisCluster_PersistenceConfig {
            aof_config: core::default::Default::default(),
            mode: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PersistenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PersistenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_PersistenceConfigRef {
        DataRedisCluster_PersistenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PersistenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> terrars::ListRef<DataRedisCluster_PersistenceConfig_AofConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> terrars::ListRef<DataRedisCluster_PersistenceConfig_RdbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_PscConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_PscConfigs {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_PscConfigs {
    type O = terrars::BlockAssignable<DataRedisCluster_PscConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PscConfigs {}

impl BuildDataRedisCluster_PscConfigs {
    pub fn build(self) -> DataRedisCluster_PscConfigs {
        DataRedisCluster_PscConfigs { network: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PscConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PscConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_PscConfigsRef {
        DataRedisCluster_PscConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PscConfigsRef {
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
pub struct DataRedisCluster_PscConnections {
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

impl DataRedisCluster_PscConnections {
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

impl terrars::ToListMappable for DataRedisCluster_PscConnections {
    type O = terrars::BlockAssignable<DataRedisCluster_PscConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PscConnections {}

impl BuildDataRedisCluster_PscConnections {
    pub fn build(self) -> DataRedisCluster_PscConnections {
        DataRedisCluster_PscConnections {
            address: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PscConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PscConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_PscConnectionsRef {
        DataRedisCluster_PscConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PscConnectionsRef {
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
pub struct DataRedisCluster_PscServiceAttachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_PscServiceAttachments {
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

impl terrars::ToListMappable for DataRedisCluster_PscServiceAttachments {
    type O = terrars::BlockAssignable<DataRedisCluster_PscServiceAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_PscServiceAttachments {}

impl BuildDataRedisCluster_PscServiceAttachments {
    pub fn build(self) -> DataRedisCluster_PscServiceAttachments {
        DataRedisCluster_PscServiceAttachments {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_PscServiceAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_PscServiceAttachmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_PscServiceAttachmentsRef {
        DataRedisCluster_PscServiceAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_PscServiceAttachmentsRef {
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
pub struct DataRedisCluster_StateInfo_UpdateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_shard_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRedisCluster_StateInfo_UpdateInfo {
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

impl terrars::ToListMappable for DataRedisCluster_StateInfo_UpdateInfo {
    type O = terrars::BlockAssignable<DataRedisCluster_StateInfo_UpdateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_StateInfo_UpdateInfo {}

impl BuildDataRedisCluster_StateInfo_UpdateInfo {
    pub fn build(self) -> DataRedisCluster_StateInfo_UpdateInfo {
        DataRedisCluster_StateInfo_UpdateInfo {
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_StateInfo_UpdateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_StateInfo_UpdateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_StateInfo_UpdateInfoRef {
        DataRedisCluster_StateInfo_UpdateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_StateInfo_UpdateInfoRef {
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
pub struct DataRedisCluster_StateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_info: std::option::Option<terrars::ListField<DataRedisCluster_StateInfo_UpdateInfo>>,
}

impl DataRedisCluster_StateInfo {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRedisCluster_StateInfo_UpdateInfo>>,
    ) -> Self {
        self.update_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_StateInfo {
    type O = terrars::BlockAssignable<DataRedisCluster_StateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_StateInfo {}

impl BuildDataRedisCluster_StateInfo {
    pub fn build(self) -> DataRedisCluster_StateInfo {
        DataRedisCluster_StateInfo { update_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_StateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_StateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_StateInfoRef {
        DataRedisCluster_StateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_StateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> terrars::ListRef<DataRedisCluster_StateInfo_UpdateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisCluster_ZoneDistributionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisCluster_ZoneDistributionConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisCluster_ZoneDistributionConfig {
    type O = terrars::BlockAssignable<DataRedisCluster_ZoneDistributionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisCluster_ZoneDistributionConfig {}

impl BuildDataRedisCluster_ZoneDistributionConfig {
    pub fn build(self) -> DataRedisCluster_ZoneDistributionConfig {
        DataRedisCluster_ZoneDistributionConfig {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedisCluster_ZoneDistributionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisCluster_ZoneDistributionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisCluster_ZoneDistributionConfigRef {
        DataRedisCluster_ZoneDistributionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisCluster_ZoneDistributionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
