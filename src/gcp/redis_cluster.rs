use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct RedisClusterData {
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
    authorization_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
    shard_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_configs: std::option::Option<std::vec::Vec<RedisClusterPscConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RedisClusterTimeoutsEl>,
    dynamic: RedisClusterDynamic,
}

struct RedisCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedisClusterData>,
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

    #[doc =
        "Set the field `authorization_mode`.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn set_authorization_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorization_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `replica_count`.\nOptional. The number of replica nodes per shard."]
    pub fn set_replica_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `psc_configs`.\n"]
    pub fn set_psc_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedisClusterPscConfigsEl>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<RedisClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<RedisClusterDiscoveryEndpointsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<RedisClusterPscConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
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
    pub fn state_info(&self) -> terrars::ListRef<RedisClusterStateInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> terrars::ListRef<RedisClusterPscConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(RedisClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorization_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                shard_count: self.shard_count,
                transit_encryption_mode: core::default::Default::default(),
                psc_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
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
        "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> terrars::ListRef<RedisClusterDiscoveryEndpointsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> terrars::ListRef<RedisClusterPscConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
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
    pub fn state_info(&self) -> terrars::ListRef<RedisClusterStateInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> terrars::ListRef<RedisClusterPscConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisClusterDiscoveryEndpointsElPscConfigEl {
    type O = terrars::BlockAssignable<RedisClusterDiscoveryEndpointsElPscConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterDiscoveryEndpointsElPscConfigEl {}

impl BuildRedisClusterDiscoveryEndpointsElPscConfigEl {
    pub fn build(self) -> RedisClusterDiscoveryEndpointsElPscConfigEl {
        RedisClusterDiscoveryEndpointsElPscConfigEl { network: core::default::Default::default() }
    }
}

pub struct RedisClusterDiscoveryEndpointsElPscConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedisClusterDiscoveryEndpointsElPscConfigElRef {
        RedisClusterDiscoveryEndpointsElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<terrars::ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>,
}

impl RedisClusterDiscoveryEndpointsEl {
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
        v: impl std::convert::Into<terrars::ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>,
    ) -> Self {
        self.psc_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisClusterDiscoveryEndpointsEl {
    type O = terrars::BlockAssignable<RedisClusterDiscoveryEndpointsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterDiscoveryEndpointsEl {}

impl BuildRedisClusterDiscoveryEndpointsEl {
    pub fn build(self) -> RedisClusterDiscoveryEndpointsEl {
        RedisClusterDiscoveryEndpointsEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            psc_config: core::default::Default::default(),
        }
    }
}

pub struct RedisClusterDiscoveryEndpointsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterDiscoveryEndpointsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterDiscoveryEndpointsElRef {
        RedisClusterDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterDiscoveryEndpointsElRef {
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
    pub fn psc_config(&self) -> terrars::ListRef<RedisClusterDiscoveryEndpointsElPscConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterPscConnectionsEl {
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

impl RedisClusterPscConnectionsEl {
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

impl terrars::ToListMappable for RedisClusterPscConnectionsEl {
    type O = terrars::BlockAssignable<RedisClusterPscConnectionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterPscConnectionsEl {}

impl BuildRedisClusterPscConnectionsEl {
    pub fn build(self) -> RedisClusterPscConnectionsEl {
        RedisClusterPscConnectionsEl {
            address: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
        }
    }
}

pub struct RedisClusterPscConnectionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterPscConnectionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterPscConnectionsElRef {
        RedisClusterPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterPscConnectionsElRef {
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
pub struct RedisClusterStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_shard_count: std::option::Option<terrars::PrimField<f64>>,
}

impl RedisClusterStateInfoElUpdateInfoEl {
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

impl terrars::ToListMappable for RedisClusterStateInfoElUpdateInfoEl {
    type O = terrars::BlockAssignable<RedisClusterStateInfoElUpdateInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterStateInfoElUpdateInfoEl {}

impl BuildRedisClusterStateInfoElUpdateInfoEl {
    pub fn build(self) -> RedisClusterStateInfoElUpdateInfoEl {
        RedisClusterStateInfoElUpdateInfoEl {
            target_replica_count: core::default::Default::default(),
            target_shard_count: core::default::Default::default(),
        }
    }
}

pub struct RedisClusterStateInfoElUpdateInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterStateInfoElUpdateInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterStateInfoElUpdateInfoElRef {
        RedisClusterStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterStateInfoElUpdateInfoElRef {
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
pub struct RedisClusterStateInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_info: std::option::Option<terrars::ListField<RedisClusterStateInfoElUpdateInfoEl>>,
}

impl RedisClusterStateInfoEl {
    #[doc = "Set the field `update_info`.\n"]
    pub fn set_update_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedisClusterStateInfoElUpdateInfoEl>>,
    ) -> Self {
        self.update_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedisClusterStateInfoEl {
    type O = terrars::BlockAssignable<RedisClusterStateInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterStateInfoEl {}

impl BuildRedisClusterStateInfoEl {
    pub fn build(self) -> RedisClusterStateInfoEl {
        RedisClusterStateInfoEl { update_info: core::default::Default::default() }
    }
}

pub struct RedisClusterStateInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterStateInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterStateInfoElRef {
        RedisClusterStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterStateInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> terrars::ListRef<RedisClusterStateInfoElUpdateInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterPscConfigsEl {
    network: terrars::PrimField<std::string::String>,
}

impl RedisClusterPscConfigsEl { }

impl terrars::ToListMappable for RedisClusterPscConfigsEl {
    type O = terrars::BlockAssignable<RedisClusterPscConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterPscConfigsEl {
    #[doc =
        "Required. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildRedisClusterPscConfigsEl {
    pub fn build(self) -> RedisClusterPscConfigsEl {
        RedisClusterPscConfigsEl { network: self.network }
    }
}

pub struct RedisClusterPscConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterPscConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterPscConfigsElRef {
        RedisClusterPscConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterPscConfigsElRef {
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
pub struct RedisClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedisClusterTimeoutsEl {
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

impl terrars::ToListMappable for RedisClusterTimeoutsEl {
    type O = terrars::BlockAssignable<RedisClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterTimeoutsEl {}

impl BuildRedisClusterTimeoutsEl {
    pub fn build(self) -> RedisClusterTimeoutsEl {
        RedisClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RedisClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedisClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterTimeoutsElRef {
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
struct RedisClusterDynamic {
    psc_configs: std::option::Option<terrars::DynamicBlock<RedisClusterPscConfigsEl>>,
}
