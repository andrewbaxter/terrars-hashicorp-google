use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerNodePoolData {
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
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<ContainerNodePoolAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerNodePoolManagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<ContainerNodePoolNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<std::vec::Vec<ContainerNodePoolPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerNodePoolTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<std::vec::Vec<ContainerNodePoolUpgradeSettingsEl>>,
    dynamic: ContainerNodePoolDynamic,
}

struct ContainerNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerNodePoolData>,
}

#[derive(Clone)]
pub struct ContainerNodePool(std::rc::Rc<ContainerNodePool_>);

impl ContainerNodePool {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_node_count`.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn set_initial_node_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location (region or zone) of the cluster."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_pods_per_node`.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn set_max_pods_per_node(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_count`.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn set_node_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_locations`.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn set_node_locations(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().node_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolManagementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNetworkConfigEl>>,
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

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolPlacementPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolUpgradeSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerNodePoolManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerNodePoolNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerNodePoolPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerNodePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerNodePool { }

impl terrars::ToListMappable for ContainerNodePool {
    type O = terrars::ListRef<ContainerNodePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerNodePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_node_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerNodePool {
    pub tf_id: std::string::String,
    #[doc =
        "The cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub cluster: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerNodePool {
        let out = ContainerNodePool(std::rc::Rc::new(ContainerNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerNodePoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerNodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerNodePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe cluster to create the node pool for. Cluster must be present in location provided for zonal clusters."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) of the cluster."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which to create the node pool. If blank, the provider-configured project will be used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerNodePoolAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerNodePoolManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerNodePoolNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerNodePoolPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerNodePoolUpgradeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolAutoscalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_min_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePoolAutoscalingEl {
    #[doc =
        "Set the field `location_policy`.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn set_location_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_node_count`.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_node_count`.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_max_node_count`.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn set_total_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_min_node_count`.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn set_total_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_min_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerNodePoolAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolAutoscalingElRef {
        ContainerNodePoolAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location_policy` after provisioning.\nLocation policy specifies the algorithm used when scaling-up the node pool. \"BALANCED\" - Is a best effort policy that aims to balance the sizes of available zones. \"ANY\" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs."]
    pub fn location_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nMaximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_max_node_count` after provisioning.\nMaximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits."]
    pub fn total_max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_min_node_count` after provisioning.\nMinimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits."]
    pub fn total_min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolManagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePoolManagementEl {
    #[doc = "Set the field `auto_repair`.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_upgrade`.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn set_auto_upgrade(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolManagementEl {
    type O = terrars::BlockAssignable<ContainerNodePoolManagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolManagementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolManagementElRef {
        ContainerNodePoolManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolManagementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_repair` after provisioning.\nWhether the nodes will be automatically repaired. Enabled by default."]
    pub fn auto_repair(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_upgrade` after provisioning.\nWhether the nodes will be automatically upgraded. Enabled by default."]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    #[doc = "Specifies the total network bandwidth tier for the NodePool."]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

pub struct ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
        ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nSpecifies the total network bandwidth tier for the NodePool."]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl { disabled: self.disabled }
    }
}

pub struct ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
        ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNetworkConfigElDynamic {
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_pod_range: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    dynamic: ContainerNodePoolNetworkConfigElDynamic,
}

impl ContainerNodePoolNetworkConfigEl {
    #[doc =
        "Set the field `create_pod_range`.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn set_create_pod_range(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_nodes`.\nWhether nodes have internal IP addresses only."]
    pub fn set_enable_private_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pod_ipv4_cidr_block`.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn set_pod_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pod_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pod_range`.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn set_pod_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_cidr_overprovision_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_cidr_overprovision_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNetworkConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNetworkConfigElRef {
        ContainerNodePoolNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_pod_range` after provisioning.\nWhether to create a new range for pod IPs in this node pool. Defaults are provided for pod_range and pod_ipv4_cidr_block if they are not specified."]
    pub fn create_pod_range(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_pod_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_nodes` after provisioning.\nWhether nodes have internal IP addresses only."]
    pub fn enable_private_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\nThe IP address range for pod IPs in this node pool. Only applicable if create_pod_range is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use."]
    pub fn pod_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_range` after provisioning.\nThe ID of the secondary range for pod IPs. If create_pod_range is true, this ID is used for the new range. If create_pod_range is false, uses an existing secondary range with this ID."]
    pub fn pod_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNetworkConfigElNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNetworkConfigElPodCidrOverprovisionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElEffectiveTaintsEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
        ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElEffectiveTaintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_driver_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_shared_clients_per_gpu: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[doc = "Set the field `gpu_sharing_strategy`.\n"]
    pub fn set_gpu_sharing_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_sharing_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_shared_clients_per_gpu`.\n"]
    pub fn set_max_shared_clients_per_gpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_shared_clients_per_gpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\n"]
    pub fn gpu_sharing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_sharing_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\n"]
    pub fn max_shared_clients_per_gpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_shared_clients_per_gpu", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<terrars::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std
    ::option
    ::Option<terrars::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
                    >,
    ) -> Self {
        self.gpu_driver_installation_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_partition_size`.\n"]
    pub fn set_gpu_partition_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_partition_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    ) -> Self {
        self.gpu_sharing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElGuestAcceleratorEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
        ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGuestAcceleratorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]
    pub fn gpu_driver_installation_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    threads_per_core: terrars::PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    #[doc =
        "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: terrars::PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl { threads_per_core: self.threads_per_core }
    }
}

pub struct ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
        ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElConfidentialNodesEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElConfidentialNodesEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElConfidentialNodesEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElConfidentialNodesEl {
        ContainerNodePoolNodeConfigElConfidentialNodesEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElConfidentialNodesElRef {
        ContainerNodePoolNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc =
        "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElFastSocketEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElFastSocketEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElFastSocketEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElFastSocketEl {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElFastSocketEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElFastSocketEl {
        ContainerNodePoolNodeConfigElFastSocketEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElFastSocketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElFastSocketElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNodeConfigElFastSocketElRef {
        ContainerNodePoolNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElFastSocketElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not NCCL Fast Socket is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGcfsConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElGcfsConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElGcfsConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElGcfsConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGcfsConfigEl {
        ContainerNodePoolNodeConfigElGcfsConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElGcfsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNodeConfigElGcfsConfigElRef {
        ContainerNodePoolNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElGvnicEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePoolNodeConfigElGvnicEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElGvnicEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElGvnicEl {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePoolNodeConfigElGvnicEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElGvnicEl {
        ContainerNodePoolNodeConfigElGvnicEl { enabled: self.enabled }
    }
}

pub struct ContainerNodePoolNodeConfigElGvnicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElGvnicElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNodeConfigElGvnicElRef {
        ContainerNodePoolNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElGvnicElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    maintenance_interval: terrars::PrimField<std::string::String>,
}

impl ContainerNodePoolNodeConfigElHostMaintenancePolicyEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    #[doc = "."]
    pub maintenance_interval: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyEl {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyEl { maintenance_interval: self.maintenance_interval }
    }
}

pub struct ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
        ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n."]
    pub fn maintenance_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota_period: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_manager_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePoolNodeConfigElKubeletConfigEl {
    #[doc = "Set the field `cpu_cfs_quota`.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn set_cpu_cfs_quota(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_cfs_quota_period`.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn set_cpu_cfs_quota_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_cfs_quota_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_pids_limit`.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn set_pod_pids_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pod_pids_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElKubeletConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElKubeletConfigEl {
    #[doc = "Control the CPU management policy on the node."]
    pub cpu_manager_policy: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElKubeletConfigElRef {
        ContainerNodePoolNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\nEnable CPU CFS quota enforcement for containers that specify CPU limits."]
    pub fn cpu_cfs_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\nSet the CPU CFS quota period value 'cpu.cfs_period_us'."]
    pub fn cpu_cfs_quota_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_manager_policy` after provisioning.\nControl the CPU management policy on the node."]
    pub fn cpu_manager_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_pids_limit` after provisioning.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn pod_pids_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    #[doc = "Set the field `cgroup_mode`.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn set_cgroup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cgroup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sysctls`.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn set_sysctls(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sysctls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElLinuxNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
        ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cgroup_mode` after provisioning.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn cgroup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sysctls` after provisioning.\nThe Linux kernel parameters to be applied to the nodes and all pods running on the nodes."]
    pub fn sysctls(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
        ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElReservationAffinityEl {
    consume_reservation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc = "Set the field `key`.\nThe label key of a reservation resource."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nThe label values of the reservation resource."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElReservationAffinityEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElReservationAffinityEl {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElReservationAffinityElRef {
        ContainerNodePoolNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consume_reservation_type` after provisioning.\nCorresponds to the type of reservation consumption."]
    pub fn consume_reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe label key of a reservation resource."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe label values of the reservation resource."]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    #[doc =
        "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElShieldedInstanceConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
        ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic {
    node_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    dynamic: ContainerNodePoolNodeConfigElSoleTenantConfigElDynamic,
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElSoleTenantConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
        ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElTaintEl {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerNodePoolNodeConfigElTaintEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElTaintEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElTaintEl {
    #[doc = "Effect for taint."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for taint."]
    pub value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElTaintElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNodeConfigElTaintElRef {
        ContainerNodePoolNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElTaintElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\nEffect for taint."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for taint."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue for taint."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl { }

impl terrars::ToListMappable for ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl { mode: self.mode }
    }
}

pub struct ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
        ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nMode is the configuration for how to expose metadata to workloads running on the node."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolNodeConfigElDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    fast_socket: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElFastSocketEl>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    gvnic: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElGvnicEl>>,
    host_maintenance_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    kubelet_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    linux_node_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    sole_tenant_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    taint: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElTaintEl>>,
    workload_metadata_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<terrars::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_maintenance_policy: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std::option::Option<std::vec::Vec<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    dynamic: ContainerNodePoolNodeConfigElDynamic,
}

impl ContainerNodePoolNodeConfigEl {
    #[doc =
        "Set the field `boot_disk_kms_key`.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn set_boot_disk_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size_gb`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `guest_accelerator`.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn set_guest_accelerator(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ContainerNodePoolNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_type`.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn set_image_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_count`.\nThe number of local SSD disks to be attached to the node."]
    pub fn set_local_ssd_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.local_ssd_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logging_variant`.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn set_logging_variant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_variant = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_group`.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn set_node_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preemptible`.\nWhether the nodes are created as preemptible VM instances."]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_labels`.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn set_resource_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot`.\nWhether the nodes are created as spot VM instances."]
    pub fn set_spot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.spot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nThe list of instance tags applied to all nodes."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_machine_features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidential_nodes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_nodes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ephemeral_storage_local_ssd_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_storage_local_ssd_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElFastSocketEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fast_socket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_socket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElGcfsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcfs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcfs_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElGvnicEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gvnic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gvnic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubelet_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubelet_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linux_node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_nvme_ssd_block_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_nvme_ssd_block_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sole_tenant_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sole_tenant_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElTaintEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workload_metadata_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workload_metadata_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerNodePoolNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolNodeConfigElRef {
        ContainerNodePoolNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn boot_disk_kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_taints` after provisioning.\nList of kubernetes taints applied to each node."]
    pub fn effective_taints(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElEffectiveTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElGuestAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_type` after provisioning.\nThe image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_count` after provisioning.\nThe number of local SSD disks to be attached to the node."]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logging_variant` after provisioning.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn logging_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe metadata key/value pairs assigned to instances in the cluster."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_group` after provisioning.\nSetting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes."]
    pub fn node_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all of the node VMs."]
    pub fn oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptible` after provisioning.\nWhether the nodes are created as preemptible VM instances."]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the node pool."]
    pub fn resource_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spot` after provisioning.\nWhether the nodes are created as spot VM instances."]
    pub fn spot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all nodes."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNodeConfigElAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElConfidentialNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElFastSocketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElGcfsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElGvnicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElHostMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElKubeletConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElLinuxNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElSoleTenantConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElTaintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> terrars::ListRef<ContainerNodePoolNodeConfigElWorkloadMetadataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolPlacementPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerNodePoolPlacementPolicyEl {
    #[doc =
        "Set the field `policy_name`.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn set_policy_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tpu_topology`.\nTPU placement topology for pod slice node pool. https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies"]
    pub fn set_tpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tpu_topology = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolPlacementPolicyEl {
    type O = terrars::BlockAssignable<ContainerNodePoolPlacementPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePoolPlacementPolicyEl {
    #[doc = "Type defines the type of placement policy"]
    pub type_: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolPlacementPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolPlacementPolicyElRef {
        ContainerNodePoolPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolPlacementPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy_name` after provisioning.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tpu_topology` after provisioning.\nTPU placement topology for pod slice node pool. https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies"]
    pub fn tpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType defines the type of placement policy"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolTimeoutsEl {
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

impl terrars::ToListMappable for ContainerNodePoolTimeoutsEl {
    type O = terrars::BlockAssignable<ContainerNodePoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolTimeoutsElRef {
        ContainerNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolTimeoutsElRef {
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
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc = "Set the field `batch_node_count`.\nNumber of blue nodes to drain in a batch."]
    pub fn set_batch_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_percentage`.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn set_batch_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_soak_duration`.\nSoak time after each batch gets drained."]
    pub fn set_batch_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.batch_soak_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O = terrars::BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_node_count` after provisioning.\nNumber of blue nodes to drain in a batch."]
    pub fn batch_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `batch_percentage` after provisioning.\nPercentage of the blue pool nodes to drain in a batch."]
    pub fn batch_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `batch_soak_duration` after provisioning.\nSoak time after each batch gets drained."]
    pub fn batch_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElDynamic,
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    #[doc =
        "Set the field `node_pool_soak_duration`.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn set_node_pool_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.node_pool_soak_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard_rollout_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_rollout_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl {
    type O = terrars::BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
        ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up."]
    pub fn node_pool_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> terrars::ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolUpgradeSettingsElDynamic {
    blue_green_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
}

#[derive(Serialize)]
pub struct ContainerNodePoolUpgradeSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std::option::Option<std::vec::Vec<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    dynamic: ContainerNodePoolUpgradeSettingsElDynamic,
}

impl ContainerNodePoolUpgradeSettingsEl {
    #[doc =
        "Set the field `max_surge`.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn set_max_surge(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable`.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn set_max_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\nUpdate strategy for the given nodepool."]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.blue_green_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.blue_green_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePoolUpgradeSettingsEl {
    type O = terrars::BlockAssignable<ContainerNodePoolUpgradeSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerNodePoolUpgradeSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePoolUpgradeSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePoolUpgradeSettingsElRef {
        ContainerNodePoolUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePoolUpgradeSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_surge` after provisioning.\nThe number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater."]
    pub fn max_surge(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable` after provisioning.\nThe number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater."]
    pub fn max_unavailable(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strategy` after provisioning.\nUpdate strategy for the given nodepool."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(&self) -> terrars::ListRef<ContainerNodePoolUpgradeSettingsElBlueGreenSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerNodePoolAutoscalingEl>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerNodePoolManagementEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNetworkConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerNodePoolNodeConfigEl>>,
    placement_policy: std::option::Option<terrars::DynamicBlock<ContainerNodePoolPlacementPolicyEl>>,
    upgrade_settings: std::option::Option<terrars::DynamicBlock<ContainerNodePoolUpgradeSettingsEl>>,
}
