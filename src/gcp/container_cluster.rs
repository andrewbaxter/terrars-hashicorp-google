use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerClusterData {
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
    allow_net_admin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datapath_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_autopilot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_intranode_visibility: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_kubernetes_alpha: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_l4_ilb_subsetting: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_legacy_abac: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_shielded_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_tpu: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_master_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_service: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ipv6_google_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_default_node_pool: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticator_groups_config: std::option::Option<std::vec::Vec<ContainerClusterAuthenticatorGroupsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<ContainerClusterBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_autoscaling: std::option::Option<std::vec::Vec<ContainerClusterClusterAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerClusterConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_management_config: std::option::Option<std::vec::Vec<ContainerClusterCostManagementConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_encryption: std::option::Option<std::vec::Vec<ContainerClusterDatabaseEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_snat_status: std::option::Option<std::vec::Vec<ContainerClusterDefaultSnatStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_config: std::option::Option<std::vec::Vec<ContainerClusterDnsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_beta_apis: std::option::Option<std::vec::Vec<ContainerClusterEnableK8sBetaApisEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<ContainerClusterFleetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_api_config: std::option::Option<std::vec::Vec<ContainerClusterGatewayApiConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_service_config: std::option::Option<std::vec::Vec<ContainerClusterIdentityServiceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation_policy: std::option::Option<std::vec::Vec<ContainerClusterIpAllocationPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<ContainerClusterLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<ContainerClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_auth: std::option::Option<std::vec::Vec<ContainerClusterMasterAuthEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_authorized_networks_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterMasterAuthorizedNetworksConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_certificates: std::option::Option<std::vec::Vec<ContainerClusterMeshCertificatesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_config: std::option::Option<std::vec::Vec<ContainerClusterMonitoringConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_policy: std::option::Option<std::vec::Vec<ContainerClusterNetworkPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool: std::option::Option<std::vec::Vec<ContainerClusterNodePoolEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_auto_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolAutoConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_defaults: std::option::Option<std::vec::Vec<ContainerClusterNodePoolDefaultsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<ContainerClusterNotificationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_cluster_config: std::option::Option<std::vec::Vec<ContainerClusterPrivateClusterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_channel: std::option::Option<std::vec::Vec<ContainerClusterReleaseChannelEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_usage_export_config: std::option::Option<std::vec::Vec<ContainerClusterResourceUsageExportConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_posture_config: std::option::Option<std::vec::Vec<ContainerClusterSecurityPostureConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_external_ips_config: std::option::Option<std::vec::Vec<ContainerClusterServiceExternalIpsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertical_pod_autoscaling: std::option::Option<std::vec::Vec<ContainerClusterVerticalPodAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identity_config: std::option::Option<std::vec::Vec<ContainerClusterWorkloadIdentityConfigEl>>,
    dynamic: ContainerClusterDynamic,
}

struct ContainerCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerClusterData>,
}

#[derive(Clone)]
pub struct ContainerCluster(std::rc::Rc<ContainerCluster_>);

impl ContainerCluster {
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

