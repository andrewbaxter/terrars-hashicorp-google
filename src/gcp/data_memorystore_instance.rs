#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataMemorystoreInstanceData_ {
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
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMemorystoreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMemorystoreInstanceData_>,
}

#[derive(Clone)]
pub struct DataMemorystoreInstance(std::rc::Rc<DataMemorystoreInstance_>);

impl DataMemorystoreInstance {
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

    #[doc =
        "Set the field `location`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type 'memorystore.googleapis.com/CertificateAuthority'."]
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
        "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> terrars::ListRef<DataMemorystoreInstance_AutomatedBackupConfigRef> {
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
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\nCross instance replication config"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
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
        "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\nImmutable. User inputs for the auto-created endpoints connections."]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n'desired_psc_auto_connections' is deprecated  Use 'desired_auto_created_endpoints' instead 'terraform import' will only work with desired_auto_created_endpoints'."]
    pub fn desired_psc_auto_connections(&self) -> terrars::ListRef<DataMemorystoreInstance_DesiredPscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.desired_psc_auto_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<DataMemorystoreInstance_DiscoveryEndpointsRef> {
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
    pub fn endpoints(&self) -> terrars::ListRef<DataMemorystoreInstance_EndpointsRef> {
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

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nGCS source for the instance."]
    pub fn gcs_source(&self) -> terrars::ListRef<DataMemorystoreInstance_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataMemorystoreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataMemorystoreInstance_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_backup_source` after provisioning.\nManaged backup source for the instance."]
    pub fn managed_backup_source(&self) -> terrars::ListRef<DataMemorystoreInstance_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<DataMemorystoreInstance_ManagedServerCaRef> {
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
    pub fn node_config(&self) -> terrars::ListRef<DataMemorystoreInstance_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_config` after provisioning.\nRepresents persistence configuration for a instance."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataMemorystoreInstance_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> terrars::ListRef<DataMemorystoreInstance_PscAttachmentDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_attachment_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<DataMemorystoreInstance_PscAutoConnectionsRef> {
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
    pub fn state_info(&self) -> terrars::ListRef<DataMemorystoreInstance_StateInfoRef> {
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

    #[doc =
        "Get a reference to the value of field `zone_distribution_config` after provisioning.\nZone distribution configuration for allocation of instance resources."]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<DataMemorystoreInstance_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataMemorystoreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMemorystoreInstance { }

impl terrars::ToListMappable for DataMemorystoreInstance {
    type O = terrars::ListRef<DataMemorystoreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMemorystoreInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_memorystore_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMemorystoreInstance {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The ID to use for the instance, which will become the final component of\nthe instance's resource name.\n\nThis value is subject to the following restrictions:\n\n* Must be 4-63 characters in length\n* Must begin with a letter or digit\n* Must contain only lowercase letters, digits, and hyphens\n* Must not end with a hyphen\n* Must be unique within a location"]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMemorystoreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMemorystoreInstance {
        let out = DataMemorystoreInstance(std::rc::Rc::new(DataMemorystoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMemorystoreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMemorystoreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMemorystoreInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `automated_backup_config` after provisioning.\nThe automated backup config for a instance."]
    pub fn automated_backup_config(&self) -> terrars::ListRef<DataMemorystoreInstance_AutomatedBackupConfigRef> {
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
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp of the instance."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_instance_replication_config` after provisioning.\nCross instance replication config"]
    pub fn cross_instance_replication_config(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cross_instance_replication_config", self.extract_ref()),
        )
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
        "Get a reference to the value of field `desired_auto_created_endpoints` after provisioning.\nImmutable. User inputs for the auto-created endpoints connections."]
    pub fn desired_auto_created_endpoints(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_auto_created_endpoints", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `desired_psc_auto_connections` after provisioning.\n'desired_psc_auto_connections' is deprecated  Use 'desired_auto_created_endpoints' instead 'terraform import' will only work with desired_auto_created_endpoints'."]
    pub fn desired_psc_auto_connections(&self) -> terrars::ListRef<DataMemorystoreInstance_DesiredPscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.desired_psc_auto_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nDeprecated. Output only. Endpoints clients can connect to the instance through."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<DataMemorystoreInstance_DiscoveryEndpointsRef> {
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
    pub fn endpoints(&self) -> terrars::ListRef<DataMemorystoreInstance_EndpointsRef> {
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

    #[doc = "Get a reference to the value of field `gcs_source` after provisioning.\nGCS source for the instance."]
    pub fn gcs_source(&self) -> terrars::ListRef<DataMemorystoreInstance_GcsSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_source", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for a cluster"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataMemorystoreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataMemorystoreInstance_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_version` after provisioning.\nThis field can be used to trigger self service update to indicate the desired maintenance version. The input to this field can be determined by the available_maintenance_versions field.\n*Note*: This field can only be specified when updating an existing cluster to a newer version. Downgrades are currently not supported!"]
    pub fn maintenance_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_backup_source` after provisioning.\nManaged backup source for the instance."]
    pub fn managed_backup_source(&self) -> terrars::ListRef<DataMemorystoreInstance_ManagedBackupSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_backup_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_server_ca` after provisioning.\nInstance's Certificate Authority. This field will only be populated if instance's transit_encryption_mode is SERVER_AUTHENTICATION"]
    pub fn managed_server_ca(&self) -> terrars::ListRef<DataMemorystoreInstance_ManagedServerCaRef> {
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
    pub fn node_config(&self) -> terrars::ListRef<DataMemorystoreInstance_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nOptional. Machine type for individual nodes of the instance. \n Possible values:\n SHARED_CORE_NANO\nCUSTOM_PICO\nCUSTOM_MICRO\nCUSTOM_MINI\nHIGHMEM_MEDIUM\nHIGHCPU_MEDIUM\nHIGHMEM_XLARGE\nSTANDARD_SMALL\nSTANDARD_LARGE\nHIGHMEM_2XLARGE"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_config` after provisioning.\nRepresents persistence configuration for a instance."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataMemorystoreInstance_PersistenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_attachment_details` after provisioning.\nConfiguration of a service attachment of the cluster, for creating PSC connections."]
    pub fn psc_attachment_details(&self) -> terrars::ListRef<DataMemorystoreInstance_PscAttachmentDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_attachment_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_auto_connections` after provisioning.\nOutput only. User inputs and resource details of the auto-created PSC connections."]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<DataMemorystoreInstance_PscAutoConnectionsRef> {
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
    pub fn state_info(&self) -> terrars::ListRef<DataMemorystoreInstance_StateInfoRef> {
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

    #[doc =
        "Get a reference to the value of field `zone_distribution_config` after provisioning.\nZone distribution configuration for allocation of instance resources."]
    pub fn zone_distribution_config(&self) -> terrars::ListRef<DataMemorystoreInstance_ZoneDistributionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_distribution_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    type O =
        terrars::BlockAssignable<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {}

impl BuildDataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
    pub fn build(self) -> DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
        DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime {
            hours: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
        DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef {
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
pub struct DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>>,
}

impl DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {}

impl BuildDataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
    pub fn build(self) -> DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
        DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule {
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
        DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_AutomatedBackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_frequency_schedule: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_AutomatedBackupConfig {
    #[doc = "Set the field `fixed_frequency_schedule`.\n"]
    pub fn set_fixed_frequency_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencySchedule>>,
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

impl terrars::ToListMappable for DataMemorystoreInstance_AutomatedBackupConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_AutomatedBackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_AutomatedBackupConfig {}

impl BuildDataMemorystoreInstance_AutomatedBackupConfig {
    pub fn build(self) -> DataMemorystoreInstance_AutomatedBackupConfig {
        DataMemorystoreInstance_AutomatedBackupConfig {
            fixed_frequency_schedule: core::default::Default::default(),
            retention: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_AutomatedBackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_AutomatedBackupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_AutomatedBackupConfigRef {
        DataMemorystoreInstance_AutomatedBackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_AutomatedBackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_frequency_schedule` after provisioning.\n"]
    pub fn fixed_frequency_schedule(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_AutomatedBackupConfig_FixedFrequencyScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_frequency_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
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

impl terrars::ToListMappable for DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {}

impl BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
    pub fn build(self) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef {
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
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
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

impl terrars::ToListMappable for DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    type O =
        terrars
        ::BlockAssignable<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {}

impl BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
    pub fn build(self) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef {
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
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_instance: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_instance: std
    ::option
    ::Option<
        terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance>,
    >,
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    #[doc = "Set the field `primary_instance`.\n"]
    pub fn set_primary_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>,
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
                        ::ListField<
                            DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstance,
                        >,
                    >,
    ) -> Self {
        self.secondary_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {}

impl BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
    pub fn build(self) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership {
            primary_instance: core::default::Default::default(),
            secondary_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_instance` after provisioning.\n"]
    pub fn secondary_instance(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership_SecondaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
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

impl terrars::ToListMappable for DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {}

impl BuildDataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
    pub fn build(self) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances {
            instance: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
        DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef {
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
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_instance: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_instances: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfig {
    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_Membership>>,
    ) -> Self {
        self.membership = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstance>,
                    >,
    ) -> Self {
        self.primary_instance = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstances>,
                    >,
    ) -> Self {
        self.secondary_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_CrossInstanceReplicationConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_CrossInstanceReplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_CrossInstanceReplicationConfig {}

impl BuildDataMemorystoreInstance_CrossInstanceReplicationConfig {
    pub fn build(self) -> DataMemorystoreInstance_CrossInstanceReplicationConfig {
        DataMemorystoreInstance_CrossInstanceReplicationConfig {
            instance_role: core::default::Default::default(),
            membership: core::default::Default::default(),
            primary_instance: core::default::Default::default(),
            secondary_instances: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_CrossInstanceReplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_CrossInstanceReplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_CrossInstanceReplicationConfigRef {
        DataMemorystoreInstance_CrossInstanceReplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_CrossInstanceReplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role", self.base))
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfig_MembershipRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_instance` after provisioning.\n"]
    pub fn primary_instance(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfig_PrimaryInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_instances` after provisioning.\n"]
    pub fn secondary_instances(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_CrossInstanceReplicationConfig_SecondaryInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_DesiredAutoCreatedEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_DesiredAutoCreatedEndpoints {
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
}

impl terrars::ToListMappable for DataMemorystoreInstance_DesiredAutoCreatedEndpoints {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_DesiredAutoCreatedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_DesiredAutoCreatedEndpoints {}

impl BuildDataMemorystoreInstance_DesiredAutoCreatedEndpoints {
    pub fn build(self) -> DataMemorystoreInstance_DesiredAutoCreatedEndpoints {
        DataMemorystoreInstance_DesiredAutoCreatedEndpoints {
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef {
        DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_DesiredAutoCreatedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_DesiredPscAutoConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_DesiredPscAutoConnections {
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
}

impl terrars::ToListMappable for DataMemorystoreInstance_DesiredPscAutoConnections {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_DesiredPscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_DesiredPscAutoConnections {}

impl BuildDataMemorystoreInstance_DesiredPscAutoConnections {
    pub fn build(self) -> DataMemorystoreInstance_DesiredPscAutoConnections {
        DataMemorystoreInstance_DesiredPscAutoConnections {
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_DesiredPscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_DesiredPscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_DesiredPscAutoConnectionsRef {
        DataMemorystoreInstance_DesiredPscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_DesiredPscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_DiscoveryEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMemorystoreInstance_DiscoveryEndpoints {
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

impl terrars::ToListMappable for DataMemorystoreInstance_DiscoveryEndpoints {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_DiscoveryEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_DiscoveryEndpoints {}

impl BuildDataMemorystoreInstance_DiscoveryEndpoints {
    pub fn build(self) -> DataMemorystoreInstance_DiscoveryEndpoints {
        DataMemorystoreInstance_DiscoveryEndpoints {
            address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_DiscoveryEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_DiscoveryEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_DiscoveryEndpointsRef {
        DataMemorystoreInstance_DiscoveryEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_DiscoveryEndpointsRef {
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
pub struct DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
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

impl DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
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

impl terrars::ToListMappable for DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {}

impl BuildDataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
    pub fn build(self) -> DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
        DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection {
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
pub struct DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
        DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef {
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
pub struct DataMemorystoreInstance_Endpoints_Connections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connection: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection>>,
}

impl DataMemorystoreInstance_Endpoints_Connections {
    #[doc = "Set the field `psc_auto_connection`.\n"]
    pub fn set_psc_auto_connection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMemorystoreInstance_Endpoints_Connections_PscAutoConnection>>,
    ) -> Self {
        self.psc_auto_connection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_Endpoints_Connections {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_Endpoints_Connections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_Endpoints_Connections {}

impl BuildDataMemorystoreInstance_Endpoints_Connections {
    pub fn build(self) -> DataMemorystoreInstance_Endpoints_Connections {
        DataMemorystoreInstance_Endpoints_Connections { psc_auto_connection: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_Endpoints_ConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_Endpoints_ConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_Endpoints_ConnectionsRef {
        DataMemorystoreInstance_Endpoints_ConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_Endpoints_ConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `psc_auto_connection` after provisioning.\n"]
    pub fn psc_auto_connection(
        &self,
    ) -> terrars::ListRef<DataMemorystoreInstance_Endpoints_Connections_PscAutoConnectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connection", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connections: std::option::Option<terrars::ListField<DataMemorystoreInstance_Endpoints_Connections>>,
}

impl DataMemorystoreInstance_Endpoints {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMemorystoreInstance_Endpoints_Connections>>,
    ) -> Self {
        self.connections = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_Endpoints {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_Endpoints {}

impl BuildDataMemorystoreInstance_Endpoints {
    pub fn build(self) -> DataMemorystoreInstance_Endpoints {
        DataMemorystoreInstance_Endpoints { connections: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_EndpointsRef {
        DataMemorystoreInstance_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> terrars::ListRef<DataMemorystoreInstance_Endpoints_ConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_GcsSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uris: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataMemorystoreInstance_GcsSource {
    #[doc = "Set the field `uris`.\n"]
    pub fn set_uris(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_GcsSource {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_GcsSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_GcsSource {}

impl BuildDataMemorystoreInstance_GcsSource {
    pub fn build(self) -> DataMemorystoreInstance_GcsSource {
        DataMemorystoreInstance_GcsSource { uris: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_GcsSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_GcsSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_GcsSourceRef {
        DataMemorystoreInstance_GcsSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_GcsSourceRef {
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
pub struct DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
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

impl terrars::ToListMappable for DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {}

impl BuildDataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
    pub fn build(self) -> DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
        DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
        DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef {
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
pub struct DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>>,
}

impl DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
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
                    ::Into<
                        terrars
                        ::ListField<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {}

impl BuildDataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
    pub fn build(self) -> DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
        DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
        DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef {
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
    ) -> terrars::ListRef<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>>,
}

impl DataMemorystoreInstance_MaintenancePolicy {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindow>>,
    ) -> Self {
        self.weekly_maintenance_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_MaintenancePolicy {}

impl BuildDataMemorystoreInstance_MaintenancePolicy {
    pub fn build(self) -> DataMemorystoreInstance_MaintenancePolicy {
        DataMemorystoreInstance_MaintenancePolicy {
            create_time: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_MaintenancePolicyRef {
        DataMemorystoreInstance_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_MaintenancePolicyRef {
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
    ) -> terrars::ListRef<DataMemorystoreInstance_MaintenancePolicy_WeeklyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_MaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_MaintenanceSchedule {
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

impl terrars::ToListMappable for DataMemorystoreInstance_MaintenanceSchedule {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_MaintenanceSchedule {}

impl BuildDataMemorystoreInstance_MaintenanceSchedule {
    pub fn build(self) -> DataMemorystoreInstance_MaintenanceSchedule {
        DataMemorystoreInstance_MaintenanceSchedule {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_MaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_MaintenanceScheduleRef {
        DataMemorystoreInstance_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_MaintenanceScheduleRef {
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
pub struct DataMemorystoreInstance_ManagedBackupSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_ManagedBackupSource {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_ManagedBackupSource {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_ManagedBackupSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_ManagedBackupSource {}

impl BuildDataMemorystoreInstance_ManagedBackupSource {
    pub fn build(self) -> DataMemorystoreInstance_ManagedBackupSource {
        DataMemorystoreInstance_ManagedBackupSource { backup: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_ManagedBackupSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_ManagedBackupSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_ManagedBackupSourceRef {
        DataMemorystoreInstance_ManagedBackupSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_ManagedBackupSourceRef {
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
pub struct DataMemorystoreInstance_ManagedServerCa_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataMemorystoreInstance_ManagedServerCa_CaCerts {
    #[doc = "Set the field `certificates`.\n"]
    pub fn set_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_ManagedServerCa_CaCerts {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_ManagedServerCa_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_ManagedServerCa_CaCerts {}

impl BuildDataMemorystoreInstance_ManagedServerCa_CaCerts {
    pub fn build(self) -> DataMemorystoreInstance_ManagedServerCa_CaCerts {
        DataMemorystoreInstance_ManagedServerCa_CaCerts { certificates: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_ManagedServerCa_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_ManagedServerCa_CaCertsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_ManagedServerCa_CaCertsRef {
        DataMemorystoreInstance_ManagedServerCa_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_ManagedServerCa_CaCertsRef {
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
pub struct DataMemorystoreInstance_ManagedServerCa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<DataMemorystoreInstance_ManagedServerCa_CaCerts>>,
}

impl DataMemorystoreInstance_ManagedServerCa {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMemorystoreInstance_ManagedServerCa_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_ManagedServerCa {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_ManagedServerCa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_ManagedServerCa {}

impl BuildDataMemorystoreInstance_ManagedServerCa {
    pub fn build(self) -> DataMemorystoreInstance_ManagedServerCa {
        DataMemorystoreInstance_ManagedServerCa { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_ManagedServerCaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_ManagedServerCaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_ManagedServerCaRef {
        DataMemorystoreInstance_ManagedServerCaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_ManagedServerCaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<DataMemorystoreInstance_ManagedServerCa_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMemorystoreInstance_NodeConfig {
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_NodeConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_NodeConfig {}

impl BuildDataMemorystoreInstance_NodeConfig {
    pub fn build(self) -> DataMemorystoreInstance_NodeConfig {
        DataMemorystoreInstance_NodeConfig { size_gb: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_NodeConfigRef {
        DataMemorystoreInstance_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_NodeConfigRef {
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
pub struct DataMemorystoreInstance_PersistenceConfig_AofConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_fsync: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_PersistenceConfig_AofConfig {
    #[doc = "Set the field `append_fsync`.\n"]
    pub fn set_append_fsync(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.append_fsync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_PersistenceConfig_AofConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_PersistenceConfig_AofConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_PersistenceConfig_AofConfig {}

impl BuildDataMemorystoreInstance_PersistenceConfig_AofConfig {
    pub fn build(self) -> DataMemorystoreInstance_PersistenceConfig_AofConfig {
        DataMemorystoreInstance_PersistenceConfig_AofConfig { append_fsync: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_PersistenceConfig_AofConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_PersistenceConfig_AofConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_PersistenceConfig_AofConfigRef {
        DataMemorystoreInstance_PersistenceConfig_AofConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_PersistenceConfig_AofConfigRef {
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
pub struct DataMemorystoreInstance_PersistenceConfig_RdbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_PersistenceConfig_RdbConfig {
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

impl terrars::ToListMappable for DataMemorystoreInstance_PersistenceConfig_RdbConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_PersistenceConfig_RdbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_PersistenceConfig_RdbConfig {}

impl BuildDataMemorystoreInstance_PersistenceConfig_RdbConfig {
    pub fn build(self) -> DataMemorystoreInstance_PersistenceConfig_RdbConfig {
        DataMemorystoreInstance_PersistenceConfig_RdbConfig {
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_PersistenceConfig_RdbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_PersistenceConfig_RdbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_PersistenceConfig_RdbConfigRef {
        DataMemorystoreInstance_PersistenceConfig_RdbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_PersistenceConfig_RdbConfigRef {
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
pub struct DataMemorystoreInstance_PersistenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aof_config: std::option::Option<terrars::ListField<DataMemorystoreInstance_PersistenceConfig_AofConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_config: std::option::Option<terrars::ListField<DataMemorystoreInstance_PersistenceConfig_RdbConfig>>,
}

impl DataMemorystoreInstance_PersistenceConfig {
    #[doc = "Set the field `aof_config`.\n"]
    pub fn set_aof_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMemorystoreInstance_PersistenceConfig_AofConfig>>,
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
        v: impl std::convert::Into<terrars::ListField<DataMemorystoreInstance_PersistenceConfig_RdbConfig>>,
    ) -> Self {
        self.rdb_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_PersistenceConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_PersistenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_PersistenceConfig {}

impl BuildDataMemorystoreInstance_PersistenceConfig {
    pub fn build(self) -> DataMemorystoreInstance_PersistenceConfig {
        DataMemorystoreInstance_PersistenceConfig {
            aof_config: core::default::Default::default(),
            mode: core::default::Default::default(),
            rdb_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_PersistenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_PersistenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_PersistenceConfigRef {
        DataMemorystoreInstance_PersistenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_PersistenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aof_config` after provisioning.\n"]
    pub fn aof_config(&self) -> terrars::ListRef<DataMemorystoreInstance_PersistenceConfig_AofConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aof_config", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_config` after provisioning.\n"]
    pub fn rdb_config(&self) -> terrars::ListRef<DataMemorystoreInstance_PersistenceConfig_RdbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdb_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_PscAttachmentDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_PscAttachmentDetails {
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

impl terrars::ToListMappable for DataMemorystoreInstance_PscAttachmentDetails {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_PscAttachmentDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_PscAttachmentDetails {}

impl BuildDataMemorystoreInstance_PscAttachmentDetails {
    pub fn build(self) -> DataMemorystoreInstance_PscAttachmentDetails {
        DataMemorystoreInstance_PscAttachmentDetails {
            connection_type: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_PscAttachmentDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_PscAttachmentDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_PscAttachmentDetailsRef {
        DataMemorystoreInstance_PscAttachmentDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_PscAttachmentDetailsRef {
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
pub struct DataMemorystoreInstance_PscAutoConnections {
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

impl DataMemorystoreInstance_PscAutoConnections {
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

impl terrars::ToListMappable for DataMemorystoreInstance_PscAutoConnections {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_PscAutoConnections {}

impl BuildDataMemorystoreInstance_PscAutoConnections {
    pub fn build(self) -> DataMemorystoreInstance_PscAutoConnections {
        DataMemorystoreInstance_PscAutoConnections {
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
pub struct DataMemorystoreInstance_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_PscAutoConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_PscAutoConnectionsRef {
        DataMemorystoreInstance_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_PscAutoConnectionsRef {
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
pub struct DataMemorystoreInstance_StateInfo_UpdateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_shard_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMemorystoreInstance_StateInfo_UpdateInfo {
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

impl terrars::ToListMappable for DataMemorystoreInstance_StateInfo_UpdateInfo {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_StateInfo_UpdateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_StateInfo_UpdateInfo {}

impl BuildDataMemorystoreInstance_StateInfo_UpdateInfo {
    pub fn build(self) -> DataMemorystoreInstance_StateInfo_UpdateInfo {
        DataMemorystoreInstance_StateInfo_UpdateInfo {
            target_engine_version: core::default::Default::default(),
            target_node_type: core::default::Default::default(),
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_StateInfo_UpdateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_StateInfo_UpdateInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_StateInfo_UpdateInfoRef {
        DataMemorystoreInstance_StateInfo_UpdateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_StateInfo_UpdateInfoRef {
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
pub struct DataMemorystoreInstance_StateInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_info: std::option::Option<terrars::ListField<DataMemorystoreInstance_StateInfo_UpdateInfo>>,
}

impl DataMemorystoreInstance_StateInfo {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMemorystoreInstance_StateInfo_UpdateInfo>>,
    ) -> Self {
        self.update_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMemorystoreInstance_StateInfo {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_StateInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_StateInfo {}

impl BuildDataMemorystoreInstance_StateInfo {
    pub fn build(self) -> DataMemorystoreInstance_StateInfo {
        DataMemorystoreInstance_StateInfo { update_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_StateInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_StateInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMemorystoreInstance_StateInfoRef {
        DataMemorystoreInstance_StateInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_StateInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> terrars::ListRef<DataMemorystoreInstance_StateInfo_UpdateInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorystoreInstance_ZoneDistributionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMemorystoreInstance_ZoneDistributionConfig {
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

impl terrars::ToListMappable for DataMemorystoreInstance_ZoneDistributionConfig {
    type O = terrars::BlockAssignable<DataMemorystoreInstance_ZoneDistributionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorystoreInstance_ZoneDistributionConfig {}

impl BuildDataMemorystoreInstance_ZoneDistributionConfig {
    pub fn build(self) -> DataMemorystoreInstance_ZoneDistributionConfig {
        DataMemorystoreInstance_ZoneDistributionConfig {
            mode: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMemorystoreInstance_ZoneDistributionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMemorystoreInstance_ZoneDistributionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMemorystoreInstance_ZoneDistributionConfigRef {
        DataMemorystoreInstance_ZoneDistributionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorystoreInstance_ZoneDistributionConfigRef {
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
