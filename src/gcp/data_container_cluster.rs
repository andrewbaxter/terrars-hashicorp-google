use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataContainerClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataContainerCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataContainerClusterData>,
}

#[derive(Clone)]
pub struct DataContainerCluster(Rc<DataContainerCluster_>);

impl DataContainerCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> ListRef<DataContainerClusterAddonsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(&self) -> ListRef<DataContainerClusterAuthenticatorGroupsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataContainerClusterBinaryAuthorizationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> ListRef<DataContainerClusterClusterAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterConfidentialNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> ListRef<DataContainerClusterCostManagementConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> ListRef<DataContainerClusterDatabaseEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datapath_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> ListRef<DataContainerClusterDefaultSnatStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Defaults to true. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the cluster will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> ListRef<DataContainerClusterDnsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_intranode_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> ListRef<DataContainerClusterEnableK8sBetaApisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_kubernetes_alpha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_l4_ilb_subsetting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_abac", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_shielded_nodes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_tpu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> ListRef<DataContainerClusterFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> ListRef<DataContainerClusterGatewayApiConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(&self) -> ListRef<DataContainerClusterIdentityServiceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> ListRef<DataContainerClusterIpAllocationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> ListRef<DataContainerClusterLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> ListRef<DataContainerClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> ListRef<DataContainerClusterMasterAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(&self) -> ListRef<DataContainerClusterMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_authorized_networks_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> ListRef<DataContainerClusterMeshCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> ListRef<DataContainerClusterMonitoringConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> ListRef<DataContainerClusterNetworkPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> ListRef<DataContainerClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> ListRef<DataContainerClusterNodePoolAutoConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> ListRef<DataContainerClusterNodePoolDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> ListRef<DataContainerClusterNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> ListRef<DataContainerClusterPrivateClusterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> ListRef<DataContainerClusterReleaseChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_default_node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(&self) -> ListRef<DataContainerClusterResourceUsageExportConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(&self) -> ListRef<DataContainerClusterSecurityPostureConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(&self) -> ListRef<DataContainerClusterServiceExternalIpsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(&self) -> ListRef<DataContainerClusterVerticalPodAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(&self) -> ListRef<DataContainerClusterWorkloadIdentityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

impl Referable for DataContainerCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataContainerCluster { }

