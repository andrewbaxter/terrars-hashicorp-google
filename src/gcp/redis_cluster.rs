use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct RedisClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    shard_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_configs: Option<Vec<RedisClusterPscConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RedisClusterTimeoutsEl>,
    dynamic: RedisClusterDynamic,
}

struct RedisCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedisClusterData>,
}

#[derive(Clone)]
pub struct RedisCluster(Rc<RedisCluster_>);

impl RedisCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `authorization_mode`.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn set_authorization_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe name of the region of the Redis cluster."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `replica_count`.\nOptional. The number of replica nodes per shard."]
    pub fn set_replica_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = Some(v.into());
        self
    }

    #[doc= "Set the field `transit_encryption_mode`.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn set_transit_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `psc_configs`.\n"]
    pub fn set_psc_configs(self, v: impl Into<BlockAssignable<RedisClusterPscConfigsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_configs = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RedisClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<RedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<RedisClusterPscConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<RedisClusterStateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> ListRef<RedisClusterPscConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for RedisCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RedisCluster { }

impl ToListMappable for RedisCluster {
    type O = ListRef<RedisClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedisCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_redis_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedisCluster {
    pub tf_id: String,
    #[doc= "Required. Number of shards for the Redis cluster."]
    pub shard_count: PrimField<f64>,
}

impl BuildRedisCluster {
    pub fn build(self, stack: &mut Stack) -> RedisCluster {
        let out = RedisCluster(Rc::new(RedisCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedisClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedisClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedisClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `authorization_mode` after provisioning.\nOptional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster. Default value: \"AUTH_MODE_DISABLED\" Possible values: [\"AUTH_MODE_UNSPECIFIED\", \"AUTH_MODE_IAM_AUTH\", \"AUTH_MODE_DISABLED\"]"]
    pub fn authorization_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp associated with the cluster creation request. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discovery_endpoints` after provisioning.\nOutput only. Endpoints created on each given network,\nfor Redis clients to connect to the cluster.\nCurrently only one endpoint is supported."]
    pub fn discovery_endpoints(&self) -> ListRef<RedisClusterDiscoveryEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.discovery_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUnique name of the resource in this scope including project and location using the form:\nprojects/{projectId}/locations/{locationId}/clusters/{clusterId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_connections` after provisioning.\nOutput only. PSC connections for discovery of the cluster topology and accessing the cluster."]
    pub fn psc_connections(&self) -> ListRef<RedisClusterPscConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Redis cluster."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes per shard."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shard_count` after provisioning.\nRequired. Number of shards for the Redis cluster."]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_gb` after provisioning.\nOutput only. Redis memory size in GB for the entire cluster."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_info` after provisioning.\nOutput only. Additional information about the current state of the cluster."]
    pub fn state_info(&self) -> ListRef<RedisClusterStateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nOptional. The in-transit encryption for the Redis cluster.\nIf not provided, encryption is disabled for the cluster. Default value: \"TRANSIT_ENCRYPTION_MODE_DISABLED\" Possible values: [\"TRANSIT_ENCRYPTION_MODE_UNSPECIFIED\", \"TRANSIT_ENCRYPTION_MODE_DISABLED\", \"TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nSystem assigned, unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `psc_configs` after provisioning.\n"]
    pub fn psc_configs(&self) -> ListRef<RedisClusterPscConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_configs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}

impl RedisClusterDiscoveryEndpointsElPscConfigEl {
    #[doc= "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterDiscoveryEndpointsElPscConfigEl {
    type O = BlockAssignable<RedisClusterDiscoveryEndpointsElPscConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterDiscoveryEndpointsElPscConfigElRef {
        RedisClusterDiscoveryEndpointsElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterDiscoveryEndpointsElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterDiscoveryEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>,
}

impl RedisClusterDiscoveryEndpointsEl {
    #[doc= "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `psc_config`.\n"]
    pub fn set_psc_config(mut self, v: impl Into<ListField<RedisClusterDiscoveryEndpointsElPscConfigEl>>) -> Self {
        self.psc_config = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterDiscoveryEndpointsEl {
    type O = BlockAssignable<RedisClusterDiscoveryEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterDiscoveryEndpointsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterDiscoveryEndpointsElRef {
        RedisClusterDiscoveryEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterDiscoveryEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<RedisClusterDiscoveryEndpointsElPscConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterPscConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
}

impl RedisClusterPscConnectionsEl {
    #[doc= "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc= "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterPscConnectionsEl {
    type O = BlockAssignable<RedisClusterPscConnectionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterPscConnectionsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPscConnectionsElRef {
        RedisClusterPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterPscConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc= "Get a reference to the value of field `forwarding_rule` after provisioning.\n"]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterStateInfoElUpdateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_shard_count: Option<PrimField<f64>>,
}

impl RedisClusterStateInfoElUpdateInfoEl {
    #[doc= "Set the field `target_replica_count`.\n"]
    pub fn set_target_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_replica_count = Some(v.into());
        self
    }

    #[doc= "Set the field `target_shard_count`.\n"]
    pub fn set_target_shard_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_shard_count = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterStateInfoElUpdateInfoEl {
    type O = BlockAssignable<RedisClusterStateInfoElUpdateInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterStateInfoElUpdateInfoElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterStateInfoElUpdateInfoElRef {
        RedisClusterStateInfoElUpdateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterStateInfoElUpdateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target_replica_count` after provisioning.\n"]
    pub fn target_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_replica_count", self.base))
    }

    #[doc= "Get a reference to the value of field `target_shard_count` after provisioning.\n"]
    pub fn target_shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_shard_count", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterStateInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update_info: Option<ListField<RedisClusterStateInfoElUpdateInfoEl>>,
}

impl RedisClusterStateInfoEl {
    #[doc= "Set the field `update_info`.\n"]
    pub fn set_update_info(mut self, v: impl Into<ListField<RedisClusterStateInfoElUpdateInfoEl>>) -> Self {
        self.update_info = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterStateInfoEl {
    type O = BlockAssignable<RedisClusterStateInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterStateInfoElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterStateInfoElRef {
        RedisClusterStateInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterStateInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `update_info` after provisioning.\n"]
    pub fn update_info(&self) -> ListRef<RedisClusterStateInfoElUpdateInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_info", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterPscConfigsEl {
    network: PrimField<String>,
}

impl RedisClusterPscConfigsEl { }

impl ToListMappable for RedisClusterPscConfigsEl {
    type O = BlockAssignable<RedisClusterPscConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedisClusterPscConfigsEl {
    #[doc= "Required. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub network: PrimField<String>,
}

impl BuildRedisClusterPscConfigsEl {
    pub fn build(self) -> RedisClusterPscConfigsEl {
        RedisClusterPscConfigsEl { network: self.network }
    }
}

pub struct RedisClusterPscConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterPscConfigsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterPscConfigsElRef {
        RedisClusterPscConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterPscConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nRequired. The consumer network where the network address of\nthe discovery endpoint will be reserved, in the form of\nprojects/{network_project_id_or_number}/global/networks/{network_id}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct RedisClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RedisClusterTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RedisClusterTimeoutsEl {
    type O = BlockAssignable<RedisClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RedisClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RedisClusterTimeoutsElRef {
        RedisClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedisClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedisClusterDynamic {
    psc_configs: Option<DynamicBlock<RedisClusterPscConfigsEl>>,
}
