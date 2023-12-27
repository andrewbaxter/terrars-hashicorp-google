use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerNodePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<Vec<ContainerNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<Vec<ContainerNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<ContainerNodePoolNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<ContainerNodePoolNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_policy: Option<Vec<ContainerNodePoolPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerNodePoolTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_settings: Option<Vec<ContainerNodePoolUpgradeSettingsEl>>,
    dynamic: ContainerNodePoolDynamic,
}

struct ContainerNodePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerNodePoolData>,
}

#[derive(Clone)]
pub struct ContainerNodePool(Rc<ContainerNodePool_>);

impl ContainerNodePool {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `initial_node_count`.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn set_initial_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe location (region or zone) of the cluster."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `max_pods_per_node`.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn set_max_pods_per_node(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_pods_per_node = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `node_count`.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn set_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_locations`.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn set_node_locations(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().node_locations = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc= "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(self, v: impl Into<BlockAssignable<ContainerNodePoolAutoscalingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `management`.\n"]
    pub fn set_management(self, v: impl Into<BlockAssignable<ContainerNodePoolManagementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_config`.\n"]
    pub fn set_network_config(self, v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `node_config`.\n"]
    pub fn set_node_config(self, v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(self, v: impl Into<BlockAssignable<ContainerNodePoolPlacementPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(self, v: impl Into<BlockAssignable<ContainerNodePoolUpgradeSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerNodePoolAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerNodePoolManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<ContainerNodePoolNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<ContainerNodePoolPlacementPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.extract_ref()))
    }
}

impl Referable for ContainerNodePool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ContainerNodePool { }

impl ToListMappable for ContainerNodePool {
    type O = ListRef<ContainerNodePoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ContainerNodePool_ {
    fn extract_resource_type(&self) -> String {
        "google_container_node_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerNodePool {
    pub tf_id: String,
    #[doc= "The cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub cluster: PrimField<String>,
}

impl BuildContainerNodePool {
    pub fn build(self, stack: &mut Stack) -> ContainerNodePool {
        let out = ContainerNodePool(Rc::new(ContainerNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerNodePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: self.cluster,
                id: core::default::Default::default(),
                initial_node_count: core::default::Default::default(),
                location: core::default::Default::default(),
                max_pods_per_node: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                node_count: core::default::Default::default(),
                node_locations: core::default::Default::default(),
                project: core::default::Default::default(),
                version: core::default::Default::default(),
                autoscaling: core::default::Default::default(),
                management: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                placement_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerNodePoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerNodePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<ContainerNodePoolAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<ContainerNodePoolManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<ContainerNodePoolNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<ContainerNodePoolPlacementPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_min_node_count: Option<PrimField<f64>>,
}

impl ContainerNodePoolAutoscalingEl {
    #[doc= "Set the field `location_policy`.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn set_location_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `max_node_count`.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `min_node_count`.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `total_max_node_count`.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn set_total_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_max_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `total_min_node_count`.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn set_total_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_min_node_count = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolAutoscalingEl {
    type O = BlockAssignable<ContainerNodePoolAutoscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolAutoscalingEl {}

impl BuildContainerNodePoolAutoscalingEl {
    pub fn build(self) -> ContainerNodePoolAutoscalingEl {
        ContainerNodePoolAutoscalingEl {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolAutoscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolAutoscalingElRef {
        ContainerNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `location_policy` after provisioning.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn location_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `total_max_node_count` after provisioning.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn total_max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_max_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `total_min_node_count` after provisioning.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn total_min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
}

impl ContainerNodePoolManagementEl {
    #[doc= "Set the field `auto_repair`.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_upgrade`.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolManagementEl {
    type O = BlockAssignable<ContainerNodePoolManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolManagementEl {}

impl BuildContainerNodePoolManagementEl {
    pub fn build(self) -> ContainerNodePoolManagementEl {
        ContainerNodePoolManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolManagementElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolManagementElRef {
        ContainerNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_repair` after provisioning.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_upgrade` after provisioning.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: PrimField<String>,
}

impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl { }

impl ToListMappable for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    #[doc= "Specifies the total network bandwidth tier for the NodePool."]
    pub total_egress_bandwidth_tier: PrimField<String>,
}

impl BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nSpecifies the total network bandwidth tier for the NodePool."]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    disabled: PrimField<bool>,
}

impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl { }

impl ToListMappable for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc= ""]
    pub disabled: PrimField<bool>,
}

impl BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl { disabled: self.disabled }
    }
}

pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNetworkConfigElDynamic {
    network_performance_config: Option<DynamicBlock<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    pod_cidr_overprovision_config: Option<
        DynamicBlock<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_pod_range: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config: Option<Vec<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config: Option<Vec<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    dynamic: ContainerNodePoolNetworkConfigElDynamic,
}

impl ContainerNodePoolNetworkConfigEl {
    #[doc= "Set the field `create_pod_range`.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn set_create_pod_range(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_pod_range = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_private_nodes`.\nWhether nodes have internal IP addresses only."]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_ipv4_cidr_block`.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn set_pod_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_ipv4_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_range`.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn set_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_range = Some(v.into());
        self
    }

    #[doc= "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_performance_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_performance_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pod_cidr_overprovision_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_cidr_overprovision_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerNodePoolNetworkConfigEl {
    type O = BlockAssignable<ContainerNodePoolNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNetworkConfigEl {}

impl BuildContainerNodePoolNetworkConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigEl {
        ContainerNodePoolNetworkConfigEl {
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerNodePoolNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNetworkConfigElRef {
        ContainerNodePoolNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_pod_range` after provisioning.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn create_pod_range(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_pod_range", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_private_nodes` after provisioning.\nWhether nodes have internal IP addresses only."]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn pod_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_ipv4_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_range` after provisioning.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> ListRef<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[doc= "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElEffectiveTaintsEl {}

impl BuildContainerNodePoolNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElEffectiveTaintsEl {
        ContainerNodePoolNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
        ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_version: Option<PrimField<String>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc= "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_driver_version = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}

impl BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_shared_clients_per_gpu: Option<PrimField<f64>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[doc= "Set the field `gpu_sharing_strategy`.\n"]
    pub fn set_gpu_sharing_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_sharing_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `max_shared_clients_per_gpu`.\n"]
    pub fn set_max_shared_clients_per_gpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_shared_clients_per_gpu = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}

impl BuildContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\n"]
    pub fn gpu_sharing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_sharing_strategy", self.base))
    }

    #[doc= "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\n"]
    pub fn max_shared_clients_per_gpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_shared_clients_per_gpu", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_installation_config: Option<
        ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_partition_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_config: Option<ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v: impl Into<ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>>,
    ) -> Self {
        self.gpu_driver_installation_config = Some(v.into());
        self
    }

    #[doc= "Set the field `gpu_partition_size`.\n"]
    pub fn set_gpu_partition_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_partition_size = Some(v.into());
        self
    }

    #[doc= "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v: impl Into<ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    ) -> Self {
        self.gpu_sharing_config = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGuestAcceleratorEl {}

impl BuildContainerNodePoolNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGuestAcceleratorEl {
        ContainerNodePoolNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]
    pub fn gpu_driver_installation_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(&self) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    threads_per_core: PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    #[doc= "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl { threads_per_core: self.threads_per_core }
    }
}

pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElConfidentialNodesEl {
    enabled: PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElConfidentialNodesEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    #[doc= "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElConfidentialNodesEl {
        ContainerNodePoolNodeConfigElConfidentialNodesEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElConfidentialNodesElRef {
        ContainerNodePoolNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    local_ssd_count: PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc= "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElFastSocketEl {
    enabled: PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElFastSocketEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElFastSocketEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElFastSocketEl {
    #[doc= "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElFastSocketEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElFastSocketEl {
        ContainerNodePoolNodeConfigElFastSocketEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElFastSocketElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElFastSocketElRef {
        ContainerNodePoolNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether or not NCCL Fast Socket is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGcfsConfigEl {
    enabled: PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElGcfsConfigEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    #[doc= "Whether or not GCFS is enabled"]
    pub enabled: PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGcfsConfigEl {
        ContainerNodePoolNodeConfigElGcfsConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElGcfsConfigElRef {
        ContainerNodePoolNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGvnicEl {
    enabled: PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElGvnicEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElGvnicEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGvnicEl {
    #[doc= "Whether or not gvnic is enabled"]
    pub enabled: PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElGvnicEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGvnicEl {
        ContainerNodePoolNodeConfigElGvnicEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElGvnicElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElGvnicElRef {
        ContainerNodePoolNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    maintenance_interval: PrimField<String>,
}

impl ContainerNodePoolNodeConfigElHostMaintenancePolicyEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    #[doc= "."]
    pub maintenance_interval: PrimField<String>,
}

impl BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyEl { maintenance_interval: self.maintenance_interval }
    }
}

pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maintenance_interval` after provisioning.\n."]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    cpu_manager_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
}

impl ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[doc= "Set the field `cpu_cfs_quota`.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_cfs_quota_period`.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_pids_limit`.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElKubeletConfigEl {
    #[doc= "Control the CPU management policy on the node."]
    pub cpu_manager_policy: PrimField<String>,
}

impl BuildContainerNodePoolNodeConfigElKubeletConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElKubeletConfigEl {
        ContainerNodePoolNodeConfigElKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: self.cpu_manager_policy,
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElKubeletConfigElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_manager_policy` after provisioning.\nControl the CPU management policy on the node."]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_pids_limit` after provisioning.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
}

impl ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[doc= "Set the field `cgroup_mode`.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `sysctls`.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElLinuxNodeConfigEl {}

impl BuildContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
        ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cgroup_mode` after provisioning.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `sysctls` after provisioning.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    local_ssd_count: PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc= "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElReservationAffinityEl {
    consume_reservation_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl ContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc= "Set the field `key`.\nThe label key of a reservation resource."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\nThe label values of the reservation resource."]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc= "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: PrimField<String>,
}

impl BuildContainerNodePoolNodeConfigElReservationAffinityEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElReservationAffinityEl {
        ContainerNodePoolNodeConfigElReservationAffinityEl {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElReservationAffinityElRef {
        ContainerNodePoolNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consume_reservation_type` after provisioning.\nCorresponds to the type of reservation consumption."]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe label key of a reservation resource."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\nThe label values of the reservation resource."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}

impl ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElShieldedInstanceConfigEl {}

impl BuildContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
        ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
        ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    key: PrimField<String>,
    operator: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc= "."]
    pub key: PrimField<String>,
    #[doc= "."]
    pub operator: PrimField<String>,
    #[doc= "."]
    pub values: ListField<PrimField<String>>,
}

impl BuildContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
        ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic {
    node_affinity: Option<DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity: Option<Vec<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    dynamic: ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic,
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[doc= "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_affinity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinity = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElSoleTenantConfigEl {}

impl BuildContainerNodePoolNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElSoleTenantConfigEl {
        ContainerNodePoolNodeConfigElSoleTenantConfigEl {
            node_affinity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElTaintEl {
    effect: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl ContainerNodePoolNodeConfigElTaintEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElTaintEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElTaintEl {
    #[doc= "Effect for taint."]
    pub effect: PrimField<String>,
    #[doc= "Key for taint."]
    pub key: PrimField<String>,
    #[doc= "Value for taint."]
    pub value: PrimField<String>,
}

impl BuildContainerNodePoolNodeConfigElTaintEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElTaintEl {
        ContainerNodePoolNodeConfigElTaintEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct ContainerNodePoolNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElTaintElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElTaintElRef {
        ContainerNodePoolNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `effect` after provisioning.\nEffect for taint."]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nKey for taint."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue for taint."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    mode: PrimField<String>,
}

impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl { }

impl ToListMappable for ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    #[doc= "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: PrimField<String>,
}

impl BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl { mode: self.mode }
    }
}

pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nMode is the configuration for how to expose metadata to workloads running on the node."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElDynamic {
    advanced_machine_features: Option<DynamicBlock<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    confidential_nodes: Option<DynamicBlock<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    ephemeral_storage_local_ssd_config: Option<
        DynamicBlock<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>,
    >,
    fast_socket: Option<DynamicBlock<ContainerNodePoolNodeConfigElFastSocketEl>>,
    gcfs_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    gvnic: Option<DynamicBlock<ContainerNodePoolNodeConfigElGvnicEl>>,
    host_maintenance_policy: Option<DynamicBlock<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    kubelet_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    linux_node_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    local_nvme_ssd_block_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    reservation_affinity: Option<DynamicBlock<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    shielded_instance_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    sole_tenant_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    taint: Option<DynamicBlock<ContainerNodePoolNodeConfigElTaintEl>>,
    workload_metadata_config: Option<DynamicBlock<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<ListField<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<Vec<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes: Option<Vec<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config: Option<Vec<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<Vec<ContainerNodePoolNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<Vec<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<Vec<ContainerNodePoolNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy: Option<Vec<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<Vec<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config: Option<Vec<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config: Option<Vec<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<Vec<ContainerNodePoolNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config: Option<Vec<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    dynamic: ContainerNodePoolNodeConfigElDynamic,
}

impl ContainerNodePoolNodeConfigEl {
    #[doc= "Set the field `boot_disk_kms_key`.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size_gb`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `guest_accelerator`.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<ListField<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = Some(v.into());
        self
    }

    #[doc= "Set the field `image_type`.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `local_ssd_count`.\nThe number of local SSD disks to be attached to the node."]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }

    #[doc= "Set the field `logging_variant`.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }

    #[doc= "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata`.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `node_group`.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }

    #[doc= "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }

    #[doc= "Set the field `preemptible`.\nWhether the nodes are created as preemptible VM instances."]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_labels`.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }

    #[doc= "Set the field `spot`.\nWhether the nodes are created as spot VM instances."]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\nThe list of instance tags applied to all nodes."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_machine_features = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_nodes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_nodes = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ephemeral_storage_local_ssd_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_storage_local_ssd_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(mut self, v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fast_socket = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_socket = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(mut self, v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcfs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcfs_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gvnic`.\n"]
    pub fn set_gvnic(mut self, v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gvnic = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gvnic = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_maintenance_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_maintenance_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubelet_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubelet_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linux_node_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_node_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_nvme_ssd_block_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_nvme_ssd_block_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation_affinity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sole_tenant_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sole_tenant_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `taint`.\n"]
    pub fn set_taint(mut self, v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.taint = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.taint = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workload_metadata_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workload_metadata_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerNodePoolNodeConfigEl {
    type O = BlockAssignable<ContainerNodePoolNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigEl {}

impl BuildContainerNodePoolNodeConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigEl {
        ContainerNodePoolNodeConfigEl {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            image_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            service_account: core::default::Default::default(),
            spot: core::default::Default::default(),
            tags: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            taint: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolNodeConfigElRef {
        ContainerNodePoolNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `effective_taints` after provisioning.\nList of kubernetes taints applied to each node."]
    pub fn effective_taints(&self) -> ListRef<ContainerNodePoolNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc= "Get a reference to the value of field `image_type` after provisioning.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\nThe number of local SSD disks to be attached to the node."]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc= "Get a reference to the value of field `logging_variant` after provisioning.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc= "Get a reference to the value of field `node_group` after provisioning.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }

    #[doc= "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\nWhether the nodes are created as preemptible VM instances."]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc= "Get a reference to the value of field `spot` after provisioning.\nWhether the nodes are created as spot VM instances."]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all nodes."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> ListRef<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc= "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> ListRef<ContainerNodePoolNodeConfigElConfidentialNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc= "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> ListRef<ContainerNodePoolNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc= "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> ListRef<ContainerNodePoolNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<ContainerNodePoolNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc= "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(&self) -> ListRef<ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<ContainerNodePoolNodeConfigElKubeletConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc= "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc= "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(&self) -> ListRef<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<ContainerNodePoolNodeConfigElReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> ListRef<ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> ListRef<ContainerNodePoolNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc= "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<ContainerNodePoolNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc= "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> ListRef<ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tpu_topology: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ContainerNodePoolPlacementPolicyEl {
    #[doc= "Set the field `policy_name`.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn set_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_name = Some(v.into());
        self
    }

    #[doc= "Set the field `tpu_topology`.\nTPU placement topology for pod slice node pool. https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies"]
    pub fn set_tpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tpu_topology = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolPlacementPolicyEl {
    type O = BlockAssignable<ContainerNodePoolPlacementPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolPlacementPolicyEl {
    #[doc= "Type defines the type of placement policy"]
    pub type_: PrimField<String>,
}

impl BuildContainerNodePoolPlacementPolicyEl {
    pub fn build(self) -> ContainerNodePoolPlacementPolicyEl {
        ContainerNodePoolPlacementPolicyEl {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct ContainerNodePoolPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolPlacementPolicyElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolPlacementPolicyElRef {
        ContainerNodePoolPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `policy_name` after provisioning.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }

    #[doc= "Get a reference to the value of field `tpu_topology` after provisioning.\nTPU placement topology for pod slice node pool. https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies"]
    pub fn tpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType defines the type of placement policy"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ContainerNodePoolTimeoutsEl {
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

impl ToListMappable for ContainerNodePoolTimeoutsEl {
    type O = BlockAssignable<ContainerNodePoolTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolTimeoutsEl {}

impl BuildContainerNodePoolTimeoutsEl {
    pub fn build(self) -> ContainerNodePoolTimeoutsEl {
        ContainerNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolTimeoutsElRef {
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

#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc= "Set the field `batch_node_count`.\nNumber of blue nodes to drain in a batch."]
    pub fn set_batch_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_percentage`.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn set_batch_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_soak_duration`.\nSoak time after each batch gets drained."]
    pub fn set_batch_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.batch_soak_duration = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O = BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}

impl BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(self) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `batch_node_count` after provisioning.\nNumber of blue nodes to drain in a batch."]
    pub fn batch_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_percentage` after provisioning.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn batch_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_soak_duration` after provisioning.\nSoak time after each batch gets drained."]
    pub fn batch_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic {
    standard_rollout_policy: Option<
        DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>,
    >,
}

#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_soak_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_rollout_policy: Option<Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic,
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[doc= "Set the field `node_pool_soak_duration`.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn set_node_pool_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_pool_soak_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard_rollout_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_rollout_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    type O = BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {}

impl BuildContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(self) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn node_pool_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElDynamic {
    blue_green_settings: Option<DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_settings: Option<Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElDynamic,
}

impl ContainerNodePoolUpgradeSettingsEl {
    #[doc= "Set the field `max_surge`.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }

    #[doc= "Set the field `max_unavailable`.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }

    #[doc= "Set the field `strategy`.\nUpdate strategy for the given nodepool."]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v: impl Into<BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.blue_green_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.blue_green_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerNodePoolUpgradeSettingsEl {
    type O = BlockAssignable<ContainerNodePoolUpgradeSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolUpgradeSettingsEl {}

impl BuildContainerNodePoolUpgradeSettingsEl {
    pub fn build(self) -> ContainerNodePoolUpgradeSettingsEl {
        ContainerNodePoolUpgradeSettingsEl {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerNodePoolUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNodePoolUpgradeSettingsElRef {
    fn new(shared: StackShared, base: String) -> ContainerNodePoolUpgradeSettingsElRef {
        ContainerNodePoolUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `max_surge` after provisioning.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc= "Get a reference to the value of field `max_unavailable` after provisioning.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc= "Get a reference to the value of field `strategy` after provisioning.\nUpdate strategy for the given nodepool."]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc= "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(&self) -> ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolDynamic {
    autoscaling: Option<DynamicBlock<ContainerNodePoolAutoscalingEl>>,
    management: Option<DynamicBlock<ContainerNodePoolManagementEl>>,
    network_config: Option<DynamicBlock<ContainerNodePoolNetworkConfigEl>>,
    node_config: Option<DynamicBlock<ContainerNodePoolNodeConfigEl>>,
    placement_policy: Option<DynamicBlock<ContainerNodePoolPlacementPolicyEl>>,
    upgrade_settings: Option<DynamicBlock<ContainerNodePoolUpgradeSettingsEl>>,
}