impl ToListMappable for DataContainerCluster {
    type O = ListRef<DataContainerClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataContainerCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_container_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataContainerCluster {
    pub tf_id: String,
    #[doc= "The name of the cluster, unique within the project and location."]
    pub name: PrimField<String>,
}

impl BuildDataContainerCluster {
    pub fn build(self, stack: &mut Stack) -> DataContainerCluster {
        let out = DataContainerCluster(Rc::new(DataContainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataContainerClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataContainerClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataContainerClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> ListRef<DataContainerClusterAddonsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(&self) -> ListRef<DataContainerClusterAuthenticatorGroupsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataContainerClusterBinaryAuthorizationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> ListRef<DataContainerClusterClusterAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterConfidentialNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> ListRef<DataContainerClusterCostManagementConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> ListRef<DataContainerClusterDatabaseEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datapath_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> ListRef<DataContainerClusterDefaultSnatStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Defaults to true. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the cluster will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> ListRef<DataContainerClusterDnsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_intranode_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> ListRef<DataContainerClusterEnableK8sBetaApisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_kubernetes_alpha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_l4_ilb_subsetting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_abac", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_shielded_nodes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_tpu", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> ListRef<DataContainerClusterFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> ListRef<DataContainerClusterGatewayApiConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(&self) -> ListRef<DataContainerClusterIdentityServiceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> ListRef<DataContainerClusterIpAllocationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> ListRef<DataContainerClusterLoggingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> ListRef<DataContainerClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> ListRef<DataContainerClusterMasterAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(&self) -> ListRef<DataContainerClusterMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_authorized_networks_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> ListRef<DataContainerClusterMeshCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> ListRef<DataContainerClusterMonitoringConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> ListRef<DataContainerClusterNetworkPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> ListRef<DataContainerClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> ListRef<DataContainerClusterNodePoolAutoConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> ListRef<DataContainerClusterNodePoolDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> ListRef<DataContainerClusterNotificationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> ListRef<DataContainerClusterPrivateClusterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> ListRef<DataContainerClusterReleaseChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_default_node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(&self) -> ListRef<DataContainerClusterResourceUsageExportConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(&self) -> ListRef<DataContainerClusterSecurityPostureConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(&self) -> ListRef<DataContainerClusterServiceExternalIpsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(&self) -> ListRef<DataContainerClusterVerticalPodAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(&self) -> ListRef<DataContainerClusterWorkloadIdentityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElCloudrunConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
}

impl DataContainerClusterAddonsConfigElCloudrunConfigEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElCloudrunConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElCloudrunConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElCloudrunConfigEl {}

impl BuildDataContainerClusterAddonsConfigElCloudrunConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElCloudrunConfigEl {
        DataContainerClusterAddonsConfigElCloudrunConfigEl {
            disabled: core::default::Default::default(),
            load_balancer_type: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElCloudrunConfigElRef {
        DataContainerClusterAddonsConfigElCloudrunConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElConfigConnectorConfigEl {}

impl BuildDataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
        DataContainerClusterAddonsConfigElConfigConnectorConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
        DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElDnsCacheConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElDnsCacheConfigEl {}

impl BuildDataContainerClusterAddonsConfigElDnsCacheConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElDnsCacheConfigEl {
        DataContainerClusterAddonsConfigElDnsCacheConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
        DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {}

impl BuildDataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {}

impl BuildDataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {}

impl BuildDataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {}

impl BuildDataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
        DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
        DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {}

impl BuildDataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
        DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl { disabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
        DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElHttpLoadBalancingEl {}

impl BuildDataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
        DataContainerClusterAddonsConfigElHttpLoadBalancingEl { disabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
        DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigElNetworkPolicyConfigEl {}

impl BuildDataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
        DataContainerClusterAddonsConfigElNetworkPolicyConfigEl { disabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
        DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudrun_config: Option<ListField<DataContainerClusterAddonsConfigElCloudrunConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_connector_config: Option<ListField<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_cache_config: Option<ListField<DataContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_persistent_disk_csi_driver_config: Option<
        ListField<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_filestore_csi_driver_config: Option<ListField<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_fuse_csi_driver_config: Option<ListField<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_backup_agent_config: Option<ListField<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_pod_autoscaling: Option<ListField<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_load_balancing: Option<ListField<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_policy_config: Option<ListField<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
}

impl DataContainerClusterAddonsConfigEl {
    #[doc= "Set the field `cloudrun_config`.\n"]
    pub fn set_cloudrun_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElCloudrunConfigEl>>,
    ) -> Self {
        self.cloudrun_config = Some(v.into());
        self
    }

    #[doc= "Set the field `config_connector_config`.\n"]
    pub fn set_config_connector_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    ) -> Self {
        self.config_connector_config = Some(v.into());
        self
    }

    #[doc= "Set the field `dns_cache_config`.\n"]
    pub fn set_dns_cache_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    ) -> Self {
        self.dns_cache_config = Some(v.into());
        self
    }

    #[doc= "Set the field `gce_persistent_disk_csi_driver_config`.\n"]
    pub fn set_gce_persistent_disk_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>>,
    ) -> Self {
        self.gce_persistent_disk_csi_driver_config = Some(v.into());
        self
    }

    #[doc= "Set the field `gcp_filestore_csi_driver_config`.\n"]
    pub fn set_gcp_filestore_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    ) -> Self {
        self.gcp_filestore_csi_driver_config = Some(v.into());
        self
    }

    #[doc= "Set the field `gcs_fuse_csi_driver_config`.\n"]
    pub fn set_gcs_fuse_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    ) -> Self {
        self.gcs_fuse_csi_driver_config = Some(v.into());
        self
    }

    #[doc= "Set the field `gke_backup_agent_config`.\n"]
    pub fn set_gke_backup_agent_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    ) -> Self {
        self.gke_backup_agent_config = Some(v.into());
        self
    }

    #[doc= "Set the field `horizontal_pod_autoscaling`.\n"]
    pub fn set_horizontal_pod_autoscaling(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    ) -> Self {
        self.horizontal_pod_autoscaling = Some(v.into());
        self
    }

    #[doc= "Set the field `http_load_balancing`.\n"]
    pub fn set_http_load_balancing(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    ) -> Self {
        self.http_load_balancing = Some(v.into());
        self
    }

    #[doc= "Set the field `network_policy_config`.\n"]
    pub fn set_network_policy_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
    ) -> Self {
        self.network_policy_config = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAddonsConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAddonsConfigEl {}

impl BuildDataContainerClusterAddonsConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigEl {
        DataContainerClusterAddonsConfigEl {
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
        }
    }
}

pub struct DataContainerClusterAddonsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAddonsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElRef {
        DataContainerClusterAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAddonsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cloudrun_config` after provisioning.\n"]
    pub fn cloudrun_config(&self) -> ListRef<DataContainerClusterAddonsConfigElCloudrunConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudrun_config", self.base))
    }

    #[doc= "Get a reference to the value of field `config_connector_config` after provisioning.\n"]
    pub fn config_connector_config(&self) -> ListRef<DataContainerClusterAddonsConfigElConfigConnectorConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config_connector_config", self.base))
    }

    #[doc= "Get a reference to the value of field `dns_cache_config` after provisioning.\n"]
    pub fn dns_cache_config(&self) -> ListRef<DataContainerClusterAddonsConfigElDnsCacheConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_cache_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gce_persistent_disk_csi_driver_config` after provisioning.\n"]
    pub fn gce_persistent_disk_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gce_persistent_disk_csi_driver_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gcp_filestore_csi_driver_config` after provisioning.\n"]
    pub fn gcp_filestore_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcp_filestore_csi_driver_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gcs_fuse_csi_driver_config` after provisioning.\n"]
    pub fn gcs_fuse_csi_driver_config(&self) -> ListRef<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcs_fuse_csi_driver_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gke_backup_agent_config` after provisioning.\n"]
    pub fn gke_backup_agent_config(&self) -> ListRef<DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_backup_agent_config", self.base))
    }

    #[doc= "Get a reference to the value of field `horizontal_pod_autoscaling` after provisioning.\n"]
    pub fn horizontal_pod_autoscaling(&self) -> ListRef<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaling", self.base))
    }

    #[doc= "Get a reference to the value of field `http_load_balancing` after provisioning.\n"]
    pub fn http_load_balancing(&self) -> ListRef<DataContainerClusterAddonsConfigElHttpLoadBalancingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_load_balancing", self.base))
    }

    #[doc= "Get a reference to the value of field `network_policy_config` after provisioning.\n"]
    pub fn network_policy_config(&self) -> ListRef<DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_policy_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterAuthenticatorGroupsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group: Option<PrimField<String>>,
}

impl DataContainerClusterAuthenticatorGroupsConfigEl {
    #[doc= "Set the field `security_group`.\n"]
    pub fn set_security_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterAuthenticatorGroupsConfigEl {
    type O = BlockAssignable<DataContainerClusterAuthenticatorGroupsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterAuthenticatorGroupsConfigEl {}

impl BuildDataContainerClusterAuthenticatorGroupsConfigEl {
    pub fn build(self) -> DataContainerClusterAuthenticatorGroupsConfigEl {
        DataContainerClusterAuthenticatorGroupsConfigEl { security_group: core::default::Default::default() }
    }
}

pub struct DataContainerClusterAuthenticatorGroupsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterAuthenticatorGroupsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAuthenticatorGroupsConfigElRef {
        DataContainerClusterAuthenticatorGroupsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterAuthenticatorGroupsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<PrimField<String>>,
}

impl DataContainerClusterBinaryAuthorizationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `evaluation_mode`.\n"]
    pub fn set_evaluation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterBinaryAuthorizationEl {
    type O = BlockAssignable<DataContainerClusterBinaryAuthorizationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterBinaryAuthorizationEl {}

impl BuildDataContainerClusterBinaryAuthorizationEl {
    pub fn build(self) -> DataContainerClusterBinaryAuthorizationEl {
        DataContainerClusterBinaryAuthorizationEl {
            enabled: core::default::Default::default(),
            evaluation_mode: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterBinaryAuthorizationElRef {
        DataContainerClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `evaluation_mode` after provisioning.\n"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    #[doc= "Set the field `auto_upgrade_start_time`.\n"]
    pub fn set_auto_upgrade_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_upgrade_start_time = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    type O =
        BlockAssignable<
            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl {
            auto_upgrade_start_time: core::default::Default::default(),
            description: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_upgrade_start_time` after provisioning.\n"]
    pub fn auto_upgrade_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade_start_time", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_options: Option<
        ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl>,
    >,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    #[doc= "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }

    #[doc= "Set the field `upgrade_options`.\n"]
    pub fn set_upgrade_options(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl,
                        >,
                    >,
    ) -> Self {
        self.upgrade_options = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
            upgrade_options: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }

    #[doc= "Get a reference to the value of field `upgrade_options` after provisioning.\n"]
    pub fn upgrade_options(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    type O =
        BlockAssignable<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc= "Set the field `batch_node_count`.\n"]
    pub fn set_batch_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_percentage`.\n"]
    pub fn set_batch_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_soak_duration`.\n"]
    pub fn set_batch_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.batch_soak_duration = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O =
        BlockAssignable<
            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `batch_node_count` after provisioning.\n"]
    pub fn batch_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_percentage` after provisioning.\n"]
    pub fn batch_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_soak_duration` after provisioning.\n"]
    pub fn batch_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_soak_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_rollout_policy: Option<
        ListField<
            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
        >,
    >,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    #[doc= "Set the field `node_pool_soak_duration`.\n"]
    pub fn set_node_pool_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_pool_soak_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
                        >,
                    >,
    ) -> Self {
        self.standard_rollout_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    type O =
        BlockAssignable<
            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\n"]
    pub fn node_pool_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> ListRef<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_settings: Option<
        ListField<
            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    #[doc= "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl,
                        >,
                    >,
    ) -> Self {
        self.blue_green_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `max_surge`.\n"]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }

    #[doc= "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }

    #[doc= "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
            blue_green_settings: core::default::Default::default(),
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(
        &self,
    ) -> ListRef<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `max_surge` after provisioning.\n"]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc= "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc= "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<
        ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_settings: Option<
        ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>,
    >,
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    #[doc= "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size`.\n"]
    pub fn set_disk_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }

    #[doc= "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl>>,
    ) -> Self {
        self.management = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }

    #[doc= "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }

    #[doc= "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl,
                        >,
                    >,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }

