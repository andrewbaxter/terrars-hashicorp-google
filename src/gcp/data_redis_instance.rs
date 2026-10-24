use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataRedisInstanceData {
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

struct DataRedisInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRedisInstanceData>,
}

#[derive(Clone)]
pub struct DataRedisInstance(std::rc::Rc<DataRedisInstance_>);

impl DataRedisInstance {
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

    #[doc = "Set the field `region`.\nThe name of the Redis region of the instance."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `alternative_location_id` after provisioning.\nOnly applicable to STANDARD_HA tier which protects the instance\nagainst zonal failures by provisioning it across two zones.\nIf provided, it must be a different zone from the one provided in\n[locationId]."]
    pub fn alternative_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alternative_location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_enabled` after provisioning.\nOptional. Indicates whether OSS Redis AUTH is enabled for the\ninstance. If set to \"true\" AUTH is enabled on the instance.\nDefault value is \"false\" meaning AUTH is disabled."]
    pub fn auth_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_string` after provisioning.\nAUTH String set on the instance. This field will only be populated if auth_enabled is true."]
    pub fn auth_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the Google Compute Engine network to which the\ninstance is connected. If left unspecified, the default network\nwill be used."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connect_mode` after provisioning.\nThe connection mode of the Redis instance. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn connect_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_location_id` after provisioning.\nThe current zone where the Redis endpoint is placed.\nFor Basic Tier instances, this will always be the same as the\n[locationId] provided by the user at creation time. For Standard Tier\ninstances, this can be either [locationId] or [alternativeLocationId]\nand can change after a failover event."]
    pub fn current_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_key` after provisioning.\nOptional. The KMS key reference that you want to use to encrypt the data at rest for this Redis\ninstance. If this is provided, CMEK is enabled."]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary and optional user-provided name for the instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHostname or IP address of the exposed Redis endpoint used by clients\nto connect to the service."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_id` after provisioning.\nThe zone where the instance will be provisioned. If not provided,\nthe service will choose a zone for the instance. For STANDARD_HA tier,\ninstances will be created across two zones for protection against\nzonal failures. If [alternativeLocationId] is also provided, it must\nbe different from [locationId]."]
    pub fn location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataRedisInstanceMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataRedisInstanceMaintenanceScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nRedis memory size in GiB."]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nodes` after provisioning.\nOutput only. Info per node."]
    pub fn nodes(&self) -> terrars::ListRef<DataRedisInstanceNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence configuration for an instance."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataRedisInstancePersistenceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_iam_identity` after provisioning.\nOutput only. Cloud IAM identity used by import / export operations\nto transfer data to/from Cloud Storage. Format is \"serviceAccount:\".\nThe value may change over time for a given instance so should be\nchecked before each import/export operation."]
    pub fn persistence_iam_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistence_iam_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number of the exposed Redis endpoint."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_endpoint` after provisioning.\nOutput only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only.\nTargets all healthy replica nodes in instance. Replication is asynchronous and replica nodes\nwill exhibit some lag behind the primary. Write requests must target 'host'."]
    pub fn read_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_endpoint_port` after provisioning.\nOutput only. The port number of the exposed readonly redis endpoint. Standard tier only.\nWrite requests should target 'port'."]
    pub fn read_endpoint_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_endpoint_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replicas_mode` after provisioning.\nOptional. Read replica mode. Can only be specified when trying to create the instance.\nIf not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.\n- READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the\ninstance cannot scale up or down the number of replicas.\n- READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance\ncan scale up and down the number of replicas. Possible values: [\"READ_REPLICAS_DISABLED\", \"READ_REPLICAS_ENABLED\"]"]
    pub fn read_replicas_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_replicas_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_configs` after provisioning.\nRedis configuration parameters, according to http://redis.io/topics/config.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs"]
    pub fn redis_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.redis_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_version` after provisioning.\nThe version of Redis software. If not provided, latest supported\nversion will be used. Please check the API documentation linked\nat the top for the latest valid values."]
    pub fn redis_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Redis region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes. The valid range for the Standard Tier with\nread replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled\nfor a Standard Tier instance, the only valid value is 1 and the default is 1.\nThe valid value for basic tier is 0 and the default is also 0."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secondary_ip_range` after provisioning.\nOptional. Additional IP range for node placement. Required when enabling read replicas on\nan existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or\n\"auto\". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address\nrange associated with the private service access connection, or \"auto\"."]
    pub fn secondary_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_certs` after provisioning.\nList of server CA certificates for the instance."]
    pub fn server_ca_certs(&self) -> terrars::ListRef<DataRedisInstanceServerCaCertsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_certs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance. Must be one of these values:\n\n- BASIC: standalone instance\n- STANDARD_HA: highly available primary/replica instances Default value: \"BASIC\" Possible values: [\"BASIC\", \"STANDARD_HA\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nThe TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.\n\n- SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication Default value: \"DISABLED\" Possible values: [\"SERVER_AUTHENTICATION\", \"DISABLED\"]"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }
}

impl terrars::Referable for DataRedisInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRedisInstance { }

impl terrars::ToListMappable for DataRedisInstance {
    type O = terrars::ListRef<DataRedisInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRedisInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_redis_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedisInstance {
    pub tf_id: std::string::String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataRedisInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRedisInstance {
        let out = DataRedisInstance(std::rc::Rc::new(DataRedisInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRedisInstanceData {
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

pub struct DataRedisInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRedisInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `alternative_location_id` after provisioning.\nOnly applicable to STANDARD_HA tier which protects the instance\nagainst zonal failures by provisioning it across two zones.\nIf provided, it must be a different zone from the one provided in\n[locationId]."]
    pub fn alternative_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alternative_location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_enabled` after provisioning.\nOptional. Indicates whether OSS Redis AUTH is enabled for the\ninstance. If set to \"true\" AUTH is enabled on the instance.\nDefault value is \"false\" meaning AUTH is disabled."]
    pub fn auth_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_string` after provisioning.\nAUTH String set on the instance. This field will only be populated if auth_enabled is true."]
    pub fn auth_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the Google Compute Engine network to which the\ninstance is connected. If left unspecified, the default network\nwill be used."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connect_mode` after provisioning.\nThe connection mode of the Redis instance. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn connect_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_location_id` after provisioning.\nThe current zone where the Redis endpoint is placed.\nFor Basic Tier instances, this will always be the same as the\n[locationId] provided by the user at creation time. For Standard Tier\ninstances, this can be either [locationId] or [alternativeLocationId]\nand can change after a failover event."]
    pub fn current_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_key` after provisioning.\nOptional. The KMS key reference that you want to use to encrypt the data at rest for this Redis\ninstance. If this is provided, CMEK is enabled."]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary and optional user-provided name for the instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHostname or IP address of the exposed Redis endpoint used by clients\nto connect to the service."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_id` after provisioning.\nThe zone where the instance will be provisioned. If not provided,\nthe service will choose a zone for the instance. For STANDARD_HA tier,\ninstances will be created across two zones for protection against\nzonal failures. If [alternativeLocationId] is also provided, it must\nbe different from [locationId]."]
    pub fn location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataRedisInstanceMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<DataRedisInstanceMaintenanceScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nRedis memory size in GiB."]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nodes` after provisioning.\nOutput only. Info per node."]
    pub fn nodes(&self) -> terrars::ListRef<DataRedisInstanceNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence configuration for an instance."]
    pub fn persistence_config(&self) -> terrars::ListRef<DataRedisInstancePersistenceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistence_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence_iam_identity` after provisioning.\nOutput only. Cloud IAM identity used by import / export operations\nto transfer data to/from Cloud Storage. Format is \"serviceAccount:\".\nThe value may change over time for a given instance so should be\nchecked before each import/export operation."]
    pub fn persistence_iam_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistence_iam_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number of the exposed Redis endpoint."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_endpoint` after provisioning.\nOutput only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only.\nTargets all healthy replica nodes in instance. Replication is asynchronous and replica nodes\nwill exhibit some lag behind the primary. Write requests must target 'host'."]
    pub fn read_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_endpoint_port` after provisioning.\nOutput only. The port number of the exposed readonly redis endpoint. Standard tier only.\nWrite requests should target 'port'."]
    pub fn read_endpoint_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_endpoint_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replicas_mode` after provisioning.\nOptional. Read replica mode. Can only be specified when trying to create the instance.\nIf not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.\n- READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the\ninstance cannot scale up or down the number of replicas.\n- READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance\ncan scale up and down the number of replicas. Possible values: [\"READ_REPLICAS_DISABLED\", \"READ_REPLICAS_ENABLED\"]"]
    pub fn read_replicas_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_replicas_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_configs` after provisioning.\nRedis configuration parameters, according to http://redis.io/topics/config.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs"]
    pub fn redis_configs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.redis_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redis_version` after provisioning.\nThe version of Redis software. If not provided, latest supported\nversion will be used. Please check the API documentation linked\nat the top for the latest valid values."]
    pub fn redis_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Redis region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes. The valid range for the Standard Tier with\nread replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled\nfor a Standard Tier instance, the only valid value is 1 and the default is 1.\nThe valid value for basic tier is 0 and the default is also 0."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secondary_ip_range` after provisioning.\nOptional. Additional IP range for node placement. Required when enabling read replicas on\nan existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or\n\"auto\". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address\nrange associated with the private service access connection, or \"auto\"."]
    pub fn secondary_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_ca_certs` after provisioning.\nList of server CA certificates for the instance."]
    pub fn server_ca_certs(&self) -> terrars::ListRef<DataRedisInstanceServerCaCertsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_ca_certs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance. Must be one of these values:\n\n- BASIC: standalone instance\n- STANDARD_HA: highly available primary/replica instances Default value: \"BASIC\" Possible values: [\"BASIC\", \"STANDARD_HA\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nThe TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.\n\n- SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication Default value: \"DISABLED\" Possible values: [\"SERVER_AUTHENTICATION\", \"DISABLED\"]"]
    pub fn transit_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
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

impl terrars::ToListMappable for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O = terrars::BlockAssignable<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}

impl BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
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
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}

impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
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
                        terrars::ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = terrars::BlockAssignable<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {}

impl BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
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
    ) -> terrars::ListRef<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}

impl DataRedisInstanceMaintenancePolicyEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
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
                    ::Into<terrars::ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        self.weekly_maintenance_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisInstanceMaintenancePolicyEl {
    type O = terrars::BlockAssignable<DataRedisInstanceMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceMaintenancePolicyEl {}

impl BuildDataRedisInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyEl {
        DataRedisInstanceMaintenancePolicyEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceMaintenancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisInstanceMaintenancePolicyElRef {
        DataRedisInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> terrars::ListRef<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_deadline_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisInstanceMaintenanceScheduleEl {
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

impl terrars::ToListMappable for DataRedisInstanceMaintenanceScheduleEl {
    type O = terrars::BlockAssignable<DataRedisInstanceMaintenanceScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceMaintenanceScheduleEl {}

impl BuildDataRedisInstanceMaintenanceScheduleEl {
    pub fn build(self) -> DataRedisInstanceMaintenanceScheduleEl {
        DataRedisInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceMaintenanceScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceMaintenanceScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisInstanceMaintenanceScheduleElRef {
        DataRedisInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceMaintenanceScheduleElRef {
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
pub struct DataRedisInstanceNodesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisInstanceNodesEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisInstanceNodesEl {
    type O = terrars::BlockAssignable<DataRedisInstanceNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceNodesEl {}

impl BuildDataRedisInstanceNodesEl {
    pub fn build(self) -> DataRedisInstanceNodesEl {
        DataRedisInstanceNodesEl {
            id: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceNodesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisInstanceNodesElRef {
        DataRedisInstanceNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceNodesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRedisInstancePersistenceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistence_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_next_snapshot_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdb_snapshot_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisInstancePersistenceConfigEl {
    #[doc = "Set the field `persistence_mode`.\n"]
    pub fn set_persistence_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.persistence_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rdb_next_snapshot_time`.\n"]
    pub fn set_rdb_next_snapshot_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rdb_next_snapshot_time = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataRedisInstancePersistenceConfigEl {
    type O = terrars::BlockAssignable<DataRedisInstancePersistenceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstancePersistenceConfigEl {}

impl BuildDataRedisInstancePersistenceConfigEl {
    pub fn build(self) -> DataRedisInstancePersistenceConfigEl {
        DataRedisInstancePersistenceConfigEl {
            persistence_mode: core::default::Default::default(),
            rdb_next_snapshot_time: core::default::Default::default(),
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstancePersistenceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstancePersistenceConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisInstancePersistenceConfigElRef {
        DataRedisInstancePersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstancePersistenceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `persistence_mode` after provisioning.\n"]
    pub fn persistence_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistence_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `rdb_next_snapshot_time` after provisioning.\n"]
    pub fn rdb_next_snapshot_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdb_next_snapshot_time", self.base))
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
pub struct DataRedisInstanceServerCaCertsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serial_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha1_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedisInstanceServerCaCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serial_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha1_fingerprint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedisInstanceServerCaCertsEl {
    type O = terrars::BlockAssignable<DataRedisInstanceServerCaCertsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedisInstanceServerCaCertsEl {}

impl BuildDataRedisInstanceServerCaCertsEl {
    pub fn build(self) -> DataRedisInstanceServerCaCertsEl {
        DataRedisInstanceServerCaCertsEl {
            cert: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}

pub struct DataRedisInstanceServerCaCertsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedisInstanceServerCaCertsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedisInstanceServerCaCertsElRef {
        DataRedisInstanceServerCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedisInstanceServerCaCertsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.base))
    }
}
