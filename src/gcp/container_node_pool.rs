#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerNodePoolData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_node_count_changes: std::option::Option<terrars::PrimField<bool>>,
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
    autoscaling: std::option::Option<std::vec::Vec<ContainerNodePool_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    best_effort_provisioning: std::option::Option<std::vec::Vec<ContainerNodePool_BestEffortProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<ContainerNodePool_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerNodePool_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<ContainerNodePool_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_drain_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeDrainConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<std::vec::Vec<ContainerNodePool_PlacementPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queued_provisioning: std::option::Option<std::vec::Vec<ContainerNodePool_QueuedProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerNodePool_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<std::vec::Vec<ContainerNodePool_UpgradeSettings>>,
    dynamic: ContainerNodePoolDynamic,
}

struct ContainerNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerNodePoolData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_node_count_changes`.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state."]
    pub fn set_ignore_node_count_changes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_node_count_changes = std::option::Option::Some(v.into());
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
        "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name. Max length is 31 characters. Prefixes with lengths longer than 14 characters will use a shortened UUID that will be more prone to collisions."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_Autoscaling>>,
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

    #[doc = "Set the field `best_effort_provisioning`.\n"]
    pub fn set_best_effort_provisioning(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_BestEffortProvisioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().best_effort_provisioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.best_effort_provisioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_MaintenancePolicy>>,
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

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_Management>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NetworkConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig>>,
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

    #[doc = "Set the field `node_drain_config`.\n"]
    pub fn set_node_drain_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeDrainConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_drain_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_drain_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_PlacementPolicy>>,
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

    #[doc = "Set the field `queued_provisioning`.\n"]
    pub fn set_queued_provisioning(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_QueuedProvisioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queued_provisioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queued_provisioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerNodePool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_UpgradeSettings>>,
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state."]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.extract_ref()))
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
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name. Max length is 31 characters. Prefixes with lengths longer than 14 characters will use a shortened UUID that will be more prone to collisions."]
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
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerNodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `best_effort_provisioning` after provisioning.\n"]
    pub fn best_effort_provisioning(&self) -> terrars::ListRef<ContainerNodePool_BestEffortProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_effort_provisioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerNodePool_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerNodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerNodePool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> terrars::ListRef<ContainerNodePool_NodeDrainConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_drain_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerNodePool_PlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> terrars::ListRef<ContainerNodePool_QueuedProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queued_provisioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePool_TimeoutsRef {
        ContainerNodePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerNodePool_UpgradeSettingsRef> {
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
            data: std::cell::RefCell::new(ContainerNodePoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_node_count_changes: core::default::Default::default(),
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
                best_effort_provisioning: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                management: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                node_drain_config: core::default::Default::default(),
                placement_policy: core::default::Default::default(),
                queued_provisioning: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state."]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.extract_ref()))
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
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name. Max length is 31 characters. Prefixes with lengths longer than 14 characters will use a shortened UUID that will be more prone to collisions."]
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
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerNodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `best_effort_provisioning` after provisioning.\n"]
    pub fn best_effort_provisioning(&self) -> terrars::ListRef<ContainerNodePool_BestEffortProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_effort_provisioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerNodePool_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerNodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerNodePool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> terrars::ListRef<ContainerNodePool_NodeDrainConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_drain_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerNodePool_PlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> terrars::ListRef<ContainerNodePool_QueuedProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queued_provisioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerNodePool_TimeoutsRef {
        ContainerNodePool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerNodePool_UpgradeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePoolDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerNodePool_Autoscaling>>,
    best_effort_provisioning: std::option::Option<terrars::DynamicBlock<ContainerNodePool_BestEffortProvisioning>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<ContainerNodePool_MaintenancePolicy>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerNodePool_Management>>,
    network_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NetworkConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig>>,
    node_drain_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeDrainConfig>>,
    placement_policy: std::option::Option<terrars::DynamicBlock<ContainerNodePool_PlacementPolicy>>,
    queued_provisioning: std::option::Option<terrars::DynamicBlock<ContainerNodePool_QueuedProvisioning>>,
    upgrade_settings: std::option::Option<terrars::DynamicBlock<ContainerNodePool_UpgradeSettings>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_Autoscaling {
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

impl ContainerNodePool_Autoscaling {
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

impl terrars::ToListMappable for ContainerNodePool_Autoscaling {
    type O = terrars::BlockAssignable<ContainerNodePool_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_Autoscaling {}

impl BuildContainerNodePool_Autoscaling {
    pub fn build(self) -> ContainerNodePool_Autoscaling {
        ContainerNodePool_Autoscaling {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_AutoscalingRef {
        ContainerNodePool_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_AutoscalingRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_BestEffortProvisioning {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_provision_nodes: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_BestEffortProvisioning {
    #[doc =
        "Set the field `min_provision_nodes`.\nMinimum number of nodes that must be provisioned for the creation to be considered successful. Can only be set when enabled is true."]
    pub fn set_min_provision_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_provision_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_BestEffortProvisioning {
    type O = terrars::BlockAssignable<ContainerNodePool_BestEffortProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_BestEffortProvisioning {
    #[doc =
        "Whether node pool creation ignores non-fatal errors like stockout to provision as many nodes as possible, eventually bringing up the target number of nodes."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_BestEffortProvisioning {
    pub fn build(self) -> ContainerNodePool_BestEffortProvisioning {
        ContainerNodePool_BestEffortProvisioning {
            enabled: self.enabled,
            min_provision_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_BestEffortProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_BestEffortProvisioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_BestEffortProvisioningRef {
        ContainerNodePool_BestEffortProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_BestEffortProvisioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether node pool creation ignores non-fatal errors like stockout to provision as many nodes as possible, eventually bringing up the target number of nodes."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_provision_nodes` after provisioning.\nMinimum number of nodes that must be provisioned for the creation to be considered successful. Can only be set when enabled is true."]
    pub fn min_provision_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_provision_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[doc =
        "Set the field `enabled`.\nWhether to enable the maintenance exclusion until the end of support for this NodePool."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    type O = terrars::BlockAssignable<ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {}

impl BuildContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    pub fn build(self) -> ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
        ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
        ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable the maintenance exclusion until the end of support for this NodePool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nEnd time of the maintenance exclusion."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nStart time of the maintenance exclusion."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_MaintenancePolicyDynamic {
    exclusion_until_end_of_support: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_until_end_of_support: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
    dynamic: ContainerNodePool_MaintenancePolicyDynamic,
}

impl ContainerNodePool_MaintenancePolicy {
    #[doc = "Set the field `exclusion_until_end_of_support`.\n"]
    pub fn set_exclusion_until_end_of_support(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_until_end_of_support = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_until_end_of_support = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_MaintenancePolicy {
    type O = terrars::BlockAssignable<ContainerNodePool_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_MaintenancePolicy {}

impl BuildContainerNodePool_MaintenancePolicy {
    pub fn build(self) -> ContainerNodePool_MaintenancePolicy {
        ContainerNodePool_MaintenancePolicy {
            exclusion_until_end_of_support: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_MaintenancePolicyRef {
        ContainerNodePool_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_until_end_of_support` after provisioning.\n"]
    pub fn exclusion_until_end_of_support(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_until_end_of_support", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_Management {
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

impl terrars::ToListMappable for ContainerNodePool_Management {
    type O = terrars::BlockAssignable<ContainerNodePool_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_Management {}

impl BuildContainerNodePool_Management {
    pub fn build(self) -> ContainerNodePool_Management {
        ContainerNodePool_Management {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_ManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_ManagementRef {
        ContainerNodePool_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_ManagementRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    #[doc = "Set the field `network`.\nName of the VPC where the additional interface belongs."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nName of the subnetwork where the additional interface belongs."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    type O = terrars::BlockAssignable<ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {}

impl BuildContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    pub fn build(self) -> ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
        ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
        ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nName of the VPC where the additional interface belongs."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nName of the subnetwork where the additional interface belongs."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    #[doc = "Set the field `max_pods_per_node`.\nThe maximum number of pods per node which use this pod network."]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secondary_pod_range`.\nThe name of the secondary range on the subnet which provides IP address for this pod range."]
    pub fn set_secondary_pod_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nName of the subnetwork where the additional pod network belongs."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    type O = terrars::BlockAssignable<ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {}

impl BuildContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    pub fn build(self) -> ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
        ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs {
            max_pods_per_node: core::default::Default::default(),
            secondary_pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
        ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node which use this pod network."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_pod_range` after provisioning.\nThe name of the secondary range on the subnet which provides IP address for this pod range."]
    pub fn secondary_pod_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_pod_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nName of the subnetwork where the additional pod network belongs."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NetworkConfig_NetworkPerformanceConfig { }

impl terrars::ToListMappable for ContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NetworkConfig_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
    #[doc =
        "Specifies the total network bandwidth tier for the NodePool. [Valid values](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.Tier) include: \"TIER_1\" and \"TIER_UNSPECIFIED\"."]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
    pub fn build(self) -> ContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
        ContainerNodePool_NetworkConfig_NetworkPerformanceConfig {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef {
        ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nSpecifies the total network bandwidth tier for the NodePool. [Valid values](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.Tier) include: \"TIER_1\" and \"TIER_UNSPECIFIED\"."]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig {
    disabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig { }

impl terrars::ToListMappable for ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig {
    pub fn build(self) -> ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig {
        ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef {
        ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NetworkConfigDynamic {
    additional_node_network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    additional_pod_network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NetworkConfig_NetworkPerformanceConfig>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_network_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_pod_range: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_node_network_configs: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_network_configs: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NetworkConfig_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig>>,
    dynamic: ContainerNodePool_NetworkConfigDynamic,
}

impl ContainerNodePool_NetworkConfig {
    #[doc = "Set the field `accelerator_network_profile`.\nThe accelerator network profile to use for this node pool."]
    pub fn set_accelerator_network_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.accelerator_network_profile = std::option::Option::Some(v.into());
        self
    }

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

    #[doc =
        "Set the field `subnetwork`.\nThe subnetwork name/path for the node pool. Format: subnetwork or projects/{project}/regions/{region}/subnetworks/{subnetwork}. This value may be specified via the nested network_config block (setting this attribute directly is supported for backward compatibility). Once created the node pool's subnetwork is immutable. If not set, the provider/API will choose the subnetwork (e.g. based on IP utilization) and report it here."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_node_network_configs`.\n"]
    pub fn set_additional_node_network_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_node_network_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_node_network_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `additional_pod_network_configs`.\n"]
    pub fn set_additional_pod_network_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_pod_network_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_pod_network_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NetworkConfig_NetworkPerformanceConfig>>,
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
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfig>>,
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

impl terrars::ToListMappable for ContainerNodePool_NetworkConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NetworkConfig {}

impl BuildContainerNodePool_NetworkConfig {
    pub fn build(self) -> ContainerNodePool_NetworkConfig {
        ContainerNodePool_NetworkConfig {
            accelerator_network_profile: core::default::Default::default(),
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            additional_node_network_configs: core::default::Default::default(),
            additional_pod_network_configs: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NetworkConfigRef {
        ContainerNodePool_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_network_profile` after provisioning.\nThe accelerator network profile to use for this node pool."]
    pub fn accelerator_network_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_network_profile", self.base))
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

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork name/path for the node pool. Format: subnetwork or projects/{project}/regions/{region}/subnetworks/{subnetwork}. This value may be specified via the nested network_config block (setting this attribute directly is supported for backward compatibility). Once created the node pool's subnetwork is immutable. If not set, the provider/API will choose the subnetwork (e.g. based on IP utilization) and report it here."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_node_network_configs` after provisioning.\n"]
    pub fn additional_node_network_configs(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_node_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_network_configs` after provisioning.\n"]
    pub fn additional_pod_network_configs(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NetworkConfig_AdditionalPodNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NetworkConfig_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NetworkConfig_PodCidrOverprovisionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_EffectiveTaints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_EffectiveTaints {
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

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_EffectiveTaints {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_EffectiveTaints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_EffectiveTaints {}

impl BuildContainerNodePool_NodeConfig_EffectiveTaints {
    pub fn build(self) -> ContainerNodePool_NodeConfig_EffectiveTaints {
        ContainerNodePool_NodeConfig_EffectiveTaints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_EffectiveTaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_EffectiveTaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_EffectiveTaintsRef {
        ContainerNodePool_NodeConfig_EffectiveTaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_EffectiveTaintsRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_monitoring_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    threads_per_core: terrars::PrimField<f64>,
}

impl ContainerNodePool_NodeConfig_AdvancedMachineFeatures {
    #[doc =
        "Set the field `enable_nested_virtualization`.\nWhether the node should have nested virtualization enabled."]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `performance_monitoring_unit`.\nLevel of Performance Monitoring Unit (PMU) requested. If unset, no access to the PMU is assumed. For existing node pools with no PMU, setting STANDARD may not produce a diff; recreate the node pool to apply it."]
    pub fn set_performance_monitoring_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.performance_monitoring_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_AdvancedMachineFeatures {
    #[doc =
        "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: terrars::PrimField<f64>,
}

impl BuildContainerNodePool_NodeConfig_AdvancedMachineFeatures {
    pub fn build(self) -> ContainerNodePool_NodeConfig_AdvancedMachineFeatures {
        ContainerNodePool_NodeConfig_AdvancedMachineFeatures {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: self.threads_per_core,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef {
        ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether the node should have nested virtualization enabled."]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\nLevel of Performance Monitoring Unit (PMU) requested. If unset, no access to the PMU is assumed. For existing node pools with no PMU, setting STANDARD may not produce a diff; recreate the node pool to apply it."]
    pub fn performance_monitoring_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_monitoring_unit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_BootDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_NodeConfig_BootDisk {
    #[doc =
        "Set the field `disk_type`.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_iops`.\nConfigured IOPs provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_throughput`.\nConfigured throughput provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_gb`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_BootDisk {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_BootDisk {}

impl BuildContainerNodePool_NodeConfig_BootDisk {
    pub fn build(self) -> ContainerNodePool_NodeConfig_BootDisk {
        ContainerNodePool_NodeConfig_BootDisk {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_BootDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_BootDiskRef {
        ContainerNodePool_NodeConfig_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_BootDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_iops` after provisioning.\nConfigured IOPs provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_throughput` after provisioning.\nConfigured throughput provisioning. Only valid with disk type hyperdisk-balanced."]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ConfidentialNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NodeConfig_ConfidentialNodes {
    #[doc =
        "Set the field `confidential_instance_type`.\nDefines the type of technology used by the confidential node."]
    pub fn set_confidential_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.confidential_instance_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ConfidentialNodes {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ConfidentialNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ConfidentialNodes {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_ConfidentialNodes {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ConfidentialNodes {
        ContainerNodePool_NodeConfig_ConfidentialNodes {
            confidential_instance_type: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ConfidentialNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ConfidentialNodesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ConfidentialNodesRef {
        ContainerNodePool_NodeConfig_ConfidentialNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ConfidentialNodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_type` after provisioning.\nDefines the type of technology used by the confidential node."]
    pub fn confidential_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidential_instance_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    secret_uri: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {

}

impl terrars
::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    type O =
        terrars
        ::BlockAssignable<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    #[doc =
        "URI for the secret that hosts a certificate. Must be in the format 'projects/PROJECT_NUM/secrets/SECRET_NAME/versions/VERSION_OR_LATEST'."]
    pub secret_uri: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    pub fn build(
        self,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
            secret_uri: self.secret_uri,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_uri` after provisioning.\nURI for the secret that hosts a certificate. Must be in the format 'projects/PROJECT_NUM/secrets/SECRET_NAME/versions/VERSION_OR_LATEST'."]
    pub fn secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigDynamic {
    gcp_secret_manager_certificate_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    fqdns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_certificate_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >,
    >,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    #[doc = "Set the field `gcp_secret_manager_certificate_config`.\n"]
    pub fn set_gcp_secret_manager_certificate_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcp_secret_manager_certificate_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcp_secret_manager_certificate_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    type O =
        terrars
        ::BlockAssignable<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    #[doc = "List of fully-qualified-domain-names. IPv4s and port specification are supported."]
    pub fqdns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    pub fn build(
        self,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
            fqdns: self.fqdns,
            gcp_secret_manager_certificate_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fqdns` after provisioning.\nList of fully-qualified-domain-names. IPv4s and port specification are supported."]
    pub fn fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fqdns", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"]
    pub fn gcp_secret_manager_certificate_config(
        &self,
    ) -> terrars
    ::ListRef<
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_secret_manager_certificate_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigDynamic {
    certificate_authority_domain_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_domain_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >,
    >,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    #[doc = "Set the field `certificate_authority_domain_config`.\n"]
    pub fn set_certificate_authority_domain_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate_authority_domain_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_authority_domain_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    #[doc = "Whether or not private registries are configured."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
            enabled: self.enabled,
            certificate_authority_domain_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not private registries are configured."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"]
    pub fn certificate_authority_domain_config(
        &self,
    ) -> terrars
    ::ListRef<
        ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_domain_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    #[doc =
        "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the certificate."]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the certificate."]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    #[doc =
        "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the client certificate."]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the client certificate."]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    #[doc =
        "Set the field `gcp_secret_manager_secret_uri`.\nURI for the Secret Manager secret that hosts the private key."]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nURI for the Secret Manager secret that hosts the private key."]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientDynamic {
    cert: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>>,
    key: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>>,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    key: terrars::PrimField<std::string::String>,
    value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    #[doc = "Configures the header key."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Configures the header value."]
    pub value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nConfigures the header key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nConfigures the header value."]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsDynamic {
    ca: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>>,
    client: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dial_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_path: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>>,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[doc =
        "Set the field `capabilities`.\nRepresent the capabilities of the registry host, specifying what operations a host is capable of performing."]
    pub fn set_capabilities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dial_timeout`.\nSpecifies the maximum duration allowed for a connection attempt to complete."]
    pub fn set_dial_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dial_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_path`.\nIndicate the host's API root endpoint is defined in the URL path rather than by the API specification."]
    pub fn set_override_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ca`.\n"]
    pub fn set_ca(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ca = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ca = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client`.\n"]
    pub fn set_client(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[doc = "Configures the registry host/mirror."]
    pub host: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
            capabilities: core::default::Default::default(),
            dial_timeout: core::default::Default::default(),
            host: self.host,
            override_path: core::default::Default::default(),
            ca: core::default::Default::default(),
            client: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capabilities` after provisioning.\nRepresent the capabilities of the registry host, specifying what operations a host is capable of performing."]
    pub fn capabilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dial_timeout` after provisioning.\nSpecifies the maximum duration allowed for a connection attempt to complete."]
    pub fn dial_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dial_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nConfigures the registry host/mirror."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_path` after provisioning.\nIndicate the host's API root endpoint is defined in the URL path rather than by the API specification."]
    pub fn override_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_path", self.base))
    }

    #[doc = "Get a reference to the value of field `ca` after provisioning.\n"]
    pub fn ca(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }

    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsDynamic {
    hosts: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
    server: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
    #[doc = "Defines the host name of the registry server."]
    pub server: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts {
            server: self.server,
            hosts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `server` after provisioning.\nDefines the host name of the registry server."]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups {
    #[doc = "Whether writable cgroups are enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups {
        ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef {
        ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether writable cgroups are enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_ContainerdConfigDynamic {
    private_registry_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>>,
    registry_hosts: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts>>,
    writable_cgroups: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_registry_access_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_hosts: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    writable_cgroups: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups>>,
    dynamic: ContainerNodePool_NodeConfig_ContainerdConfigDynamic,
}

impl ContainerNodePool_NodeConfig_ContainerdConfig {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_registry_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_registry_access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHosts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.registry_hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.registry_hosts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.writable_cgroups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.writable_cgroups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ContainerdConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ContainerdConfig {}

impl BuildContainerNodePool_NodeConfig_ContainerdConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ContainerdConfig {
        ContainerNodePool_NodeConfig_ContainerdConfig {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ContainerdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ContainerdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ContainerdConfigRef {
        ContainerNodePool_NodeConfig_ContainerdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ContainerdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]
    pub fn private_registry_access_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_RegistryHostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registry_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfig_WritableCgroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.writable_cgroups", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_count: std::option::Option<terrars::PrimField<f64>>,
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[doc =
        "Set the field `data_cache_count`.\nNumber of local SSDs to be utilized for GKE Data Cache. Uses NVMe interfaces."]
    pub fn set_data_cache_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_cache_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[doc =
        "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
        ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: self.local_ssd_count,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef {
        ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_cache_count` after provisioning.\nNumber of local SSDs to be utilized for GKE Data Cache. Uses NVMe interfaces."]
    pub fn data_cache_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_cache_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_count` after provisioning.\nNumber of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_FastSocket {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NodeConfig_FastSocket { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_FastSocket {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_FastSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_FastSocket {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_FastSocket {
    pub fn build(self) -> ContainerNodePool_NodeConfig_FastSocket {
        ContainerNodePool_NodeConfig_FastSocket { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_FastSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_FastSocketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_FastSocketRef {
        ContainerNodePool_NodeConfig_FastSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_FastSocketRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_GcfsConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NodeConfig_GcfsConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_GcfsConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_GcfsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_GcfsConfig {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_GcfsConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_GcfsConfig {
        ContainerNodePool_NodeConfig_GcfsConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_GcfsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_GcfsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_GcfsConfigRef {
        ContainerNodePool_NodeConfig_GcfsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_GcfsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    gpu_driver_version: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    #[doc = "Mode for how the GPU driver is installed."]
    pub gpu_driver_version: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
        ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
            gpu_driver_version: self.gpu_driver_version,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
        ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gpu_driver_version` after provisioning.\nMode for how the GPU driver is installed."]
    pub fn gpu_driver_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
    gpu_sharing_strategy: terrars::PrimField<std::string::String>,
    max_shared_clients_per_gpu: terrars::PrimField<f64>,
}

impl ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
    #[doc =
        "The type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)"]
    pub gpu_sharing_strategy: terrars::PrimField<std::string::String>,
    #[doc = "The maximum number of containers that can share a GPU."]
    pub max_shared_clients_per_gpu: terrars::PrimField<f64>,
}

impl BuildContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
        ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig {
            gpu_sharing_strategy: self.gpu_sharing_strategy,
            max_shared_clients_per_gpu: self.max_shared_clients_per_gpu,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
        ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\nThe type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)"]
    pub fn gpu_sharing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_sharing_strategy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\nThe maximum number of containers that can share a GPU."]
    pub fn max_shared_clients_per_gpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_shared_clients_per_gpu", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_GuestAcceleratorDynamic {
    gpu_driver_installation_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>>,
    gpu_sharing_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_GuestAccelerator {
    count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
    dynamic: ContainerNodePool_NodeConfig_GuestAcceleratorDynamic,
}

impl ContainerNodePool_NodeConfig_GuestAccelerator {
    #[doc =
        "Set the field `gpu_partition_size`.\nSize of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)"]
    pub fn set_gpu_partition_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_partition_size = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gpu_driver_installation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gpu_driver_installation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gpu_sharing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gpu_sharing_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_GuestAccelerator {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_GuestAccelerator {
    #[doc = "The number of the accelerator cards exposed to an instance."]
    pub count: terrars::PrimField<f64>,
    #[doc = "The accelerator type resource name."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_GuestAccelerator {
    pub fn build(self) -> ContainerNodePool_NodeConfig_GuestAccelerator {
        ContainerNodePool_NodeConfig_GuestAccelerator {
            count: self.count,
            gpu_partition_size: core::default::Default::default(),
            type_: self.type_,
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_GuestAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_GuestAcceleratorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_GuestAcceleratorRef {
        ContainerNodePool_NodeConfig_GuestAcceleratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_GuestAcceleratorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nThe number of the accelerator cards exposed to an instance."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gpu_partition_size` after provisioning.\nSize of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)"]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type resource name."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]
    pub fn gpu_driver_installation_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_GuestAccelerator_GpuSharingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_Gvnic {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_NodeConfig_Gvnic { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_Gvnic {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_Gvnic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_Gvnic {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_NodeConfig_Gvnic {
    pub fn build(self) -> ContainerNodePool_NodeConfig_Gvnic {
        ContainerNodePool_NodeConfig_Gvnic { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_GvnicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_GvnicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_GvnicRef {
        ContainerNodePool_NodeConfig_GvnicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_GvnicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_container_restart_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
    #[doc =
        "Set the field `max_container_restart_period`.\nThe maximum duration the backoff delay can accrue to for container restarts."]
    pub fn set_max_container_restart_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_container_restart_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
        ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff {
            max_container_restart_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
        ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_container_restart_period` after provisioning.\nThe maximum duration the backoff delay can accrue to for container restarts."]
    pub fn max_container_restart_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_container_restart_period", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_available: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    #[doc = "Set the field `imagefs_available`.\nDefines percentage of minimum reclaim for imagefs.available."]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imagefs_inodes_free`.\nDefines percentage of minimum reclaim for imagefs.inodesFree."]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\nDefines percentage of minimum reclaim for memory.available."]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\nDefines percentage of minimum reclaim for nodefs.available."]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_inodes_free`.\nDefines percentage of minimum reclaim for nodefs.inodesFree."]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\nDefines percentage of minimum reclaim for pid.available."]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines percentage of minimum reclaim for imagefs.available."]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines percentage of minimum reclaim for imagefs.inodesFree."]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_available` after provisioning.\nDefines percentage of minimum reclaim for memory.available."]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines percentage of minimum reclaim for nodefs.available."]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines percentage of minimum reclaim for nodefs.inodesFree."]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pid_available` after provisioning.\nDefines percentage of minimum reclaim for pid.available."]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_available: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
    #[doc = "Set the field `imagefs_available`.\nDefines percentage of soft eviction threshold for imagefs.available."]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `imagefs_inodes_free`.\nDefines percentage of soft eviction threshold for imagefs.inodesFree."]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\nDefines quantity of soft eviction threshold for memory.available."]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\nDefines percentage of soft eviction threshold for nodefs.available."]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_inodes_free`.\nDefines percentage of soft eviction threshold for nodefs.inodesFree."]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\nDefines percentage of soft eviction threshold for pid.available."]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines percentage of soft eviction threshold for imagefs.available."]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines percentage of soft eviction threshold for imagefs.inodesFree."]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_available` after provisioning.\nDefines quantity of soft eviction threshold for memory.available."]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines percentage of soft eviction threshold for nodefs.available."]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines percentage of soft eviction threshold for nodefs.inodesFree."]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pid_available` after provisioning.\nDefines percentage of soft eviction threshold for pid.available."]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imagefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_available: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodefs_inodes_free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_available: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    #[doc =
        "Set the field `imagefs_available`.\nDefines grace period for the imagefs.available soft eviction threshold"]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `imagefs_inodes_free`.\nDefines grace period for the imagefs.inodesFree soft eviction threshold."]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\nDefines grace period for the memory.available soft eviction threshold."]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\nDefines grace period for the nodefs.available soft eviction threshold."]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nodefs_inodes_free`.\nDefines grace period for the nodefs.inodesFree soft eviction threshold."]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\nDefines grace period for the pid.available soft eviction threshold."]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
        ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `imagefs_available` after provisioning.\nDefines grace period for the imagefs.available soft eviction threshold"]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\nDefines grace period for the imagefs.inodesFree soft eviction threshold."]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_available` after provisioning.\nDefines grace period for the memory.available soft eviction threshold."]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_available` after provisioning.\nDefines grace period for the nodefs.available soft eviction threshold."]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\nDefines grace period for the nodefs.inodesFree soft eviction threshold."]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pid_available` after provisioning.\nDefines grace period for the pid.available soft eviction threshold."]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {
    #[doc =
        "Set the field `policy`.\nThe Memory Manager policy to use. This policy guides how memory and hugepages are allocated and managed for pods on the node, influencing NUMA affinity."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager {
        ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager { policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef {
        ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nThe Memory Manager policy to use. This policy guides how memory and hugepages are allocated and managed for pods on the node, influencing NUMA affinity."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
    #[doc =
        "Set the field `policy`.\nThe Topology Manager policy to use. This policy dictates how resource alignment is handled on the node."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scope`.\nThe Topology Manager scope, defining the granularity at which policy decisions are applied. Valid values are \"container\" (resources are aligned per container within a pod) or \"pod\" (resources are aligned for the entire pod)."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
        ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef {
        ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nThe Topology Manager policy to use. This policy dictates how resource alignment is handled on the node."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe Topology Manager scope, defining the granularity at which policy decisions are applied. Valid values are \"container\" (resources are aligned per container within a pod) or \"pod\" (resources are aligned for the entire pod)."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_KubeletConfigDynamic {
    crash_loop_back_off: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    eviction_minimum_reclaim: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    eviction_soft: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft>>,
    eviction_soft_grace_period: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
    memory_manager: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager>>,
    topology_manager: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_KubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_unsafe_sysctls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_log_max_files: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_log_max_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manager_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_max_pod_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_gc_high_threshold_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_gc_low_threshold_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_maximum_gc_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_minimum_gc_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_kubelet_readonly_port_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_image_pulls: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shutdown_grace_period_critical_pods_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shutdown_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_process_oom_kill: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crash_loop_back_off: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_minimum_reclaim: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft_grace_period: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_manager: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_manager: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager>>,
    dynamic: ContainerNodePool_NodeConfig_KubeletConfigDynamic,
}

impl ContainerNodePool_NodeConfig_KubeletConfig {
    #[doc =
        "Set the field `allowed_unsafe_sysctls`.\nDefines a comma-separated allowlist of unsafe sysctls or sysctl patterns which can be set on the Pods."]
    pub fn set_allowed_unsafe_sysctls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_unsafe_sysctls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `container_log_max_files`.\nDefines the maximum number of container log files that can be present for a container."]
    pub fn set_container_log_max_files(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_log_max_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `container_log_max_size`.\nDefines the maximum size of the container log file before it is rotated."]
    pub fn set_container_log_max_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_log_max_size = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `cpu_manager_policy`.\nControl the CPU management policy on the node."]
    pub fn set_cpu_manager_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_manager_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `eviction_max_pod_grace_period_seconds`.\nDefines the maximum allowed grace period (in seconds) to use when terminating pods in response to a soft eviction threshold being met."]
    pub fn set_eviction_max_pod_grace_period_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.eviction_max_pod_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_gc_high_threshold_percent`.\nDefines the percent of disk usage after which image garbage collection is always run."]
    pub fn set_image_gc_high_threshold_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_gc_high_threshold_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_gc_low_threshold_percent`.\nDefines the percent of disk usage before which image garbage collection is never run. Lowest disk usage to garbage collect to."]
    pub fn set_image_gc_low_threshold_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_gc_low_threshold_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_maximum_gc_age`.\nDefines the maximum age an image can be unused before it is garbage collected."]
    pub fn set_image_maximum_gc_age(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_maximum_gc_age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_minimum_gc_age`.\nDefines the minimum age for an unused image before it is garbage collected."]
    pub fn set_image_minimum_gc_age(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_minimum_gc_age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `insecure_kubelet_readonly_port_enabled`.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_image_pulls`.\nSet the maximum number of image pulls in parallel."]
    pub fn set_max_parallel_image_pulls(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallel_image_pulls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_pids_limit`.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn set_pod_pids_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pod_pids_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shutdown_grace_period_critical_pods_seconds`.\nControls the portion of total grace period (in seconds) that is specifically reserved for terminating critical pods."]
    pub fn set_shutdown_grace_period_critical_pods_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.shutdown_grace_period_critical_pods_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shutdown_grace_period_seconds`.\nControls the total duration of time (in seconds) the node delays shutdown."]
    pub fn set_shutdown_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shutdown_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_process_oom_kill`.\nDefines whether to enable single process OOM killer."]
    pub fn set_single_process_oom_kill(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_process_oom_kill = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crash_loop_back_off`.\n"]
    pub fn set_crash_loop_back_off(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crash_loop_back_off = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crash_loop_back_off = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eviction_minimum_reclaim`.\n"]
    pub fn set_eviction_minimum_reclaim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaim>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eviction_minimum_reclaim = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_minimum_reclaim = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eviction_soft`.\n"]
    pub fn set_eviction_soft(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoft>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eviction_soft = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_soft = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eviction_soft_grace_period`.\n"]
    pub fn set_eviction_soft_grace_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eviction_soft_grace_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eviction_soft_grace_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_manager`.\n"]
    pub fn set_memory_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_MemoryManager>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_manager = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_manager = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `topology_manager`.\n"]
    pub fn set_topology_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig_TopologyManager>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.topology_manager = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.topology_manager = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_KubeletConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_KubeletConfig {}

impl BuildContainerNodePool_NodeConfig_KubeletConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_KubeletConfig {
        ContainerNodePool_NodeConfig_KubeletConfig {
            allowed_unsafe_sysctls: core::default::Default::default(),
            container_log_max_files: core::default::Default::default(),
            container_log_max_size: core::default::Default::default(),
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            eviction_max_pod_grace_period_seconds: core::default::Default::default(),
            image_gc_high_threshold_percent: core::default::Default::default(),
            image_gc_low_threshold_percent: core::default::Default::default(),
            image_maximum_gc_age: core::default::Default::default(),
            image_minimum_gc_age: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            max_parallel_image_pulls: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
            shutdown_grace_period_critical_pods_seconds: core::default::Default::default(),
            shutdown_grace_period_seconds: core::default::Default::default(),
            single_process_oom_kill: core::default::Default::default(),
            crash_loop_back_off: core::default::Default::default(),
            eviction_minimum_reclaim: core::default::Default::default(),
            eviction_soft: core::default::Default::default(),
            eviction_soft_grace_period: core::default::Default::default(),
            memory_manager: core::default::Default::default(),
            topology_manager: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_KubeletConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_KubeletConfigRef {
        ContainerNodePool_NodeConfig_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_KubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_unsafe_sysctls` after provisioning.\nDefines a comma-separated allowlist of unsafe sysctls or sysctl patterns which can be set on the Pods."]
    pub fn allowed_unsafe_sysctls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_unsafe_sysctls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_log_max_files` after provisioning.\nDefines the maximum number of container log files that can be present for a container."]
    pub fn container_log_max_files(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_log_max_files", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_log_max_size` after provisioning.\nDefines the maximum size of the container log file before it is rotated."]
    pub fn container_log_max_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_log_max_size", self.base))
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
        "Get a reference to the value of field `eviction_max_pod_grace_period_seconds` after provisioning.\nDefines the maximum allowed grace period (in seconds) to use when terminating pods in response to a soft eviction threshold being met."]
    pub fn eviction_max_pod_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eviction_max_pod_grace_period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_gc_high_threshold_percent` after provisioning.\nDefines the percent of disk usage after which image garbage collection is always run."]
    pub fn image_gc_high_threshold_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_gc_high_threshold_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_gc_low_threshold_percent` after provisioning.\nDefines the percent of disk usage before which image garbage collection is never run. Lowest disk usage to garbage collect to."]
    pub fn image_gc_low_threshold_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_gc_low_threshold_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_maximum_gc_age` after provisioning.\nDefines the maximum age an image can be unused before it is garbage collected."]
    pub fn image_maximum_gc_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_maximum_gc_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_minimum_gc_age` after provisioning.\nDefines the minimum age for an unused image before it is garbage collected."]
    pub fn image_minimum_gc_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_minimum_gc_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_parallel_image_pulls` after provisioning.\nSet the maximum number of image pulls in parallel."]
    pub fn max_parallel_image_pulls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_image_pulls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_pids_limit` after provisioning.\nControls the maximum number of processes allowed to run in a pod."]
    pub fn pod_pids_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shutdown_grace_period_critical_pods_seconds` after provisioning.\nControls the portion of total grace period (in seconds) that is specifically reserved for terminating critical pods."]
    pub fn shutdown_grace_period_critical_pods_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.shutdown_grace_period_critical_pods_seconds", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `shutdown_grace_period_seconds` after provisioning.\nControls the total duration of time (in seconds) the node delays shutdown."]
    pub fn shutdown_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shutdown_grace_period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `single_process_oom_kill` after provisioning.\nDefines whether to enable single process OOM killer."]
    pub fn single_process_oom_kill(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_process_oom_kill", self.base))
    }

    #[doc = "Get a reference to the value of field `crash_loop_back_off` after provisioning.\n"]
    pub fn crash_loop_back_off(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_CrashLoopBackOffRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crash_loop_back_off", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_minimum_reclaim", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft_grace_period", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_MemoryManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_manager", self.base))
    }

    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfig_TopologyManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ptp_kvm_time_sync: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\nWhether to enable accurate time synchronization with PTP-KVM."]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_ptp_kvm_time_sync` after provisioning.\nWhether to enable accurate time synchronization with PTP-KVM."]
    pub fn enable_ptp_kvm_time_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ptp_kvm_time_sync", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\nThe Secret Manager secret URI of the init script."]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_generation`.\nThe GCS generation of the init script."]
    pub fn set_gcs_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gcs_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_uri`.\nThe GCS URI of the init script."]
    pub fn set_gcs_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
            gcs_generation: core::default::Default::default(),
            gcs_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\nThe Secret Manager secret URI of the init script."]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcs_generation` after provisioning.\nThe GCS generation of the init script."]
    pub fn gcs_generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_uri` after provisioning.\nThe GCS URI of the init script."]
    pub fn gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitDynamic {
    init_script: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_script: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>>,
    dynamic: ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitDynamic,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[doc = "Set the field `init_script`.\n"]
    pub fn set_init_script(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.init_script = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.init_script = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit {
            init_script: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `init_script` after provisioning.\n"]
    pub fn init_script(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.init_script", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_1g: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_2m: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    #[doc = "Set the field `hugepage_size_1g`.\nAmount of 1G hugepages."]
    pub fn set_hugepage_size_1g(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hugepage_size_1g = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hugepage_size_2m`.\nAmount of 2M hugepages."]
    pub fn set_hugepage_size_2m(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hugepage_size_2m = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hugepage_size_1g` after provisioning.\nAmount of 1G hugepages."]
    pub fn hugepage_size_1g(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugepage_size_1g", self.base))
    }

    #[doc = "Get a reference to the value of field `hugepage_size_2m` after provisioning.\nAmount of 2M hugepages."]
    pub fn hugepage_size_2m(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugepage_size_2m", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[doc = "Set the field `policy`.\nThe policy for kernel module loading."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
            policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\nThe policy for kernel module loading."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    #[doc = "Set the field `swap_size_gib`.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn set_swap_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `swap_size_percent`.\nSpecifies the size of the swap space as a percentage of the boot disk size."]
    pub fn set_swap_size_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `swap_size_gib` after provisioning.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn swap_size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_gib", self.base))
    }

    #[doc =
        "Get a reference to the value of field `swap_size_percent` after provisioning.\nSpecifies the size of the swap space as a percentage of the boot disk size."]
    pub fn swap_size_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[doc = "Set the field `disk_count`.\nThe number of physical local NVMe SSD disks to attach."]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    type O =
        terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
            disk_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_count` after provisioning.\nThe number of physical local NVMe SSD disks to attach."]
    pub fn disk_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[doc = "Set the field `disabled`.\nIf true, swap space will not be encrypted. Defaults to false (encrypted)."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, swap space will not be encrypted. Defaults to false (encrypted)."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    #[doc = "Set the field `swap_size_gib`.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn set_swap_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `swap_size_percent`.\nSpecifies the size of the swap space as a percentage of the ephemeral local SSD capacity."]
    pub fn set_swap_size_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    type O =
        terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `swap_size_gib` after provisioning.\nSpecifies the size of the swap space in gibibytes (GiB)."]
    pub fn swap_size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_gib", self.base))
    }

    #[doc =
        "Get a reference to the value of field `swap_size_percent` after provisioning.\nSpecifies the size of the swap space as a percentage of the ephemeral local SSD capacity."]
    pub fn swap_size_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigDynamic {
    boot_disk_profile: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>>,
    dedicated_local_ssd_profile: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>>,
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>>,
    ephemeral_local_ssd_profile: std
    ::option
    ::Option<
        terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>,
    >,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_local_ssd_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_local_ssd_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>>,
    dynamic: ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigDynamic,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
    #[doc = "Set the field `enabled`.\nEnables or disables swap for the node pool."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot_disk_profile`.\n"]
    pub fn set_boot_disk_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boot_disk_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dedicated_local_ssd_profile`.\n"]
    pub fn set_dedicated_local_ssd_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dedicated_local_ssd_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dedicated_local_ssd_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>,
                    >,
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

    #[doc = "Set the field `ephemeral_local_ssd_profile`.\n"]
    pub fn set_ephemeral_local_ssd_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ephemeral_local_ssd_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_local_ssd_profile = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig {
            enabled: core::default::Default::default(),
            boot_disk_profile: core::default::Default::default(),
            dedicated_local_ssd_profile: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            ephemeral_local_ssd_profile: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables or disables swap for the node pool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk_profile` after provisioning.\n"]
    pub fn boot_disk_profile(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]
    pub fn dedicated_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_local_ssd_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]
    pub fn ephemeral_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_local_ssd_profile", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_NodeConfig_LinuxNodeConfigDynamic {
    accurate_time_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    custom_node_init: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    hugepages_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    node_kernel_module_loading: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    swap_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transparent_hugepage_defrag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transparent_hugepage_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accurate_time_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_node_init: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepages_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kernel_module_loading: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig>>,
    dynamic: ContainerNodePool_NodeConfig_LinuxNodeConfigDynamic,
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfig {
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

    #[doc = "Set the field `transparent_hugepage_defrag`.\nThe Linux kernel transparent hugepage defrag setting."]
    pub fn set_transparent_hugepage_defrag(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transparent_hugepage_defrag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transparent_hugepage_enabled`.\nThe Linux kernel transparent hugepage setting."]
    pub fn set_transparent_hugepage_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transparent_hugepage_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accurate_time_config`.\n"]
    pub fn set_accurate_time_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accurate_time_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accurate_time_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_node_init`.\n"]
    pub fn set_custom_node_init(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_node_init = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_node_init = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hugepages_config`.\n"]
    pub fn set_hugepages_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hugepages_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hugepages_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_kernel_module_loading`.\n"]
    pub fn set_node_kernel_module_loading(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_kernel_module_loading = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_kernel_module_loading = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `swap_config`.\n"]
    pub fn set_swap_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.swap_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.swap_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LinuxNodeConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LinuxNodeConfig {}

impl BuildContainerNodePool_NodeConfig_LinuxNodeConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LinuxNodeConfig {
        ContainerNodePool_NodeConfig_LinuxNodeConfig {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
            transparent_hugepage_defrag: core::default::Default::default(),
            transparent_hugepage_enabled: core::default::Default::default(),
            accurate_time_config: core::default::Default::default(),
            custom_node_init: core::default::Default::default(),
            hugepages_config: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            swap_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LinuxNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LinuxNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LinuxNodeConfigRef {
        ContainerNodePool_NodeConfig_LinuxNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LinuxNodeConfigRef {
    #[allow(dead_code)]
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

    #[doc =
        "Get a reference to the value of field `transparent_hugepage_defrag` after provisioning.\nThe Linux kernel transparent hugepage defrag setting."]
    pub fn transparent_hugepage_defrag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transparent_hugepage_defrag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transparent_hugepage_enabled` after provisioning.\nThe Linux kernel transparent hugepage setting."]
    pub fn transparent_hugepage_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transparent_hugepage_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `accurate_time_config` after provisioning.\n"]
    pub fn accurate_time_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accurate_time_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_node_init` after provisioning.\n"]
    pub fn custom_node_init(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_CustomNodeInitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_node_init", self.base))
    }

    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_HugepagesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hugepages_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kernel_module_loading", self.base))
    }

    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfig_SwapConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig {
        ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig { local_ssd_count: self.local_ssd_count }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef {
        ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_NodeImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_NodeImageConfig {
    #[doc =
        "Set the field `image`.\nThe Operating System image for the node pool. This is a private feature, please contact your Google account team for allowlisting this feature."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_project`.\nThe GCP project storing the Operating System image for the node pool. This is a private feature, please contact your Google account team for allowlisting this feature."]
    pub fn set_image_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_NodeImageConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_NodeImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_NodeImageConfig {}

impl BuildContainerNodePool_NodeConfig_NodeImageConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_NodeImageConfig {
        ContainerNodePool_NodeConfig_NodeImageConfig {
            image: core::default::Default::default(),
            image_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_NodeImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_NodeImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_NodeImageConfigRef {
        ContainerNodePool_NodeConfig_NodeImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_NodeImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe Operating System image for the node pool. This is a private feature, please contact your Google account team for allowlisting this feature."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_project` after provisioning.\nThe GCP project storing the Operating System image for the node pool. This is a private feature, please contact your Google account team for allowlisting this feature."]
    pub fn image_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_project", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ReservationAffinity {
    consume_reservation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerNodePool_NodeConfig_ReservationAffinity {
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

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ReservationAffinity {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ReservationAffinity {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_ReservationAffinity {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ReservationAffinity {
        ContainerNodePool_NodeConfig_ReservationAffinity {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ReservationAffinityRef {
        ContainerNodePool_NodeConfig_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ReservationAffinityRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_SandboxConfig {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_SandboxConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_SandboxConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_SandboxConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_SandboxConfig {
    #[doc = "Type of the sandbox to use for the node (e.g. 'GVISOR')."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_SandboxConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_SandboxConfig {
        ContainerNodePool_NodeConfig_SandboxConfig { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_SandboxConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_SandboxConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_SandboxConfigRef {
        ContainerNodePool_NodeConfig_SandboxConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_SandboxConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the sandbox to use for the node (e.g. 'GVISOR')."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_SecondaryBootDisks {
    disk_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_SecondaryBootDisks {
    #[doc = "Set the field `mode`.\nMode for how the secondary boot disk is used."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_SecondaryBootDisks {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_SecondaryBootDisks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_SecondaryBootDisks {
    #[doc = "Disk image to create the secondary boot disk from"]
    pub disk_image: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_SecondaryBootDisks {
    pub fn build(self) -> ContainerNodePool_NodeConfig_SecondaryBootDisks {
        ContainerNodePool_NodeConfig_SecondaryBootDisks {
            disk_image: self.disk_image,
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_SecondaryBootDisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_SecondaryBootDisksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_SecondaryBootDisksRef {
        ContainerNodePool_NodeConfig_SecondaryBootDisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_SecondaryBootDisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_image` after provisioning.\nDisk image to create the secondary boot disk from"]
    pub fn disk_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nMode for how the secondary boot disk is used."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_NodeConfig_ShieldedInstanceConfig {
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

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_ShieldedInstanceConfig {}

impl BuildContainerNodePool_NodeConfig_ShieldedInstanceConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_ShieldedInstanceConfig {
        ContainerNodePool_NodeConfig_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef {
        ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
    #[doc = "."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
    pub fn build(self) -> ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
        ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinityRef {
        ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    #[allow(dead_code)]
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
struct ContainerNodePool_NodeConfig_SoleTenantConfigDynamic {
    node_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_SoleTenantConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity>>,
    dynamic: ContainerNodePool_NodeConfig_SoleTenantConfigDynamic,
}

impl ContainerNodePool_NodeConfig_SoleTenantConfig {
    #[doc =
        "Set the field `min_node_cpus`.\nSpecifies the minimum number of vCPUs that each sole tenant node must have to use CPU overcommit. If not specified, the CPU overcommit feature is disabled."]
    pub fn set_min_node_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_SoleTenantConfig_NodeAffinity>>,
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

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_SoleTenantConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_SoleTenantConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_SoleTenantConfig {}

impl BuildContainerNodePool_NodeConfig_SoleTenantConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_SoleTenantConfig {
        ContainerNodePool_NodeConfig_SoleTenantConfig {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_SoleTenantConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_SoleTenantConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_SoleTenantConfigRef {
        ContainerNodePool_NodeConfig_SoleTenantConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_SoleTenantConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_node_cpus` after provisioning.\nSpecifies the minimum number of vCPUs that each sole tenant node must have to use CPU overcommit. If not specified, the CPU overcommit feature is disabled."]
    pub fn min_node_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_Taint {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_Taint { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_Taint {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_Taint {
    #[doc = "Effect for taint."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for taint."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_Taint {
    pub fn build(self) -> ContainerNodePool_NodeConfig_Taint {
        ContainerNodePool_NodeConfig_Taint {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_TaintRef {
        ContainerNodePool_NodeConfig_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_TaintRef {
    #[allow(dead_code)]
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
pub struct ContainerNodePool_NodeConfig_TaintConfig {
    architecture_taint_behavior: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_TaintConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_TaintConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_TaintConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_TaintConfig {
    #[doc = "Architecture taint behavior. Controls, how we apply taints based on the node architecture."]
    pub architecture_taint_behavior: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_TaintConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_TaintConfig {
        ContainerNodePool_NodeConfig_TaintConfig { architecture_taint_behavior: self.architecture_taint_behavior }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_TaintConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_TaintConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfig_TaintConfigRef {
        ContainerNodePool_NodeConfig_TaintConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_TaintConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `architecture_taint_behavior` after provisioning.\nArchitecture taint behavior. Controls, how we apply taints based on the node architecture."]
    pub fn architecture_taint_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture_taint_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_WindowsNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    osversion: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_NodeConfig_WindowsNodeConfig {
    #[doc =
        "Set the field `osversion`.\nThe OS Version of the windows nodepool.Values are OS_VERSION_UNSPECIFIED,OS_VERSION_LTSC2019 and OS_VERSION_LTSC2022"]
    pub fn set_osversion(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.osversion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_WindowsNodeConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_WindowsNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_WindowsNodeConfig {}

impl BuildContainerNodePool_NodeConfig_WindowsNodeConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_WindowsNodeConfig {
        ContainerNodePool_NodeConfig_WindowsNodeConfig { osversion: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_WindowsNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_WindowsNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_WindowsNodeConfigRef {
        ContainerNodePool_NodeConfig_WindowsNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_WindowsNodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `osversion` after provisioning.\nThe OS Version of the windows nodepool.Values are OS_VERSION_UNSPECIFIED,OS_VERSION_LTSC2019 and OS_VERSION_LTSC2022"]
    pub fn osversion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.osversion", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig_WorkloadMetadataConfig {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_NodeConfig_WorkloadMetadataConfig { }

impl terrars::ToListMappable for ContainerNodePool_NodeConfig_WorkloadMetadataConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig_WorkloadMetadataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig_WorkloadMetadataConfig {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_NodeConfig_WorkloadMetadataConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig_WorkloadMetadataConfig {
        ContainerNodePool_NodeConfig_WorkloadMetadataConfig { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef {
        ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef {
    #[allow(dead_code)]
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
struct ContainerNodePool_NodeConfigDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_AdvancedMachineFeatures>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_BootDisk>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ConfidentialNodes>>,
    containerd_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ContainerdConfig>>,
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    fast_socket: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_FastSocket>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_GcfsConfig>>,
    guest_accelerator: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_GuestAccelerator>>,
    gvnic: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_Gvnic>>,
    kubelet_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_KubeletConfig>>,
    linux_node_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LinuxNodeConfig>>,
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig>>,
    node_image_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_NodeImageConfig>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ReservationAffinity>>,
    sandbox_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_SandboxConfig>>,
    secondary_boot_disks: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_SecondaryBootDisks>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_ShieldedInstanceConfig>>,
    sole_tenant_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_SoleTenantConfig>>,
    taint: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_Taint>>,
    taint_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_TaintConfig>>,
    windows_node_config: std::option::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_WindowsNodeConfig>>,
    workload_metadata_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_NodeConfig_WorkloadMetadataConfig>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_storage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flex_start: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpudirect_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_run_duration: std::option::Option<terrars::PrimField<std::string::String>>,
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
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_pools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ConfidentialNodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containerd_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ContainerdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_FastSocket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_GcfsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_GuestAccelerator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_Gvnic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_KubeletConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LinuxNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_image_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_NodeImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_SandboxConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_boot_disks: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_SecondaryBootDisks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_SoleTenantConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_Taint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_TaintConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_node_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_WindowsNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std::option::Option<std::vec::Vec<ContainerNodePool_NodeConfig_WorkloadMetadataConfig>>,
    dynamic: ContainerNodePool_NodeConfigDynamic,
}

impl ContainerNodePool_NodeConfig {
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
        "Set the field `enable_confidential_storage`.\nIf enabled boot disks are configured with confidential mode."]
    pub fn set_enable_confidential_storage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flex_start`.\nEnables Flex Start provisioning model for the node pool"]
    pub fn set_flex_start(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flex_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpudirect_strategy`.\nThe type of GPUDirect strategy to enable on the node."]
    pub fn set_gpudirect_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpudirect_strategy = std::option::Option::Some(v.into());
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
        "Set the field `local_ssd_encryption_mode`.\nLocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node."]
    pub fn set_local_ssd_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_encryption_mode = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `max_run_duration`.\nThe runtime of each node in the node pool in seconds, terminated by 's'. Example: \"3600s\"."]
    pub fn set_max_run_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_run_duration = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_pools`.\nThe list of Storage Pools where boot disks are provisioned."]
    pub fn set_storage_pools(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.storage_pools = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_AdvancedMachineFeatures>>,
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

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_BootDisk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boot_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ConfidentialNodes>>,
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

    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ContainerdConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containerd_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containerd_config = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_FastSocket>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_GcfsConfig>>,
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

    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_GuestAccelerator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guest_accelerator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_Gvnic>>,
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

    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_KubeletConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LinuxNodeConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfig>>,
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

    #[doc = "Set the field `node_image_config`.\n"]
    pub fn set_node_image_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_NodeImageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_image_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ReservationAffinity>>,
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

    #[doc = "Set the field `sandbox_config`.\n"]
    pub fn set_sandbox_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_SandboxConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sandbox_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sandbox_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_boot_disks`.\n"]
    pub fn set_secondary_boot_disks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_SecondaryBootDisks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secondary_boot_disks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_boot_disks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_ShieldedInstanceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_SoleTenantConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_Taint>>,
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

    #[doc = "Set the field `taint_config`.\n"]
    pub fn set_taint_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_TaintConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `windows_node_config`.\n"]
    pub fn set_windows_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_WindowsNodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.windows_node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_NodeConfig_WorkloadMetadataConfig>>,
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

impl terrars::ToListMappable for ContainerNodePool_NodeConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeConfig {}

impl BuildContainerNodePool_NodeConfig {
    pub fn build(self) -> ContainerNodePool_NodeConfig {
        ContainerNodePool_NodeConfig {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            enable_confidential_storage: core::default::Default::default(),
            flex_start: core::default::Default::default(),
            gpudirect_strategy: core::default::Default::default(),
            image_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            local_ssd_encryption_mode: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            spot: core::default::Default::default(),
            storage_pools: core::default::Default::default(),
            tags: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            node_image_config: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            sandbox_config: core::default::Default::default(),
            secondary_boot_disks: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            taint: core::default::Default::default(),
            taint_config: core::default::Default::default(),
            windows_node_config: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeConfigRef {
        ContainerNodePool_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeConfigRef {
    #[allow(dead_code)]
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
    pub fn effective_taints(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_EffectiveTaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_storage` after provisioning.\nIf enabled boot disks are configured with confidential mode."]
    pub fn enable_confidential_storage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flex_start` after provisioning.\nEnables Flex Start provisioning model for the node pool"]
    pub fn flex_start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flex_start", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gpudirect_strategy` after provisioning.\nThe type of GPUDirect strategy to enable on the node."]
    pub fn gpudirect_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpudirect_strategy", self.base))
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
        "Get a reference to the value of field `local_ssd_encryption_mode` after provisioning.\nLocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node."]
    pub fn local_ssd_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_encryption_mode", self.base))
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
        "Get a reference to the value of field `max_run_duration` after provisioning.\nThe runtime of each node in the node pool in seconds, terminated by 's'. Example: \"3600s\"."]
    pub fn max_run_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_run_duration", self.base))
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
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
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
        "Get a reference to the value of field `storage_pools` after provisioning.\nThe list of Storage Pools where boot disks are provisioned."]
    pub fn storage_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all nodes."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ContainerdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containerd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_EphemeralStorageLocalSsdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_FastSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_GcfsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_GuestAcceleratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_GvnicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_KubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_LinuxNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<ContainerNodePool_NodeConfig_LocalNvmeSsdBlockConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_image_config` after provisioning.\n"]
    pub fn node_image_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_NodeImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_image_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_SandboxConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sandbox_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_SecondaryBootDisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_boot_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_SoleTenantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_TaintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `taint_config` after provisioning.\n"]
    pub fn taint_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_TaintConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_WindowsNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> terrars::ListRef<ContainerNodePool_NodeConfig_WorkloadMetadataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_NodeDrainConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_termination_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pdb_timeout_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_pdb_during_node_pool_deletion: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerNodePool_NodeDrainConfig {
    #[doc =
        "Set the field `grace_termination_duration`.\nThe duration of the grace termination period for node drain."]
    pub fn set_grace_termination_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.grace_termination_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pdb_timeout_duration`.\nThe duration of the PDB timeout period for node drain."]
    pub fn set_pdb_timeout_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pdb_timeout_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `respect_pdb_during_node_pool_deletion`.\nWhether to respect PodDisruptionBudget policy during node pool deletion."]
    pub fn set_respect_pdb_during_node_pool_deletion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.respect_pdb_during_node_pool_deletion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_NodeDrainConfig {
    type O = terrars::BlockAssignable<ContainerNodePool_NodeDrainConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_NodeDrainConfig {}

impl BuildContainerNodePool_NodeDrainConfig {
    pub fn build(self) -> ContainerNodePool_NodeDrainConfig {
        ContainerNodePool_NodeDrainConfig {
            grace_termination_duration: core::default::Default::default(),
            pdb_timeout_duration: core::default::Default::default(),
            respect_pdb_during_node_pool_deletion: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_NodeDrainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_NodeDrainConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_NodeDrainConfigRef {
        ContainerNodePool_NodeDrainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_NodeDrainConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grace_termination_duration` after provisioning.\nThe duration of the grace termination period for node drain."]
    pub fn grace_termination_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_termination_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pdb_timeout_duration` after provisioning.\nThe duration of the PDB timeout period for node drain."]
    pub fn pdb_timeout_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pdb_timeout_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `respect_pdb_during_node_pool_deletion` after provisioning.\nWhether to respect PodDisruptionBudget policy during node pool deletion."]
    pub fn respect_pdb_during_node_pool_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_pdb_during_node_pool_deletion", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_PlacementPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerNodePool_PlacementPolicy {
    #[doc =
        "Set the field `policy_name`.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn set_policy_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tpu_topology`.\nThe TPU topology like \"2x4\" or \"2x2x2\". https://cloud.google.com/kubernetes-engine/docs/concepts/plan-tpus#topology"]
    pub fn set_tpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tpu_topology = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerNodePool_PlacementPolicy {
    type O = terrars::BlockAssignable<ContainerNodePool_PlacementPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_PlacementPolicy {
    #[doc = "Type defines the type of placement policy"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerNodePool_PlacementPolicy {
    pub fn build(self) -> ContainerNodePool_PlacementPolicy {
        ContainerNodePool_PlacementPolicy {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_PlacementPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_PlacementPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_PlacementPolicyRef {
        ContainerNodePool_PlacementPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_PlacementPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy_name` after provisioning.\nIf set, refers to the name of a custom resource policy supplied by the user. The resource policy must be in the same project and region as the node pool. If not found, InvalidArgument error is returned."]
    pub fn policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tpu_topology` after provisioning.\nThe TPU topology like \"2x4\" or \"2x2x2\". https://cloud.google.com/kubernetes-engine/docs/concepts/plan-tpus#topology"]
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
pub struct ContainerNodePool_QueuedProvisioning {
    enabled: terrars::PrimField<bool>,
}

impl ContainerNodePool_QueuedProvisioning { }

impl terrars::ToListMappable for ContainerNodePool_QueuedProvisioning {
    type O = terrars::BlockAssignable<ContainerNodePool_QueuedProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_QueuedProvisioning {
    #[doc = "Whether nodes in this node pool are obtainable solely through the ProvisioningRequest API"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerNodePool_QueuedProvisioning {
    pub fn build(self) -> ContainerNodePool_QueuedProvisioning {
        ContainerNodePool_QueuedProvisioning { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_QueuedProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_QueuedProvisioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_QueuedProvisioningRef {
        ContainerNodePool_QueuedProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_QueuedProvisioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether nodes in this node pool are obtainable solely through the ProvisioningRequest API"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNodePool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_Timeouts {
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

impl terrars::ToListMappable for ContainerNodePool_Timeouts {
    type O = terrars::BlockAssignable<ContainerNodePool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_Timeouts {}

impl BuildContainerNodePool_Timeouts {
    pub fn build(self) -> ContainerNodePool_Timeouts {
        ContainerNodePool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_TimeoutsRef {
        ContainerNodePool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_TimeoutsRef {
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
pub struct ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
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

impl terrars::ToListMappable for ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    type O = terrars::BlockAssignable<ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {}

impl BuildContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    pub fn build(self) -> ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
        ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
        ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    #[allow(dead_code)]
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
struct ContainerNodePool_UpgradeSettings_BlueGreenSettingsDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_UpgradeSettings_BlueGreenSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<std::vec::Vec<ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>>,
    dynamic: ContainerNodePool_UpgradeSettings_BlueGreenSettingsDynamic,
}

impl ContainerNodePool_UpgradeSettings_BlueGreenSettings {
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
                        ::BlockAssignable<ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>,
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

impl terrars::ToListMappable for ContainerNodePool_UpgradeSettings_BlueGreenSettings {
    type O = terrars::BlockAssignable<ContainerNodePool_UpgradeSettings_BlueGreenSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_UpgradeSettings_BlueGreenSettings {}

impl BuildContainerNodePool_UpgradeSettings_BlueGreenSettings {
    pub fn build(self) -> ContainerNodePool_UpgradeSettings_BlueGreenSettings {
        ContainerNodePool_UpgradeSettings_BlueGreenSettings {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef {
        ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ContainerNodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerNodePool_UpgradeSettingsDynamic {
    blue_green_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerNodePool_UpgradeSettings_BlueGreenSettings>>,
}

#[derive(Serialize)]
pub struct ContainerNodePool_UpgradeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std::option::Option<std::vec::Vec<ContainerNodePool_UpgradeSettings_BlueGreenSettings>>,
    dynamic: ContainerNodePool_UpgradeSettingsDynamic,
}

impl ContainerNodePool_UpgradeSettings {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNodePool_UpgradeSettings_BlueGreenSettings>>,
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

impl terrars::ToListMappable for ContainerNodePool_UpgradeSettings {
    type O = terrars::BlockAssignable<ContainerNodePool_UpgradeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNodePool_UpgradeSettings {}

impl BuildContainerNodePool_UpgradeSettings {
    pub fn build(self) -> ContainerNodePool_UpgradeSettings {
        ContainerNodePool_UpgradeSettings {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerNodePool_UpgradeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNodePool_UpgradeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNodePool_UpgradeSettingsRef {
        ContainerNodePool_UpgradeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNodePool_UpgradeSettingsRef {
    #[allow(dead_code)]
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
    pub fn blue_green_settings(&self) -> terrars::ListRef<ContainerNodePool_UpgradeSettings_BlueGreenSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}