    #[doc= "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl>>,
    ) -> Self {
        self.upgrade_settings = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {}

impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            image_type: core::default::Default::default(),
            management: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc= "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}

impl DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[doc= "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }

    #[doc= "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElResourceLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingElResourceLimitsEl {}

impl BuildDataContainerClusterClusterAutoscalingElResourceLimitsEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElResourceLimitsEl {
        DataContainerClusterClusterAutoscalingElResourceLimitsEl {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
        DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc= "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_provisioning_defaults: Option<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_limits: Option<ListField<DataContainerClusterClusterAutoscalingElResourceLimitsEl>>,
}

impl DataContainerClusterClusterAutoscalingEl {
    #[doc= "Set the field `auto_provisioning_defaults`.\n"]
    pub fn set_auto_provisioning_defaults(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    ) -> Self {
        self.auto_provisioning_defaults = Some(v.into());
        self
    }

    #[doc= "Set the field `autoscaling_profile`.\n"]
    pub fn set_autoscaling_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autoscaling_profile = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_limits`.\n"]
    pub fn set_resource_limits(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElResourceLimitsEl>>,
    ) -> Self {
        self.resource_limits = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterClusterAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterClusterAutoscalingEl {}

impl BuildDataContainerClusterClusterAutoscalingEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingEl {
        DataContainerClusterClusterAutoscalingEl {
            auto_provisioning_defaults: core::default::Default::default(),
            autoscaling_profile: core::default::Default::default(),
            enabled: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterClusterAutoscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterClusterAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterClusterAutoscalingElRef {
        DataContainerClusterClusterAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterClusterAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_provisioning_defaults` after provisioning.\n"]
    pub fn auto_provisioning_defaults(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_defaults", self.base))
    }

    #[doc= "Get a reference to the value of field `autoscaling_profile` after provisioning.\n"]
    pub fn autoscaling_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_profile", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(&self) -> ListRef<DataContainerClusterClusterAutoscalingElResourceLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterConfidentialNodesEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterConfidentialNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterConfidentialNodesEl {}

impl BuildDataContainerClusterConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterConfidentialNodesEl {
        DataContainerClusterConfidentialNodesEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterConfidentialNodesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterConfidentialNodesElRef {
        DataContainerClusterConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterCostManagementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterCostManagementConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterCostManagementConfigEl {
    type O = BlockAssignable<DataContainerClusterCostManagementConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterCostManagementConfigEl {}

impl BuildDataContainerClusterCostManagementConfigEl {
    pub fn build(self) -> DataContainerClusterCostManagementConfigEl {
        DataContainerClusterCostManagementConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterCostManagementConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterCostManagementConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterCostManagementConfigElRef {
        DataContainerClusterCostManagementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterCostManagementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterDatabaseEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl DataContainerClusterDatabaseEncryptionEl {
    #[doc= "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterDatabaseEncryptionEl {
    type O = BlockAssignable<DataContainerClusterDatabaseEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterDatabaseEncryptionEl {}

impl BuildDataContainerClusterDatabaseEncryptionEl {
    pub fn build(self) -> DataContainerClusterDatabaseEncryptionEl {
        DataContainerClusterDatabaseEncryptionEl {
            key_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterDatabaseEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterDatabaseEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDatabaseEncryptionElRef {
        DataContainerClusterDatabaseEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterDatabaseEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterDefaultSnatStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterDefaultSnatStatusEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterDefaultSnatStatusEl {
    type O = BlockAssignable<DataContainerClusterDefaultSnatStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterDefaultSnatStatusEl {}

impl BuildDataContainerClusterDefaultSnatStatusEl {
    pub fn build(self) -> DataContainerClusterDefaultSnatStatusEl {
        DataContainerClusterDefaultSnatStatusEl { disabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterDefaultSnatStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterDefaultSnatStatusElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDefaultSnatStatusElRef {
        DataContainerClusterDefaultSnatStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterDefaultSnatStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterDnsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns_scope: Option<PrimField<String>>,
}

impl DataContainerClusterDnsConfigEl {
    #[doc= "Set the field `cluster_dns`.\n"]
    pub fn set_cluster_dns(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_dns_domain`.\n"]
    pub fn set_cluster_dns_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_dns_scope`.\n"]
    pub fn set_cluster_dns_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns_scope = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterDnsConfigEl {
    type O = BlockAssignable<DataContainerClusterDnsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterDnsConfigEl {}

impl BuildDataContainerClusterDnsConfigEl {
    pub fn build(self) -> DataContainerClusterDnsConfigEl {
        DataContainerClusterDnsConfigEl {
            cluster_dns: core::default::Default::default(),
            cluster_dns_domain: core::default::Default::default(),
            cluster_dns_scope: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterDnsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterDnsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDnsConfigElRef {
        DataContainerClusterDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterDnsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_dns` after provisioning.\n"]
    pub fn cluster_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_dns_domain` after provisioning.\n"]
    pub fn cluster_dns_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_dns_scope` after provisioning.\n"]
    pub fn cluster_dns_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterEnableK8sBetaApisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_apis: Option<SetField<PrimField<String>>>,
}

impl DataContainerClusterEnableK8sBetaApisEl {
    #[doc= "Set the field `enabled_apis`.\n"]
    pub fn set_enabled_apis(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.enabled_apis = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterEnableK8sBetaApisEl {
    type O = BlockAssignable<DataContainerClusterEnableK8sBetaApisEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterEnableK8sBetaApisEl {}

impl BuildDataContainerClusterEnableK8sBetaApisEl {
    pub fn build(self) -> DataContainerClusterEnableK8sBetaApisEl {
        DataContainerClusterEnableK8sBetaApisEl { enabled_apis: core::default::Default::default() }
    }
}

pub struct DataContainerClusterEnableK8sBetaApisElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterEnableK8sBetaApisElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterEnableK8sBetaApisElRef {
        DataContainerClusterEnableK8sBetaApisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterEnableK8sBetaApisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled_apis` after provisioning.\n"]
    pub fn enabled_apis(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_apis", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_registered: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

impl DataContainerClusterFleetEl {
    #[doc= "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }

    #[doc= "Set the field `pre_registered`.\n"]
    pub fn set_pre_registered(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pre_registered = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterFleetEl {
    type O = BlockAssignable<DataContainerClusterFleetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterFleetEl {}

impl BuildDataContainerClusterFleetEl {
    pub fn build(self) -> DataContainerClusterFleetEl {
        DataContainerClusterFleetEl {
            membership: core::default::Default::default(),
            pre_registered: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterFleetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterFleetElRef {
        DataContainerClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc= "Get a reference to the value of field `pre_registered` after provisioning.\n"]
    pub fn pre_registered(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_registered", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterGatewayApiConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
}

impl DataContainerClusterGatewayApiConfigEl {
    #[doc= "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterGatewayApiConfigEl {
    type O = BlockAssignable<DataContainerClusterGatewayApiConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterGatewayApiConfigEl {}

impl BuildDataContainerClusterGatewayApiConfigEl {
    pub fn build(self) -> DataContainerClusterGatewayApiConfigEl {
        DataContainerClusterGatewayApiConfigEl { channel: core::default::Default::default() }
    }
}

pub struct DataContainerClusterGatewayApiConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterGatewayApiConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterGatewayApiConfigElRef {
        DataContainerClusterGatewayApiConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterGatewayApiConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterIdentityServiceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterIdentityServiceConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterIdentityServiceConfigEl {
    type O = BlockAssignable<DataContainerClusterIdentityServiceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterIdentityServiceConfigEl {}

impl BuildDataContainerClusterIdentityServiceConfigEl {
    pub fn build(self) -> DataContainerClusterIdentityServiceConfigEl {
        DataContainerClusterIdentityServiceConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterIdentityServiceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterIdentityServiceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterIdentityServiceConfigElRef {
        DataContainerClusterIdentityServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterIdentityServiceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range_names: Option<SetField<PrimField<String>>>,
}

impl DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    #[doc= "Set the field `pod_range_names`.\n"]
    pub fn set_pod_range_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.pod_range_names = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {}

impl BuildDataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
        DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
            pod_range_names: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
        DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pod_range_names` after provisioning.\n"]
    pub fn pod_range_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.pod_range_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {}

impl BuildDataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
        DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
        DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_pod_ranges_config: Option<ListField<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config: Option<
        ListField<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
}

impl DataContainerClusterIpAllocationPolicyEl {
    #[doc= "Set the field `additional_pod_ranges_config`.\n"]
    pub fn set_additional_pod_ranges_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    ) -> Self {
        self.additional_pod_ranges_config = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ipv4_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_secondary_range_name = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        self.pod_cidr_overprovision_config = Some(v.into());
        self
    }

    #[doc= "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_ipv4_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_secondary_range_name = Some(v.into());
        self
    }

    #[doc= "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterIpAllocationPolicyEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterIpAllocationPolicyEl {}

impl BuildDataContainerClusterIpAllocationPolicyEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyEl {
        DataContainerClusterIpAllocationPolicyEl {
            additional_pod_ranges_config: core::default::Default::default(),
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            stack_type: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterIpAllocationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterIpAllocationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterIpAllocationPolicyElRef {
        DataContainerClusterIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterIpAllocationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `additional_pod_ranges_config` after provisioning.\n"]
    pub fn additional_pod_ranges_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_pod_ranges_config", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }

    #[doc= "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_components: Option<ListField<PrimField<String>>>,
}

impl DataContainerClusterLoggingConfigEl {
    #[doc= "Set the field `enable_components`.\n"]
    pub fn set_enable_components(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enable_components = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterLoggingConfigEl {
    type O = BlockAssignable<DataContainerClusterLoggingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterLoggingConfigEl {}

impl BuildDataContainerClusterLoggingConfigEl {
    pub fn build(self) -> DataContainerClusterLoggingConfigEl {
        DataContainerClusterLoggingConfigEl { enable_components: core::default::Default::default() }
    }
}

pub struct DataContainerClusterLoggingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterLoggingConfigElRef {
        DataContainerClusterLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}

impl DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    #[doc= "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {}

impl BuildDataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
        DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
        DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}

impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {}

impl BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
            scope: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_options: Option<ListField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}

impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[doc= "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }

    #[doc= "Set the field `exclusion_name`.\n"]
    pub fn set_exclusion_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exclusion_name = Some(v.into());
        self
    }

    #[doc= "Set the field `exclusion_options`.\n"]
    pub fn set_exclusion_options(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>>,
    ) -> Self {
        self.exclusion_options = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {}

impl BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
            end_time: core::default::Default::default(),
            exclusion_name: core::default::Default::default(),
            exclusion_options: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc= "Get a reference to the value of field `exclusion_name` after provisioning.\n"]
    pub fn exclusion_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclusion_name", self.base))
    }

    #[doc= "Get a reference to the value of field `exclusion_options` after provisioning.\n"]
    pub fn exclusion_options(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion_options", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}

impl DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    #[doc= "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }

    #[doc= "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recurrence = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElRecurringWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMaintenancePolicyElRecurringWindowEl {}

impl BuildDataContainerClusterMaintenancePolicyElRecurringWindowEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElRecurringWindowEl {
        DataContainerClusterMaintenancePolicyElRecurringWindowEl {
            end_time: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
        DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc= "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_maintenance_window: Option<ListField<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_exclusion: Option<SetField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_window: Option<ListField<DataContainerClusterMaintenancePolicyElRecurringWindowEl>>,
}

impl DataContainerClusterMaintenancePolicyEl {
    #[doc= "Set the field `daily_maintenance_window`.\n"]
    pub fn set_daily_maintenance_window(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    ) -> Self {
        self.daily_maintenance_window = Some(v.into());
        self
    }

    #[doc= "Set the field `maintenance_exclusion`.\n"]
    pub fn set_maintenance_exclusion(
        mut self,
        v: impl Into<SetField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    ) -> Self {
        self.maintenance_exclusion = Some(v.into());
        self
    }

    #[doc= "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElRecurringWindowEl>>,
    ) -> Self {
        self.recurring_window = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMaintenancePolicyEl {}

impl BuildDataContainerClusterMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyEl {
        DataContainerClusterMaintenancePolicyEl {
            daily_maintenance_window: core::default::Default::default(),
            maintenance_exclusion: core::default::Default::default(),
            recurring_window: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMaintenancePolicyElRef {
        DataContainerClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `daily_maintenance_window` after provisioning.\n"]
    pub fn daily_maintenance_window(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.daily_maintenance_window", self.base))
    }

    #[doc= "Get a reference to the value of field `maintenance_exclusion` after provisioning.\n"]
    pub fn maintenance_exclusion(&self) -> SetRef<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.maintenance_exclusion", self.base))
    }

    #[doc= "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(&self) -> ListRef<DataContainerClusterMaintenancePolicyElRecurringWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthElClientCertificateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_client_certificate: Option<PrimField<bool>>,
}

impl DataContainerClusterMasterAuthElClientCertificateConfigEl {
    #[doc= "Set the field `issue_client_certificate`.\n"]
    pub fn set_issue_client_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.issue_client_certificate = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMasterAuthElClientCertificateConfigEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthElClientCertificateConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMasterAuthElClientCertificateConfigEl {}

impl BuildDataContainerClusterMasterAuthElClientCertificateConfigEl {
    pub fn build(self) -> DataContainerClusterMasterAuthElClientCertificateConfigEl {
        DataContainerClusterMasterAuthElClientCertificateConfigEl {
            issue_client_certificate: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMasterAuthElClientCertificateConfigElRef {
        DataContainerClusterMasterAuthElClientCertificateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `issue_client_certificate` after provisioning.\n"]
    pub fn issue_client_certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_client_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_config: Option<ListField<DataContainerClusterMasterAuthElClientCertificateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ca_certificate: Option<PrimField<String>>,
}

impl DataContainerClusterMasterAuthEl {
    #[doc= "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `client_certificate_config`.\n"]
    pub fn set_client_certificate_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterMasterAuthElClientCertificateConfigEl>>,
    ) -> Self {
        self.client_certificate_config = Some(v.into());
        self
    }

    #[doc= "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_ca_certificate`.\n"]
    pub fn set_cluster_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ca_certificate = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMasterAuthEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMasterAuthEl {}

impl BuildDataContainerClusterMasterAuthEl {
    pub fn build(self) -> DataContainerClusterMasterAuthEl {
        DataContainerClusterMasterAuthEl {
            client_certificate: core::default::Default::default(),
            client_certificate_config: core::default::Default::default(),
            client_key: core::default::Default::default(),
            cluster_ca_certificate: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMasterAuthElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMasterAuthElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMasterAuthElRef {
        DataContainerClusterMasterAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMasterAuthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `client_certificate_config` after provisioning.\n"]
    pub fn client_certificate_config(&self) -> ListRef<DataContainerClusterMasterAuthElClientCertificateConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_certificate_config", self.base))
    }

    #[doc= "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\n"]
    pub fn cluster_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}

impl DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc= "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {}

impl BuildDataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(self) -> DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
        DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthorizedNetworksConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks: Option<SetField<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_public_cidrs_access_enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterMasterAuthorizedNetworksConfigEl {
    #[doc= "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl Into<SetField<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    ) -> Self {
        self.cidr_blocks = Some(v.into());
        self
    }

    #[doc= "Set the field `gcp_public_cidrs_access_enabled`.\n"]
    pub fn set_gcp_public_cidrs_access_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gcp_public_cidrs_access_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMasterAuthorizedNetworksConfigEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthorizedNetworksConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMasterAuthorizedNetworksConfigEl {}

impl BuildDataContainerClusterMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> DataContainerClusterMasterAuthorizedNetworksConfigEl {
        DataContainerClusterMasterAuthorizedNetworksConfigEl {
            cidr_blocks: core::default::Default::default(),
            gcp_public_cidrs_access_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMasterAuthorizedNetworksConfigElRef {
        DataContainerClusterMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> SetRef<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef> {
        SetRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc= "Get a reference to the value of field `gcp_public_cidrs_access_enabled` after provisioning.\n"]
    pub fn gcp_public_cidrs_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_public_cidrs_access_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMeshCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_certificates: Option<PrimField<bool>>,
}

impl DataContainerClusterMeshCertificatesEl {
    #[doc= "Set the field `enable_certificates`.\n"]
    pub fn set_enable_certificates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_certificates = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMeshCertificatesEl {
    type O = BlockAssignable<DataContainerClusterMeshCertificatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMeshCertificatesEl {}

impl BuildDataContainerClusterMeshCertificatesEl {
    pub fn build(self) -> DataContainerClusterMeshCertificatesEl {
        DataContainerClusterMeshCertificatesEl { enable_certificates: core::default::Default::default() }
    }
}

pub struct DataContainerClusterMeshCertificatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMeshCertificatesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMeshCertificatesElRef {
        DataContainerClusterMeshCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMeshCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_certificates` after provisioning.\n"]
    pub fn enable_certificates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_metrics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relay_mode: Option<PrimField<String>>,
}

impl DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[doc= "Set the field `enable_metrics`.\n"]
    pub fn set_enable_metrics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_metrics = Some(v.into());
        self
    }

    #[doc= "Set the field `relay_mode`.\n"]
    pub fn set_relay_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.relay_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    type O = BlockAssignable<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {}

impl BuildDataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    pub fn build(self) -> DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
        DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
            enable_metrics: core::default::Default::default(),
            relay_mode: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
        DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_metrics` after provisioning.\n"]
    pub fn enable_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_metrics", self.base))
    }

    #[doc= "Get a reference to the value of field `relay_mode` after provisioning.\n"]
    pub fn relay_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.relay_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    type O = BlockAssignable<DataContainerClusterMonitoringConfigElManagedPrometheusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMonitoringConfigElManagedPrometheusEl {}

impl BuildDataContainerClusterMonitoringConfigElManagedPrometheusEl {
    pub fn build(self) -> DataContainerClusterMonitoringConfigElManagedPrometheusEl {
        DataContainerClusterMonitoringConfigElManagedPrometheusEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
        DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_datapath_observability_config: Option<
        ListField<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_components: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_prometheus: Option<ListField<DataContainerClusterMonitoringConfigElManagedPrometheusEl>>,
}

impl DataContainerClusterMonitoringConfigEl {
    #[doc= "Set the field `advanced_datapath_observability_config`.\n"]
    pub fn set_advanced_datapath_observability_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>>,
    ) -> Self {
        self.advanced_datapath_observability_config = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_components`.\n"]
    pub fn set_enable_components(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enable_components = Some(v.into());
        self
    }

    #[doc= "Set the field `managed_prometheus`.\n"]
    pub fn set_managed_prometheus(
        mut self,
        v: impl Into<ListField<DataContainerClusterMonitoringConfigElManagedPrometheusEl>>,
    ) -> Self {
        self.managed_prometheus = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterMonitoringConfigEl {
    type O = BlockAssignable<DataContainerClusterMonitoringConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterMonitoringConfigEl {}

impl BuildDataContainerClusterMonitoringConfigEl {
    pub fn build(self) -> DataContainerClusterMonitoringConfigEl {
        DataContainerClusterMonitoringConfigEl {
            advanced_datapath_observability_config: core::default::Default::default(),
            enable_components: core::default::Default::default(),
            managed_prometheus: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterMonitoringConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMonitoringConfigElRef {
        DataContainerClusterMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `advanced_datapath_observability_config` after provisioning.\n"]
    pub fn advanced_datapath_observability_config(
        &self,
    ) -> ListRef<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_datapath_observability_config", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }

    #[doc= "Get a reference to the value of field `managed_prometheus` after provisioning.\n"]
    pub fn managed_prometheus(&self) -> ListRef<DataContainerClusterMonitoringConfigElManagedPrometheusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNetworkPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<PrimField<String>>,
}

impl DataContainerClusterNetworkPolicyEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNetworkPolicyEl {
    type O = BlockAssignable<DataContainerClusterNetworkPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNetworkPolicyEl {}

impl BuildDataContainerClusterNetworkPolicyEl {
    pub fn build(self) -> DataContainerClusterNetworkPolicyEl {
        DataContainerClusterNetworkPolicyEl {
            enabled: core::default::Default::default(),
            provider: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNetworkPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNetworkPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNetworkPolicyElRef {
        DataContainerClusterNetworkPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNetworkPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    #[doc= "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {}

impl BuildDataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
        DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
        DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodeConfigElConfidentialNodesEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElConfidentialNodesEl {}

impl BuildDataContainerClusterNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElConfidentialNodesEl {
        DataContainerClusterNodeConfigElConfidentialNodesEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElConfidentialNodesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElConfidentialNodesElRef {
        DataContainerClusterNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElEffectiveTaintsEl {
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

impl ToListMappable for DataContainerClusterNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElEffectiveTaintsEl {}

impl BuildDataContainerClusterNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElEffectiveTaintsEl {
        DataContainerClusterNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElEffectiveTaintsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElEffectiveTaintsElRef {
        DataContainerClusterNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElEffectiveTaintsElRef {
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
pub struct DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}

impl DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {}

impl BuildDataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
        DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElFastSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodeConfigElFastSocketEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElFastSocketEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElFastSocketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElFastSocketEl {}

impl BuildDataContainerClusterNodeConfigElFastSocketEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElFastSocketEl {
        DataContainerClusterNodeConfigElFastSocketEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElFastSocketElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElFastSocketElRef {
        DataContainerClusterNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGcfsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodeConfigElGcfsConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElGcfsConfigEl {}

impl BuildDataContainerClusterNodeConfigElGcfsConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGcfsConfigEl {
        DataContainerClusterNodeConfigElGcfsConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElGcfsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElGcfsConfigElRef {
        DataContainerClusterNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_version: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc= "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_driver_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}

impl BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_shared_clients_per_gpu: Option<PrimField<f64>>,
}

impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
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

impl ToListMappable for DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}

impl BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
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
pub struct DataContainerClusterNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_installation_config: Option<
        ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_partition_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_config: Option<ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElGuestAcceleratorEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>>,
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
        v: impl Into<ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
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

impl ToListMappable for DataContainerClusterNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorEl {}

impl BuildDataContainerClusterNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGuestAcceleratorEl {
        DataContainerClusterNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElGuestAcceleratorElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElGuestAcceleratorElRef {
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
    ) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(&self) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGvnicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodeConfigElGvnicEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElGvnicEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGvnicEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElGvnicEl {}

impl BuildDataContainerClusterNodeConfigElGvnicEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGvnicEl {
        DataContainerClusterNodeConfigElGvnicEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElGvnicElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElGvnicElRef {
        DataContainerClusterNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_interval: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    #[doc= "Set the field `maintenance_interval`.\n"]
    pub fn set_maintenance_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_interval = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElHostMaintenancePolicyEl {}

impl BuildDataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
        DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
        DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maintenance_interval` after provisioning.\n"]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
}

impl DataContainerClusterNodeConfigElKubeletConfigEl {
    #[doc= "Set the field `cpu_cfs_quota`.\n"]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_cfs_quota_period`.\n"]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_manager_policy`.\n"]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_pids_limit`.\n"]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElKubeletConfigEl {}

impl BuildDataContainerClusterNodeConfigElKubeletConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigEl {
        DataContainerClusterNodeConfigElKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElKubeletConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElKubeletConfigElRef {
        DataContainerClusterNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\n"]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\n"]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_manager_policy` after provisioning.\n"]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_pids_limit` after provisioning.\n"]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
}

impl DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    #[doc= "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `sysctls`.\n"]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigEl {}

impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLinuxNodeConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `sysctls` after provisioning.\n"]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}

impl DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {}

impl BuildDataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
        DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
        DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataContainerClusterNodeConfigElReservationAffinityEl {
    #[doc= "Set the field `consume_reservation_type`.\n"]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElReservationAffinityEl {}

impl BuildDataContainerClusterNodeConfigElReservationAffinityEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElReservationAffinityEl {
        DataContainerClusterNodeConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElReservationAffinityElRef {
        DataContainerClusterNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consume_reservation_type` after provisioning.\n"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}

impl DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElShieldedInstanceConfigEl {}

impl BuildDataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
        DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
        DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}

impl DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {}

impl BuildDataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
        DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity: Option<SetField<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

impl DataContainerClusterNodeConfigElSoleTenantConfigEl {
    #[doc= "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<SetField<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        self.node_affinity = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElSoleTenantConfigEl {}

impl BuildDataContainerClusterNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSoleTenantConfigEl {
        DataContainerClusterNodeConfigElSoleTenantConfigEl { node_affinity: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElSoleTenantConfigElRef {
        DataContainerClusterNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_affinity` after provisioning.\n"]
    pub fn node_affinity(&self) -> SetRef<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_affinity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElTaintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElTaintEl {
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

impl ToListMappable for DataContainerClusterNodeConfigElTaintEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElTaintEl {}

impl BuildDataContainerClusterNodeConfigElTaintEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElTaintEl {
        DataContainerClusterNodeConfigElTaintEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElTaintElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElTaintElRef {
        DataContainerClusterNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElTaintElRef {
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
pub struct DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigElWorkloadMetadataConfigEl {}

impl BuildDataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
        DataContainerClusterNodeConfigElWorkloadMetadataConfigEl { mode: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
        DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<ListField<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes: Option<ListField<DataContainerClusterNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_taints: Option<ListField<DataContainerClusterNodeConfigElEffectiveTaintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config: Option<
        ListField<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<ListField<DataContainerClusterNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<ListField<DataContainerClusterNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<ListField<DataContainerClusterNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<ListField<DataContainerClusterNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy: Option<ListField<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<ListField<DataContainerClusterNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config: Option<ListField<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
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
    reservation_affinity: Option<ListField<DataContainerClusterNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<ListField<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config: Option<ListField<DataContainerClusterNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<ListField<DataContainerClusterNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config: Option<ListField<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
}

impl DataContainerClusterNodeConfigEl {
    #[doc= "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        self.advanced_machine_features = Some(v.into());
        self
    }

    #[doc= "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        self.confidential_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `effective_taints`.\n"]
    pub fn set_effective_taints(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElEffectiveTaintsEl>>,
    ) -> Self {
        self.effective_taints = Some(v.into());
        self
    }

    #[doc= "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        self.ephemeral_storage_local_ssd_config = Some(v.into());
        self
    }

    #[doc= "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(mut self, v: impl Into<ListField<DataContainerClusterNodeConfigElFastSocketEl>>) -> Self {
        self.fast_socket = Some(v.into());
        self
    }

    #[doc= "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(mut self, v: impl Into<ListField<DataContainerClusterNodeConfigElGcfsConfigEl>>) -> Self {
        self.gcfs_config = Some(v.into());
        self
    }

    #[doc= "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = Some(v.into());
        self
    }

    #[doc= "Set the field `gvnic`.\n"]
    pub fn set_gvnic(mut self, v: impl Into<ListField<DataContainerClusterNodeConfigElGvnicEl>>) -> Self {
        self.gvnic = Some(v.into());
        self
    }

    #[doc= "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        self.host_maintenance_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }

    #[doc= "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        self.kubelet_config = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        self.linux_node_config = Some(v.into());
        self
    }

    #[doc= "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        self.local_nvme_ssd_block_config = Some(v.into());
        self
    }

    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }

    #[doc= "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }

    #[doc= "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `node_group`.\n"]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }

    #[doc= "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }

    #[doc= "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }

    #[doc= "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        self.reservation_affinity = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }

    #[doc= "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }

    #[doc= "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        self.sole_tenant_config = Some(v.into());
        self
    }

    #[doc= "Set the field `spot`.\n"]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `taint`.\n"]
    pub fn set_taint(mut self, v: impl Into<ListField<DataContainerClusterNodeConfigElTaintEl>>) -> Self {
        self.taint = Some(v.into());
        self
    }

    #[doc= "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        self.workload_metadata_config = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodeConfigEl {}

impl BuildDataContainerClusterNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigEl {
        DataContainerClusterNodeConfigEl {
            advanced_machine_features: core::default::Default::default(),
            boot_disk_kms_key: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            effective_taints: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            image_type: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            spot: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElRef {
        DataContainerClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> ListRef<DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc= "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc= "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterNodeConfigElConfidentialNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `effective_taints` after provisioning.\n"]
    pub fn effective_taints(&self) -> ListRef<DataContainerClusterNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc= "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc= "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> ListRef<DataContainerClusterNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc= "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> ListRef<DataContainerClusterNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc= "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<DataContainerClusterNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc= "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(&self) -> ListRef<DataContainerClusterNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc= "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc= "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(&self) -> ListRef<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc= "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc= "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }

    #[doc= "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<DataContainerClusterNodeConfigElReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> ListRef<DataContainerClusterNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> ListRef<DataContainerClusterNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc= "Get a reference to the value of field `spot` after provisioning.\n"]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc= "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<DataContainerClusterNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc= "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> ListRef<DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElAutoscalingEl {
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

impl DataContainerClusterNodePoolElAutoscalingEl {
    #[doc= "Set the field `location_policy`.\n"]
    pub fn set_location_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `max_node_count`.\n"]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `min_node_count`.\n"]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `total_max_node_count`.\n"]
    pub fn set_total_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_max_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `total_min_node_count`.\n"]
    pub fn set_total_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_min_node_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElAutoscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElAutoscalingEl {}

impl BuildDataContainerClusterNodePoolElAutoscalingEl {
    pub fn build(self) -> DataContainerClusterNodePoolElAutoscalingEl {
        DataContainerClusterNodePoolElAutoscalingEl {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElAutoscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElAutoscalingElRef {
        DataContainerClusterNodePoolElAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `location_policy` after provisioning.\n"]
    pub fn location_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `total_max_node_count` after provisioning.\n"]
    pub fn total_max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_max_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `total_min_node_count` after provisioning.\n"]
    pub fn total_min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElManagementEl {
    #[doc= "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElManagementEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElManagementEl {}

impl BuildDataContainerClusterNodePoolElManagementEl {
    pub fn build(self) -> DataContainerClusterNodePoolElManagementEl {
        DataContainerClusterNodePoolElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElManagementElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElManagementElRef {
        DataContainerClusterNodePoolElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    #[doc= "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {}

impl BuildDataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
        DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {}

impl BuildDataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
        DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_pod_range: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config: Option<
        ListField<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config: Option<
        ListField<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNetworkConfigEl {
    #[doc= "Set the field `create_pod_range`.\n"]
    pub fn set_create_pod_range(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_pod_range = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>>,
    ) -> Self {
        self.network_performance_config = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        self.pod_cidr_overprovision_config = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_ipv4_cidr_block`.\n"]
    pub fn set_pod_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_ipv4_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_range`.\n"]
    pub fn set_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_range = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNetworkConfigEl {}

impl BuildDataContainerClusterNodePoolElNetworkConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNetworkConfigEl {
        DataContainerClusterNodePoolElNetworkConfigEl {
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNetworkConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_pod_range` after provisioning.\n"]
    pub fn create_pod_range(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_pod_range", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\n"]
    pub fn pod_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_ipv4_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_range` after provisioning.\n"]
    pub fn pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}

impl DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    #[doc= "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
        DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
        DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
        DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
        DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
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

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
        DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
        DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
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
pub struct DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}

impl DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
        DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElFastSocketEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
        DataContainerClusterNodePoolElNodeConfigElFastSocketEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
        DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
        DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_version: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc= "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_driver_version = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_shared_clients_per_gpu: Option<PrimField<f64>>,
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
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

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
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
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_installation_config: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_partition_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
                        >,
                    >,
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
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
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

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
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
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc= "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGvnicEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElGvnicEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElGvnicEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGvnicEl {
        DataContainerClusterNodePoolElNodeConfigElGvnicEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
        DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_interval: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    #[doc= "Set the field `maintenance_interval`.\n"]
    pub fn set_maintenance_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_interval = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
        DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
        DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maintenance_interval` after provisioning.\n"]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
}

impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[doc= "Set the field `cpu_cfs_quota`.\n"]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_cfs_quota_period`.\n"]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_manager_policy`.\n"]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `pod_pids_limit`.\n"]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\n"]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\n"]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_manager_policy` after provisioning.\n"]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `pod_pids_limit` after provisioning.\n"]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
}

impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    #[doc= "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `sysctls`.\n"]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            sysctls: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `sysctls` after provisioning.\n"]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}

impl DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    #[doc= "Set the field `consume_reservation_type`.\n"]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
        DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
        DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consume_reservation_type` after provisioning.\n"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}

impl DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
        DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}

impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity: Option<SetField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}

impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[doc= "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<SetField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        self.node_affinity = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
            node_affinity: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_affinity` after provisioning.\n"]
    pub fn node_affinity(&self) -> SetRef<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_affinity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElTaintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElTaintEl {
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

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElTaintEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElTaintEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElTaintEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElTaintEl {
        DataContainerClusterNodePoolElNodeConfigElTaintEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElTaintElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElTaintElRef {
        DataContainerClusterNodePoolElNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElTaintElRef {
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
pub struct DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
        DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
            mode: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<ListField<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes: Option<ListField<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_taints: Option<ListField<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<ListField<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy: Option<ListField<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
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
    reservation_affinity: Option<ListField<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<ListField<DataContainerClusterNodePoolElNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
}

impl DataContainerClusterNodePoolElNodeConfigEl {
    #[doc= "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        self.advanced_machine_features = Some(v.into());
        self
    }

    #[doc= "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        self.confidential_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `effective_taints`.\n"]
    pub fn set_effective_taints(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>>,
    ) -> Self {
        self.effective_taints = Some(v.into());
        self
    }

    #[doc= "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        self.ephemeral_storage_local_ssd_config = Some(v.into());
        self
    }

    #[doc= "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    ) -> Self {
        self.fast_socket = Some(v.into());
        self
    }

    #[doc= "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    ) -> Self {
        self.gcfs_config = Some(v.into());
        self
    }

    #[doc= "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = Some(v.into());
        self
    }

    #[doc= "Set the field `gvnic`.\n"]
    pub fn set_gvnic(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGvnicEl>>) -> Self {
        self.gvnic = Some(v.into());
        self
    }

    #[doc= "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        self.host_maintenance_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }

    #[doc= "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        self.kubelet_config = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        self.linux_node_config = Some(v.into());
        self
    }

    #[doc= "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        self.local_nvme_ssd_block_config = Some(v.into());
        self
    }

    #[doc= "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }

    #[doc= "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }

    #[doc= "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `node_group`.\n"]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }

    #[doc= "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }

    #[doc= "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }

    #[doc= "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        self.reservation_affinity = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }

    #[doc= "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }

    #[doc= "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        self.sole_tenant_config = Some(v.into());
        self
    }

    #[doc= "Set the field `spot`.\n"]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `taint`.\n"]
    pub fn set_taint(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElTaintEl>>) -> Self {
        self.taint = Some(v.into());
        self
    }

    #[doc= "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        self.workload_metadata_config = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElNodeConfigEl {}

impl BuildDataContainerClusterNodePoolElNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigEl {
        DataContainerClusterNodePoolElNodeConfigEl {
            advanced_machine_features: core::default::Default::default(),
            boot_disk_kms_key: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            effective_taints: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            image_type: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            spot: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc= "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc= "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `effective_taints` after provisioning.\n"]
    pub fn effective_taints(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc= "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc= "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc= "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc= "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc= "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc= "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc= "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc= "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc= "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }

    #[doc= "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc= "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc= "Get a reference to the value of field `spot` after provisioning.\n"]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc= "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc= "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tpu_topology: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElPlacementPolicyEl {
    #[doc= "Set the field `policy_name`.\n"]
    pub fn set_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_name = Some(v.into());
        self
    }

    #[doc= "Set the field `tpu_topology`.\n"]
    pub fn set_tpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tpu_topology = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElPlacementPolicyEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElPlacementPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElPlacementPolicyEl {}

impl BuildDataContainerClusterNodePoolElPlacementPolicyEl {
    pub fn build(self) -> DataContainerClusterNodePoolElPlacementPolicyEl {
        DataContainerClusterNodePoolElPlacementPolicyEl {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElPlacementPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElPlacementPolicyElRef {
        DataContainerClusterNodePoolElPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `policy_name` after provisioning.\n"]
    pub fn policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }

    #[doc= "Get a reference to the value of field `tpu_topology` after provisioning.\n"]
    pub fn tpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc= "Set the field `batch_node_count`.\n"]
    pub fn set_batch_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_percentage`.\n"]
    pub fn set_batch_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_soak_duration`.\n"]
    pub fn set_batch_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.batch_soak_duration = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {}

impl BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    pub fn build(self) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `batch_node_count` after provisioning.\n"]
    pub fn batch_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_percentage` after provisioning.\n"]
    pub fn batch_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_soak_duration` after provisioning.\n"]
    pub fn batch_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool_soak_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_rollout_policy: Option<
        ListField<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl>,
    >,
}

impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    #[doc= "Set the field `node_pool_soak_duration`.\n"]
    pub fn set_node_pool_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_pool_soak_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
                        >,
                    >,
    ) -> Self {
        self.standard_rollout_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {}

impl BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\n"]
    pub fn node_pool_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_settings: Option<ListField<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolElUpgradeSettingsEl {
    #[doc= "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
    ) -> Self {
        self.blue_green_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `max_surge`.\n"]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }

    #[doc= "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }

    #[doc= "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolElUpgradeSettingsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElUpgradeSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsEl {}

impl BuildDataContainerClusterNodePoolElUpgradeSettingsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElUpgradeSettingsEl {
        DataContainerClusterNodePoolElUpgradeSettingsEl {
            blue_green_settings: core::default::Default::default(),
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElUpgradeSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElUpgradeSettingsElRef {
        DataContainerClusterNodePoolElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(&self) -> ListRef<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `max_surge` after provisioning.\n"]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc= "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc= "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<ListField<DataContainerClusterNodePoolElAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_group_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_instance_group_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<DataContainerClusterNodePoolElManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<ListField<DataContainerClusterNodePoolElNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_policy: Option<ListField<DataContainerClusterNodePoolElPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_settings: Option<ListField<DataContainerClusterNodePoolElUpgradeSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolEl {
    #[doc= "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElAutoscalingEl>>) -> Self {
        self.autoscaling = Some(v.into());
        self
    }

    #[doc= "Set the field `initial_node_count`.\n"]
    pub fn set_initial_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_group_urls`.\n"]
    pub fn set_instance_group_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instance_group_urls = Some(v.into());
        self
    }

    #[doc= "Set the field `managed_instance_group_urls`.\n"]
    pub fn set_managed_instance_group_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.managed_instance_group_urls = Some(v.into());
        self
    }

    #[doc= "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElManagementEl>>) -> Self {
        self.management = Some(v.into());
        self
    }

    #[doc= "Set the field `max_pods_per_node`.\n"]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `network_config`.\n"]
    pub fn set_network_config(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElNetworkConfigEl>>) -> Self {
        self.network_config = Some(v.into());
        self
    }

    #[doc= "Set the field `node_config`.\n"]
    pub fn set_node_config(mut self, v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigEl>>) -> Self {
        self.node_config = Some(v.into());
        self
    }

    #[doc= "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `node_locations`.\n"]
    pub fn set_node_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.node_locations = Some(v.into());
        self
    }

    #[doc= "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElPlacementPolicyEl>>,
    ) -> Self {
        self.placement_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElUpgradeSettingsEl>>,
    ) -> Self {
        self.upgrade_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolEl {
    type O = BlockAssignable<DataContainerClusterNodePoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolEl {}

impl BuildDataContainerClusterNodePoolEl {
    pub fn build(self) -> DataContainerClusterNodePoolEl {
        DataContainerClusterNodePoolEl {
            autoscaling: core::default::Default::default(),
            initial_node_count: core::default::Default::default(),
            instance_group_urls: core::default::Default::default(),
            managed_instance_group_urls: core::default::Default::default(),
            management: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
            name: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
            network_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_locations: core::default::Default::default(),
            placement_policy: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElRef {
        DataContainerClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<DataContainerClusterNodePoolElAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc= "Get a reference to the value of field `initial_node_count` after provisioning.\n"]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_group_urls` after provisioning.\n"]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.base))
    }

    #[doc= "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\n"]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.base))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<DataContainerClusterNodePoolElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc= "Get a reference to the value of field `max_pods_per_node` after provisioning.\n"]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `node_locations` after provisioning.\n"]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_locations", self.base))
    }

    #[doc= "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<DataContainerClusterNodePoolElPlacementPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<DataContainerClusterNodePoolElUpgradeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}

impl DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolAutoConfigElNetworkTagsEl {}

impl BuildDataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
        DataContainerClusterNodePoolAutoConfigElNetworkTagsEl { tags: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
        DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
}

impl DataContainerClusterNodePoolAutoConfigEl {
    #[doc= "Set the field `network_tags`.\n"]
    pub fn set_network_tags(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
    ) -> Self {
        self.network_tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolAutoConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolAutoConfigEl {}

impl BuildDataContainerClusterNodePoolAutoConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigEl {
        DataContainerClusterNodePoolAutoConfigEl { network_tags: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolAutoConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolAutoConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolAutoConfigElRef {
        DataContainerClusterNodePoolAutoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolAutoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(&self) -> ListRef<DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
}

impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[doc= "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {}

impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    pub fn build(self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
            logging_variant: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config_defaults: Option<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
}

impl DataContainerClusterNodePoolDefaultsEl {
    #[doc= "Set the field `node_config_defaults`.\n"]
    pub fn set_node_config_defaults(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
    ) -> Self {
        self.node_config_defaults = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNodePoolDefaultsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNodePoolDefaultsEl {}

impl BuildDataContainerClusterNodePoolDefaultsEl {
    pub fn build(self) -> DataContainerClusterNodePoolDefaultsEl {
        DataContainerClusterNodePoolDefaultsEl { node_config_defaults: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNodePoolDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNodePoolDefaultsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolDefaultsElRef {
        DataContainerClusterNodePoolDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNodePoolDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `node_config_defaults` after provisioning.\n"]
    pub fn node_config_defaults(&self) -> ListRef<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config_defaults", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigElPubsubElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<ListField<PrimField<String>>>,
}

impl DataContainerClusterNotificationConfigElPubsubElFilterEl {
    #[doc= "Set the field `event_type`.\n"]
    pub fn set_event_type(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.event_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNotificationConfigElPubsubElFilterEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigElPubsubElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNotificationConfigElPubsubElFilterEl {}

impl BuildDataContainerClusterNotificationConfigElPubsubElFilterEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigElPubsubElFilterEl {
        DataContainerClusterNotificationConfigElPubsubElFilterEl { event_type: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNotificationConfigElPubsubElFilterElRef {
        DataContainerClusterNotificationConfigElPubsubElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigElPubsubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<ListField<DataContainerClusterNotificationConfigElPubsubElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}

impl DataContainerClusterNotificationConfigElPubsubEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<ListField<DataContainerClusterNotificationConfigElPubsubElFilterEl>>,
    ) -> Self {
        self.filter = Some(v.into());
        self
    }

    #[doc= "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNotificationConfigElPubsubEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigElPubsubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNotificationConfigElPubsubEl {}

impl BuildDataContainerClusterNotificationConfigElPubsubEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigElPubsubEl {
        DataContainerClusterNotificationConfigElPubsubEl {
            enabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterNotificationConfigElPubsubElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNotificationConfigElPubsubElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNotificationConfigElPubsubElRef {
        DataContainerClusterNotificationConfigElPubsubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNotificationConfigElPubsubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataContainerClusterNotificationConfigElPubsubElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub: Option<ListField<DataContainerClusterNotificationConfigElPubsubEl>>,
}

impl DataContainerClusterNotificationConfigEl {
    #[doc= "Set the field `pubsub`.\n"]
    pub fn set_pubsub(mut self, v: impl Into<ListField<DataContainerClusterNotificationConfigElPubsubEl>>) -> Self {
        self.pubsub = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterNotificationConfigEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterNotificationConfigEl {}

impl BuildDataContainerClusterNotificationConfigEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigEl {
        DataContainerClusterNotificationConfigEl { pubsub: core::default::Default::default() }
    }
}

pub struct DataContainerClusterNotificationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNotificationConfigElRef {
        DataContainerClusterNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> ListRef<DataContainerClusterNotificationConfigElPubsubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    type O = BlockAssignable<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {}

impl BuildDataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    pub fn build(self) -> DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
        DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
        DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterPrivateClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_global_access_config: Option<ListField<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoint_subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_endpoint: Option<PrimField<String>>,
}

impl DataContainerClusterPrivateClusterConfigEl {
    #[doc= "Set the field `enable_private_endpoint`.\n"]
    pub fn set_enable_private_endpoint(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `master_global_access_config`.\n"]
    pub fn set_master_global_access_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
    ) -> Self {
        self.master_global_access_config = Some(v.into());
        self
    }

    #[doc= "Set the field `master_ipv4_cidr_block`.\n"]
    pub fn set_master_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_ipv4_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `peering_name`.\n"]
    pub fn set_peering_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peering_name = Some(v.into());
        self
    }

    #[doc= "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `private_endpoint_subnetwork`.\n"]
    pub fn set_private_endpoint_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint_subnetwork = Some(v.into());
        self
    }

    #[doc= "Set the field `public_endpoint`.\n"]
    pub fn set_public_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterPrivateClusterConfigEl {
    type O = BlockAssignable<DataContainerClusterPrivateClusterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterPrivateClusterConfigEl {}

impl BuildDataContainerClusterPrivateClusterConfigEl {
    pub fn build(self) -> DataContainerClusterPrivateClusterConfigEl {
        DataContainerClusterPrivateClusterConfigEl {
            enable_private_endpoint: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            master_global_access_config: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            peering_name: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_subnetwork: core::default::Default::default(),
            public_endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterPrivateClusterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterPrivateClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterPrivateClusterConfigElRef {
        DataContainerClusterPrivateClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterPrivateClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_private_endpoint` after provisioning.\n"]
    pub fn enable_private_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `master_global_access_config` after provisioning.\n"]
    pub fn master_global_access_config(
        &self,
    ) -> ListRef<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_global_access_config", self.base))
    }

    #[doc= "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\n"]
    pub fn master_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `peering_name` after provisioning.\n"]
    pub fn peering_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering_name", self.base))
    }

    #[doc= "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `private_endpoint_subnetwork` after provisioning.\n"]
    pub fn private_endpoint_subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_subnetwork", self.base))
    }

    #[doc= "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterReleaseChannelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
}

impl DataContainerClusterReleaseChannelEl {
    #[doc= "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterReleaseChannelEl {
    type O = BlockAssignable<DataContainerClusterReleaseChannelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterReleaseChannelEl {}

impl BuildDataContainerClusterReleaseChannelEl {
    pub fn build(self) -> DataContainerClusterReleaseChannelEl {
        DataContainerClusterReleaseChannelEl { channel: core::default::Default::default() }
    }
}

pub struct DataContainerClusterReleaseChannelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterReleaseChannelElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterReleaseChannelElRef {
        DataContainerClusterReleaseChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterReleaseChannelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
}

impl DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    #[doc= "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    type O = BlockAssignable<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {}

impl BuildDataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    pub fn build(self) -> DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
        DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
            dataset_id: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
        DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterResourceUsageExportConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_destination: Option<ListField<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_egress_metering: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_consumption_metering: Option<PrimField<bool>>,
}

impl DataContainerClusterResourceUsageExportConfigEl {
    #[doc= "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v: impl Into<ListField<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
    ) -> Self {
        self.bigquery_destination = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_network_egress_metering`.\n"]
    pub fn set_enable_network_egress_metering(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_network_egress_metering = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_resource_consumption_metering`.\n"]
    pub fn set_enable_resource_consumption_metering(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_resource_consumption_metering = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterResourceUsageExportConfigEl {
    type O = BlockAssignable<DataContainerClusterResourceUsageExportConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterResourceUsageExportConfigEl {}

impl BuildDataContainerClusterResourceUsageExportConfigEl {
    pub fn build(self) -> DataContainerClusterResourceUsageExportConfigEl {
        DataContainerClusterResourceUsageExportConfigEl {
            bigquery_destination: core::default::Default::default(),
            enable_network_egress_metering: core::default::Default::default(),
            enable_resource_consumption_metering: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterResourceUsageExportConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterResourceUsageExportConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterResourceUsageExportConfigElRef {
        DataContainerClusterResourceUsageExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterResourceUsageExportConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> ListRef<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_network_egress_metering` after provisioning.\n"]
    pub fn enable_network_egress_metering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_network_egress_metering", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_resource_consumption_metering` after provisioning.\n"]
    pub fn enable_resource_consumption_metering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_consumption_metering", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterSecurityPostureConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_mode: Option<PrimField<String>>,
}

impl DataContainerClusterSecurityPostureConfigEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `vulnerability_mode`.\n"]
    pub fn set_vulnerability_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vulnerability_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterSecurityPostureConfigEl {
    type O = BlockAssignable<DataContainerClusterSecurityPostureConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterSecurityPostureConfigEl {}

impl BuildDataContainerClusterSecurityPostureConfigEl {
    pub fn build(self) -> DataContainerClusterSecurityPostureConfigEl {
        DataContainerClusterSecurityPostureConfigEl {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}

pub struct DataContainerClusterSecurityPostureConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterSecurityPostureConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterSecurityPostureConfigElRef {
        DataContainerClusterSecurityPostureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterSecurityPostureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `vulnerability_mode` after provisioning.\n"]
    pub fn vulnerability_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterServiceExternalIpsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterServiceExternalIpsConfigEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterServiceExternalIpsConfigEl {
    type O = BlockAssignable<DataContainerClusterServiceExternalIpsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterServiceExternalIpsConfigEl {}

impl BuildDataContainerClusterServiceExternalIpsConfigEl {
    pub fn build(self) -> DataContainerClusterServiceExternalIpsConfigEl {
        DataContainerClusterServiceExternalIpsConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterServiceExternalIpsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterServiceExternalIpsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterServiceExternalIpsConfigElRef {
        DataContainerClusterServiceExternalIpsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterServiceExternalIpsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterVerticalPodAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataContainerClusterVerticalPodAutoscalingEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterVerticalPodAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterVerticalPodAutoscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterVerticalPodAutoscalingEl {}

impl BuildDataContainerClusterVerticalPodAutoscalingEl {
    pub fn build(self) -> DataContainerClusterVerticalPodAutoscalingEl {
        DataContainerClusterVerticalPodAutoscalingEl { enabled: core::default::Default::default() }
    }
}

pub struct DataContainerClusterVerticalPodAutoscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterVerticalPodAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterVerticalPodAutoscalingElRef {
        DataContainerClusterVerticalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterVerticalPodAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_pool: Option<PrimField<String>>,
}

impl DataContainerClusterWorkloadIdentityConfigEl {
    #[doc= "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_pool = Some(v.into());
        self
    }
}

impl ToListMappable for DataContainerClusterWorkloadIdentityConfigEl {
    type O = BlockAssignable<DataContainerClusterWorkloadIdentityConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerClusterWorkloadIdentityConfigEl {}

impl BuildDataContainerClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> DataContainerClusterWorkloadIdentityConfigEl {
        DataContainerClusterWorkloadIdentityConfigEl { workload_pool: core::default::Default::default() }
    }
}

pub struct DataContainerClusterWorkloadIdentityConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerClusterWorkloadIdentityConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterWorkloadIdentityConfigElRef {
        DataContainerClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerClusterWorkloadIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workload_pool", self.base))
    }
}