    #[doc = "Set the field `allow_net_admin`.\nEnable NET_ADMIN for this cluster."]
    pub fn set_allow_net_admin(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_net_admin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_ipv4_cidr`.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn set_cluster_ipv4_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_ipv4_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `datapath_provider`.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn set_datapath_provider(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().datapath_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_max_pods_per_node`.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn set_default_max_pods_per_node(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether or not to allow Terraform to destroy the instance. Defaults to true. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the cluster will fail."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n Description of the cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_autopilot`.\nEnable Autopilot for this cluster."]
    pub fn set_enable_autopilot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_autopilot = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_intranode_visibility`.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn set_enable_intranode_visibility(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_intranode_visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_kubernetes_alpha`.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn set_enable_kubernetes_alpha(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_kubernetes_alpha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_l4_ilb_subsetting`.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn set_enable_l4_ilb_subsetting(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_l4_ilb_subsetting = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_legacy_abac`.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn set_enable_legacy_abac(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_legacy_abac = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_shielded_nodes`.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn set_enable_shielded_nodes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_shielded_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_tpu`.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn set_enable_tpu(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_tpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_node_count`.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn set_initial_node_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logging_service`.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn set_logging_service(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logging_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_master_version`.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn set_min_master_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().min_master_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitoring_service`.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn set_monitoring_service(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().monitoring_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `networking_mode`.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn set_networking_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().networking_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_locations`.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn set_node_locations(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().node_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_version`.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn set_node_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ipv6_google_access`.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn set_private_ipv6_google_access(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().private_ipv6_google_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remove_default_node_pool`.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn set_remove_default_node_pool(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_default_node_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_labels`.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster."]
    pub fn set_resource_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn set_subnetwork(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `addons_config`.\n"]
    pub fn set_addons_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addons_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addons_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authenticator_groups_config`.\n"]
    pub fn set_authenticator_groups_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAuthenticatorGroupsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authenticator_groups_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authenticator_groups_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterBinaryAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binary_authorization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binary_authorization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cluster_autoscaling`.\n"]
    pub fn set_cluster_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterClusterAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterConfidentialNodesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confidential_nodes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confidential_nodes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_management_config`.\n"]
    pub fn set_cost_management_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterCostManagementConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cost_management_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cost_management_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_encryption`.\n"]
    pub fn set_database_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterDatabaseEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_snat_status`.\n"]
    pub fn set_default_snat_status(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterDefaultSnatStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_snat_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_snat_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_config`.\n"]
    pub fn set_dns_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterDnsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enable_k8s_beta_apis`.\n"]
    pub fn set_enable_k8s_beta_apis(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterEnableK8sBetaApisEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enable_k8s_beta_apis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enable_k8s_beta_apis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterFleetEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gateway_api_config`.\n"]
    pub fn set_gateway_api_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterGatewayApiConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gateway_api_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gateway_api_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_service_config`.\n"]
    pub fn set_identity_service_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterIdentityServiceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_service_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_service_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_allocation_policy`.\n"]
    pub fn set_ip_allocation_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterIpAllocationPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ip_allocation_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ip_allocation_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMaintenancePolicyEl>>,
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

    #[doc = "Set the field `master_auth`.\n"]
    pub fn set_master_auth(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMasterAuthEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMasterAuthorizedNetworksConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_authorized_networks_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_authorized_networks_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mesh_certificates`.\n"]
    pub fn set_mesh_certificates(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMeshCertificatesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mesh_certificates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mesh_certificates = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_config`.\n"]
    pub fn set_monitoring_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMonitoringConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_policy`.\n"]
    pub fn set_network_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNetworkPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigEl>>,
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

    #[doc = "Set the field `node_pool`.\n"]
    pub fn set_node_pool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_pool_auto_config`.\n"]
    pub fn set_node_pool_auto_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolAutoConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool_auto_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool_auto_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_pool_defaults`.\n"]
    pub fn set_node_pool_defaults(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolDefaultsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool_defaults = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_cluster_config`.\n"]
    pub fn set_private_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterPrivateClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `release_channel`.\n"]
    pub fn set_release_channel(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterReleaseChannelEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().release_channel = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.release_channel = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_usage_export_config`.\n"]
    pub fn set_resource_usage_export_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterResourceUsageExportConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_usage_export_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_usage_export_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterSecurityPostureConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_posture_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_posture_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_external_ips_config`.\n"]
    pub fn set_service_external_ips_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterServiceExternalIpsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_external_ips_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_external_ips_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vertical_pod_autoscaling`.\n"]
    pub fn set_vertical_pod_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterVerticalPodAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vertical_pod_autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vertical_pod_autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workload_identity_config`.\n"]
    pub fn set_workload_identity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterWorkloadIdentityConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload_identity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload_identity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datapath_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Defaults to true. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the cluster will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_intranode_visibility", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_kubernetes_alpha", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_l4_ilb_subsetting", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_abac", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_shielded_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_tpu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_default_node_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster."]
    pub fn resource_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\n"]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<ContainerClusterAuthenticatorGroupsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\n"]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<ContainerClusterClusterAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerClusterConfidentialNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\n"]
    pub fn cost_management_config(&self) -> terrars::ListRef<ContainerClusterCostManagementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerClusterDatabaseEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\n"]
    pub fn default_snat_status(&self) -> terrars::ListRef<ContainerClusterDefaultSnatStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ContainerClusterDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\n"]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<ContainerClusterEnableK8sBetaApisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\n"]
    pub fn gateway_api_config(&self) -> terrars::ListRef<ContainerClusterGatewayApiConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\n"]
    pub fn identity_service_config(&self) -> terrars::ListRef<ContainerClusterIdentityServiceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<ContainerClusterIpAllocationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerClusterLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\n"]
    pub fn master_auth(&self) -> terrars::ListRef<ContainerClusterMasterAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterMasterAuthorizedNetworksConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\n"]
    pub fn mesh_certificates(&self) -> terrars::ListRef<ContainerClusterMeshCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerClusterMonitoringConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\n"]
    pub fn network_policy(&self) -> terrars::ListRef<ContainerClusterNetworkPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<ContainerClusterNodePoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\n"]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<ContainerClusterNodePoolAutoConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\n"]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<ContainerClusterNodePoolDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<ContainerClusterNotificationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(&self) -> terrars::ListRef<ContainerClusterPrivateClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\n"]
    pub fn release_channel(&self) -> terrars::ListRef<ContainerClusterReleaseChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\n"]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<ContainerClusterResourceUsageExportConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerClusterSecurityPostureConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\n"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<ContainerClusterServiceExternalIpsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerClusterTimeoutsElRef {
        ContainerClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\n"]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<ContainerClusterVerticalPodAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerClusterWorkloadIdentityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerCluster { }

impl terrars::ToListMappable for ContainerCluster {
    type O = terrars::ListRef<ContainerClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerCluster {
    pub tf_id: std::string::String,
    #[doc = "The name of the cluster, unique within the project and location."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerCluster {
        let out = ContainerCluster(std::rc::Rc::new(ContainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_net_admin: core::default::Default::default(),
                cluster_ipv4_cidr: core::default::Default::default(),
                datapath_provider: core::default::Default::default(),
                default_max_pods_per_node: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_autopilot: core::default::Default::default(),
                enable_intranode_visibility: core::default::Default::default(),
                enable_kubernetes_alpha: core::default::Default::default(),
                enable_l4_ilb_subsetting: core::default::Default::default(),
                enable_legacy_abac: core::default::Default::default(),
                enable_shielded_nodes: core::default::Default::default(),
                enable_tpu: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_node_count: core::default::Default::default(),
                location: core::default::Default::default(),
                logging_service: core::default::Default::default(),
                min_master_version: core::default::Default::default(),
                monitoring_service: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                networking_mode: core::default::Default::default(),
                node_locations: core::default::Default::default(),
                node_version: core::default::Default::default(),
                private_ipv6_google_access: core::default::Default::default(),
                project: core::default::Default::default(),
                remove_default_node_pool: core::default::Default::default(),
                resource_labels: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                addons_config: core::default::Default::default(),
                authenticator_groups_config: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                cluster_autoscaling: core::default::Default::default(),
                confidential_nodes: core::default::Default::default(),
                cost_management_config: core::default::Default::default(),
                database_encryption: core::default::Default::default(),
                default_snat_status: core::default::Default::default(),
                dns_config: core::default::Default::default(),
                enable_k8s_beta_apis: core::default::Default::default(),
                fleet: core::default::Default::default(),
                gateway_api_config: core::default::Default::default(),
                identity_service_config: core::default::Default::default(),
                ip_allocation_policy: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                master_auth: core::default::Default::default(),
                master_authorized_networks_config: core::default::Default::default(),
                mesh_certificates: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                network_policy: core::default::Default::default(),
                node_config: core::default::Default::default(),
                node_pool: core::default::Default::default(),
                node_pool_auto_config: core::default::Default::default(),
                node_pool_defaults: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                private_cluster_config: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                resource_usage_export_config: core::default::Default::default(),
                security_posture_config: core::default::Default::default(),
                service_external_ips_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vertical_pod_autoscaling: core::default::Default::default(),
                workload_identity_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datapath_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Defaults to true. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the cluster will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_intranode_visibility", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_kubernetes_alpha", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_l4_ilb_subsetting", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_abac", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_shielded_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_tpu", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_default_node_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster."]
    pub fn resource_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\n"]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<ContainerClusterAuthenticatorGroupsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\n"]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<ContainerClusterClusterAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerClusterConfidentialNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\n"]
    pub fn cost_management_config(&self) -> terrars::ListRef<ContainerClusterCostManagementConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerClusterDatabaseEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\n"]
    pub fn default_snat_status(&self) -> terrars::ListRef<ContainerClusterDefaultSnatStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ContainerClusterDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\n"]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<ContainerClusterEnableK8sBetaApisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\n"]
    pub fn gateway_api_config(&self) -> terrars::ListRef<ContainerClusterGatewayApiConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\n"]
    pub fn identity_service_config(&self) -> terrars::ListRef<ContainerClusterIdentityServiceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<ContainerClusterIpAllocationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerClusterLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\n"]
    pub fn master_auth(&self) -> terrars::ListRef<ContainerClusterMasterAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterMasterAuthorizedNetworksConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\n"]
    pub fn mesh_certificates(&self) -> terrars::ListRef<ContainerClusterMeshCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerClusterMonitoringConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\n"]
    pub fn network_policy(&self) -> terrars::ListRef<ContainerClusterNetworkPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<ContainerClusterNodePoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\n"]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<ContainerClusterNodePoolAutoConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\n"]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<ContainerClusterNodePoolDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<ContainerClusterNotificationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(&self) -> terrars::ListRef<ContainerClusterPrivateClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\n"]
    pub fn release_channel(&self) -> terrars::ListRef<ContainerClusterReleaseChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\n"]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<ContainerClusterResourceUsageExportConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerClusterSecurityPostureConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\n"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<ContainerClusterServiceExternalIpsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerClusterTimeoutsElRef {
        ContainerClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\n"]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<ContainerClusterVerticalPodAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerClusterWorkloadIdentityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElCloudrunConfigEl {
    disabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterAddonsConfigElCloudrunConfigEl {
    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.load_balancer_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterAddonsConfigElCloudrunConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElCloudrunConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElCloudrunConfigEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElCloudrunConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElCloudrunConfigEl {
        ContainerClusterAddonsConfigElCloudrunConfigEl {
            disabled: self.disabled,
            load_balancer_type: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterAddonsConfigElCloudrunConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElCloudrunConfigElRef {
        ContainerClusterAddonsConfigElCloudrunConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElConfigConnectorConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElConfigConnectorConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElConfigConnectorConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElConfigConnectorConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElConfigConnectorConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElConfigConnectorConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElConfigConnectorConfigEl {
        ContainerClusterAddonsConfigElConfigConnectorConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElConfigConnectorConfigElRef {
        ContainerClusterAddonsConfigElConfigConnectorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElDnsCacheConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElDnsCacheConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElDnsCacheConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElDnsCacheConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElDnsCacheConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElDnsCacheConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElDnsCacheConfigEl {
        ContainerClusterAddonsConfigElDnsCacheConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElDnsCacheConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElDnsCacheConfigElRef {
        ContainerClusterAddonsConfigElDnsCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
        ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
        ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
        ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
        ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
        ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
        ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElGkeBackupAgentConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElGkeBackupAgentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
        ContainerClusterAddonsConfigElGkeBackupAgentConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
        ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
        ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
        ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElHttpLoadBalancingEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElHttpLoadBalancingEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElHttpLoadBalancingEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElHttpLoadBalancingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElHttpLoadBalancingEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElHttpLoadBalancingEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElHttpLoadBalancingEl {
        ContainerClusterAddonsConfigElHttpLoadBalancingEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElHttpLoadBalancingElRef {
        ContainerClusterAddonsConfigElHttpLoadBalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterAddonsConfigElNetworkPolicyConfigEl { }

impl terrars::ToListMappable for ContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigElNetworkPolicyConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigElNetworkPolicyConfigEl {
        ContainerClusterAddonsConfigElNetworkPolicyConfigEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
        ContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterAddonsConfigElDynamic {
    cloudrun_config: std::option::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElCloudrunConfigEl>>,
    config_connector_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    dns_cache_config: std::option::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    gce_persistent_disk_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>>,
    gcp_filestore_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    gcs_fuse_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    gke_backup_agent_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    horizontal_pod_autoscaling: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    http_load_balancing: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    network_policy_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterAddonsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudrun_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElCloudrunConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_connector_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_cache_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_persistent_disk_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_filestore_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_fuse_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_backup_agent_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaling: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_load_balancing: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_policy_config: std::option::Option<std::vec::Vec<ContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
    dynamic: ContainerClusterAddonsConfigElDynamic,
}

impl ContainerClusterAddonsConfigEl {
    #[doc = "Set the field `cloudrun_config`.\n"]
    pub fn set_cloudrun_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElCloudrunConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudrun_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudrun_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_connector_config`.\n"]
    pub fn set_config_connector_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config_connector_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config_connector_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_cache_config`.\n"]
    pub fn set_dns_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_cache_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gce_persistent_disk_csi_driver_config`.\n"]
    pub fn set_gce_persistent_disk_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_persistent_disk_csi_driver_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_persistent_disk_csi_driver_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcp_filestore_csi_driver_config`.\n"]
    pub fn set_gcp_filestore_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcp_filestore_csi_driver_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcp_filestore_csi_driver_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_fuse_csi_driver_config`.\n"]
    pub fn set_gcs_fuse_csi_driver_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_fuse_csi_driver_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_fuse_csi_driver_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gke_backup_agent_config`.\n"]
    pub fn set_gke_backup_agent_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke_backup_agent_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_backup_agent_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `horizontal_pod_autoscaling`.\n"]
    pub fn set_horizontal_pod_autoscaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.horizontal_pod_autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.horizontal_pod_autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_load_balancing`.\n"]
    pub fn set_http_load_balancing(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_load_balancing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_load_balancing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_policy_config`.\n"]
    pub fn set_network_policy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterAddonsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAddonsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAddonsConfigEl {}

impl BuildContainerClusterAddonsConfigEl {
    pub fn build(self) -> ContainerClusterAddonsConfigEl {
        ContainerClusterAddonsConfigEl {
            cloudrun_config: core::default::Default::default(),
            config_connector_config: core::default::Default::default(),
            dns_cache_config: core::default::Default::default(),
            gce_persistent_disk_csi_driver_config: core::default::Default::default(),
            gcp_filestore_csi_driver_config: core::default::Default::default(),
            gcs_fuse_csi_driver_config: core::default::Default::default(),
            gke_backup_agent_config: core::default::Default::default(),
            horizontal_pod_autoscaling: core::default::Default::default(),
            http_load_balancing: core::default::Default::default(),
            network_policy_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterAddonsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAddonsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterAddonsConfigElRef {
        ContainerClusterAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAddonsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudrun_config` after provisioning.\n"]
    pub fn cloudrun_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElCloudrunConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudrun_config", self.base))
    }

    #[doc = "Get a reference to the value of field `config_connector_config` after provisioning.\n"]
    pub fn config_connector_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElConfigConnectorConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_connector_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_cache_config` after provisioning.\n"]
    pub fn dns_cache_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElDnsCacheConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_persistent_disk_csi_driver_config` after provisioning.\n"]
    pub fn gce_persistent_disk_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_persistent_disk_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_filestore_csi_driver_config` after provisioning.\n"]
    pub fn gcp_filestore_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_filestore_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_fuse_csi_driver_config` after provisioning.\n"]
    pub fn gcs_fuse_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_fuse_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_backup_agent_config` after provisioning.\n"]
    pub fn gke_backup_agent_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElGkeBackupAgentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_backup_agent_config", self.base))
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaling` after provisioning.\n"]
    pub fn horizontal_pod_autoscaling(
        &self,
    ) -> terrars::ListRef<ContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `http_load_balancing` after provisioning.\n"]
    pub fn http_load_balancing(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElHttpLoadBalancingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_load_balancing", self.base))
    }

    #[doc = "Get a reference to the value of field `network_policy_config` after provisioning.\n"]
    pub fn network_policy_config(&self) -> terrars::ListRef<ContainerClusterAddonsConfigElNetworkPolicyConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterAuthenticatorGroupsConfigEl {
    security_group: terrars::PrimField<std::string::String>,
}

impl ContainerClusterAuthenticatorGroupsConfigEl { }

impl terrars::ToListMappable for ContainerClusterAuthenticatorGroupsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterAuthenticatorGroupsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterAuthenticatorGroupsConfigEl {
    #[doc =
        "The name of the RBAC security group for use with Google security groups in Kubernetes RBAC. Group name must be in format gke-security-groups@yourdomain.com."]
    pub security_group: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterAuthenticatorGroupsConfigEl {
    pub fn build(self) -> ContainerClusterAuthenticatorGroupsConfigEl {
        ContainerClusterAuthenticatorGroupsConfigEl { security_group: self.security_group }
    }
}

pub struct ContainerClusterAuthenticatorGroupsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterAuthenticatorGroupsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterAuthenticatorGroupsConfigElRef {
        ContainerClusterAuthenticatorGroupsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterAuthenticatorGroupsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `security_group` after provisioning.\nThe name of the RBAC security group for use with Google security groups in Kubernetes RBAC. Group name must be in format gke-security-groups@yourdomain.com."]
    pub fn security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterBinaryAuthorizationEl {
    #[doc = "Set the field `enabled`.\nEnable Binary Authorization for this cluster."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_mode`.\nMode of operation for Binary Authorization policy evaluation."]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<ContainerClusterBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterBinaryAuthorizationEl {}

impl BuildContainerClusterBinaryAuthorizationEl {
    pub fn build(self) -> ContainerClusterBinaryAuthorizationEl {
        ContainerClusterBinaryAuthorizationEl {
            enabled: core::default::Default::default(),
            evaluation_mode: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterBinaryAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterBinaryAuthorizationElRef {
        ContainerClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable Binary Authorization for this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for Binary Authorization policy evaluation."]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    #[doc = "Set the field `auto_upgrade_start_time`.\n"]
    pub fn set_auto_upgrade_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auto_upgrade_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
            auto_upgrade_start_time: core::default::Default::default(),
            description: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_upgrade_start_time` after provisioning.\n"]
    pub fn auto_upgrade_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    #[doc =
        "Set the field `auto_repair`.\nSpecifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered."]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_upgrade`.\nSpecifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes."]
    pub fn set_auto_upgrade(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    type O = terrars::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_repair` after provisioning.\nSpecifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered."]
    pub fn auto_repair(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_upgrade` after provisioning.\nSpecifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes."]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upgrade_options` after provisioning.\nSpecifies the Auto Upgrade knobs for the node pool."]
    pub fn upgrade_options(
        &self,
    ) -> terrars
    ::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
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

impl terrars
::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    type O =
        terrars
        ::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
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
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc = "Set the field `batch_node_count`.\nNumber of blue nodes to drain in a batch."]
    pub fn set_batch_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `batch_percentage`.\nPercentage of the bool pool nodes to drain in a batch. The range of this field should be (0.0, 1.0]."]
    pub fn set_batch_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `batch_soak_duration`.\nSoak time after each batch gets drained.\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_batch_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.batch_soak_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O =
        terrars
        ::BlockAssignable<
            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(
        self,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_node_count` after provisioning.\nNumber of blue nodes to drain in a batch."]
    pub fn batch_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `batch_percentage` after provisioning.\nPercentage of the bool pool nodes to drain in a batch. The range of this field should be (0.0, 1.0]."]
    pub fn batch_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `batch_soak_duration` after provisioning.\nSoak time after each batch gets drained.\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn batch_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
        >,
    >,
    dynamic: ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElDynamic,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    #[doc =
        "Set the field `node_pool_soak_duration`.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up.\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
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
                            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
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

impl terrars
::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(
        self,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\nTime needed after draining entire blue pool. After this period, blue pool will be cleaned up.\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn node_pool_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> terrars
    ::ListRef<
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElDynamic {
    blue_green_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl>,
    >,
    dynamic: ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElDynamic,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    #[doc =
        "Set the field `max_surge`.\nThe maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process."]
    pub fn set_max_surge(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable`.\nThe maximum number of nodes that can be simultaneously unavailable during the upgrade process."]
    pub fn set_max_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\nUpdate strategy of the node pool."]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
                        >,
                    >,
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

impl terrars::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    type O =
        terrars::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_surge` after provisioning.\nThe maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process."]
    pub fn max_surge(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable` after provisioning.\nThe maximum number of nodes that can be simultaneously unavailable during the upgrade process."]
    pub fn max_unavailable(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\nUpdate strategy of the node pool."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(
        &self,
    ) -> terrars
    ::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElDynamic {
    management: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>>,
    shielded_instance_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl>,
    >,
    upgrade_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>>,
    dynamic: ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElDynamic,
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    #[doc =
        "Set the field `boot_disk_kms_key`.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn set_boot_disk_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size`.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_disk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nType of the disk attached to each node."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_type`.\nThe default image type used by NAP once a new node pool is being created."]
    pub fn set_image_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_scopes`.\nScopes that are used by NAP when creating node pools."]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl,
                        >,
                    >,
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

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upgrade_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upgrade_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    type O = terrars::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {}

impl BuildContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            image_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            management: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
        ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\nThe Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool."]
    pub fn boot_disk_kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size` after provisioning.\nSize of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk attached to each node."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_type` after provisioning.\nThe default image type used by NAP once a new node pool is being created."]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_scopes` after provisioning.\nScopes that are used by NAP when creating node pools."]
    pub fn oauth_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(
        &self,
    ) -> terrars::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(
        &self,
    ) -> terrars::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    resource_type: terrars::PrimField<std::string::String>,
}

impl ContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[doc = "Set the field `maximum`.\nMaximum amount of the resource in the cluster."]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\nMinimum amount of the resource in the cluster."]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterClusterAutoscalingElResourceLimitsEl {
    type O = terrars::BlockAssignable<ContainerClusterClusterAutoscalingElResourceLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[doc =
        "The type of the resource. For example, cpu and memory. See the guide to using Node Auto-Provisioning for a list of types."]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterClusterAutoscalingElResourceLimitsEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingElResourceLimitsEl {
        ContainerClusterClusterAutoscalingElResourceLimitsEl {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            resource_type: self.resource_type,
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElResourceLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterClusterAutoscalingElResourceLimitsElRef {
        ContainerClusterClusterAutoscalingElResourceLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `maximum` after provisioning.\nMaximum amount of the resource in the cluster."]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum` after provisioning.\nMinimum amount of the resource in the cluster."]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe type of the resource. For example, cpu and memory. See the guide to using Node Auto-Provisioning for a list of types."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterClusterAutoscalingElDynamic {
    auto_provisioning_defaults: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    resource_limits: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterClusterAutoscalingElResourceLimitsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterClusterAutoscalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_provisioning_defaults: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_limits: std::option::Option<std::vec::Vec<ContainerClusterClusterAutoscalingElResourceLimitsEl>>,
    dynamic: ContainerClusterClusterAutoscalingElDynamic,
}

impl ContainerClusterClusterAutoscalingEl {
    #[doc =
        "Set the field `autoscaling_profile`.\nConfiguration options for the Autoscaling profile feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability when deciding to remove nodes from a cluster. Can be BALANCED or OPTIMIZE_UTILIZATION. Defaults to BALANCED."]
    pub fn set_autoscaling_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autoscaling_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nWhether node auto-provisioning is enabled. Resource limits for cpu and memory must be defined to enable node auto-provisioning."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_provisioning_defaults`.\n"]
    pub fn set_auto_provisioning_defaults(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_provisioning_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_provisioning_defaults = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_limits`.\n"]
    pub fn set_resource_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterClusterAutoscalingElResourceLimitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_limits = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterClusterAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerClusterClusterAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterClusterAutoscalingEl {}

impl BuildContainerClusterClusterAutoscalingEl {
    pub fn build(self) -> ContainerClusterClusterAutoscalingEl {
        ContainerClusterClusterAutoscalingEl {
            autoscaling_profile: core::default::Default::default(),
            enabled: core::default::Default::default(),
            auto_provisioning_defaults: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterClusterAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterClusterAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterClusterAutoscalingElRef {
        ContainerClusterClusterAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterClusterAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_profile` after provisioning.\nConfiguration options for the Autoscaling profile feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability when deciding to remove nodes from a cluster. Can be BALANCED or OPTIMIZE_UTILIZATION. Defaults to BALANCED."]
    pub fn autoscaling_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether node auto-provisioning is enabled. Resource limits for cpu and memory must be defined to enable node auto-provisioning."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_provisioning_defaults` after provisioning.\n"]
    pub fn auto_provisioning_defaults(
        &self,
    ) -> terrars::ListRef<ContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(&self) -> terrars::ListRef<ContainerClusterClusterAutoscalingElResourceLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterConfidentialNodesEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterConfidentialNodesEl { }

impl terrars::ToListMappable for ContainerClusterConfidentialNodesEl {
    type O = terrars::BlockAssignable<ContainerClusterConfidentialNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterConfidentialNodesEl {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterConfidentialNodesEl {
    pub fn build(self) -> ContainerClusterConfidentialNodesEl {
        ContainerClusterConfidentialNodesEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterConfidentialNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterConfidentialNodesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterConfidentialNodesElRef {
        ContainerClusterConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterConfidentialNodesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterCostManagementConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterCostManagementConfigEl { }

impl terrars::ToListMappable for ContainerClusterCostManagementConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterCostManagementConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterCostManagementConfigEl {
    #[doc =
        "Whether to enable GKE cost allocation. When you enable GKE cost allocation, the cluster name and namespace of your GKE workloads appear in the labels field of the billing export to BigQuery. Defaults to false."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterCostManagementConfigEl {
    pub fn build(self) -> ContainerClusterCostManagementConfigEl {
        ContainerClusterCostManagementConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterCostManagementConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterCostManagementConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterCostManagementConfigElRef {
        ContainerClusterCostManagementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterCostManagementConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable GKE cost allocation. When you enable GKE cost allocation, the cluster name and namespace of your GKE workloads appear in the labels field of the billing export to BigQuery. Defaults to false."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterDatabaseEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    state: terrars::PrimField<std::string::String>,
}

impl ContainerClusterDatabaseEncryptionEl {
    #[doc = "Set the field `key_name`.\nThe key to use to encrypt/decrypt secrets."]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterDatabaseEncryptionEl {
    type O = terrars::BlockAssignable<ContainerClusterDatabaseEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterDatabaseEncryptionEl {
    #[doc = "ENCRYPTED or DECRYPTED."]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterDatabaseEncryptionEl {
    pub fn build(self) -> ContainerClusterDatabaseEncryptionEl {
        ContainerClusterDatabaseEncryptionEl {
            key_name: core::default::Default::default(),
            state: self.state,
        }
    }
}

pub struct ContainerClusterDatabaseEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterDatabaseEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterDatabaseEncryptionElRef {
        ContainerClusterDatabaseEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterDatabaseEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe key to use to encrypt/decrypt secrets."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nENCRYPTED or DECRYPTED."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterDefaultSnatStatusEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterDefaultSnatStatusEl { }

impl terrars::ToListMappable for ContainerClusterDefaultSnatStatusEl {
    type O = terrars::BlockAssignable<ContainerClusterDefaultSnatStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterDefaultSnatStatusEl {
    #[doc =
        "When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic."]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterDefaultSnatStatusEl {
    pub fn build(self) -> ContainerClusterDefaultSnatStatusEl {
        ContainerClusterDefaultSnatStatusEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterDefaultSnatStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterDefaultSnatStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterDefaultSnatStatusElRef {
        ContainerClusterDefaultSnatStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterDefaultSnatStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhen disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterDnsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterDnsConfigEl {
    #[doc = "Set the field `cluster_dns`.\nWhich in-cluster DNS provider should be used."]
    pub fn set_cluster_dns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_dns_domain`.\nThe suffix used for all cluster service records."]
    pub fn set_cluster_dns_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_dns_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_dns_scope`.\nThe scope of access to cluster DNS records."]
    pub fn set_cluster_dns_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_dns_scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterDnsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterDnsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterDnsConfigEl {}

impl BuildContainerClusterDnsConfigEl {
    pub fn build(self) -> ContainerClusterDnsConfigEl {
        ContainerClusterDnsConfigEl {
            cluster_dns: core::default::Default::default(),
            cluster_dns_domain: core::default::Default::default(),
            cluster_dns_scope: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterDnsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterDnsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterDnsConfigElRef {
        ContainerClusterDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterDnsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_dns` after provisioning.\nWhich in-cluster DNS provider should be used."]
    pub fn cluster_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_dns_domain` after provisioning.\nThe suffix used for all cluster service records."]
    pub fn cluster_dns_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_dns_scope` after provisioning.\nThe scope of access to cluster DNS records."]
    pub fn cluster_dns_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterEnableK8sBetaApisEl {
    enabled_apis: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterEnableK8sBetaApisEl { }

impl terrars::ToListMappable for ContainerClusterEnableK8sBetaApisEl {
    type O = terrars::BlockAssignable<ContainerClusterEnableK8sBetaApisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterEnableK8sBetaApisEl {
    #[doc = "Enabled Kubernetes Beta APIs."]
    pub enabled_apis: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterEnableK8sBetaApisEl {
    pub fn build(self) -> ContainerClusterEnableK8sBetaApisEl {
        ContainerClusterEnableK8sBetaApisEl { enabled_apis: self.enabled_apis }
    }
}

pub struct ContainerClusterEnableK8sBetaApisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterEnableK8sBetaApisElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterEnableK8sBetaApisElRef {
        ContainerClusterEnableK8sBetaApisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterEnableK8sBetaApisElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_apis` after provisioning.\nEnabled Kubernetes Beta APIs."]
    pub fn enabled_apis(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_apis", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterFleetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterFleetEl {
    #[doc = "Set the field `project`.\nThe Fleet host project of the cluster."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterFleetEl {
    type O = terrars::BlockAssignable<ContainerClusterFleetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterFleetEl {}

impl BuildContainerClusterFleetEl {
    pub fn build(self) -> ContainerClusterFleetEl {
        ContainerClusterFleetEl { project: core::default::Default::default() }
    }
}

pub struct ContainerClusterFleetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterFleetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterFleetElRef {
        ContainerClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterFleetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nFull resource name of the registered fleet membership of the cluster."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pre_registered` after provisioning.\nWhether the cluster has been registered via the fleet API."]
    pub fn pre_registered(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_registered", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe Fleet host project of the cluster."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterGatewayApiConfigEl {
    channel: terrars::PrimField<std::string::String>,
}

impl ContainerClusterGatewayApiConfigEl { }

impl terrars::ToListMappable for ContainerClusterGatewayApiConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterGatewayApiConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterGatewayApiConfigEl {
    #[doc = "The Gateway API release channel to use for Gateway API."]
    pub channel: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterGatewayApiConfigEl {
    pub fn build(self) -> ContainerClusterGatewayApiConfigEl {
        ContainerClusterGatewayApiConfigEl { channel: self.channel }
    }
}

pub struct ContainerClusterGatewayApiConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterGatewayApiConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterGatewayApiConfigElRef {
        ContainerClusterGatewayApiConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterGatewayApiConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Gateway API release channel to use for Gateway API."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterIdentityServiceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterIdentityServiceConfigEl {
    #[doc = "Set the field `enabled`.\nWhether to enable the Identity Service component."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterIdentityServiceConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterIdentityServiceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterIdentityServiceConfigEl {}

impl BuildContainerClusterIdentityServiceConfigEl {
    pub fn build(self) -> ContainerClusterIdentityServiceConfigEl {
        ContainerClusterIdentityServiceConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ContainerClusterIdentityServiceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterIdentityServiceConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterIdentityServiceConfigElRef {
        ContainerClusterIdentityServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterIdentityServiceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable the Identity Service component."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    pod_range_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl { }

impl terrars::ToListMappable for ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    #[doc = "Name for pod secondary ipv4 range which has the actual range defined ahead."]
    pub pod_range_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    pub fn build(self) -> ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
        ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl { pod_range_names: self.pod_range_names }
    }
}

pub struct ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
        ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pod_range_names` after provisioning.\nName for pod secondary ipv4 range which has the actual range defined ahead."]
    pub fn pod_range_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.pod_range_names", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl { }

impl terrars::ToListMappable for ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
        ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
        ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterIpAllocationPolicyElDynamic {
    additional_pod_ranges_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterIpAllocationPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_ranges_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>>,
    dynamic: ContainerClusterIpAllocationPolicyElDynamic,
}

impl ContainerClusterIpAllocationPolicyEl {
    #[doc =
        "Set the field `cluster_ipv4_cidr_block`.\nThe IP address range for the cluster pod IPs. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use."]
    pub fn set_cluster_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_secondary_range_name`.\nThe name of the existing secondary range in the cluster's subnetwork to use for pod IP addresses. Alternatively, cluster_ipv4_cidr_block can be used to automatically create a GKE-managed one."]
    pub fn set_cluster_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services_ipv4_cidr_block`.\nThe IP address range of the services IPs in this cluster. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use."]
    pub fn set_services_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services_secondary_range_name`.\nThe name of the existing secondary range in the cluster's subnetwork to use for service ClusterIPs. Alternatively, services_ipv4_cidr_block can be used to automatically create a GKE-managed one."]
    pub fn set_services_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stack_type`.\nThe IP Stack type of the cluster. Choose between IPV4 and IPV4_IPV6. Default type is IPV4 Only if not set"]
    pub fn set_stack_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_pod_ranges_config`.\n"]
    pub fn set_additional_pod_ranges_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_pod_ranges_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_pod_ranges_config = std::option::Option::Some(d);
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
                    ::Into<
                        terrars::BlockAssignable<ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>,
                    >,
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

impl terrars::ToListMappable for ContainerClusterIpAllocationPolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterIpAllocationPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterIpAllocationPolicyEl {}

impl BuildContainerClusterIpAllocationPolicyEl {
    pub fn build(self) -> ContainerClusterIpAllocationPolicyEl {
        ContainerClusterIpAllocationPolicyEl {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            additional_pod_ranges_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterIpAllocationPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterIpAllocationPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterIpAllocationPolicyElRef {
        ContainerClusterIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterIpAllocationPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\nThe IP address range for the cluster pod IPs. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use."]
    pub fn cluster_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\nThe name of the existing secondary range in the cluster's subnetwork to use for pod IP addresses. Alternatively, cluster_ipv4_cidr_block can be used to automatically create a GKE-managed one."]
    pub fn cluster_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\nThe IP address range of the services IPs in this cluster. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use."]
    pub fn services_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_secondary_range_name` after provisioning.\nThe name of the existing secondary range in the cluster's subnetwork to use for service ClusterIPs. Alternatively, services_ipv4_cidr_block can be used to automatically create a GKE-managed one."]
    pub fn services_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe IP Stack type of the cluster. Choose between IPV4 and IPV4_IPV6. Default type is IPV4 Only if not set"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_ranges_config` after provisioning.\n"]
    pub fn additional_pod_ranges_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_ranges_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterLoggingConfigEl {
    enable_components: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterLoggingConfigEl { }

impl terrars::ToListMappable for ContainerClusterLoggingConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterLoggingConfigEl {
    #[doc =
        "GKE components exposing logs. Valid values include SYSTEM_COMPONENTS, APISERVER, CONTROLLER_MANAGER, SCHEDULER, and WORKLOADS."]
    pub enable_components: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterLoggingConfigEl {
    pub fn build(self) -> ContainerClusterLoggingConfigEl {
        ContainerClusterLoggingConfigEl { enable_components: self.enable_components }
    }
}

pub struct ContainerClusterLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterLoggingConfigElRef {
        ContainerClusterLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_components` after provisioning.\nGKE components exposing logs. Valid values include SYSTEM_COMPONENTS, APISERVER, CONTROLLER_MANAGER, SCHEDULER, and WORKLOADS."]
    pub fn enable_components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    start_time: terrars::PrimField<std::string::String>,
}

impl ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl { }

impl terrars::ToListMappable for ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    type O = terrars::BlockAssignable<ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    pub fn build(self) -> ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
        ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl { start_time: self.start_time }
    }
}

pub struct ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
        ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    scope: terrars::PrimField<std::string::String>,
}

impl ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl { }

impl terrars::ToListMappable for ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    type O = terrars::BlockAssignable<ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    #[doc = "The scope of automatic upgrades to restrict in the exclusion window."]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    pub fn build(self) -> ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
        ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl { scope: self.scope }
    }
}

pub struct ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
        ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of automatic upgrades to restrict in the exclusion window."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterMaintenancePolicyElMaintenanceExclusionElDynamic {
    exclusion_options: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    end_time: terrars::PrimField<std::string::String>,
    exclusion_name: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_options: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>>,
    dynamic: ContainerClusterMaintenancePolicyElMaintenanceExclusionElDynamic,
}

impl ContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[doc = "Set the field `exclusion_options`.\n"]
    pub fn set_exclusion_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    type O = terrars::BlockAssignable<ContainerClusterMaintenancePolicyElMaintenanceExclusionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[doc = ""]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub exclusion_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    pub fn build(self) -> ContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
        ContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
            end_time: self.end_time,
            exclusion_name: self.exclusion_name,
            start_time: self.start_time,
            exclusion_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
        ContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_name` after provisioning.\n"]
    pub fn exclusion_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclusion_name", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_options` after provisioning.\n"]
    pub fn exclusion_options(
        &self,
    ) -> terrars::ListRef<ContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMaintenancePolicyElRecurringWindowEl {
    end_time: terrars::PrimField<std::string::String>,
    recurrence: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ContainerClusterMaintenancePolicyElRecurringWindowEl { }

impl terrars::ToListMappable for ContainerClusterMaintenancePolicyElRecurringWindowEl {
    type O = terrars::BlockAssignable<ContainerClusterMaintenancePolicyElRecurringWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMaintenancePolicyElRecurringWindowEl {
    #[doc = ""]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterMaintenancePolicyElRecurringWindowEl {
    pub fn build(self) -> ContainerClusterMaintenancePolicyElRecurringWindowEl {
        ContainerClusterMaintenancePolicyElRecurringWindowEl {
            end_time: self.end_time,
            recurrence: self.recurrence,
            start_time: self.start_time,
        }
    }
}

pub struct ContainerClusterMaintenancePolicyElRecurringWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMaintenancePolicyElRecurringWindowElRef {
        ContainerClusterMaintenancePolicyElRecurringWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterMaintenancePolicyElDynamic {
    daily_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    maintenance_exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    recurring_window: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMaintenancePolicyElRecurringWindowEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_maintenance_window: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusion: std::option::Option<std::vec::Vec<ContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_window: std::option::Option<std::vec::Vec<ContainerClusterMaintenancePolicyElRecurringWindowEl>>,
    dynamic: ContainerClusterMaintenancePolicyElDynamic,
}

impl ContainerClusterMaintenancePolicyEl {
    #[doc = "Set the field `daily_maintenance_window`.\n"]
    pub fn set_daily_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily_maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_exclusion`.\n"]
    pub fn set_maintenance_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_exclusion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_exclusion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMaintenancePolicyElRecurringWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recurring_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recurring_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMaintenancePolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMaintenancePolicyEl {}

impl BuildContainerClusterMaintenancePolicyEl {
    pub fn build(self) -> ContainerClusterMaintenancePolicyEl {
        ContainerClusterMaintenancePolicyEl {
            daily_maintenance_window: core::default::Default::default(),
            maintenance_exclusion: core::default::Default::default(),
            recurring_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMaintenancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterMaintenancePolicyElRef {
        ContainerClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_maintenance_window` after provisioning.\n"]
    pub fn daily_maintenance_window(
        &self,
    ) -> terrars::ListRef<ContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(&self) -> terrars::ListRef<ContainerClusterMaintenancePolicyElRecurringWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMasterAuthElClientCertificateConfigEl {
    issue_client_certificate: terrars::PrimField<bool>,
}

impl ContainerClusterMasterAuthElClientCertificateConfigEl { }

impl terrars::ToListMappable for ContainerClusterMasterAuthElClientCertificateConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterMasterAuthElClientCertificateConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMasterAuthElClientCertificateConfigEl {
    #[doc = "Whether client certificate authorization is enabled for this cluster."]
    pub issue_client_certificate: terrars::PrimField<bool>,
}

impl BuildContainerClusterMasterAuthElClientCertificateConfigEl {
    pub fn build(self) -> ContainerClusterMasterAuthElClientCertificateConfigEl {
        ContainerClusterMasterAuthElClientCertificateConfigEl {
            issue_client_certificate: self.issue_client_certificate,
        }
    }
}

pub struct ContainerClusterMasterAuthElClientCertificateConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMasterAuthElClientCertificateConfigElRef {
        ContainerClusterMasterAuthElClientCertificateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `issue_client_certificate` after provisioning.\nWhether client certificate authorization is enabled for this cluster."]
    pub fn issue_client_certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issue_client_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterMasterAuthElDynamic {
    client_certificate_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMasterAuthElClientCertificateConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterMasterAuthEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_config: std::option::Option<std::vec::Vec<ContainerClusterMasterAuthElClientCertificateConfigEl>>,
    dynamic: ContainerClusterMasterAuthElDynamic,
}

impl ContainerClusterMasterAuthEl {
    #[doc = "Set the field `client_certificate_config`.\n"]
    pub fn set_client_certificate_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMasterAuthElClientCertificateConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_certificate_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_certificate_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMasterAuthEl {
    type O = terrars::BlockAssignable<ContainerClusterMasterAuthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMasterAuthEl {}

impl BuildContainerClusterMasterAuthEl {
    pub fn build(self) -> ContainerClusterMasterAuthEl {
        ContainerClusterMasterAuthEl {
            client_certificate_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMasterAuthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMasterAuthElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterMasterAuthElRef {
        ContainerClusterMasterAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMasterAuthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nBase64 encoded public certificate used by clients to authenticate to the cluster endpoint."]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nBase64 encoded private key used by clients to authenticate to the cluster endpoint."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\nBase64 encoded public certificate that is the root of trust for the cluster."]
    pub fn cluster_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate_config` after provisioning.\n"]
    pub fn client_certificate_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterMasterAuthElClientCertificateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "Set the field `display_name`.\nField for users to identify CIDR blocks."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    type O = terrars::BlockAssignable<ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "External network that can access Kubernetes master through HTTPS. Must be specified in CIDR notation."]
    pub cidr_block: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(self) -> ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
        ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: self.cidr_block,
            display_name: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr_block` after provisioning.\nExternal network that can access Kubernetes master through HTTPS. Must be specified in CIDR notation."]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nField for users to identify CIDR blocks."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterMasterAuthorizedNetworksConfigElDynamic {
    cidr_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterMasterAuthorizedNetworksConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_public_cidrs_access_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<std::vec::Vec<ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    dynamic: ContainerClusterMasterAuthorizedNetworksConfigElDynamic,
}

impl ContainerClusterMasterAuthorizedNetworksConfigEl {
    #[doc =
        "Set the field `gcp_public_cidrs_access_enabled`.\nWhether master is accessbile via Google Compute Engine Public IP addresses."]
    pub fn set_gcp_public_cidrs_access_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.gcp_public_cidrs_access_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cidr_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_blocks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMasterAuthorizedNetworksConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterMasterAuthorizedNetworksConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMasterAuthorizedNetworksConfigEl {}

impl BuildContainerClusterMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> ContainerClusterMasterAuthorizedNetworksConfigEl {
        ContainerClusterMasterAuthorizedNetworksConfigEl {
            gcp_public_cidrs_access_enabled: core::default::Default::default(),
            cidr_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMasterAuthorizedNetworksConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMasterAuthorizedNetworksConfigElRef {
        ContainerClusterMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_public_cidrs_access_enabled` after provisioning.\nWhether master is accessbile via Google Compute Engine Public IP addresses."]
    pub fn gcp_public_cidrs_access_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_public_cidrs_access_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMeshCertificatesEl {
    enable_certificates: terrars::PrimField<bool>,
}

impl ContainerClusterMeshCertificatesEl { }

impl terrars::ToListMappable for ContainerClusterMeshCertificatesEl {
    type O = terrars::BlockAssignable<ContainerClusterMeshCertificatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMeshCertificatesEl {
    #[doc =
        "When enabled the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub enable_certificates: terrars::PrimField<bool>,
}

impl BuildContainerClusterMeshCertificatesEl {
    pub fn build(self) -> ContainerClusterMeshCertificatesEl {
        ContainerClusterMeshCertificatesEl { enable_certificates: self.enable_certificates }
    }
}

pub struct ContainerClusterMeshCertificatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMeshCertificatesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterMeshCertificatesElRef {
        ContainerClusterMeshCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMeshCertificatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_certificates` after provisioning.\nWhen enabled the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn enable_certificates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    enable_metrics: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relay_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[doc = "Set the field `relay_mode`.\nMode used to make Relay available."]
    pub fn set_relay_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.relay_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[doc = "Whether or not the advanced datapath metrics are enabled."]
    pub enable_metrics: terrars::PrimField<bool>,
}

impl BuildContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    pub fn build(self) -> ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
        ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
            enable_metrics: self.enable_metrics,
            relay_mode: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
        ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_metrics` after provisioning.\nWhether or not the advanced datapath metrics are enabled."]
    pub fn enable_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_metrics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relay_mode` after provisioning.\nMode used to make Relay available."]
    pub fn relay_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterMonitoringConfigElManagedPrometheusEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterMonitoringConfigElManagedPrometheusEl { }

impl terrars::ToListMappable for ContainerClusterMonitoringConfigElManagedPrometheusEl {
    type O = terrars::BlockAssignable<ContainerClusterMonitoringConfigElManagedPrometheusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMonitoringConfigElManagedPrometheusEl {
    #[doc = "Whether or not the managed collection is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterMonitoringConfigElManagedPrometheusEl {
    pub fn build(self) -> ContainerClusterMonitoringConfigElManagedPrometheusEl {
        ContainerClusterMonitoringConfigElManagedPrometheusEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterMonitoringConfigElManagedPrometheusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterMonitoringConfigElManagedPrometheusElRef {
        ContainerClusterMonitoringConfigElManagedPrometheusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the managed collection is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterMonitoringConfigElDynamic {
    advanced_datapath_observability_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>>,
    managed_prometheus: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMonitoringConfigElManagedPrometheusEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterMonitoringConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_components: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_datapath_observability_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_prometheus: std::option::Option<std::vec::Vec<ContainerClusterMonitoringConfigElManagedPrometheusEl>>,
    dynamic: ContainerClusterMonitoringConfigElDynamic,
}

impl ContainerClusterMonitoringConfigEl {
    #[doc =
        "Set the field `enable_components`.\nGKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT and STATEFULSET."]
    pub fn set_enable_components(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enable_components = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_datapath_observability_config`.\n"]
    pub fn set_advanced_datapath_observability_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_datapath_observability_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_datapath_observability_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_prometheus`.\n"]
    pub fn set_managed_prometheus(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterMonitoringConfigElManagedPrometheusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_prometheus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_prometheus = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterMonitoringConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterMonitoringConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterMonitoringConfigEl {}

impl BuildContainerClusterMonitoringConfigEl {
    pub fn build(self) -> ContainerClusterMonitoringConfigEl {
        ContainerClusterMonitoringConfigEl {
            enable_components: core::default::Default::default(),
            advanced_datapath_observability_config: core::default::Default::default(),
            managed_prometheus: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterMonitoringConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterMonitoringConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterMonitoringConfigElRef {
        ContainerClusterMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterMonitoringConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_components` after provisioning.\nGKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT and STATEFULSET."]
    pub fn enable_components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_datapath_observability_config` after provisioning.\n"]
    pub fn advanced_datapath_observability_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_datapath_observability_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_prometheus` after provisioning.\n"]
    pub fn managed_prometheus(&self) -> terrars::ListRef<ContainerClusterMonitoringConfigElManagedPrometheusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNetworkPolicyEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNetworkPolicyEl {
    #[doc = "Set the field `provider`.\nThe selected network policy provider."]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNetworkPolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterNetworkPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNetworkPolicyEl {
    #[doc = "Whether network policy is enabled on the cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNetworkPolicyEl {
    pub fn build(self) -> ContainerClusterNetworkPolicyEl {
        ContainerClusterNetworkPolicyEl {
            enabled: self.enabled,
            provider: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNetworkPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNetworkPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNetworkPolicyElRef {
        ContainerClusterNetworkPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNetworkPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether network policy is enabled on the cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provider` after provisioning.\nThe selected network policy provider."]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodeConfigElEffectiveTaintsEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElEffectiveTaintsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElEffectiveTaintsEl {}

impl BuildContainerClusterNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> ContainerClusterNodeConfigElEffectiveTaintsEl {
        ContainerClusterNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElEffectiveTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElEffectiveTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElEffectiveTaintsElRef {
        ContainerClusterNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElEffectiveTaintsElRef {
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
pub struct ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_driver_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}

impl BuildContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_shared_clients_per_gpu: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}

impl BuildContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
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
pub struct ContainerClusterNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<terrars::ListField<ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std
    ::option
    ::Option<terrars::ListField<ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodeConfigElGuestAcceleratorEl {
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
                        ::ListField<ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
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
                    ::Into<terrars::ListField<ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElGuestAcceleratorEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElGuestAcceleratorEl {}

impl BuildContainerClusterNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> ContainerClusterNodeConfigElGuestAcceleratorEl {
        ContainerClusterNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElGuestAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElGuestAcceleratorElRef {
        ContainerClusterNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElGuestAcceleratorElRef {
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
    ) -> terrars::ListRef<ContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    threads_per_core: terrars::PrimField<f64>,
}

impl ContainerClusterNodeConfigElAdvancedMachineFeaturesEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    #[doc =
        "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> ContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
        ContainerClusterNodeConfigElAdvancedMachineFeaturesEl { threads_per_core: self.threads_per_core }
    }
}

pub struct ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
        ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
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
pub struct ContainerClusterNodeConfigElConfidentialNodesEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodeConfigElConfidentialNodesEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElConfidentialNodesEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElConfidentialNodesEl {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> ContainerClusterNodeConfigElConfidentialNodesEl {
        ContainerClusterNodeConfigElConfidentialNodesEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodeConfigElConfidentialNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElConfidentialNodesElRef {
        ContainerClusterNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElConfidentialNodesElRef {
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
pub struct ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc =
        "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
        ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
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
pub struct ContainerClusterNodeConfigElFastSocketEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodeConfigElFastSocketEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElFastSocketEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElFastSocketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElFastSocketEl {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodeConfigElFastSocketEl {
    pub fn build(self) -> ContainerClusterNodeConfigElFastSocketEl {
        ContainerClusterNodeConfigElFastSocketEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodeConfigElFastSocketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElFastSocketElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodeConfigElFastSocketElRef {
        ContainerClusterNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElFastSocketElRef {
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
pub struct ContainerClusterNodeConfigElGcfsConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodeConfigElGcfsConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElGcfsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElGcfsConfigEl {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodeConfigElGcfsConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElGcfsConfigEl {
        ContainerClusterNodeConfigElGcfsConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodeConfigElGcfsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElGcfsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodeConfigElGcfsConfigElRef {
        ContainerClusterNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElGvnicEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodeConfigElGvnicEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElGvnicEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElGvnicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElGvnicEl {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodeConfigElGvnicEl {
    pub fn build(self) -> ContainerClusterNodeConfigElGvnicEl {
        ContainerClusterNodeConfigElGvnicEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodeConfigElGvnicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElGvnicElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodeConfigElGvnicElRef {
        ContainerClusterNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElGvnicElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElHostMaintenancePolicyEl {
    maintenance_interval: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodeConfigElHostMaintenancePolicyEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElHostMaintenancePolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElHostMaintenancePolicyEl {
    #[doc = "."]
    pub maintenance_interval: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> ContainerClusterNodeConfigElHostMaintenancePolicyEl {
        ContainerClusterNodeConfigElHostMaintenancePolicyEl { maintenance_interval: self.maintenance_interval }
    }
}

pub struct ContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElHostMaintenancePolicyElRef {
        ContainerClusterNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n."]
    pub fn maintenance_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota_period: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_manager_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerClusterNodeConfigElKubeletConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElKubeletConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElKubeletConfigEl {
    #[doc = "Control the CPU management policy on the node."]
    pub cpu_manager_policy: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodeConfigElKubeletConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElKubeletConfigEl {
        ContainerClusterNodeConfigElKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: self.cpu_manager_policy,
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElKubeletConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElKubeletConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElKubeletConfigElRef {
        ContainerClusterNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElKubeletConfigElRef {
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
pub struct ContainerClusterNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ContainerClusterNodeConfigElLinuxNodeConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElLinuxNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElLinuxNodeConfigEl {}

impl BuildContainerClusterNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElLinuxNodeConfigEl {
        ContainerClusterNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElLinuxNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElLinuxNodeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElLinuxNodeConfigElRef {
        ContainerClusterNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElLinuxNodeConfigElRef {
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
pub struct ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
        ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
        ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
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
pub struct ContainerClusterNodeConfigElReservationAffinityEl {
    consume_reservation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerClusterNodeConfigElReservationAffinityEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElReservationAffinityEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElReservationAffinityEl {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodeConfigElReservationAffinityEl {
    pub fn build(self) -> ContainerClusterNodeConfigElReservationAffinityEl {
        ContainerClusterNodeConfigElReservationAffinityEl {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElReservationAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElReservationAffinityElRef {
        ContainerClusterNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElReservationAffinityElRef {
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
pub struct ContainerClusterNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterNodeConfigElShieldedInstanceConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElShieldedInstanceConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElShieldedInstanceConfigEl {}

impl BuildContainerClusterNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElShieldedInstanceConfigEl {
        ContainerClusterNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElShieldedInstanceConfigElRef {
        ContainerClusterNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElShieldedInstanceConfigElRef {
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
pub struct ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
        ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

pub struct ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
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
struct ContainerClusterNodeConfigElSoleTenantConfigElDynamic {
    node_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    dynamic: ContainerClusterNodeConfigElSoleTenantConfigElDynamic,
}

impl ContainerClusterNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
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

impl terrars::ToListMappable for ContainerClusterNodeConfigElSoleTenantConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElSoleTenantConfigEl {}

impl BuildContainerClusterNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElSoleTenantConfigEl {
        ContainerClusterNodeConfigElSoleTenantConfigEl {
            node_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodeConfigElSoleTenantConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElSoleTenantConfigElRef {
        ContainerClusterNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigElTaintEl {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodeConfigElTaintEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElTaintEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElTaintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElTaintEl {
    #[doc = "Effect for taint."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for taint."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodeConfigElTaintEl {
    pub fn build(self) -> ContainerClusterNodeConfigElTaintEl {
        ContainerClusterNodeConfigElTaintEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct ContainerClusterNodeConfigElTaintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElTaintElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodeConfigElTaintElRef {
        ContainerClusterNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElTaintElRef {
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
pub struct ContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodeConfigElWorkloadMetadataConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigElWorkloadMetadataConfigEl {
        ContainerClusterNodeConfigElWorkloadMetadataConfigEl { mode: self.mode }
    }
}

pub struct ContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
        ContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
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
struct ContainerClusterNodeConfigElDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElConfidentialNodesEl>>,
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    fast_socket: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElFastSocketEl>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElGcfsConfigEl>>,
    gvnic: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElGvnicEl>>,
    host_maintenance_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    kubelet_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElKubeletConfigEl>>,
    linux_node_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElReservationAffinityEl>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    sole_tenant_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElSoleTenantConfigEl>>,
    taint: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElTaintEl>>,
    workload_metadata_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<terrars::ListField<ContainerClusterNodeConfigElGuestAcceleratorEl>>,
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
    advanced_machine_features: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_maintenance_policy: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std::option::Option<std::vec::Vec<ContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
    dynamic: ContainerClusterNodeConfigElDynamic,
}

impl ContainerClusterNodeConfigEl {
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
        v: impl std::convert::Into<terrars::ListField<ContainerClusterNodeConfigElGuestAcceleratorEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElConfidentialNodesEl>>,
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
                    ::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElFastSocketEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElGcfsConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElGvnicEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElKubeletConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElLinuxNodeConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElReservationAffinityEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElSoleTenantConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElTaintEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
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

impl terrars::ToListMappable for ContainerClusterNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodeConfigEl {}

impl BuildContainerClusterNodeConfigEl {
    pub fn build(self) -> ContainerClusterNodeConfigEl {
        ContainerClusterNodeConfigEl {
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

pub struct ContainerClusterNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodeConfigElRef {
        ContainerClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodeConfigElRef {
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
    pub fn effective_taints(&self) -> terrars::ListRef<ContainerClusterNodeConfigElEffectiveTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ContainerClusterNodeConfigElGuestAcceleratorElRef> {
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
    ) -> terrars::ListRef<ContainerClusterNodeConfigElAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerClusterNodeConfigElConfidentialNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<ContainerClusterNodeConfigElFastSocketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElGcfsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<ContainerClusterNodeConfigElGvnicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(&self) -> terrars::ListRef<ContainerClusterNodeConfigElHostMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElKubeletConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElLinuxNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ContainerClusterNodeConfigElReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElSoleTenantConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<ContainerClusterNodeConfigElTaintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> terrars::ListRef<ContainerClusterNodeConfigElWorkloadMetadataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElAutoscalingEl {
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

impl ContainerClusterNodePoolElAutoscalingEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElAutoscalingEl {}

impl BuildContainerClusterNodePoolElAutoscalingEl {
    pub fn build(self) -> ContainerClusterNodePoolElAutoscalingEl {
        ContainerClusterNodePoolElAutoscalingEl {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolElAutoscalingElRef {
        ContainerClusterNodePoolElAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElAutoscalingElRef {
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
pub struct ContainerClusterNodePoolElManagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterNodePoolElManagementEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElManagementEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElManagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElManagementEl {}

impl BuildContainerClusterNodePoolElManagementEl {
    pub fn build(self) -> ContainerClusterNodePoolElManagementEl {
        ContainerClusterNodePoolElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElManagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElManagementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolElManagementElRef {
        ContainerClusterNodePoolElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElManagementElRef {
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
pub struct ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    #[doc = "Specifies the total network bandwidth tier for the NodePool."]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
        ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

pub struct ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
        ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
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
pub struct ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    disabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
        ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl { disabled: self.disabled }
    }
}

pub struct ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
        ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNodePoolElNetworkConfigElDynamic {
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNetworkConfigEl {
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
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>>,
    dynamic: ContainerClusterNodePoolElNetworkConfigElDynamic,
}

impl ContainerClusterNodePoolElNetworkConfigEl {
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
                    ::Into<
                        terrars::BlockAssignable<ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>,
                    >,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>,
                    >,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNetworkConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNetworkConfigEl {}

impl BuildContainerClusterNodePoolElNetworkConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNetworkConfigEl {
        ContainerClusterNodePoolElNetworkConfigEl {
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

pub struct ContainerClusterNodePoolElNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolElNetworkConfigElRef {
        ContainerClusterNodePoolElNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNetworkConfigElRef {
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
    ) -> terrars::ListRef<ContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {}

impl BuildContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
        ContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
        ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_driver_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O =
        terrars
        ::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_shared_clients_per_gpu: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<
        terrars::ListField<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std
    ::option
    ::Option<terrars::ListField<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
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
                        ::ListField<
                            ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
                        >,
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
                    ::Into<
                        terrars::ListField<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>,
                    >,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {}

impl BuildContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
        ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
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
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    threads_per_core: terrars::PrimField<f64>,
}

impl ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    #[doc =
        "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
        ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl { threads_per_core: self.threads_per_core }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
        ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
        ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
        ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc =
        "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
        ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
            local_ssd_count: self.local_ssd_count,
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElFastSocketEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodePoolElNodeConfigElFastSocketEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElFastSocketEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElFastSocketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElFastSocketEl {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodePoolElNodeConfigElFastSocketEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElFastSocketEl {
        ContainerClusterNodePoolElNodeConfigElFastSocketEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElFastSocketElRef {
        ContainerClusterNodePoolElNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElFastSocketElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodePoolElNodeConfigElGcfsConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
        ContainerClusterNodePoolElNodeConfigElGcfsConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
        ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not GCFS is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElGvnicEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterNodePoolElNodeConfigElGvnicEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElGvnicEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGvnicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElGvnicEl {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNodePoolElNodeConfigElGvnicEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElGvnicEl {
        ContainerClusterNodePoolElNodeConfigElGvnicEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElGvnicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElGvnicElRef {
        ContainerClusterNodePoolElNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not gvnic is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    maintenance_interval: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    #[doc = "."]
    pub maintenance_interval: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
        ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: self.maintenance_interval,
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
        ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n."]
    pub fn maintenance_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_cfs_quota_period: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_manager_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[doc = "Control the CPU management policy on the node."]
    pub cpu_manager_policy: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
        ContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: self.cpu_manager_policy,
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
        ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
        ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
        ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
        ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl { local_ssd_count: self.local_ssd_count }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
        ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    consume_reservation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
        ContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
        ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
        ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
        ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
        ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
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
struct ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElDynamic {
    node_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    dynamic: ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElDynamic,
}

impl ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>,
                    >,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
        ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
            node_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
        ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigElTaintEl {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodePoolElNodeConfigElTaintEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElTaintEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElTaintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElTaintEl {
    #[doc = "Effect for taint."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for taint."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNodeConfigElTaintEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElTaintEl {
        ContainerClusterNodePoolElNodeConfigElTaintEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElTaintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElTaintElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElTaintElRef {
        ContainerClusterNodePoolElNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElTaintElRef {
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
pub struct ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl { }

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
        ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl { mode: self.mode }
    }
}

pub struct ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
        ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
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
struct ContainerClusterNodePoolElNodeConfigElDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    confidential_nodes: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    fast_socket: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    gvnic: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElGvnicEl>>,
    host_maintenance_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    kubelet_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    linux_node_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    sole_tenant_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    taint: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElTaintEl>>,
    workload_metadata_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<terrars::ListField<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
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
    advanced_machine_features: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_maintenance_policy: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
    dynamic: ContainerClusterNodePoolElNodeConfigElDynamic,
}

impl ContainerClusterNodePoolElNodeConfigEl {
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
        v: impl std::convert::Into<terrars::ListField<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>,
                    >,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElGvnicEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElTaintEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElNodeConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElNodeConfigEl {}

impl BuildContainerClusterNodePoolElNodeConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolElNodeConfigEl {
        ContainerClusterNodePoolElNodeConfigEl {
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

pub struct ContainerClusterNodePoolElNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolElNodeConfigElRef {
        ContainerClusterNodePoolElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElNodeConfigElRef {
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
    pub fn effective_taints(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef> {
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
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElFastSocketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElGcfsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElGvnicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElKubeletConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElTaintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElPlacementPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerClusterNodePoolElPlacementPolicyEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElPlacementPolicyEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElPlacementPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElPlacementPolicyEl {
    #[doc = "Type defines the type of placement policy"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterNodePoolElPlacementPolicyEl {
    pub fn build(self) -> ContainerClusterNodePoolElPlacementPolicyEl {
        ContainerClusterNodePoolElPlacementPolicyEl {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct ContainerClusterNodePoolElPlacementPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElPlacementPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElPlacementPolicyElRef {
        ContainerClusterNodePoolElPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElPlacementPolicyElRef {
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
pub struct ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
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

impl terrars::ToListMappable for ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O =
        terrars
        ::BlockAssignable<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}

impl BuildContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(self) -> ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
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
struct ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<
        terrars::DynamicBlock<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>,
    >,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>>,
    dynamic: ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElDynamic,
}

impl ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
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
                            ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {}

impl BuildContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(self) -> ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
        ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
        ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
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
    ) -> terrars::ListRef<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNodePoolElUpgradeSettingsElDynamic {
    blue_green_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolElUpgradeSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
    dynamic: ContainerClusterNodePoolElUpgradeSettingsElDynamic,
}

impl ContainerClusterNodePoolElUpgradeSettingsEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
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

impl terrars::ToListMappable for ContainerClusterNodePoolElUpgradeSettingsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolElUpgradeSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolElUpgradeSettingsEl {}

impl BuildContainerClusterNodePoolElUpgradeSettingsEl {
    pub fn build(self) -> ContainerClusterNodePoolElUpgradeSettingsEl {
        ContainerClusterNodePoolElUpgradeSettingsEl {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElUpgradeSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElUpgradeSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolElUpgradeSettingsElRef {
        ContainerClusterNodePoolElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElUpgradeSettingsElRef {
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
    pub fn blue_green_settings(
        &self,
    ) -> terrars::ListRef<ContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNodePoolElDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElAutoscalingEl>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElManagementEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNetworkConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElNodeConfigEl>>,
    placement_policy: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElPlacementPolicyEl>>,
    upgrade_settings: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolElUpgradeSettingsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_node_count: std::option::Option<terrars::PrimField<f64>>,
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
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElManagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<std::vec::Vec<ContainerClusterNodePoolElUpgradeSettingsEl>>,
    dynamic: ContainerClusterNodePoolElDynamic,
}

impl ContainerClusterNodePoolEl {
    #[doc =
        "Set the field `initial_node_count`.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn set_initial_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_pods_per_node`.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn set_name_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_count`.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_locations`.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn set_node_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.node_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElManagementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElPlacementPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolElUpgradeSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upgrade_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upgrade_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolEl {}

impl BuildContainerClusterNodePoolEl {
    pub fn build(self) -> ContainerClusterNodePoolEl {
        ContainerClusterNodePoolEl {
            initial_node_count: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
            name: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_locations: core::default::Default::default(),
            version: core::default::Default::default(),
            autoscaling: core::default::Default::default(),
            management: core::default::Default::default(),
            network_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            placement_policy: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolElRef {
        ContainerClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `initial_node_count` after provisioning.\nThe initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource."]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_urls` after provisioning.\nThe resource URLs of the managed instance groups associated with this node pool."]
    pub fn instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\nList of instance group URLs which have been assigned to this node pool."]
    pub fn managed_instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods per node in this node pool. Note that this does not work on node pools which are \"route-based\" - that is, node pools belonging to clusters that do not have IP Aliasing enabled."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the node pool. If left blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version for the nodes in this pool. Note that if this field and auto_upgrade are both specified, they will fight each other for what the node version should be, so setting both is highly discouraged. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerClusterNodePoolElAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerClusterNodePoolElManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerClusterNodePoolElNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerClusterNodePoolElPlacementPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerClusterNodePoolElUpgradeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[doc = "Set the field `tags`.\nList of network tags applied to auto-provisioned node pools."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolAutoConfigElNetworkTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolAutoConfigElNetworkTagsEl {}

impl BuildContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    pub fn build(self) -> ContainerClusterNodePoolAutoConfigElNetworkTagsEl {
        ContainerClusterNodePoolAutoConfigElNetworkTagsEl { tags: core::default::Default::default() }
    }
}

pub struct ContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
        ContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nList of network tags applied to auto-provisioned node pools."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNodePoolAutoConfigElDynamic {
    network_tags: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolAutoConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<std::vec::Vec<ContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
    dynamic: ContainerClusterNodePoolAutoConfigElDynamic,
}

impl ContainerClusterNodePoolAutoConfigEl {
    #[doc = "Set the field `network_tags`.\n"]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolAutoConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolAutoConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolAutoConfigEl {}

impl BuildContainerClusterNodePoolAutoConfigEl {
    pub fn build(self) -> ContainerClusterNodePoolAutoConfigEl {
        ContainerClusterNodePoolAutoConfigEl {
            network_tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolAutoConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolAutoConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolAutoConfigElRef {
        ContainerClusterNodePoolAutoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolAutoConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(&self) -> terrars::ListRef<ContainerClusterNodePoolAutoConfigElNetworkTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_variant: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[doc =
        "Set the field `logging_variant`.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn set_logging_variant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_variant = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {}

impl BuildContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    pub fn build(self) -> ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
        ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl { logging_variant: core::default::Default::default() }
    }
}

pub struct ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
        ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `logging_variant` after provisioning.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn logging_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNodePoolDefaultsElDynamic {
    node_config_defaults: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNodePoolDefaultsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config_defaults: std::option::Option<std::vec::Vec<ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
    dynamic: ContainerClusterNodePoolDefaultsElDynamic,
}

impl ContainerClusterNodePoolDefaultsEl {
    #[doc = "Set the field `node_config_defaults`.\n"]
    pub fn set_node_config_defaults(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_config_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config_defaults = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNodePoolDefaultsEl {
    type O = terrars::BlockAssignable<ContainerClusterNodePoolDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNodePoolDefaultsEl {}

impl BuildContainerClusterNodePoolDefaultsEl {
    pub fn build(self) -> ContainerClusterNodePoolDefaultsEl {
        ContainerClusterNodePoolDefaultsEl {
            node_config_defaults: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNodePoolDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNodePoolDefaultsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNodePoolDefaultsElRef {
        ContainerClusterNodePoolDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNodePoolDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_config_defaults` after provisioning.\n"]
    pub fn node_config_defaults(&self) -> terrars::ListRef<ContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config_defaults", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterNotificationConfigElPubsubElFilterEl {
    event_type: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterNotificationConfigElPubsubElFilterEl { }

impl terrars::ToListMappable for ContainerClusterNotificationConfigElPubsubElFilterEl {
    type O = terrars::BlockAssignable<ContainerClusterNotificationConfigElPubsubElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNotificationConfigElPubsubElFilterEl {
    #[doc =
        "Can be used to filter what notifications are sent. Valid values include include UPGRADE_AVAILABLE_EVENT, UPGRADE_EVENT and SECURITY_BULLETIN_EVENT"]
    pub event_type: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerClusterNotificationConfigElPubsubElFilterEl {
    pub fn build(self) -> ContainerClusterNotificationConfigElPubsubElFilterEl {
        ContainerClusterNotificationConfigElPubsubElFilterEl { event_type: self.event_type }
    }
}

pub struct ContainerClusterNotificationConfigElPubsubElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNotificationConfigElPubsubElFilterElRef {
        ContainerClusterNotificationConfigElPubsubElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_type` after provisioning.\nCan be used to filter what notifications are sent. Valid values include include UPGRADE_AVAILABLE_EVENT, UPGRADE_EVENT and SECURITY_BULLETIN_EVENT"]
    pub fn event_type(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNotificationConfigElPubsubElDynamic {
    filter: std::option::Option<terrars::DynamicBlock<ContainerClusterNotificationConfigElPubsubElFilterEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNotificationConfigElPubsubEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<ContainerClusterNotificationConfigElPubsubElFilterEl>>,
    dynamic: ContainerClusterNotificationConfigElPubsubElDynamic,
}

impl ContainerClusterNotificationConfigElPubsubEl {
    #[doc =
        "Set the field `topic`.\nThe pubsub topic to push upgrade notifications to. Must be in the same project as the cluster. Must be in the format: projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNotificationConfigElPubsubElFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNotificationConfigElPubsubEl {
    type O = terrars::BlockAssignable<ContainerClusterNotificationConfigElPubsubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNotificationConfigElPubsubEl {
    #[doc = "Whether or not the notification config is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterNotificationConfigElPubsubEl {
    pub fn build(self) -> ContainerClusterNotificationConfigElPubsubEl {
        ContainerClusterNotificationConfigElPubsubEl {
            enabled: self.enabled,
            topic: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNotificationConfigElPubsubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNotificationConfigElPubsubElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterNotificationConfigElPubsubElRef {
        ContainerClusterNotificationConfigElPubsubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNotificationConfigElPubsubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the notification config is enabled"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe pubsub topic to push upgrade notifications to. Must be in the same project as the cluster. Must be in the format: projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ContainerClusterNotificationConfigElPubsubElFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterNotificationConfigElDynamic {
    pubsub: std::option::Option<terrars::DynamicBlock<ContainerClusterNotificationConfigElPubsubEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterNotificationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub: std::option::Option<std::vec::Vec<ContainerClusterNotificationConfigElPubsubEl>>,
    dynamic: ContainerClusterNotificationConfigElDynamic,
}

impl ContainerClusterNotificationConfigEl {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerClusterNotificationConfigElPubsubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterNotificationConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterNotificationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterNotificationConfigEl {}

impl BuildContainerClusterNotificationConfigEl {
    pub fn build(self) -> ContainerClusterNotificationConfigEl {
        ContainerClusterNotificationConfigEl {
            pubsub: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterNotificationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterNotificationConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterNotificationConfigElRef {
        ContainerClusterNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterNotificationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> terrars::ListRef<ContainerClusterNotificationConfigElPubsubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl { }

impl terrars::ToListMappable for ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    #[doc = "Whether the cluster master is accessible globally or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    pub fn build(self) -> ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
        ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
        ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the cluster master is accessible globally or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterPrivateClusterConfigElDynamic {
    master_global_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterPrivateClusterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_global_access_config: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
    dynamic: ContainerClusterPrivateClusterConfigElDynamic,
}

impl ContainerClusterPrivateClusterConfigEl {
    #[doc =
        "Set the field `enable_private_endpoint`.\nWhen true, the cluster's private endpoint is used as the cluster endpoint and access through the public endpoint is disabled. When false, either endpoint can be used."]
    pub fn set_enable_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_private_nodes`.\nEnables the private cluster feature, creating a private endpoint on the cluster. In a private cluster, nodes only have RFC 1918 private addresses and communicate with the master's private endpoint via private networking."]
    pub fn set_enable_private_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `master_ipv4_cidr_block`.\nThe IP range in CIDR notation to use for the hosted master network. This range will be used for assigning private IP addresses to the cluster master(s) and the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network, and it must be a /28 subnet. See Private Cluster Limitations for more details. This field only applies to private clusters, when enable_private_nodes is true."]
    pub fn set_master_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_endpoint_subnetwork`.\nSubnetwork in cluster's network where master's endpoint will be provisioned."]
    pub fn set_private_endpoint_subnetwork(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_endpoint_subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_global_access_config`.\n"]
    pub fn set_master_global_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_global_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_global_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterPrivateClusterConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterPrivateClusterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterPrivateClusterConfigEl {}

impl BuildContainerClusterPrivateClusterConfigEl {
    pub fn build(self) -> ContainerClusterPrivateClusterConfigEl {
        ContainerClusterPrivateClusterConfigEl {
            enable_private_endpoint: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            private_endpoint_subnetwork: core::default::Default::default(),
            master_global_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterPrivateClusterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterPrivateClusterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterPrivateClusterConfigElRef {
        ContainerClusterPrivateClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterPrivateClusterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_private_endpoint` after provisioning.\nWhen true, the cluster's private endpoint is used as the cluster endpoint and access through the public endpoint is disabled. When false, either endpoint can be used."]
    pub fn enable_private_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_private_nodes` after provisioning.\nEnables the private cluster feature, creating a private endpoint on the cluster. In a private cluster, nodes only have RFC 1918 private addresses and communicate with the master's private endpoint via private networking."]
    pub fn enable_private_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\nThe IP range in CIDR notation to use for the hosted master network. This range will be used for assigning private IP addresses to the cluster master(s) and the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network, and it must be a /28 subnet. See Private Cluster Limitations for more details. This field only applies to private clusters, when enable_private_nodes is true."]
    pub fn master_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peering_name` after provisioning.\nThe name of the peering between this cluster and the Google owned VPC."]
    pub fn peering_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peering_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint` after provisioning.\nThe internal IP address of this cluster's master endpoint."]
    pub fn private_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_subnetwork` after provisioning.\nSubnetwork in cluster's network where master's endpoint will be provisioned."]
    pub fn private_endpoint_subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_endpoint` after provisioning.\nThe external IP address of this cluster's master endpoint."]
    pub fn public_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `master_global_access_config` after provisioning.\n"]
    pub fn master_global_access_config(
        &self,
    ) -> terrars::ListRef<ContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_global_access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterReleaseChannelEl {
    channel: terrars::PrimField<std::string::String>,
}

impl ContainerClusterReleaseChannelEl { }

impl terrars::ToListMappable for ContainerClusterReleaseChannelEl {
    type O = terrars::BlockAssignable<ContainerClusterReleaseChannelEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterReleaseChannelEl {
    #[doc =
        "The selected release channel. Accepted values are:\n* UNSPECIFIED: Not set.\n* RAPID: Weekly upgrade cadence; Early testers and developers who requires new features.\n* REGULAR: Multiple per month upgrade cadence; Production users who need features not yet offered in the Stable channel.\n* STABLE: Every few months upgrade cadence; Production users who need stability above all else, and for whom frequent upgrades are too risky."]
    pub channel: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterReleaseChannelEl {
    pub fn build(self) -> ContainerClusterReleaseChannelEl {
        ContainerClusterReleaseChannelEl { channel: self.channel }
    }
}

pub struct ContainerClusterReleaseChannelElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterReleaseChannelElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterReleaseChannelElRef {
        ContainerClusterReleaseChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterReleaseChannelElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe selected release channel. Accepted values are:\n* UNSPECIFIED: Not set.\n* RAPID: Weekly upgrade cadence; Early testers and developers who requires new features.\n* REGULAR: Multiple per month upgrade cadence; Production users who need features not yet offered in the Stable channel.\n* STABLE: Every few months upgrade cadence; Production users who need stability above all else, and for whom frequent upgrades are too risky."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    dataset_id: terrars::PrimField<std::string::String>,
}

impl ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl { }

impl terrars::ToListMappable for ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    type O = terrars::BlockAssignable<ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    #[doc = "The ID of a BigQuery Dataset."]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    pub fn build(self) -> ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
        ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl { dataset_id: self.dataset_id }
    }
}

pub struct ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
        ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of a BigQuery Dataset."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterResourceUsageExportConfigElDynamic {
    bigquery_destination: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
}

#[derive(Serialize)]
pub struct ContainerClusterResourceUsageExportConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_egress_metering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_consumption_metering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination: std
    ::option
    ::Option<std::vec::Vec<ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
    dynamic: ContainerClusterResourceUsageExportConfigElDynamic,
}

impl ContainerClusterResourceUsageExportConfigEl {
    #[doc =
        "Set the field `enable_network_egress_metering`.\nWhether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic."]
    pub fn set_enable_network_egress_metering(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_egress_metering = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_resource_consumption_metering`.\nWhether to enable resource consumption metering on this cluster. When enabled, a table will be created in the resource export BigQuery dataset to store resource consumption data. The resulting table can be joined with the resource usage table or with BigQuery billing export. Defaults to true."]
    pub fn set_enable_resource_consumption_metering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_consumption_metering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerClusterResourceUsageExportConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterResourceUsageExportConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterResourceUsageExportConfigEl {}

impl BuildContainerClusterResourceUsageExportConfigEl {
    pub fn build(self) -> ContainerClusterResourceUsageExportConfigEl {
        ContainerClusterResourceUsageExportConfigEl {
            enable_network_egress_metering: core::default::Default::default(),
            enable_resource_consumption_metering: core::default::Default::default(),
            bigquery_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterResourceUsageExportConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterResourceUsageExportConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerClusterResourceUsageExportConfigElRef {
        ContainerClusterResourceUsageExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterResourceUsageExportConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_network_egress_metering` after provisioning.\nWhether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic."]
    pub fn enable_network_egress_metering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_egress_metering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_resource_consumption_metering` after provisioning.\nWhether to enable resource consumption metering on this cluster. When enabled, a table will be created in the resource export BigQuery dataset to store resource consumption data. The resulting table can be joined with the resource usage table or with BigQuery billing export. Defaults to true."]
    pub fn enable_resource_consumption_metering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_consumption_metering", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> terrars::ListRef<ContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterSecurityPostureConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterSecurityPostureConfigEl {
    #[doc =
        "Set the field `mode`.\nSets the mode of the Kubernetes security posture API's off-cluster features. Available options include DISABLED and BASIC."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vulnerability_mode`.\nSets the mode of the Kubernetes security posture API's workload vulnerability scanning. Available options include VULNERABILITY_DISABLED, VULNERABILITY_BASIC and VULNERABILITY_ENTERPRISE."]
    pub fn set_vulnerability_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vulnerability_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterSecurityPostureConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterSecurityPostureConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterSecurityPostureConfigEl {}

impl BuildContainerClusterSecurityPostureConfigEl {
    pub fn build(self) -> ContainerClusterSecurityPostureConfigEl {
        ContainerClusterSecurityPostureConfigEl {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterSecurityPostureConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterSecurityPostureConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterSecurityPostureConfigElRef {
        ContainerClusterSecurityPostureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterSecurityPostureConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSets the mode of the Kubernetes security posture API's off-cluster features. Available options include DISABLED and BASIC."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vulnerability_mode` after provisioning.\nSets the mode of the Kubernetes security posture API's workload vulnerability scanning. Available options include VULNERABILITY_DISABLED, VULNERABILITY_BASIC and VULNERABILITY_ENTERPRISE."]
    pub fn vulnerability_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterServiceExternalIpsConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterServiceExternalIpsConfigEl { }

impl terrars::ToListMappable for ContainerClusterServiceExternalIpsConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterServiceExternalIpsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterServiceExternalIpsConfigEl {
    #[doc = "When enabled, services with exterenal ips specified will be allowed."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterServiceExternalIpsConfigEl {
    pub fn build(self) -> ContainerClusterServiceExternalIpsConfigEl {
        ContainerClusterServiceExternalIpsConfigEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterServiceExternalIpsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterServiceExternalIpsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterServiceExternalIpsConfigElRef {
        ContainerClusterServiceExternalIpsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterServiceExternalIpsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, services with exterenal ips specified will be allowed."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterTimeoutsEl {
    type O = terrars::BlockAssignable<ContainerClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterTimeoutsEl {}

impl BuildContainerClusterTimeoutsEl {
    pub fn build(self) -> ContainerClusterTimeoutsEl {
        ContainerClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ContainerClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterTimeoutsElRef {
        ContainerClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterVerticalPodAutoscalingEl {
    enabled: terrars::PrimField<bool>,
}

impl ContainerClusterVerticalPodAutoscalingEl { }

impl terrars::ToListMappable for ContainerClusterVerticalPodAutoscalingEl {
    type O = terrars::BlockAssignable<ContainerClusterVerticalPodAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterVerticalPodAutoscalingEl {
    #[doc = "Enables vertical pod autoscaling."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerClusterVerticalPodAutoscalingEl {
    pub fn build(self) -> ContainerClusterVerticalPodAutoscalingEl {
        ContainerClusterVerticalPodAutoscalingEl { enabled: self.enabled }
    }
}

pub struct ContainerClusterVerticalPodAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterVerticalPodAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterVerticalPodAutoscalingElRef {
        ContainerClusterVerticalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterVerticalPodAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables vertical pod autoscaling."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerClusterWorkloadIdentityConfigEl {
    #[doc = "Set the field `workload_pool`.\nThe workload pool to attach all Kubernetes service accounts to."]
    pub fn set_workload_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerClusterWorkloadIdentityConfigEl {
    type O = terrars::BlockAssignable<ContainerClusterWorkloadIdentityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerClusterWorkloadIdentityConfigEl {}

impl BuildContainerClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> ContainerClusterWorkloadIdentityConfigEl {
        ContainerClusterWorkloadIdentityConfigEl { workload_pool: core::default::Default::default() }
    }
}

pub struct ContainerClusterWorkloadIdentityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerClusterWorkloadIdentityConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerClusterWorkloadIdentityConfigElRef {
        ContainerClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerClusterWorkloadIdentityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workload_pool` after provisioning.\nThe workload pool to attach all Kubernetes service accounts to."]
    pub fn workload_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_pool", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterDynamic {
    addons_config: std::option::Option<terrars::DynamicBlock<ContainerClusterAddonsConfigEl>>,
    authenticator_groups_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterAuthenticatorGroupsConfigEl>>,
    binary_authorization: std::option::Option<terrars::DynamicBlock<ContainerClusterBinaryAuthorizationEl>>,
    cluster_autoscaling: std::option::Option<terrars::DynamicBlock<ContainerClusterClusterAutoscalingEl>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerClusterConfidentialNodesEl>>,
    cost_management_config: std::option::Option<terrars::DynamicBlock<ContainerClusterCostManagementConfigEl>>,
    database_encryption: std::option::Option<terrars::DynamicBlock<ContainerClusterDatabaseEncryptionEl>>,
    default_snat_status: std::option::Option<terrars::DynamicBlock<ContainerClusterDefaultSnatStatusEl>>,
    dns_config: std::option::Option<terrars::DynamicBlock<ContainerClusterDnsConfigEl>>,
    enable_k8s_beta_apis: std::option::Option<terrars::DynamicBlock<ContainerClusterEnableK8sBetaApisEl>>,
    fleet: std::option::Option<terrars::DynamicBlock<ContainerClusterFleetEl>>,
    gateway_api_config: std::option::Option<terrars::DynamicBlock<ContainerClusterGatewayApiConfigEl>>,
    identity_service_config: std::option::Option<terrars::DynamicBlock<ContainerClusterIdentityServiceConfigEl>>,
    ip_allocation_policy: std::option::Option<terrars::DynamicBlock<ContainerClusterIpAllocationPolicyEl>>,
    logging_config: std::option::Option<terrars::DynamicBlock<ContainerClusterLoggingConfigEl>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<ContainerClusterMaintenancePolicyEl>>,
    master_auth: std::option::Option<terrars::DynamicBlock<ContainerClusterMasterAuthEl>>,
    master_authorized_networks_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterMasterAuthorizedNetworksConfigEl>>,
    mesh_certificates: std::option::Option<terrars::DynamicBlock<ContainerClusterMeshCertificatesEl>>,
    monitoring_config: std::option::Option<terrars::DynamicBlock<ContainerClusterMonitoringConfigEl>>,
    network_policy: std::option::Option<terrars::DynamicBlock<ContainerClusterNetworkPolicyEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodeConfigEl>>,
    node_pool: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolEl>>,
    node_pool_auto_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolAutoConfigEl>>,
    node_pool_defaults: std::option::Option<terrars::DynamicBlock<ContainerClusterNodePoolDefaultsEl>>,
    notification_config: std::option::Option<terrars::DynamicBlock<ContainerClusterNotificationConfigEl>>,
    private_cluster_config: std::option::Option<terrars::DynamicBlock<ContainerClusterPrivateClusterConfigEl>>,
    release_channel: std::option::Option<terrars::DynamicBlock<ContainerClusterReleaseChannelEl>>,
    resource_usage_export_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterResourceUsageExportConfigEl>>,
    security_posture_config: std::option::Option<terrars::DynamicBlock<ContainerClusterSecurityPostureConfigEl>>,
    service_external_ips_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerClusterServiceExternalIpsConfigEl>>,
    vertical_pod_autoscaling: std::option::Option<terrars::DynamicBlock<ContainerClusterVerticalPodAutoscalingEl>>,
    workload_identity_config: std::option::Option<terrars::DynamicBlock<ContainerClusterWorkloadIdentityConfigEl>>,
}
