#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerClusterData_ {
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
    autopilot_privileged_admission: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datapath_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataplane_optimization_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_emulated_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_l4_lb_firewall_reconciliation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_autopilot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cilium_clusterwide_network_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_fqdn_network_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_intranode_visibility: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_kubernetes_alpha: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_l4_ilb_subsetting: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_legacy_abac: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multi_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_shielded_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_tpu: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_node_count_changes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    in_transit_encryption_config: std::option::Option<terrars::PrimField<std::string::String>>,
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
    skip_node_pool_refresh: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous_authentication_config: std::option::Option<std::vec::Vec<ContainerCluster_AnonymousAuthenticationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticator_groups_config: std::option::Option<std::vec::Vec<ContainerCluster_AuthenticatorGroupsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autopilot_cluster_policy_config: std::option::Option<std::vec::Vec<ContainerCluster_AutopilotClusterPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<ContainerCluster_BinaryAuthorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_autoscaling: std::option::Option<std::vec::Vec<ContainerCluster_ClusterAutoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerCluster_ConfidentialNodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_endpoints_config: std::option::Option<std::vec::Vec<ContainerCluster_ControlPlaneEndpointsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_management_config: std::option::Option<std::vec::Vec<ContainerCluster_CostManagementConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_encryption: std::option::Option<std::vec::Vec<ContainerCluster_DatabaseEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_snat_status: std::option::Option<std::vec::Vec<ContainerCluster_DefaultSnatStatus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_config: std::option::Option<std::vec::Vec<ContainerCluster_DnsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_beta_apis: std::option::Option<std::vec::Vec<ContainerCluster_EnableK8sBetaApis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enterprise_config: std::option::Option<std::vec::Vec<ContainerCluster_EnterpriseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<ContainerCluster_Fleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_api_config: std::option::Option<std::vec::Vec<ContainerCluster_GatewayApiConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_auto_upgrade_config: std::option::Option<std::vec::Vec<ContainerCluster_GkeAutoUpgradeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_service_config: std::option::Option<std::vec::Vec<ContainerCluster_IdentityServiceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation_policy: std::option::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<ContainerCluster_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_opentelemetry_config: std::option::Option<std::vec::Vec<ContainerCluster_ManagedOpentelemetryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_auth: std::option::Option<std::vec::Vec<ContainerCluster_MasterAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_authorized_networks_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MasterAuthorizedNetworksConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_certificates: std::option::Option<std::vec::Vec<ContainerCluster_MeshCertificates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_config: std::option::Option<std::vec::Vec<ContainerCluster_MonitoringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std::option::Option<std::vec::Vec<ContainerCluster_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_policy: std::option::Option<std::vec::Vec<ContainerCluster_NetworkPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_creation_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeCreationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool: std::option::Option<std::vec::Vec<ContainerCluster_NodePool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_auto_config: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolAutoConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_defaults: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<ContainerCluster_NotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_autoscaling: std::option::Option<std::vec::Vec<ContainerCluster_PodAutoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_cluster_config: std::option::Option<std::vec::Vec<ContainerCluster_PrivateClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rbac_binding_config: std::option::Option<std::vec::Vec<ContainerCluster_RbacBindingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_channel: std::option::Option<std::vec::Vec<ContainerCluster_ReleaseChannel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_usage_export_config: std::option::Option<std::vec::Vec<ContainerCluster_ResourceUsageExportConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_safe_upgrade: std::option::Option<std::vec::Vec<ContainerCluster_RollbackSafeUpgrade>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_config: std::option::Option<std::vec::Vec<ContainerCluster_SecretManagerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_sync_config: std::option::Option<std::vec::Vec<ContainerCluster_SecretSyncConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_posture_config: std::option::Option<std::vec::Vec<ContainerCluster_SecurityPostureConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_external_ips_config: std::option::Option<std::vec::Vec<ContainerCluster_ServiceExternalIpsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed_keys_config: std::option::Option<std::vec::Vec<ContainerCluster_UserManagedKeysConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertical_pod_autoscaling: std::option::Option<std::vec::Vec<ContainerCluster_VerticalPodAutoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identity_config: std::option::Option<std::vec::Vec<ContainerCluster_WorkloadIdentityConfig>>,
    dynamic: ContainerClusterDynamic,
}

struct ContainerCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerClusterData_>,
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
        "Set the field `autopilot_privileged_admission`.\nThe customer allowlist Cloud Storage paths for the cluster. These paths are used with the `--autopilot-privileged-admission` flag to authorize privileged workloads in Autopilot clusters. To allow default partner allowlists, set to []. To allow no allowlists, set to [\"\"]."]
    pub fn set_autopilot_privileged_admission(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().autopilot_privileged_admission = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `dataplane_optimization_mode`.\nThe desired dataplane optimization mode."]
    pub fn set_dataplane_optimization_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dataplane_optimization_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_max_pods_per_node`.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn set_default_max_pods_per_node(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhen the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the cluster will fail. When the field is set to false, deleting the cluster is allowed."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n Description of the cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_emulated_version`.\nThe desired emulated version for the cluster. Used to complete a rollback-safe upgrade after a soak period. Must be in major.minor format (e.g., \"1.31\"). To complete the upgrade declaratively, set this field to the target minor version. Removing this field from your configuration will not trigger completion."]
    pub fn set_desired_emulated_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().desired_emulated_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_l4_lb_firewall_reconciliation`.\nDisable L4 load balancer VPC firewalls to enable firewall policies."]
    pub fn set_disable_l4_lb_firewall_reconciliation(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().disable_l4_lb_firewall_reconciliation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_autopilot`.\nEnable Autopilot for this cluster."]
    pub fn set_enable_autopilot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_autopilot = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_cilium_clusterwide_network_policy`.\nWhether Cilium cluster-wide network policy is enabled on this cluster."]
    pub fn set_enable_cilium_clusterwide_network_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_cilium_clusterwide_network_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_fqdn_network_policy`.\nWhether FQDN Network Policy is enabled on this cluster."]
    pub fn set_enable_fqdn_network_policy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_fqdn_network_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `enable_multi_networking`.\nWhether multi-networking is enabled for this cluster."]
    pub fn set_enable_multi_networking(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_multi_networking = std::option::Option::Some(v.into());
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
        "Set the field `ignore_node_count_changes`.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state for all node pools in the cluster."]
    pub fn set_ignore_node_count_changes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_node_count_changes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `in_transit_encryption_config`.\nDefines the config of in-transit encryption"]
    pub fn set_in_transit_encryption_config(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().in_transit_encryption_config = std::option::Option::Some(v.into());
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
        "Set the field `resource_labels`.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_resource_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_node_pool_refresh`.\nIf true, the provider will not refresh the inline node_pool state from the API during cluster reads. Set this to true only when all node pools are managed via separate google_container_node_pool resources; it substantially improves plan/apply performance on clusters with a high node pool count. Must not be set to true when inline node_pool blocks are defined on this resource."]
    pub fn set_skip_node_pool_refresh(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_node_pool_refresh = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig>>,
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

    #[doc = "Set the field `anonymous_authentication_config`.\n"]
    pub fn set_anonymous_authentication_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AnonymousAuthenticationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anonymous_authentication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anonymous_authentication_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authenticator_groups_config`.\n"]
    pub fn set_authenticator_groups_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AuthenticatorGroupsConfig>>,
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

    #[doc = "Set the field `autopilot_cluster_policy_config`.\n"]
    pub fn set_autopilot_cluster_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AutopilotClusterPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autopilot_cluster_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autopilot_cluster_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_BinaryAuthorization>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ConfidentialNodes>>,
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

    #[doc = "Set the field `control_plane_endpoints_config`.\n"]
    pub fn set_control_plane_endpoints_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_endpoints_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_endpoints_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_management_config`.\n"]
    pub fn set_cost_management_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_CostManagementConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_DatabaseEncryption>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_DefaultSnatStatus>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_DnsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_EnableK8sBetaApis>>,
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

    #[doc = "Set the field `enterprise_config`.\n"]
    pub fn set_enterprise_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_EnterpriseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enterprise_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enterprise_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_Fleet>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_GatewayApiConfig>>,
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

    #[doc = "Set the field `gke_auto_upgrade_config`.\n"]
    pub fn set_gke_auto_upgrade_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_GkeAutoUpgradeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke_auto_upgrade_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke_auto_upgrade_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_service_config`.\n"]
    pub fn set_identity_service_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_IdentityServiceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_LoggingConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy>>,
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

    #[doc = "Set the field `managed_opentelemetry_config`.\n"]
    pub fn set_managed_opentelemetry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ManagedOpentelemetryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_opentelemetry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_opentelemetry_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_auth`.\n"]
    pub fn set_master_auth(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MasterAuth>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MasterAuthorizedNetworksConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MeshCertificates>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MonitoringConfig>>,
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

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NetworkPerformanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_policy`.\n"]
    pub fn set_network_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NetworkPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig>>,
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

    #[doc = "Set the field `node_creation_config`.\n"]
    pub fn set_node_creation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeCreationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_pool`.\n"]
    pub fn set_node_pool(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolDefaults>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NotificationConfig>>,
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

    #[doc = "Set the field `pod_autoscaling`.\n"]
    pub fn set_pod_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_PodAutoscaling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pod_autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pod_autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_cluster_config`.\n"]
    pub fn set_private_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_PrivateClusterConfig>>,
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

    #[doc = "Set the field `rbac_binding_config`.\n"]
    pub fn set_rbac_binding_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_RbacBindingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rbac_binding_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rbac_binding_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `release_channel`.\n"]
    pub fn set_release_channel(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ReleaseChannel>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ResourceUsageExportConfig>>,
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

    #[doc = "Set the field `rollback_safe_upgrade`.\n"]
    pub fn set_rollback_safe_upgrade(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_RollbackSafeUpgrade>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollback_safe_upgrade = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollback_safe_upgrade = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_manager_config`.\n"]
    pub fn set_secret_manager_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_SecretManagerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_manager_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_manager_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_sync_config`.\n"]
    pub fn set_secret_sync_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_SecretSyncConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_sync_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_sync_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_SecurityPostureConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ServiceExternalIpsConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed_keys_config`.\n"]
    pub fn set_user_managed_keys_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_UserManagedKeysConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_managed_keys_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_managed_keys_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vertical_pod_autoscaling`.\n"]
    pub fn set_vertical_pod_autoscaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_VerticalPodAutoscaling>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_WorkloadIdentityConfig>>,
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
        "Get a reference to the value of field `autopilot_privileged_admission` after provisioning.\nThe customer allowlist Cloud Storage paths for the cluster. These paths are used with the `--autopilot-privileged-admission` flag to authorize privileged workloads in Autopilot clusters. To allow default partner allowlists, set to []. To allow no allowlists, set to [\"\"]."]
    pub fn autopilot_privileged_admission(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_privileged_admission", self.extract_ref()),
        )
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
        "Get a reference to the value of field `dataplane_optimization_mode` after provisioning.\nThe desired dataplane optimization mode."]
    pub fn dataplane_optimization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataplane_optimization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhen the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the cluster will fail. When the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_emulated_version` after provisioning.\nThe desired emulated version for the cluster. Used to complete a rollback-safe upgrade after a soak period. Must be in major.minor format (e.g., \"1.31\"). To complete the upgrade declaratively, set this field to the target minor version. Removing this field from your configuration will not trigger completion."]
    pub fn desired_emulated_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_emulated_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_l4_lb_firewall_reconciliation` after provisioning.\nDisable L4 load balancer VPC firewalls to enable firewall policies."]
    pub fn disable_l4_lb_firewall_reconciliation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_l4_lb_firewall_reconciliation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `emulated_version` after provisioning.\nThe current emulated Kubernetes version running on the GKE cluster control plane."]
    pub fn emulated_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.emulated_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cilium_clusterwide_network_policy` after provisioning.\nWhether Cilium cluster-wide network policy is enabled on this cluster."]
    pub fn enable_cilium_clusterwide_network_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cilium_clusterwide_network_policy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_fqdn_network_policy` after provisioning.\nWhether FQDN Network Policy is enabled on this cluster."]
    pub fn enable_fqdn_network_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fqdn_network_policy", self.extract_ref()))
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
        "Get a reference to the value of field `enable_multi_networking` after provisioning.\nWhether multi-networking is enabled for this cluster."]
    pub fn enable_multi_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multi_networking", self.extract_ref()))
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
        "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state for all node pools in the cluster."]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `in_transit_encryption_config` after provisioning.\nDefines the config of in-transit encryption"]
    pub fn in_transit_encryption_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_transit_encryption_config", self.extract_ref()))
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
        "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Get a reference to the value of field `skip_node_pool_refresh` after provisioning.\nIf true, the provider will not refresh the inline node_pool state from the API during cluster reads. Set this to true only when all node pools are managed via separate google_container_node_pool resources; it substantially improves plan/apply performance on clusters with a high node pool count. Must not be set to true when inline node_pool blocks are defined on this resource."]
    pub fn skip_node_pool_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_node_pool_refresh", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\n"]
    pub fn anonymous_authentication_config(&self) -> terrars::ListRef<ContainerCluster_AnonymousAuthenticationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\n"]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<ContainerCluster_AuthenticatorGroupsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\n"]
    pub fn autopilot_cluster_policy_config(&self) -> terrars::ListRef<ContainerCluster_AutopilotClusterPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\n"]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<ContainerCluster_ClusterAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerCluster_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\n"]
    pub fn control_plane_endpoints_config(&self) -> terrars::ListRef<ContainerCluster_ControlPlaneEndpointsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\n"]
    pub fn cost_management_config(&self) -> terrars::ListRef<ContainerCluster_CostManagementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerCluster_DatabaseEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\n"]
    pub fn default_snat_status(&self) -> terrars::ListRef<ContainerCluster_DefaultSnatStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ContainerCluster_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\n"]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<ContainerCluster_EnableK8sBetaApisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enterprise_config` after provisioning.\n"]
    pub fn enterprise_config(&self) -> terrars::ListRef<ContainerCluster_EnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\n"]
    pub fn gateway_api_config(&self) -> terrars::ListRef<ContainerCluster_GatewayApiConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\n"]
    pub fn gke_auto_upgrade_config(&self) -> terrars::ListRef<ContainerCluster_GkeAutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\n"]
    pub fn identity_service_config(&self) -> terrars::ListRef<ContainerCluster_IdentityServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<ContainerCluster_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_opentelemetry_config` after provisioning.\n"]
    pub fn managed_opentelemetry_config(&self) -> terrars::ListRef<ContainerCluster_ManagedOpentelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_opentelemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\n"]
    pub fn master_auth(&self) -> terrars::ListRef<ContainerCluster_MasterAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\n"]
    pub fn mesh_certificates(&self) -> terrars::ListRef<ContainerCluster_MeshCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> terrars::ListRef<ContainerCluster_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\n"]
    pub fn network_policy(&self) -> terrars::ListRef<ContainerCluster_NetworkPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_creation_config` after provisioning.\n"]
    pub fn node_creation_config(&self) -> terrars::ListRef<ContainerCluster_NodeCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<ContainerCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\n"]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<ContainerCluster_NodePoolAutoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\n"]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<ContainerCluster_NodePoolDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<ContainerCluster_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pod_autoscaling` after provisioning.\n"]
    pub fn pod_autoscaling(&self) -> terrars::ListRef<ContainerCluster_PodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(&self) -> terrars::ListRef<ContainerCluster_PrivateClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rbac_binding_config` after provisioning.\n"]
    pub fn rbac_binding_config(&self) -> terrars::ListRef<ContainerCluster_RbacBindingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rbac_binding_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\n"]
    pub fn release_channel(&self) -> terrars::ListRef<ContainerCluster_ReleaseChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\n"]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<ContainerCluster_ResourceUsageExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_safe_upgrade` after provisioning.\n"]
    pub fn rollback_safe_upgrade(&self) -> terrars::ListRef<ContainerCluster_RollbackSafeUpgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_safe_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_manager_config` after provisioning.\n"]
    pub fn secret_manager_config(&self) -> terrars::ListRef<ContainerCluster_SecretManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_sync_config` after provisioning.\n"]
    pub fn secret_sync_config(&self) -> terrars::ListRef<ContainerCluster_SecretSyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_sync_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\n"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<ContainerCluster_ServiceExternalIpsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerCluster_TimeoutsRef {
        ContainerCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_managed_keys_config` after provisioning.\n"]
    pub fn user_managed_keys_config(&self) -> terrars::ListRef<ContainerCluster_UserManagedKeysConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed_keys_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\n"]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<ContainerCluster_VerticalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerCluster_WorkloadIdentityConfigRef> {
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
            data: std::cell::RefCell::new(ContainerClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_net_admin: core::default::Default::default(),
                autopilot_privileged_admission: core::default::Default::default(),
                cluster_ipv4_cidr: core::default::Default::default(),
                datapath_provider: core::default::Default::default(),
                dataplane_optimization_mode: core::default::Default::default(),
                default_max_pods_per_node: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_emulated_version: core::default::Default::default(),
                disable_l4_lb_firewall_reconciliation: core::default::Default::default(),
                enable_autopilot: core::default::Default::default(),
                enable_cilium_clusterwide_network_policy: core::default::Default::default(),
                enable_fqdn_network_policy: core::default::Default::default(),
                enable_intranode_visibility: core::default::Default::default(),
                enable_kubernetes_alpha: core::default::Default::default(),
                enable_l4_ilb_subsetting: core::default::Default::default(),
                enable_legacy_abac: core::default::Default::default(),
                enable_multi_networking: core::default::Default::default(),
                enable_shielded_nodes: core::default::Default::default(),
                enable_tpu: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_node_count_changes: core::default::Default::default(),
                in_transit_encryption_config: core::default::Default::default(),
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
                skip_node_pool_refresh: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                addons_config: core::default::Default::default(),
                anonymous_authentication_config: core::default::Default::default(),
                authenticator_groups_config: core::default::Default::default(),
                autopilot_cluster_policy_config: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                cluster_autoscaling: core::default::Default::default(),
                confidential_nodes: core::default::Default::default(),
                control_plane_endpoints_config: core::default::Default::default(),
                cost_management_config: core::default::Default::default(),
                database_encryption: core::default::Default::default(),
                default_snat_status: core::default::Default::default(),
                dns_config: core::default::Default::default(),
                enable_k8s_beta_apis: core::default::Default::default(),
                enterprise_config: core::default::Default::default(),
                fleet: core::default::Default::default(),
                gateway_api_config: core::default::Default::default(),
                gke_auto_upgrade_config: core::default::Default::default(),
                identity_service_config: core::default::Default::default(),
                ip_allocation_policy: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                managed_opentelemetry_config: core::default::Default::default(),
                master_auth: core::default::Default::default(),
                master_authorized_networks_config: core::default::Default::default(),
                mesh_certificates: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                network_performance_config: core::default::Default::default(),
                network_policy: core::default::Default::default(),
                node_config: core::default::Default::default(),
                node_creation_config: core::default::Default::default(),
                node_pool: core::default::Default::default(),
                node_pool_auto_config: core::default::Default::default(),
                node_pool_defaults: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                pod_autoscaling: core::default::Default::default(),
                private_cluster_config: core::default::Default::default(),
                rbac_binding_config: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                resource_usage_export_config: core::default::Default::default(),
                rollback_safe_upgrade: core::default::Default::default(),
                secret_manager_config: core::default::Default::default(),
                secret_sync_config: core::default::Default::default(),
                security_posture_config: core::default::Default::default(),
                service_external_ips_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_managed_keys_config: core::default::Default::default(),
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
        "Get a reference to the value of field `autopilot_privileged_admission` after provisioning.\nThe customer allowlist Cloud Storage paths for the cluster. These paths are used with the `--autopilot-privileged-admission` flag to authorize privileged workloads in Autopilot clusters. To allow default partner allowlists, set to []. To allow no allowlists, set to [\"\"]."]
    pub fn autopilot_privileged_admission(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_privileged_admission", self.extract_ref()),
        )
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
        "Get a reference to the value of field `dataplane_optimization_mode` after provisioning.\nThe desired dataplane optimization mode."]
    pub fn dataplane_optimization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataplane_optimization_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhen the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the cluster will fail. When the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_emulated_version` after provisioning.\nThe desired emulated version for the cluster. Used to complete a rollback-safe upgrade after a soak period. Must be in major.minor format (e.g., \"1.31\"). To complete the upgrade declaratively, set this field to the target minor version. Removing this field from your configuration will not trigger completion."]
    pub fn desired_emulated_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_emulated_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_l4_lb_firewall_reconciliation` after provisioning.\nDisable L4 load balancer VPC firewalls to enable firewall policies."]
    pub fn disable_l4_lb_firewall_reconciliation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_l4_lb_firewall_reconciliation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `emulated_version` after provisioning.\nThe current emulated Kubernetes version running on the GKE cluster control plane."]
    pub fn emulated_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.emulated_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_autopilot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cilium_clusterwide_network_policy` after provisioning.\nWhether Cilium cluster-wide network policy is enabled on this cluster."]
    pub fn enable_cilium_clusterwide_network_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cilium_clusterwide_network_policy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_fqdn_network_policy` after provisioning.\nWhether FQDN Network Policy is enabled on this cluster."]
    pub fn enable_fqdn_network_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fqdn_network_policy", self.extract_ref()))
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
        "Get a reference to the value of field `enable_multi_networking` after provisioning.\nWhether multi-networking is enabled for this cluster."]
    pub fn enable_multi_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multi_networking", self.extract_ref()))
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
        "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state for all node pools in the cluster."]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `in_transit_encryption_config` after provisioning.\nDefines the config of in-transit encryption"]
    pub fn in_transit_encryption_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_transit_encryption_config", self.extract_ref()))
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
        "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Get a reference to the value of field `skip_node_pool_refresh` after provisioning.\nIf true, the provider will not refresh the inline node_pool state from the API during cluster reads. Set this to true only when all node pools are managed via separate google_container_node_pool resources; it substantially improves plan/apply performance on clusters with a high node pool count. Must not be set to true when inline node_pool blocks are defined on this resource."]
    pub fn skip_node_pool_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_node_pool_refresh", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_ipv4_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\n"]
    pub fn anonymous_authentication_config(&self) -> terrars::ListRef<ContainerCluster_AnonymousAuthenticationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\n"]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<ContainerCluster_AuthenticatorGroupsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\n"]
    pub fn autopilot_cluster_policy_config(&self) -> terrars::ListRef<ContainerCluster_AutopilotClusterPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<ContainerCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\n"]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<ContainerCluster_ClusterAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerCluster_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\n"]
    pub fn control_plane_endpoints_config(&self) -> terrars::ListRef<ContainerCluster_ControlPlaneEndpointsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\n"]
    pub fn cost_management_config(&self) -> terrars::ListRef<ContainerCluster_CostManagementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(&self) -> terrars::ListRef<ContainerCluster_DatabaseEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\n"]
    pub fn default_snat_status(&self) -> terrars::ListRef<ContainerCluster_DefaultSnatStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ContainerCluster_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\n"]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<ContainerCluster_EnableK8sBetaApisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enterprise_config` after provisioning.\n"]
    pub fn enterprise_config(&self) -> terrars::ListRef<ContainerCluster_EnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<ContainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\n"]
    pub fn gateway_api_config(&self) -> terrars::ListRef<ContainerCluster_GatewayApiConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\n"]
    pub fn gke_auto_upgrade_config(&self) -> terrars::ListRef<ContainerCluster_GkeAutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\n"]
    pub fn identity_service_config(&self) -> terrars::ListRef<ContainerCluster_IdentityServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<ContainerCluster_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<ContainerCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_opentelemetry_config` after provisioning.\n"]
    pub fn managed_opentelemetry_config(&self) -> terrars::ListRef<ContainerCluster_ManagedOpentelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_opentelemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\n"]
    pub fn master_auth(&self) -> terrars::ListRef<ContainerCluster_MasterAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\n"]
    pub fn mesh_certificates(&self) -> terrars::ListRef<ContainerCluster_MeshCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(&self) -> terrars::ListRef<ContainerCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> terrars::ListRef<ContainerCluster_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\n"]
    pub fn network_policy(&self) -> terrars::ListRef<ContainerCluster_NetworkPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_creation_config` after provisioning.\n"]
    pub fn node_creation_config(&self) -> terrars::ListRef<ContainerCluster_NodeCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<ContainerCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\n"]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<ContainerCluster_NodePoolAutoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\n"]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<ContainerCluster_NodePoolDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<ContainerCluster_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pod_autoscaling` after provisioning.\n"]
    pub fn pod_autoscaling(&self) -> terrars::ListRef<ContainerCluster_PodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(&self) -> terrars::ListRef<ContainerCluster_PrivateClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rbac_binding_config` after provisioning.\n"]
    pub fn rbac_binding_config(&self) -> terrars::ListRef<ContainerCluster_RbacBindingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rbac_binding_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\n"]
    pub fn release_channel(&self) -> terrars::ListRef<ContainerCluster_ReleaseChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\n"]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<ContainerCluster_ResourceUsageExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_safe_upgrade` after provisioning.\n"]
    pub fn rollback_safe_upgrade(&self) -> terrars::ListRef<ContainerCluster_RollbackSafeUpgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_safe_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_manager_config` after provisioning.\n"]
    pub fn secret_manager_config(&self) -> terrars::ListRef<ContainerCluster_SecretManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_sync_config` after provisioning.\n"]
    pub fn secret_sync_config(&self) -> terrars::ListRef<ContainerCluster_SecretSyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_sync_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<ContainerCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\n"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<ContainerCluster_ServiceExternalIpsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerCluster_TimeoutsRef {
        ContainerCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_managed_keys_config` after provisioning.\n"]
    pub fn user_managed_keys_config(&self) -> terrars::ListRef<ContainerCluster_UserManagedKeysConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed_keys_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\n"]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<ContainerCluster_VerticalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ContainerCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerClusterDynamic {
    addons_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig>>,
    anonymous_authentication_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AnonymousAuthenticationConfig>>,
    authenticator_groups_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AuthenticatorGroupsConfig>>,
    autopilot_cluster_policy_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AutopilotClusterPolicyConfig>>,
    binary_authorization: std::option::Option<terrars::DynamicBlock<ContainerCluster_BinaryAuthorization>>,
    cluster_autoscaling: std::option::Option<terrars::DynamicBlock<ContainerCluster_ClusterAutoscaling>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerCluster_ConfidentialNodes>>,
    control_plane_endpoints_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ControlPlaneEndpointsConfig>>,
    cost_management_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_CostManagementConfig>>,
    database_encryption: std::option::Option<terrars::DynamicBlock<ContainerCluster_DatabaseEncryption>>,
    default_snat_status: std::option::Option<terrars::DynamicBlock<ContainerCluster_DefaultSnatStatus>>,
    dns_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_DnsConfig>>,
    enable_k8s_beta_apis: std::option::Option<terrars::DynamicBlock<ContainerCluster_EnableK8sBetaApis>>,
    enterprise_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_EnterpriseConfig>>,
    fleet: std::option::Option<terrars::DynamicBlock<ContainerCluster_Fleet>>,
    gateway_api_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_GatewayApiConfig>>,
    gke_auto_upgrade_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_GkeAutoUpgradeConfig>>,
    identity_service_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_IdentityServiceConfig>>,
    ip_allocation_policy: std::option::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy>>,
    logging_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_LoggingConfig>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy>>,
    managed_opentelemetry_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ManagedOpentelemetryConfig>>,
    master_auth: std::option::Option<terrars::DynamicBlock<ContainerCluster_MasterAuth>>,
    master_authorized_networks_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MasterAuthorizedNetworksConfig>>,
    mesh_certificates: std::option::Option<terrars::DynamicBlock<ContainerCluster_MeshCertificates>>,
    monitoring_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_MonitoringConfig>>,
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NetworkPerformanceConfig>>,
    network_policy: std::option::Option<terrars::DynamicBlock<ContainerCluster_NetworkPolicy>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig>>,
    node_creation_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeCreationConfig>>,
    node_pool: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool>>,
    node_pool_auto_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePoolAutoConfig>>,
    node_pool_defaults: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePoolDefaults>>,
    notification_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NotificationConfig>>,
    pod_autoscaling: std::option::Option<terrars::DynamicBlock<ContainerCluster_PodAutoscaling>>,
    private_cluster_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_PrivateClusterConfig>>,
    rbac_binding_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_RbacBindingConfig>>,
    release_channel: std::option::Option<terrars::DynamicBlock<ContainerCluster_ReleaseChannel>>,
    resource_usage_export_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ResourceUsageExportConfig>>,
    rollback_safe_upgrade: std::option::Option<terrars::DynamicBlock<ContainerCluster_RollbackSafeUpgrade>>,
    secret_manager_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_SecretManagerConfig>>,
    secret_sync_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_SecretSyncConfig>>,
    security_posture_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_SecurityPostureConfig>>,
    service_external_ips_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ServiceExternalIpsConfig>>,
    user_managed_keys_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_UserManagedKeysConfig>>,
    vertical_pod_autoscaling: std::option::Option<terrars::DynamicBlock<ContainerCluster_VerticalPodAutoscaling>>,
    workload_identity_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_WorkloadIdentityConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_AgentSandboxConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_AgentSandboxConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_AgentSandboxConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_AgentSandboxConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_AgentSandboxConfig {
    #[doc = "Whether the Agent Sandbox feature is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_AgentSandboxConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_AgentSandboxConfig {
        ContainerCluster_AddonsConfig_AgentSandboxConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_AgentSandboxConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_AgentSandboxConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_AgentSandboxConfigRef {
        ContainerCluster_AddonsConfig_AgentSandboxConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_AgentSandboxConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Agent Sandbox feature is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_CloudrunConfig {
    disabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_AddonsConfig_CloudrunConfig {
    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.load_balancer_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_CloudrunConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_CloudrunConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_CloudrunConfig {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_CloudrunConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_CloudrunConfig {
        ContainerCluster_AddonsConfig_CloudrunConfig {
            disabled: self.disabled,
            load_balancer_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_CloudrunConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_CloudrunConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_CloudrunConfigRef {
        ContainerCluster_AddonsConfig_CloudrunConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_CloudrunConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_AddonsConfig_ConfigConnectorConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_ConfigConnectorConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_ConfigConnectorConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_ConfigConnectorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_ConfigConnectorConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_ConfigConnectorConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_ConfigConnectorConfig {
        ContainerCluster_AddonsConfig_ConfigConnectorConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
        ContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_DnsCacheConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_DnsCacheConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_DnsCacheConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_DnsCacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_DnsCacheConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_DnsCacheConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_DnsCacheConfig {
        ContainerCluster_AddonsConfig_DnsCacheConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_DnsCacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_DnsCacheConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_DnsCacheConfigRef {
        ContainerCluster_AddonsConfig_DnsCacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_DnsCacheConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
        ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
        ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
        ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
        ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
        ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
        ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_GkeBackupAgentConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_GkeBackupAgentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_GkeBackupAgentConfig {
        ContainerCluster_AddonsConfig_GkeBackupAgentConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
        ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
        ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
        ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    disabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_HorizontalPodAutoscaling { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_HorizontalPodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    pub fn build(self) -> ContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
        ContainerCluster_AddonsConfig_HorizontalPodAutoscaling { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
        ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_HttpLoadBalancing {
    disabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_HttpLoadBalancing { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_HttpLoadBalancing {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_HttpLoadBalancing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_HttpLoadBalancing {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_HttpLoadBalancing {
    pub fn build(self) -> ContainerCluster_AddonsConfig_HttpLoadBalancing {
        ContainerCluster_AddonsConfig_HttpLoadBalancing { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_HttpLoadBalancingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_HttpLoadBalancingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_HttpLoadBalancingRef {
        ContainerCluster_AddonsConfig_HttpLoadBalancingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_HttpLoadBalancingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_multi_nic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_legacy_lustre_port: std::option::Option<terrars::PrimField<bool>>,
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    #[doc =
        "Set the field `disable_multi_nic`.\nWhen set to true, this disables multi-NIC support for the Lustre CSI driver. By default, GKE enables multi-NIC support, which\n\t\t\t\t\t\t\t\t\t\tallows the Lustre CSI driver to automatically detect and configure all suitable network interfaces on a node to maximize I/O performance for demanding workloads."]
    pub fn set_disable_multi_nic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_multi_nic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_legacy_lustre_port`.\nIf set to true, the Lustre CSI driver will initialize LNet (the virtual network layer for Lustre kernel module) using port 6988.\n\t\t\t\t\t\t\t\t\t\tThis flag is required to workaround a port conflict with the gke-metadata-server on GKE nodes."]
    pub fn set_enable_legacy_lustre_port(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_legacy_lustre_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_LustreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    #[doc = "Whether the Lustre CSI driver is enabled for this cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_LustreCsiDriverConfig {
        ContainerCluster_AddonsConfig_LustreCsiDriverConfig {
            disable_multi_nic: core::default::Default::default(),
            enable_legacy_lustre_port: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
        ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_multi_nic` after provisioning.\nWhen set to true, this disables multi-NIC support for the Lustre CSI driver. By default, GKE enables multi-NIC support, which\n\t\t\t\t\t\t\t\t\t\tallows the Lustre CSI driver to automatically detect and configure all suitable network interfaces on a node to maximize I/O performance for demanding workloads."]
    pub fn disable_multi_nic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_multi_nic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_legacy_lustre_port` after provisioning.\nIf set to true, the Lustre CSI driver will initialize LNet (the virtual network layer for Lustre kernel module) using port 6988.\n\t\t\t\t\t\t\t\t\t\tThis flag is required to workaround a port conflict with the gke-metadata-server on GKE nodes."]
    pub fn enable_legacy_lustre_port(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_lustre_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Lustre CSI driver is enabled for this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_NetworkPolicyConfig {
    disabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_NetworkPolicyConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_NetworkPolicyConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_NetworkPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_NetworkPolicyConfig {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_NetworkPolicyConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_NetworkPolicyConfig {
        ContainerCluster_AddonsConfig_NetworkPolicyConfig { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
        ContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_NodeReadinessConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_NodeReadinessConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_NodeReadinessConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_NodeReadinessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_NodeReadinessConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_NodeReadinessConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_NodeReadinessConfig {
        ContainerCluster_AddonsConfig_NodeReadinessConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_NodeReadinessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_NodeReadinessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_NodeReadinessConfigRef {
        ContainerCluster_AddonsConfig_NodeReadinessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_NodeReadinessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
        ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
        ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_PodSnapshotConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_PodSnapshotConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_PodSnapshotConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_PodSnapshotConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_PodSnapshotConfig {
    #[doc = "Whether the Pod Snapshot feature is enabled for this cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_PodSnapshotConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_PodSnapshotConfig {
        ContainerCluster_AddonsConfig_PodSnapshotConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_PodSnapshotConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_PodSnapshotConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_PodSnapshotConfigRef {
        ContainerCluster_AddonsConfig_PodSnapshotConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_PodSnapshotConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Pod Snapshot feature is enabled for this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
        ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
        ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
        ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
        ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_AddonsConfig_RayOperatorConfigDynamic {
    ray_cluster_logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>>,
    ray_cluster_monitoring_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_cluster_logging_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_cluster_monitoring_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>>,
    dynamic: ContainerCluster_AddonsConfig_RayOperatorConfigDynamic,
}

impl ContainerCluster_AddonsConfig_RayOperatorConfig {
    #[doc = "Set the field `ray_cluster_logging_config`.\n"]
    pub fn set_ray_cluster_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ray_cluster_logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ray_cluster_logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ray_cluster_monitoring_config`.\n"]
    pub fn set_ray_cluster_monitoring_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ray_cluster_monitoring_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ray_cluster_monitoring_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_RayOperatorConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_RayOperatorConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_RayOperatorConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_RayOperatorConfig {
        ContainerCluster_AddonsConfig_RayOperatorConfig {
            enabled: self.enabled,
            ray_cluster_logging_config: core::default::Default::default(),
            ray_cluster_monitoring_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_RayOperatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_RayOperatorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_RayOperatorConfigRef {
        ContainerCluster_AddonsConfig_RayOperatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_RayOperatorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ray_cluster_logging_config` after provisioning.\n"]
    pub fn ray_cluster_logging_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_cluster_logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ray_cluster_monitoring_config` after provisioning.\n"]
    pub fn ray_cluster_monitoring_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_cluster_monitoring_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_SliceControllerConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_SliceControllerConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_SliceControllerConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_SliceControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_SliceControllerConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_SliceControllerConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_SliceControllerConfig {
        ContainerCluster_AddonsConfig_SliceControllerConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_SliceControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_SliceControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_SliceControllerConfigRef {
        ContainerCluster_AddonsConfig_SliceControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_SliceControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_SlurmOperatorConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_SlurmOperatorConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_SlurmOperatorConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_SlurmOperatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_SlurmOperatorConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_SlurmOperatorConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_SlurmOperatorConfig {
        ContainerCluster_AddonsConfig_SlurmOperatorConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
        ContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig_StatefulHaConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_AddonsConfig_StatefulHaConfig { }

impl terrars::ToListMappable for ContainerCluster_AddonsConfig_StatefulHaConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig_StatefulHaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig_StatefulHaConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_AddonsConfig_StatefulHaConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig_StatefulHaConfig {
        ContainerCluster_AddonsConfig_StatefulHaConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfig_StatefulHaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfig_StatefulHaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AddonsConfig_StatefulHaConfigRef {
        ContainerCluster_AddonsConfig_StatefulHaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfig_StatefulHaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_AddonsConfigDynamic {
    agent_sandbox_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_AgentSandboxConfig>>,
    cloudrun_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_CloudrunConfig>>,
    config_connector_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_ConfigConnectorConfig>>,
    dns_cache_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_DnsCacheConfig>>,
    gce_persistent_disk_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>>,
    gcp_filestore_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>>,
    gcs_fuse_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>>,
    gke_backup_agent_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_GkeBackupAgentConfig>>,
    high_scale_checkpointing_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>>,
    horizontal_pod_autoscaling: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_HorizontalPodAutoscaling>>,
    http_load_balancing: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_HttpLoadBalancing>>,
    lustre_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_LustreCsiDriverConfig>>,
    network_policy_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_NetworkPolicyConfig>>,
    node_readiness_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_NodeReadinessConfig>>,
    parallelstore_csi_driver_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>>,
    pod_snapshot_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_PodSnapshotConfig>>,
    ray_operator_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_RayOperatorConfig>>,
    slice_controller_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_SliceControllerConfig>>,
    slurm_operator_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_SlurmOperatorConfig>>,
    stateful_ha_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_AddonsConfig_StatefulHaConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_AddonsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_sandbox_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_AgentSandboxConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudrun_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_CloudrunConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_connector_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_ConfigConnectorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_cache_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_DnsCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_persistent_disk_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_filestore_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_fuse_csi_driver_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_backup_agent_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_GkeBackupAgentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_scale_checkpointing_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaling: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_HorizontalPodAutoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_load_balancing: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_HttpLoadBalancing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lustre_csi_driver_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_LustreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_policy_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_NetworkPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_readiness_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_NodeReadinessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelstore_csi_driver_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_snapshot_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_PodSnapshotConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_operator_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_RayOperatorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slice_controller_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_SliceControllerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slurm_operator_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_SlurmOperatorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_ha_config: std::option::Option<std::vec::Vec<ContainerCluster_AddonsConfig_StatefulHaConfig>>,
    dynamic: ContainerCluster_AddonsConfigDynamic,
}

impl ContainerCluster_AddonsConfig {
    #[doc = "Set the field `agent_sandbox_config`.\n"]
    pub fn set_agent_sandbox_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_AgentSandboxConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_sandbox_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_sandbox_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudrun_config`.\n"]
    pub fn set_cloudrun_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_CloudrunConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_ConfigConnectorConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_DnsCacheConfig>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_GkeBackupAgentConfig>>,
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

    #[doc = "Set the field `high_scale_checkpointing_config`.\n"]
    pub fn set_high_scale_checkpointing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.high_scale_checkpointing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.high_scale_checkpointing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `horizontal_pod_autoscaling`.\n"]
    pub fn set_horizontal_pod_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_HorizontalPodAutoscaling>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_HttpLoadBalancing>>,
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

    #[doc = "Set the field `lustre_csi_driver_config`.\n"]
    pub fn set_lustre_csi_driver_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_LustreCsiDriverConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lustre_csi_driver_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lustre_csi_driver_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_policy_config`.\n"]
    pub fn set_network_policy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_NetworkPolicyConfig>>,
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

    #[doc = "Set the field `node_readiness_config`.\n"]
    pub fn set_node_readiness_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_NodeReadinessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_readiness_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_readiness_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelstore_csi_driver_config`.\n"]
    pub fn set_parallelstore_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parallelstore_csi_driver_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelstore_csi_driver_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pod_snapshot_config`.\n"]
    pub fn set_pod_snapshot_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_PodSnapshotConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_snapshot_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_snapshot_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ray_operator_config`.\n"]
    pub fn set_ray_operator_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_RayOperatorConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ray_operator_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ray_operator_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slice_controller_config`.\n"]
    pub fn set_slice_controller_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_SliceControllerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slice_controller_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slice_controller_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slurm_operator_config`.\n"]
    pub fn set_slurm_operator_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_SlurmOperatorConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slurm_operator_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slurm_operator_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_ha_config`.\n"]
    pub fn set_stateful_ha_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_AddonsConfig_StatefulHaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateful_ha_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_ha_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_AddonsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AddonsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AddonsConfig {}

impl BuildContainerCluster_AddonsConfig {
    pub fn build(self) -> ContainerCluster_AddonsConfig {
        ContainerCluster_AddonsConfig {
            agent_sandbox_config: core::default::Default::default(),
            cloudrun_config: core::default::Default::default(),
            config_connector_config: core::default::Default::default(),
            dns_cache_config: core::default::Default::default(),
            gce_persistent_disk_csi_driver_config: core::default::Default::default(),
            gcp_filestore_csi_driver_config: core::default::Default::default(),
            gcs_fuse_csi_driver_config: core::default::Default::default(),
            gke_backup_agent_config: core::default::Default::default(),
            high_scale_checkpointing_config: core::default::Default::default(),
            horizontal_pod_autoscaling: core::default::Default::default(),
            http_load_balancing: core::default::Default::default(),
            lustre_csi_driver_config: core::default::Default::default(),
            network_policy_config: core::default::Default::default(),
            node_readiness_config: core::default::Default::default(),
            parallelstore_csi_driver_config: core::default::Default::default(),
            pod_snapshot_config: core::default::Default::default(),
            ray_operator_config: core::default::Default::default(),
            slice_controller_config: core::default::Default::default(),
            slurm_operator_config: core::default::Default::default(),
            stateful_ha_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AddonsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AddonsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_AddonsConfigRef {
        ContainerCluster_AddonsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AddonsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_sandbox_config` after provisioning.\n"]
    pub fn agent_sandbox_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_AgentSandboxConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_sandbox_config", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudrun_config` after provisioning.\n"]
    pub fn cloudrun_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_CloudrunConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudrun_config", self.base))
    }

    #[doc = "Get a reference to the value of field `config_connector_config` after provisioning.\n"]
    pub fn config_connector_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_ConfigConnectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_connector_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_cache_config` after provisioning.\n"]
    pub fn dns_cache_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_DnsCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_persistent_disk_csi_driver_config` after provisioning.\n"]
    pub fn gce_persistent_disk_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_persistent_disk_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_filestore_csi_driver_config` after provisioning.\n"]
    pub fn gcp_filestore_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_filestore_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_fuse_csi_driver_config` after provisioning.\n"]
    pub fn gcs_fuse_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_fuse_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_backup_agent_config` after provisioning.\n"]
    pub fn gke_backup_agent_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_GkeBackupAgentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_backup_agent_config", self.base))
    }

    #[doc = "Get a reference to the value of field `high_scale_checkpointing_config` after provisioning.\n"]
    pub fn high_scale_checkpointing_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.high_scale_checkpointing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaling` after provisioning.\n"]
    pub fn horizontal_pod_autoscaling(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `http_load_balancing` after provisioning.\n"]
    pub fn http_load_balancing(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_HttpLoadBalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_load_balancing", self.base))
    }

    #[doc = "Get a reference to the value of field `lustre_csi_driver_config` after provisioning.\n"]
    pub fn lustre_csi_driver_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_LustreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lustre_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `network_policy_config` after provisioning.\n"]
    pub fn network_policy_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_NetworkPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_readiness_config` after provisioning.\n"]
    pub fn node_readiness_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_NodeReadinessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_readiness_config", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelstore_csi_driver_config` after provisioning.\n"]
    pub fn parallelstore_csi_driver_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelstore_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_snapshot_config` after provisioning.\n"]
    pub fn pod_snapshot_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_PodSnapshotConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_snapshot_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ray_operator_config` after provisioning.\n"]
    pub fn ray_operator_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_RayOperatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_operator_config", self.base))
    }

    #[doc = "Get a reference to the value of field `slice_controller_config` after provisioning.\n"]
    pub fn slice_controller_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_SliceControllerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slice_controller_config", self.base))
    }

    #[doc = "Get a reference to the value of field `slurm_operator_config` after provisioning.\n"]
    pub fn slurm_operator_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_SlurmOperatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slurm_operator_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_ha_config` after provisioning.\n"]
    pub fn stateful_ha_config(&self) -> terrars::ListRef<ContainerCluster_AddonsConfig_StatefulHaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_ha_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AnonymousAuthenticationConfig {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_AnonymousAuthenticationConfig { }

impl terrars::ToListMappable for ContainerCluster_AnonymousAuthenticationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AnonymousAuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AnonymousAuthenticationConfig {
    #[doc =
        "Setting this to LIMITED will restrict authentication of anonymous users to health check endpoints only.\n Accepted values are:\n* ENABLED: Authentication of anonymous users is enabled for all endpoints.\n* LIMITED: Anonymous access is only allowed for health check endpoints."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_AnonymousAuthenticationConfig {
    pub fn build(self) -> ContainerCluster_AnonymousAuthenticationConfig {
        ContainerCluster_AnonymousAuthenticationConfig { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AnonymousAuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AnonymousAuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AnonymousAuthenticationConfigRef {
        ContainerCluster_AnonymousAuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AnonymousAuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSetting this to LIMITED will restrict authentication of anonymous users to health check endpoints only.\n Accepted values are:\n* ENABLED: Authentication of anonymous users is enabled for all endpoints.\n* LIMITED: Anonymous access is only allowed for health check endpoints."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_AuthenticatorGroupsConfig {
    security_group: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_AuthenticatorGroupsConfig { }

impl terrars::ToListMappable for ContainerCluster_AuthenticatorGroupsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AuthenticatorGroupsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AuthenticatorGroupsConfig {
    #[doc =
        "The name of the RBAC security group for use with Google security groups in Kubernetes RBAC. Group name must be in format gke-security-groups@yourdomain.com."]
    pub security_group: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_AuthenticatorGroupsConfig {
    pub fn build(self) -> ContainerCluster_AuthenticatorGroupsConfig {
        ContainerCluster_AuthenticatorGroupsConfig { security_group: self.security_group }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AuthenticatorGroupsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AuthenticatorGroupsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_AuthenticatorGroupsConfigRef {
        ContainerCluster_AuthenticatorGroupsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AuthenticatorGroupsConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_AutopilotClusterPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_standard_node_pools: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_system_impersonation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_system_mutation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_unsafe_webhooks: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_AutopilotClusterPolicyConfig {
    #[doc =
        "Set the field `no_standard_node_pools`.\nIf true, prevents standard node pools and requires only autopilot node pools."]
    pub fn set_no_standard_node_pools(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_standard_node_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_system_impersonation`.\nIf true, prevents impersonation and CSRs for GKE System users."]
    pub fn set_no_system_impersonation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_system_impersonation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_system_mutation`.\nIf true, prevents creation and mutation of resources in GKE managed namespaces and cluster-scoped GKE managed resources."]
    pub fn set_no_system_mutation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_system_mutation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_unsafe_webhooks`.\nIf true, unsafe webhooks are not allowed."]
    pub fn set_no_unsafe_webhooks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_unsafe_webhooks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_AutopilotClusterPolicyConfig {
    type O = terrars::BlockAssignable<ContainerCluster_AutopilotClusterPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_AutopilotClusterPolicyConfig {}

impl BuildContainerCluster_AutopilotClusterPolicyConfig {
    pub fn build(self) -> ContainerCluster_AutopilotClusterPolicyConfig {
        ContainerCluster_AutopilotClusterPolicyConfig {
            no_standard_node_pools: core::default::Default::default(),
            no_system_impersonation: core::default::Default::default(),
            no_system_mutation: core::default::Default::default(),
            no_unsafe_webhooks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_AutopilotClusterPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_AutopilotClusterPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_AutopilotClusterPolicyConfigRef {
        ContainerCluster_AutopilotClusterPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_AutopilotClusterPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_standard_node_pools` after provisioning.\nIf true, prevents standard node pools and requires only autopilot node pools."]
    pub fn no_standard_node_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_standard_node_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_system_impersonation` after provisioning.\nIf true, prevents impersonation and CSRs for GKE System users."]
    pub fn no_system_impersonation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_system_impersonation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_system_mutation` after provisioning.\nIf true, prevents creation and mutation of resources in GKE managed namespaces and cluster-scoped GKE managed resources."]
    pub fn no_system_mutation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_system_mutation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_unsafe_webhooks` after provisioning.\nIf true, unsafe webhooks are not allowed."]
    pub fn no_unsafe_webhooks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_unsafe_webhooks", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_BinaryAuthorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_BinaryAuthorization {
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

impl terrars::ToListMappable for ContainerCluster_BinaryAuthorization {
    type O = terrars::BlockAssignable<ContainerCluster_BinaryAuthorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_BinaryAuthorization {}

impl BuildContainerCluster_BinaryAuthorization {
    pub fn build(self) -> ContainerCluster_BinaryAuthorization {
        ContainerCluster_BinaryAuthorization {
            enabled: core::default::Default::default(),
            evaluation_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_BinaryAuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_BinaryAuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_BinaryAuthorizationRef {
        ContainerCluster_BinaryAuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_BinaryAuthorizationRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
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
::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    type O =
        terrars
        ::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
            auto_upgrade_start_time: core::default::Default::default(),
            description: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
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

impl terrars::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    type O = terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodeConfig_ShieldedInstanceConfig {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ShieldedInstanceConfig {}

impl BuildContainerCluster_NodeConfig_ShieldedInstanceConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_ShieldedInstanceConfig {
        ContainerCluster_NodeConfig_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
        ContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
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
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
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
::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    type O =
        terrars
        ::BlockAssignable<
            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    pub fn build(
        self,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
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
struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
        >,
    >,
    dynamic: ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsDynamic,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
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
                            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
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
::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
    type O =
        terrars
        ::BlockAssignable<
            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
    pub fn build(
        self,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef {
    #[allow(dead_code)]
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
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsDynamic {
    blue_green_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings>,
    >,
}

#[derive(Serialize)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
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
        std::vec::Vec<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings>,
    >,
    dynamic: ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsDynamic,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
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
                            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettings,
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

impl terrars::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
    type O = terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef {
    #[allow(dead_code)]
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
    ::ListRef<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings_BlueGreenSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsDynamic {
    management: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    upgrade_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
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
    ::Option<std::vec::Vec<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings>>,
    dynamic: ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsDynamic,
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
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
                        ::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
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
                            ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettings,
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

impl terrars::ToListMappable for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    type O = terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {}

impl BuildContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
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

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
        ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
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
    ) -> terrars::ListRef<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(
        &self,
    ) -> terrars::ListRef<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_UpgradeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ClusterAutoscaling_ResourceLimits {
    maximum: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    resource_type: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_ClusterAutoscaling_ResourceLimits {
    #[doc = "Set the field `minimum`.\nMinimum amount of the resource in the cluster."]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_ClusterAutoscaling_ResourceLimits {
    type O = terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_ResourceLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling_ResourceLimits {
    #[doc = "Maximum amount of the resource in the cluster."]
    pub maximum: terrars::PrimField<f64>,
    #[doc =
        "The type of the resource. For example, cpu and memory. See the guide to using Node Auto-Provisioning for a list of types."]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_ClusterAutoscaling_ResourceLimits {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling_ResourceLimits {
        ContainerCluster_ClusterAutoscaling_ResourceLimits {
            maximum: self.maximum,
            minimum: core::default::Default::default(),
            resource_type: self.resource_type,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
        ContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    #[allow(dead_code)]
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
struct ContainerCluster_ClusterAutoscalingDynamic {
    auto_provisioning_defaults: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>>,
    resource_limits: std::option::Option<terrars::DynamicBlock<ContainerCluster_ClusterAutoscaling_ResourceLimits>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_ClusterAutoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_provisioning_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_compute_class_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_provisioning_defaults: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_limits: std::option::Option<std::vec::Vec<ContainerCluster_ClusterAutoscaling_ResourceLimits>>,
    dynamic: ContainerCluster_ClusterAutoscalingDynamic,
}

impl ContainerCluster_ClusterAutoscaling {
    #[doc =
        "Set the field `auto_provisioning_locations`.\nThe list of Google Compute Engine zones in which the NodePool's nodes can be created by NAP."]
    pub fn set_auto_provisioning_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.auto_provisioning_locations = std::option::Option::Some(v.into());
        self
    }

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
        "Set the field `default_compute_class_enabled`.\nSpecifies whether default compute class behaviour is enabled. If enabled, cluster autoscaler will use Compute Class with name default for all the workloads, if not overriden."]
    pub fn set_default_compute_class_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_compute_class_enabled = std::option::Option::Some(v.into());
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling_ResourceLimits>>,
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

impl terrars::ToListMappable for ContainerCluster_ClusterAutoscaling {
    type O = terrars::BlockAssignable<ContainerCluster_ClusterAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ClusterAutoscaling {}

impl BuildContainerCluster_ClusterAutoscaling {
    pub fn build(self) -> ContainerCluster_ClusterAutoscaling {
        ContainerCluster_ClusterAutoscaling {
            auto_provisioning_locations: core::default::Default::default(),
            autoscaling_profile: core::default::Default::default(),
            default_compute_class_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            auto_provisioning_defaults: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ClusterAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ClusterAutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_ClusterAutoscalingRef {
        ContainerCluster_ClusterAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ClusterAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_provisioning_locations` after provisioning.\nThe list of Google Compute Engine zones in which the NodePool's nodes can be created by NAP."]
    pub fn auto_provisioning_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_locations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_profile` after provisioning.\nConfiguration options for the Autoscaling profile feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability when deciding to remove nodes from a cluster. Can be BALANCED or OPTIMIZE_UTILIZATION. Defaults to BALANCED."]
    pub fn autoscaling_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_compute_class_enabled` after provisioning.\nSpecifies whether default compute class behaviour is enabled. If enabled, cluster autoscaler will use Compute Class with name default for all the workloads, if not overriden."]
    pub fn default_compute_class_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_compute_class_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether node auto-provisioning is enabled. Resource limits for cpu and memory must be defined to enable node auto-provisioning."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_provisioning_defaults` after provisioning.\n"]
    pub fn auto_provisioning_defaults(
        &self,
    ) -> terrars::ListRef<ContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(&self) -> terrars::ListRef<ContainerCluster_ClusterAutoscaling_ResourceLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ConfidentialNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_ConfidentialNodes {
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

impl terrars::ToListMappable for ContainerCluster_ConfidentialNodes {
    type O = terrars::BlockAssignable<ContainerCluster_ConfidentialNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ConfidentialNodes {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_ConfidentialNodes {
    pub fn build(self) -> ContainerCluster_ConfidentialNodes {
        ContainerCluster_ConfidentialNodes {
            confidential_instance_type: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ConfidentialNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ConfidentialNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_ConfidentialNodesRef {
        ContainerCluster_ConfidentialNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ConfidentialNodesRef {
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
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Confidential Nodes feature is enabled for all nodes in this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_external_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_certs_via_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_tokens_via_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    #[doc =
        "Set the field `allow_external_traffic`.\nControls whether user traffic is allowed over this endpoint. Note that GCP-managed services may still use the endpoint even if this is false."]
    pub fn set_allow_external_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_external_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_k8s_certs_via_dns`.\nControls whether the k8s certs auth is allowed via dns."]
    pub fn set_enable_k8s_certs_via_dns(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_k8s_certs_via_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_k8s_tokens_via_dns`.\nControls whether the k8s token auth is allowed via dns."]
    pub fn set_enable_k8s_tokens_via_dns(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_k8s_tokens_via_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nThe cluster's DNS endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {}

impl BuildContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    pub fn build(self) -> ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
        ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
            allow_external_traffic: core::default::Default::default(),
            enable_k8s_certs_via_dns: core::default::Default::default(),
            enable_k8s_tokens_via_dns: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
        ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_external_traffic` after provisioning.\nControls whether user traffic is allowed over this endpoint. Note that GCP-managed services may still use the endpoint even if this is false."]
    pub fn allow_external_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_external_traffic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_k8s_certs_via_dns` after provisioning.\nControls whether the k8s certs auth is allowed via dns."]
    pub fn enable_k8s_certs_via_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_k8s_certs_via_dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_k8s_tokens_via_dns` after provisioning.\nControls whether the k8s token auth is allowed via dns."]
    pub fn enable_k8s_tokens_via_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_k8s_tokens_via_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe cluster's DNS endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    #[doc = "Set the field `enabled`.\nControls whether to allow direct IP access."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {}

impl BuildContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    pub fn build(self) -> ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
        ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
        ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nControls whether to allow direct IP access."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_ControlPlaneEndpointsConfigDynamic {
    dns_endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>>,
    ip_endpoints_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_ControlPlaneEndpointsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_endpoint_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_endpoints_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>>,
    dynamic: ContainerCluster_ControlPlaneEndpointsConfigDynamic,
}

impl ContainerCluster_ControlPlaneEndpointsConfig {
    #[doc = "Set the field `dns_endpoint_config`.\n"]
    pub fn set_dns_endpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_endpoints_config`.\n"]
    pub fn set_ip_endpoints_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_endpoints_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_endpoints_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_ControlPlaneEndpointsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ControlPlaneEndpointsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ControlPlaneEndpointsConfig {}

impl BuildContainerCluster_ControlPlaneEndpointsConfig {
    pub fn build(self) -> ContainerCluster_ControlPlaneEndpointsConfig {
        ContainerCluster_ControlPlaneEndpointsConfig {
            dns_endpoint_config: core::default::Default::default(),
            ip_endpoints_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ControlPlaneEndpointsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ControlPlaneEndpointsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ControlPlaneEndpointsConfigRef {
        ContainerCluster_ControlPlaneEndpointsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ControlPlaneEndpointsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_endpoint_config` after provisioning.\n"]
    pub fn dns_endpoint_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_endpoints_config` after provisioning.\n"]
    pub fn ip_endpoints_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_endpoints_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_CostManagementConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_CostManagementConfig { }

impl terrars::ToListMappable for ContainerCluster_CostManagementConfig {
    type O = terrars::BlockAssignable<ContainerCluster_CostManagementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_CostManagementConfig {
    #[doc =
        "Whether to enable GKE cost allocation. When you enable GKE cost allocation, the cluster name and namespace of your GKE workloads appear in the labels field of the billing export to BigQuery. Defaults to false."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_CostManagementConfig {
    pub fn build(self) -> ContainerCluster_CostManagementConfig {
        ContainerCluster_CostManagementConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_CostManagementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_CostManagementConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_CostManagementConfigRef {
        ContainerCluster_CostManagementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_CostManagementConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_DatabaseEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    state: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_DatabaseEncryption {
    #[doc = "Set the field `key_name`.\nThe key to use to encrypt/decrypt secrets."]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_DatabaseEncryption {
    type O = terrars::BlockAssignable<ContainerCluster_DatabaseEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_DatabaseEncryption {
    #[doc = "ENCRYPTED, ALL_OBJECTS_ENCRYPTION_ENABLED or DECRYPTED."]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_DatabaseEncryption {
    pub fn build(self) -> ContainerCluster_DatabaseEncryption {
        ContainerCluster_DatabaseEncryption {
            key_name: core::default::Default::default(),
            state: self.state,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_DatabaseEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_DatabaseEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_DatabaseEncryptionRef {
        ContainerCluster_DatabaseEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_DatabaseEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_name` after provisioning.\nThe key to use to encrypt/decrypt secrets."]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nENCRYPTED, ALL_OBJECTS_ENCRYPTION_ENABLED or DECRYPTED."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_DefaultSnatStatus {
    disabled: terrars::PrimField<bool>,
}

impl ContainerCluster_DefaultSnatStatus { }

impl terrars::ToListMappable for ContainerCluster_DefaultSnatStatus {
    type O = terrars::BlockAssignable<ContainerCluster_DefaultSnatStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_DefaultSnatStatus {
    #[doc =
        "When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic."]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_DefaultSnatStatus {
    pub fn build(self) -> ContainerCluster_DefaultSnatStatus {
        ContainerCluster_DefaultSnatStatus { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_DefaultSnatStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_DefaultSnatStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_DefaultSnatStatusRef {
        ContainerCluster_DefaultSnatStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_DefaultSnatStatusRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_DnsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additive_vpc_scope_dns_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_DnsConfig {
    #[doc = "Set the field `additive_vpc_scope_dns_domain`.\nEnable additive VPC scope DNS in a GKE cluster."]
    pub fn set_additive_vpc_scope_dns_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additive_vpc_scope_dns_domain = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for ContainerCluster_DnsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_DnsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_DnsConfig {}

impl BuildContainerCluster_DnsConfig {
    pub fn build(self) -> ContainerCluster_DnsConfig {
        ContainerCluster_DnsConfig {
            additive_vpc_scope_dns_domain: core::default::Default::default(),
            cluster_dns: core::default::Default::default(),
            cluster_dns_domain: core::default::Default::default(),
            cluster_dns_scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_DnsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_DnsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_DnsConfigRef {
        ContainerCluster_DnsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_DnsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additive_vpc_scope_dns_domain` after provisioning.\nEnable additive VPC scope DNS in a GKE cluster."]
    pub fn additive_vpc_scope_dns_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additive_vpc_scope_dns_domain", self.base))
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
pub struct ContainerCluster_EnableK8sBetaApis {
    enabled_apis: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_EnableK8sBetaApis { }

impl terrars::ToListMappable for ContainerCluster_EnableK8sBetaApis {
    type O = terrars::BlockAssignable<ContainerCluster_EnableK8sBetaApis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_EnableK8sBetaApis {
    #[doc = "Enabled Kubernetes Beta APIs."]
    pub enabled_apis: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_EnableK8sBetaApis {
    pub fn build(self) -> ContainerCluster_EnableK8sBetaApis {
        ContainerCluster_EnableK8sBetaApis { enabled_apis: self.enabled_apis }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_EnableK8sBetaApisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_EnableK8sBetaApisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_EnableK8sBetaApisRef {
        ContainerCluster_EnableK8sBetaApisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_EnableK8sBetaApisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_apis` after provisioning.\nEnabled Kubernetes Beta APIs."]
    pub fn enabled_apis(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_apis", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_EnterpriseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_EnterpriseConfig {
    #[doc =
        "Set the field `desired_tier`.\nIndicates the desired cluster tier. Available options include STANDARD and ENTERPRISE."]
    pub fn set_desired_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desired_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_EnterpriseConfig {
    type O = terrars::BlockAssignable<ContainerCluster_EnterpriseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_EnterpriseConfig {}

impl BuildContainerCluster_EnterpriseConfig {
    pub fn build(self) -> ContainerCluster_EnterpriseConfig {
        ContainerCluster_EnterpriseConfig { desired_tier: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_EnterpriseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_EnterpriseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_EnterpriseConfigRef {
        ContainerCluster_EnterpriseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_EnterpriseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_tier` after provisioning.\nIndicates the effective cluster tier. Available options include STANDARD and ENTERPRISE."]
    pub fn cluster_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `desired_tier` after provisioning.\nIndicates the desired cluster tier. Available options include STANDARD and ENTERPRISE."]
    pub fn desired_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_Fleet {
    #[doc = "Set the field `membership_type`.\nThe type of the cluster's fleet membership."]
    pub fn set_membership_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe Fleet host project of the cluster."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_Fleet {
    type O = terrars::BlockAssignable<ContainerCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_Fleet {}

impl BuildContainerCluster_Fleet {
    pub fn build(self) -> ContainerCluster_Fleet {
        ContainerCluster_Fleet {
            membership_type: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_FleetRef {
        ContainerCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nFull resource name of the registered fleet membership of the cluster."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `membership_id` after provisioning.\nShort name of the fleet membership, for example \"member-1\"."]
    pub fn membership_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `membership_location` after provisioning.\nLocation of the fleet membership, for example \"us-central1\"."]
    pub fn membership_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `membership_type` after provisioning.\nThe type of the cluster's fleet membership."]
    pub fn membership_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_type", self.base))
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
pub struct ContainerCluster_GatewayApiConfig {
    channel: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_GatewayApiConfig { }

impl terrars::ToListMappable for ContainerCluster_GatewayApiConfig {
    type O = terrars::BlockAssignable<ContainerCluster_GatewayApiConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_GatewayApiConfig {
    #[doc = "The Gateway API release channel to use for Gateway API."]
    pub channel: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_GatewayApiConfig {
    pub fn build(self) -> ContainerCluster_GatewayApiConfig {
        ContainerCluster_GatewayApiConfig { channel: self.channel }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_GatewayApiConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_GatewayApiConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_GatewayApiConfigRef {
        ContainerCluster_GatewayApiConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_GatewayApiConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_GkeAutoUpgradeConfig {
    patch_mode: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_GkeAutoUpgradeConfig { }

impl terrars::ToListMappable for ContainerCluster_GkeAutoUpgradeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_GkeAutoUpgradeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_GkeAutoUpgradeConfig {
    #[doc =
        "The selected auto-upgrade patch type. Accepted values are:\n* ACCELERATED: Upgrades to the latest available patch version in a given minor and release channel."]
    pub patch_mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_GkeAutoUpgradeConfig {
    pub fn build(self) -> ContainerCluster_GkeAutoUpgradeConfig {
        ContainerCluster_GkeAutoUpgradeConfig { patch_mode: self.patch_mode }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_GkeAutoUpgradeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_GkeAutoUpgradeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_GkeAutoUpgradeConfigRef {
        ContainerCluster_GkeAutoUpgradeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_GkeAutoUpgradeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `patch_mode` after provisioning.\nThe selected auto-upgrade patch type. Accepted values are:\n* ACCELERATED: Upgrades to the latest available patch version in a given minor and release channel."]
    pub fn patch_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_IdentityServiceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_IdentityServiceConfig {
    #[doc = "Set the field `enabled`.\nWhether to enable the Identity Service component."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_IdentityServiceConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IdentityServiceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IdentityServiceConfig {}

impl BuildContainerCluster_IdentityServiceConfig {
    pub fn build(self) -> ContainerCluster_IdentityServiceConfig {
        ContainerCluster_IdentityServiceConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IdentityServiceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IdentityServiceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_IdentityServiceConfigRef {
        ContainerCluster_IdentityServiceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IdentityServiceConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_ipv4_range_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    subnetwork: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    #[doc =
        "Set the field `pod_ipv4_range_names`.\nList of secondary ranges names within this subnetwork that can be used for pod IPs."]
    pub fn set_pod_ipv4_range_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pod_ipv4_range_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nStatus of the subnetwork, If in draining status, subnet will not be selected for new node pools."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    #[doc = "Name of the subnetwork. This can be the full path of the subnetwork or just the name."]
    pub subnetwork: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
        ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
            pod_ipv4_range_names: core::default::Default::default(),
            status: core::default::Default::default(),
            subnetwork: self.subnetwork,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
        ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pod_ipv4_range_names` after provisioning.\nList of secondary ranges names within this subnetwork that can be used for pod IPs."]
    pub fn pod_ipv4_range_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_ipv4_range_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the subnetwork, If in draining status, subnet will not be selected for new node pools."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nName of the subnetwork. This can be the full path of the subnetwork or just the name."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    pod_range_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig { }

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    #[doc = "Name for pod secondary ipv4 range which has the actual range defined ahead."]
    pub pod_range_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
        ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig { pod_range_names: self.pod_range_names }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
        ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_IpAllocationPolicy_AutoIpamConfig { }

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AutoIpamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    #[doc = "The flag that enables Auto IPAM on this cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy_AutoIpamConfig {
        ContainerCluster_IpAllocationPolicy_AutoIpamConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
        ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nThe flag that enables Auto IPAM on this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    network_tier: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_IpAllocationPolicy_NetworkTierConfig { }

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_NetworkTierConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    #[doc = "Network tier configuration."]
    pub network_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy_NetworkTierConfig {
        ContainerCluster_IpAllocationPolicy_NetworkTierConfig { network_tier: self.network_tier }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
        ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nNetwork tier configuration."]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    disabled: terrars::PrimField<bool>,
}

impl ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig { }

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    #[doc = ""]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
        ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig { disabled: self.disabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
        ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
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
struct ContainerCluster_IpAllocationPolicyDynamic {
    additional_ip_ranges_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>>,
    additional_pod_ranges_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>>,
    auto_ipam_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_AutoIpamConfig>>,
    network_tier_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_NetworkTierConfig>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_IpAllocationPolicy {
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
    additional_ip_ranges_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_ranges_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_ipam_config: std::option::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_AutoIpamConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier_config: std::option::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_NetworkTierConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    dynamic: ContainerCluster_IpAllocationPolicyDynamic,
}

impl ContainerCluster_IpAllocationPolicy {
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

    #[doc = "Set the field `additional_ip_ranges_config`.\n"]
    pub fn set_additional_ip_ranges_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_ip_ranges_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_ip_ranges_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `additional_pod_ranges_config`.\n"]
    pub fn set_additional_pod_ranges_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>>,
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

    #[doc = "Set the field `auto_ipam_config`.\n"]
    pub fn set_auto_ipam_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_AutoIpamConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_ipam_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_ipam_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_tier_config`.\n"]
    pub fn set_network_tier_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_NetworkTierConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_tier_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_tier_config = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_IpAllocationPolicy {
    type O = terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_IpAllocationPolicy {}

impl BuildContainerCluster_IpAllocationPolicy {
    pub fn build(self) -> ContainerCluster_IpAllocationPolicy {
        ContainerCluster_IpAllocationPolicy {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            additional_ip_ranges_config: core::default::Default::default(),
            additional_pod_ranges_config: core::default::Default::default(),
            auto_ipam_config: core::default::Default::default(),
            network_tier_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_IpAllocationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_IpAllocationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_IpAllocationPolicyRef {
        ContainerCluster_IpAllocationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_IpAllocationPolicyRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `additional_ip_ranges_config` after provisioning.\n"]
    pub fn additional_ip_ranges_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_ip_ranges_config", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_ranges_config` after provisioning.\n"]
    pub fn additional_pod_ranges_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_ranges_config", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_ipam_config` after provisioning.\n"]
    pub fn auto_ipam_config(&self) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_AutoIpamConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_ipam_config", self.base))
    }

    #[doc = "Get a reference to the value of field `network_tier_config` after provisioning.\n"]
    pub fn network_tier_config(&self) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_NetworkTierConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tier_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_LoggingConfig {
    enable_components: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_LoggingConfig { }

impl terrars::ToListMappable for ContainerCluster_LoggingConfig {
    type O = terrars::BlockAssignable<ContainerCluster_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_LoggingConfig {
    #[doc =
        "GKE components exposing logs. Valid values include SYSTEM_COMPONENTS, APISERVER, CONTROLLER_MANAGER, KCP_CONNECTION, KCP_SSHD, KCP_HPA, KCP_VPA, SCHEDULER, and WORKLOADS."]
    pub enable_components: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_LoggingConfig {
    pub fn build(self) -> ContainerCluster_LoggingConfig {
        ContainerCluster_LoggingConfig { enable_components: self.enable_components }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_LoggingConfigRef {
        ContainerCluster_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_components` after provisioning.\nGKE components exposing logs. Valid values include SYSTEM_COMPONENTS, APISERVER, CONTROLLER_MANAGER, KCP_CONNECTION, KCP_SSHD, KCP_HPA, KCP_VPA, SCHEDULER, and WORKLOADS."]
    pub fn enable_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    start_time: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow { }

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
        ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow { start_time: self.start_time }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
        ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_MaintenancePolicy_DisruptionBudget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor_version_disruption_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_version_disruption_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_MaintenancePolicy_DisruptionBudget {
    #[doc = "Set the field `minor_version_disruption_interval`.\n"]
    pub fn set_minor_version_disruption_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minor_version_disruption_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patch_version_disruption_interval`.\n"]
    pub fn set_patch_version_disruption_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.patch_version_disruption_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_DisruptionBudget {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_DisruptionBudget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_DisruptionBudget {}

impl BuildContainerCluster_MaintenancePolicy_DisruptionBudget {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_DisruptionBudget {
        ContainerCluster_MaintenancePolicy_DisruptionBudget {
            minor_version_disruption_interval: core::default::Default::default(),
            patch_version_disruption_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
        ContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_disruption_time` after provisioning.\n"]
    pub fn last_disruption_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_disruption_time", self.base))
    }

    #[doc = "Get a reference to the value of field `last_minor_version_disruption_time` after provisioning.\n"]
    pub fn last_minor_version_disruption_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_minor_version_disruption_time", self.base))
    }

    #[doc = "Get a reference to the value of field `minor_version_disruption_interval` after provisioning.\n"]
    pub fn minor_version_disruption_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minor_version_disruption_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `patch_version_disruption_interval` after provisioning.\n"]
    pub fn patch_version_disruption_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_version_disruption_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    scope: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    #[doc = "Set the field `end_time_behavior`.\nThe behavior of the exclusion end time."]
    pub fn set_end_time_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    #[doc = "The scope of automatic upgrades to restrict in the exclusion window."]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
        ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
            end_time_behavior: core::default::Default::default(),
            scope: self.scope,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
        ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time_behavior` after provisioning.\nThe behavior of the exclusion end time."]
    pub fn end_time_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of automatic upgrades to restrict in the exclusion window."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_MaintenancePolicy_MaintenanceExclusionDynamic {
    exclusion_options: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    exclusion_name: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_options: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>>,
    dynamic: ContainerCluster_MaintenancePolicy_MaintenanceExclusionDynamic,
}

impl ContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusion_options`.\n"]
    pub fn set_exclusion_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>,
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

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_MaintenanceExclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    #[doc = ""]
    pub exclusion_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_MaintenanceExclusion {
        ContainerCluster_MaintenancePolicy_MaintenanceExclusion {
            end_time: core::default::Default::default(),
            exclusion_name: self.exclusion_name,
            start_time: self.start_time,
            exclusion_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
        ContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil { }

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    #[doc = ""]
    pub day: terrars::PrimField<f64>,
    #[doc = ""]
    pub month: terrars::PrimField<f64>,
    #[doc = ""]
    pub year: terrars::PrimField<f64>,
}

impl BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\n"]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\n"]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    hours: terrars::PrimField<f64>,
    minutes: terrars::PrimField<f64>,
    seconds: terrars::PrimField<f64>,
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime { }

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    #[doc = ""]
    pub hours: terrars::PrimField<f64>,
    #[doc = ""]
    pub minutes: terrars::PrimField<f64>,
    #[doc = ""]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
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

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowDynamic {
    delay_until: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>>,
    window_start_time: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    recurrence: terrars::PrimField<std::string::String>,
    window_duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay_until: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_start_time: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>>,
    dynamic: ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowDynamic,
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    #[doc = "Set the field `delay_until`.\n"]
    pub fn set_delay_until(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delay_until = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delay_until = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window_start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window_start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    #[doc = ""]
    pub recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub window_duration: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
            recurrence: self.recurrence,
            window_duration: self.window_duration,
            delay_until: core::default::Default::default(),
            window_start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
        ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `window_duration` after provisioning.\n"]
    pub fn window_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `delay_until` after provisioning.\n"]
    pub fn delay_until(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay_until", self.base))
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy_RecurringWindow {
    end_time: terrars::PrimField<std::string::String>,
    recurrence: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_MaintenancePolicy_RecurringWindow { }

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy_RecurringWindow {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy_RecurringWindow {
    #[doc = ""]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MaintenancePolicy_RecurringWindow {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy_RecurringWindow {
        ContainerCluster_MaintenancePolicy_RecurringWindow {
            end_time: self.end_time,
            recurrence: self.recurrence,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicy_RecurringWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicy_RecurringWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MaintenancePolicy_RecurringWindowRef {
        ContainerCluster_MaintenancePolicy_RecurringWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicy_RecurringWindowRef {
    #[allow(dead_code)]
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
struct ContainerCluster_MaintenancePolicyDynamic {
    daily_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>>,
    disruption_budget: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_DisruptionBudget>>,
    maintenance_exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_MaintenanceExclusion>>,
    recurring_maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>>,
    recurring_window: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MaintenancePolicy_RecurringWindow>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_maintenance_window: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disruption_budget: std::option::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_DisruptionBudget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusion: std::option::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_MaintenanceExclusion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_maintenance_window: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_window: std::option::Option<std::vec::Vec<ContainerCluster_MaintenancePolicy_RecurringWindow>>,
    dynamic: ContainerCluster_MaintenancePolicyDynamic,
}

impl ContainerCluster_MaintenancePolicy {
    #[doc = "Set the field `daily_maintenance_window`.\n"]
    pub fn set_daily_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>>,
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

    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_DisruptionBudget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disruption_budget = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disruption_budget = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_MaintenanceExclusion>>,
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

    #[doc = "Set the field `recurring_maintenance_window`.\n"]
    pub fn set_recurring_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recurring_maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recurring_maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MaintenancePolicy_RecurringWindow>>,
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

impl terrars::ToListMappable for ContainerCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<ContainerCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MaintenancePolicy {}

impl BuildContainerCluster_MaintenancePolicy {
    pub fn build(self) -> ContainerCluster_MaintenancePolicy {
        ContainerCluster_MaintenancePolicy {
            daily_maintenance_window: core::default::Default::default(),
            disruption_budget: core::default::Default::default(),
            maintenance_exclusion: core::default::Default::default(),
            recurring_maintenance_window: core::default::Default::default(),
            recurring_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_MaintenancePolicyRef {
        ContainerCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_maintenance_window` after provisioning.\n"]
    pub fn daily_maintenance_window(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(&self) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_DisruptionBudgetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disruption_budget", self.base))
    }

    #[doc = "Get a reference to the value of field `recurring_maintenance_window` after provisioning.\n"]
    pub fn recurring_maintenance_window(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(&self) -> terrars::ListRef<ContainerCluster_MaintenancePolicy_RecurringWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ManagedOpentelemetryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_ManagedOpentelemetryConfig {
    #[doc =
        "Set the field `scope`.\nThe scope of the Managed OpenTelemetry pipeline. Available options include NONE and COLLECTION_AND_INSTRUMENTATION_COMPONENTS. See https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#managedopentelemetryconfig for more information. To disable the feature, explicitly set this to NONE."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_ManagedOpentelemetryConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ManagedOpentelemetryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ManagedOpentelemetryConfig {}

impl BuildContainerCluster_ManagedOpentelemetryConfig {
    pub fn build(self) -> ContainerCluster_ManagedOpentelemetryConfig {
        ContainerCluster_ManagedOpentelemetryConfig { scope: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ManagedOpentelemetryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ManagedOpentelemetryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ManagedOpentelemetryConfigRef {
        ContainerCluster_ManagedOpentelemetryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ManagedOpentelemetryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of the Managed OpenTelemetry pipeline. Available options include NONE and COLLECTION_AND_INSTRUMENTATION_COMPONENTS. See https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#managedopentelemetryconfig for more information. To disable the feature, explicitly set this to NONE."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MasterAuth_ClientCertificateConfig {
    issue_client_certificate: terrars::PrimField<bool>,
}

impl ContainerCluster_MasterAuth_ClientCertificateConfig { }

impl terrars::ToListMappable for ContainerCluster_MasterAuth_ClientCertificateConfig {
    type O = terrars::BlockAssignable<ContainerCluster_MasterAuth_ClientCertificateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MasterAuth_ClientCertificateConfig {
    #[doc = "Whether client certificate authorization is enabled for this cluster."]
    pub issue_client_certificate: terrars::PrimField<bool>,
}

impl BuildContainerCluster_MasterAuth_ClientCertificateConfig {
    pub fn build(self) -> ContainerCluster_MasterAuth_ClientCertificateConfig {
        ContainerCluster_MasterAuth_ClientCertificateConfig {
            issue_client_certificate: self.issue_client_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MasterAuth_ClientCertificateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MasterAuth_ClientCertificateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MasterAuth_ClientCertificateConfigRef {
        ContainerCluster_MasterAuth_ClientCertificateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MasterAuth_ClientCertificateConfigRef {
    #[allow(dead_code)]
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
struct ContainerCluster_MasterAuthDynamic {
    client_certificate_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MasterAuth_ClientCertificateConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MasterAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_config: std::option::Option<std::vec::Vec<ContainerCluster_MasterAuth_ClientCertificateConfig>>,
    dynamic: ContainerCluster_MasterAuthDynamic,
}

impl ContainerCluster_MasterAuth {
    #[doc = "Set the field `client_certificate_config`.\n"]
    pub fn set_client_certificate_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MasterAuth_ClientCertificateConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_MasterAuth {
    type O = terrars::BlockAssignable<ContainerCluster_MasterAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MasterAuth {}

impl BuildContainerCluster_MasterAuth {
    pub fn build(self) -> ContainerCluster_MasterAuth {
        ContainerCluster_MasterAuth {
            client_certificate_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MasterAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MasterAuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_MasterAuthRef {
        ContainerCluster_MasterAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MasterAuthRef {
    #[allow(dead_code)]
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
    pub fn client_certificate_config(&self) -> terrars::ListRef<ContainerCluster_MasterAuth_ClientCertificateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "Set the field `display_name`.\nField for users to identify CIDR blocks."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    type O = terrars::BlockAssignable<ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "External network that can access Kubernetes master through HTTPS. Must be specified in CIDR notation."]
    pub cidr_block: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    pub fn build(self) -> ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
        ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
            cidr_block: self.cidr_block,
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
        ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    #[allow(dead_code)]
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
struct ContainerCluster_MasterAuthorizedNetworksConfigDynamic {
    cidr_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MasterAuthorizedNetworksConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_public_cidrs_access_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_enforcement_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<std::vec::Vec<ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>>,
    dynamic: ContainerCluster_MasterAuthorizedNetworksConfigDynamic,
}

impl ContainerCluster_MasterAuthorizedNetworksConfig {
    #[doc =
        "Set the field `gcp_public_cidrs_access_enabled`.\nWhether Kubernetes master is accessible via Google Compute Engine Public IPs."]
    pub fn set_gcp_public_cidrs_access_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.gcp_public_cidrs_access_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_endpoint_enforcement_enabled`.\nWhether authorized networks is enforced on the private endpoint or not. Defaults to false."]
    pub fn set_private_endpoint_enforcement_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.private_endpoint_enforcement_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>>,
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

impl terrars::ToListMappable for ContainerCluster_MasterAuthorizedNetworksConfig {
    type O = terrars::BlockAssignable<ContainerCluster_MasterAuthorizedNetworksConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MasterAuthorizedNetworksConfig {}

impl BuildContainerCluster_MasterAuthorizedNetworksConfig {
    pub fn build(self) -> ContainerCluster_MasterAuthorizedNetworksConfig {
        ContainerCluster_MasterAuthorizedNetworksConfig {
            gcp_public_cidrs_access_enabled: core::default::Default::default(),
            private_endpoint_enforcement_enabled: core::default::Default::default(),
            cidr_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MasterAuthorizedNetworksConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MasterAuthorizedNetworksConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MasterAuthorizedNetworksConfigRef {
        ContainerCluster_MasterAuthorizedNetworksConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MasterAuthorizedNetworksConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_public_cidrs_access_enabled` after provisioning.\nWhether Kubernetes master is accessible via Google Compute Engine Public IPs."]
    pub fn gcp_public_cidrs_access_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_public_cidrs_access_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_enforcement_enabled` after provisioning.\nWhether authorized networks is enforced on the private endpoint or not. Defaults to false."]
    pub fn private_endpoint_enforcement_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_enforcement_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MeshCertificates {
    enable_certificates: terrars::PrimField<bool>,
}

impl ContainerCluster_MeshCertificates { }

impl terrars::ToListMappable for ContainerCluster_MeshCertificates {
    type O = terrars::BlockAssignable<ContainerCluster_MeshCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MeshCertificates {
    #[doc =
        "When enabled the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub enable_certificates: terrars::PrimField<bool>,
}

impl BuildContainerCluster_MeshCertificates {
    pub fn build(self) -> ContainerCluster_MeshCertificates {
        ContainerCluster_MeshCertificates { enable_certificates: self.enable_certificates }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MeshCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MeshCertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_MeshCertificatesRef {
        ContainerCluster_MeshCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MeshCertificatesRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    enable_metrics: terrars::PrimField<bool>,
    enable_relay: terrars::PrimField<bool>,
}

impl ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig { }

impl terrars::ToListMappable for ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    type O = terrars::BlockAssignable<ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    #[doc = "Whether or not the advanced datapath metrics are enabled."]
    pub enable_metrics: terrars::PrimField<bool>,
    #[doc = "Whether or not Relay is enabled."]
    pub enable_relay: terrars::PrimField<bool>,
}

impl BuildContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    pub fn build(self) -> ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
        ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
            enable_metrics: self.enable_metrics,
            enable_relay: self.enable_relay,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
        ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_metrics` after provisioning.\nWhether or not the advanced datapath metrics are enabled."]
    pub fn enable_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_metrics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_relay` after provisioning.\nWhether or not Relay is enabled."]
    pub fn enable_relay(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_relay", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    scope: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig { }

impl terrars::ToListMappable for ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    type O = terrars::BlockAssignable<ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    #[doc = "The scope of auto-monitoring."]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    pub fn build(self) -> ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
        ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig { scope: self.scope }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
        ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\nThe scope of auto-monitoring."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_MonitoringConfig_ManagedPrometheusDynamic {
    auto_monitoring_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MonitoringConfig_ManagedPrometheus {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_monitoring_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>>,
    dynamic: ContainerCluster_MonitoringConfig_ManagedPrometheusDynamic,
}

impl ContainerCluster_MonitoringConfig_ManagedPrometheus {
    #[doc = "Set the field `auto_monitoring_config`.\n"]
    pub fn set_auto_monitoring_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_monitoring_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_monitoring_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_MonitoringConfig_ManagedPrometheus {
    type O = terrars::BlockAssignable<ContainerCluster_MonitoringConfig_ManagedPrometheus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MonitoringConfig_ManagedPrometheus {
    #[doc = "Whether or not the managed collection is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_MonitoringConfig_ManagedPrometheus {
    pub fn build(self) -> ContainerCluster_MonitoringConfig_ManagedPrometheus {
        ContainerCluster_MonitoringConfig_ManagedPrometheus {
            enabled: self.enabled,
            auto_monitoring_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_MonitoringConfig_ManagedPrometheusRef {
        ContainerCluster_MonitoringConfig_ManagedPrometheusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the managed collection is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_monitoring_config` after provisioning.\n"]
    pub fn auto_monitoring_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_monitoring_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_MonitoringConfigDynamic {
    advanced_datapath_observability_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>>,
    managed_prometheus: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_MonitoringConfig_ManagedPrometheus>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_MonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_components: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_datapath_observability_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_prometheus: std::option::Option<std::vec::Vec<ContainerCluster_MonitoringConfig_ManagedPrometheus>>,
    dynamic: ContainerCluster_MonitoringConfigDynamic,
}

impl ContainerCluster_MonitoringConfig {
    #[doc =
        "Set the field `enable_components`.\nGKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT, STATEFULSET, KUBELET, CADVISOR, DCGM and JOBSET."]
    pub fn set_enable_components(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
                        ::BlockAssignable<ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_MonitoringConfig_ManagedPrometheus>>,
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

impl terrars::ToListMappable for ContainerCluster_MonitoringConfig {
    type O = terrars::BlockAssignable<ContainerCluster_MonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_MonitoringConfig {}

impl BuildContainerCluster_MonitoringConfig {
    pub fn build(self) -> ContainerCluster_MonitoringConfig {
        ContainerCluster_MonitoringConfig {
            enable_components: core::default::Default::default(),
            advanced_datapath_observability_config: core::default::Default::default(),
            managed_prometheus: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_MonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_MonitoringConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_MonitoringConfigRef {
        ContainerCluster_MonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_MonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_components` after provisioning.\nGKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT, STATEFULSET, KUBELET, CADVISOR, DCGM and JOBSET."]
    pub fn enable_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_datapath_observability_config` after provisioning.\n"]
    pub fn advanced_datapath_observability_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_datapath_observability_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_prometheus` after provisioning.\n"]
    pub fn managed_prometheus(&self) -> terrars::ListRef<ContainerCluster_MonitoringConfig_ManagedPrometheusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NetworkPerformanceConfig {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NetworkPerformanceConfig { }

impl terrars::ToListMappable for ContainerCluster_NetworkPerformanceConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NetworkPerformanceConfig {
    #[doc = "Specifies the total network bandwidth tier for NodePools in the cluster."]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NetworkPerformanceConfig {
    pub fn build(self) -> ContainerCluster_NetworkPerformanceConfig {
        ContainerCluster_NetworkPerformanceConfig { total_egress_bandwidth_tier: self.total_egress_bandwidth_tier }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NetworkPerformanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NetworkPerformanceConfigRef {
        ContainerCluster_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NetworkPerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nSpecifies the total network bandwidth tier for NodePools in the cluster."]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NetworkPolicy {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NetworkPolicy {
    #[doc = "Set the field `provider`.\nThe selected network policy provider."]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NetworkPolicy {
    type O = terrars::BlockAssignable<ContainerCluster_NetworkPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NetworkPolicy {
    #[doc = "Whether network policy is enabled on the cluster."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NetworkPolicy {
    pub fn build(self) -> ContainerCluster_NetworkPolicy {
        ContainerCluster_NetworkPolicy {
            enabled: self.enabled,
            provider: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NetworkPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NetworkPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NetworkPolicyRef {
        ContainerCluster_NetworkPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NetworkPolicyRef {
    #[allow(dead_code)]
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
pub struct ContainerCluster_NodeConfig_EffectiveTaints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_EffectiveTaints {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_EffectiveTaints {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_EffectiveTaints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_EffectiveTaints {}

impl BuildContainerCluster_NodeConfig_EffectiveTaints {
    pub fn build(self) -> ContainerCluster_NodeConfig_EffectiveTaints {
        ContainerCluster_NodeConfig_EffectiveTaints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_EffectiveTaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_EffectiveTaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_EffectiveTaintsRef {
        ContainerCluster_NodeConfig_EffectiveTaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_EffectiveTaintsRef {
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
pub struct ContainerCluster_NodeConfig_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_monitoring_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    threads_per_core: terrars::PrimField<f64>,
}

impl ContainerCluster_NodeConfig_AdvancedMachineFeatures {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_AdvancedMachineFeatures {
    #[doc =
        "The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub threads_per_core: terrars::PrimField<f64>,
}

impl BuildContainerCluster_NodeConfig_AdvancedMachineFeatures {
    pub fn build(self) -> ContainerCluster_NodeConfig_AdvancedMachineFeatures {
        ContainerCluster_NodeConfig_AdvancedMachineFeatures {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: self.threads_per_core,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
        ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
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
pub struct ContainerCluster_NodeConfig_BootDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodeConfig_BootDisk {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_BootDisk {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_BootDisk {}

impl BuildContainerCluster_NodeConfig_BootDisk {
    pub fn build(self) -> ContainerCluster_NodeConfig_BootDisk {
        ContainerCluster_NodeConfig_BootDisk {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_BootDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_BootDiskRef {
        ContainerCluster_NodeConfig_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_BootDiskRef {
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
pub struct ContainerCluster_NodeConfig_ConfidentialNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodeConfig_ConfidentialNodes {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ConfidentialNodes {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ConfidentialNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ConfidentialNodes {
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_ConfidentialNodes {
    pub fn build(self) -> ContainerCluster_NodeConfig_ConfidentialNodes {
        ContainerCluster_NodeConfig_ConfidentialNodes {
            confidential_instance_type: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ConfidentialNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ConfidentialNodesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ConfidentialNodesRef {
        ContainerCluster_NodeConfig_ConfidentialNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ConfidentialNodesRef {
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
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    secret_uri: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {

}

impl terrars
::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    type O =
        terrars
        ::BlockAssignable<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    #[doc =
        "URI for the secret that hosts a certificate. Must be in the format 'projects/PROJECT_NUM/secrets/SECRET_NAME/versions/VERSION_OR_LATEST'."]
    pub secret_uri: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    pub fn build(
        self,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
            secret_uri: self.secret_uri,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
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
struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigDynamic {
    gcp_secret_manager_certificate_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    fqdns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_certificate_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >,
    >,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
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
                            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
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
::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    type O =
        terrars
        ::BlockAssignable<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    #[doc = "List of fully-qualified-domain-names. IPv4s and port specification are supported."]
    pub fqdns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    pub fn build(
        self,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
            fqdns: self.fqdns,
            gcp_secret_manager_certificate_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
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
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_secret_manager_certificate_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigDynamic {
    certificate_authority_domain_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_domain_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >,
    >,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
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
                            ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    #[doc = "Whether or not private registries are configured."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
            enabled: self.enabled,
            certificate_authority_domain_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
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
        ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_domain_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
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
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
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
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
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
struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientDynamic {
    cert: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>>,
    key: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>>,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
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
                            ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert,
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    key: terrars::PrimField<std::string::String>,
    value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    #[doc = "Configures the header key."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Configures the header value."]
    pub value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
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
struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsDynamic {
    ca: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>>,
    client: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dial_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_path: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>>,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
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
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>,
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>,
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[doc = "Configures the registry host/mirror."]
    pub host: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
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
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
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
    pub fn ca(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }

    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsDynamic {
    hosts: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    server: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    #[doc = "Defines the host name of the registry server."]
    pub server: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
            server: self.server,
            hosts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
        ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
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
    pub fn hosts(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    #[doc = "Whether writable cgroups are enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
        ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
        ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
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
struct ContainerCluster_NodeConfig_ContainerdConfigDynamic {
    private_registry_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>>,
    registry_hosts: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>>,
    writable_cgroups: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_ContainerdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_registry_access_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_hosts: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    writable_cgroups: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>>,
    dynamic: ContainerCluster_NodeConfig_ContainerdConfigDynamic,
}

impl ContainerCluster_NodeConfig_ContainerdConfig {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ContainerdConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ContainerdConfig {}

impl BuildContainerCluster_NodeConfig_ContainerdConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_ContainerdConfig {
        ContainerCluster_NodeConfig_ContainerdConfig {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ContainerdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ContainerdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ContainerdConfigRef {
        ContainerCluster_NodeConfig_ContainerdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ContainerdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]
    pub fn private_registry_access_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registry_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.writable_cgroups", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_count: std::option::Option<terrars::PrimField<f64>>,
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[doc =
        "Set the field `data_cache_count`.\nNumber of local SSDs to be utilized for GKE Data Cache. Uses NVMe interfaces."]
    pub fn set_data_cache_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_cache_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[doc =
        "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD must be 375 or 3000 GB in size, and all local SSDs must share the same size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
        ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: self.local_ssd_count,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
        ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
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
pub struct ContainerCluster_NodeConfig_FastSocket {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodeConfig_FastSocket { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_FastSocket {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_FastSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_FastSocket {
    #[doc = "Whether or not NCCL Fast Socket is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_FastSocket {
    pub fn build(self) -> ContainerCluster_NodeConfig_FastSocket {
        ContainerCluster_NodeConfig_FastSocket { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_FastSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_FastSocketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_FastSocketRef {
        ContainerCluster_NodeConfig_FastSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_FastSocketRef {
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
pub struct ContainerCluster_NodeConfig_GcfsConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodeConfig_GcfsConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_GcfsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_GcfsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_GcfsConfig {
    #[doc = "Whether or not GCFS is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_GcfsConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_GcfsConfig {
        ContainerCluster_NodeConfig_GcfsConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_GcfsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_GcfsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_GcfsConfigRef {
        ContainerCluster_NodeConfig_GcfsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_GcfsConfigRef {
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
pub struct ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    gpu_driver_version: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    #[doc = "Mode for how the GPU driver is installed."]
    pub gpu_driver_version: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
        ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
            gpu_driver_version: self.gpu_driver_version,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
        ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
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
pub struct ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    gpu_sharing_strategy: terrars::PrimField<std::string::String>,
    max_shared_clients_per_gpu: terrars::PrimField<f64>,
}

impl ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    #[doc =
        "The type of GPU sharing strategy to enable on the GPU node. Possible values are described in the API package (https://pkg.go.dev/google.golang.org/api/container/v1#GPUSharingConfig)"]
    pub gpu_sharing_strategy: terrars::PrimField<std::string::String>,
    #[doc = "The maximum number of containers that can share a GPU."]
    pub max_shared_clients_per_gpu: terrars::PrimField<f64>,
}

impl BuildContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
        ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
            gpu_sharing_strategy: self.gpu_sharing_strategy,
            max_shared_clients_per_gpu: self.max_shared_clients_per_gpu,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
        ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
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
struct ContainerCluster_NodeConfig_GuestAcceleratorDynamic {
    gpu_driver_installation_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>>,
    gpu_sharing_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_GuestAccelerator {
    count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
    dynamic: ContainerCluster_NodeConfig_GuestAcceleratorDynamic,
}

impl ContainerCluster_NodeConfig_GuestAccelerator {
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_GuestAccelerator {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_GuestAccelerator {
    #[doc = "The number of the accelerator cards exposed to an instance."]
    pub count: terrars::PrimField<f64>,
    #[doc = "The accelerator type resource name."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_GuestAccelerator {
    pub fn build(self) -> ContainerCluster_NodeConfig_GuestAccelerator {
        ContainerCluster_NodeConfig_GuestAccelerator {
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
pub struct ContainerCluster_NodeConfig_GuestAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_GuestAcceleratorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_GuestAcceleratorRef {
        ContainerCluster_NodeConfig_GuestAcceleratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_GuestAcceleratorRef {
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
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_Gvnic {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodeConfig_Gvnic { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_Gvnic {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_Gvnic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_Gvnic {
    #[doc = "Whether or not gvnic is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodeConfig_Gvnic {
    pub fn build(self) -> ContainerCluster_NodeConfig_Gvnic {
        ContainerCluster_NodeConfig_Gvnic { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_GvnicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_GvnicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_GvnicRef {
        ContainerCluster_NodeConfig_GvnicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_GvnicRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_container_restart_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
        ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
            max_container_restart_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
        ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
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

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
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

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
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

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
        ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    #[doc =
        "Set the field `policy`.\nThe Memory Manager policy to use. This policy guides how memory and hugepages are allocated and managed for pods on the node, influencing NUMA affinity."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_MemoryManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_MemoryManager {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
        ContainerCluster_NodeConfig_KubeletConfig_MemoryManager { policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
        ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
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
pub struct ContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_TopologyManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig_TopologyManager {}

impl BuildContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
        ContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
        ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
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
struct ContainerCluster_NodeConfig_KubeletConfigDynamic {
    crash_loop_back_off: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    eviction_minimum_reclaim: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    eviction_soft: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>>,
    eviction_soft_grace_period: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
    memory_manager: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_MemoryManager>>,
    topology_manager: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig_TopologyManager>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_KubeletConfig {
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
    crash_loop_back_off: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_minimum_reclaim: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft_grace_period: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_manager: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_MemoryManager>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_manager: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig_TopologyManager>>,
    dynamic: ContainerCluster_NodeConfig_KubeletConfigDynamic,
}

impl ContainerCluster_NodeConfig_KubeletConfig {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
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
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>>,
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
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_MemoryManager>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig_TopologyManager>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_KubeletConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_KubeletConfig {}

impl BuildContainerCluster_NodeConfig_KubeletConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_KubeletConfig {
        ContainerCluster_NodeConfig_KubeletConfig {
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
pub struct ContainerCluster_NodeConfig_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_KubeletConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_KubeletConfigRef {
        ContainerCluster_NodeConfig_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_KubeletConfigRef {
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
    pub fn crash_loop_back_off(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crash_loop_back_off", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_minimum_reclaim", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft_grace_period", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_manager", self.base))
    }

    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ptp_kvm_time_sync: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\nWhether to enable accurate time synchronization with PTP-KVM."]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
        ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
        ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
            gcs_generation: core::default::Default::default(),
            gcs_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
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
struct ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitDynamic {
    init_script: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_script: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>>,
    dynamic: ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitDynamic,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[doc = "Set the field `init_script`.\n"]
    pub fn set_init_script(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
        ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
            init_script: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `init_script` after provisioning.\n"]
    pub fn init_script(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.init_script", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_1g: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_2m: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
        ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[doc = "Set the field `policy`.\nThe policy for kernel module loading."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
        ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
            policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[doc = "Set the field `disk_count`.\nThe number of physical local NVMe SSD disks to attach."]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    type O =
        terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
            disk_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[doc = "Set the field `disabled`.\nIf true, swap space will not be encrypted. Defaults to false (encrypted)."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    type O =
        terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
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
struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigDynamic {
    boot_disk_profile: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>>,
    dedicated_local_ssd_profile: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>>,
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>>,
    ephemeral_local_ssd_profile: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_local_ssd_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_local_ssd_profile: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>>,
    dynamic: ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigDynamic,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>,
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
                            ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile,
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
                        ::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>,
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
                            ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
        ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
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
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]
    pub fn dedicated_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_local_ssd_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]
    pub fn ephemeral_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_local_ssd_profile", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodeConfig_LinuxNodeConfigDynamic {
    accurate_time_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    custom_node_init: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    hugepages_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    node_kernel_module_loading: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    swap_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LinuxNodeConfig {
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
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_node_init: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepages_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kernel_module_loading: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>>,
    dynamic: ContainerCluster_NodeConfig_LinuxNodeConfigDynamic,
}

impl ContainerCluster_NodeConfig_LinuxNodeConfig {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
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
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LinuxNodeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LinuxNodeConfig {}

impl BuildContainerCluster_NodeConfig_LinuxNodeConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LinuxNodeConfig {
        ContainerCluster_NodeConfig_LinuxNodeConfig {
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
pub struct ContainerCluster_NodeConfig_LinuxNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LinuxNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LinuxNodeConfigRef {
        ContainerCluster_NodeConfig_LinuxNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LinuxNodeConfigRef {
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
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accurate_time_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_node_init` after provisioning.\n"]
    pub fn custom_node_init(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_node_init", self.base))
    }

    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hugepages_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kernel_module_loading", self.base))
    }

    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    local_ssd_count: terrars::PrimField<f64>,
}

impl ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    #[doc = "Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size."]
    pub local_ssd_count: terrars::PrimField<f64>,
}

impl BuildContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
        ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig { local_ssd_count: self.local_ssd_count }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
        ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
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
pub struct ContainerCluster_NodeConfig_NodeImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_NodeImageConfig {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_NodeImageConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_NodeImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_NodeImageConfig {}

impl BuildContainerCluster_NodeConfig_NodeImageConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_NodeImageConfig {
        ContainerCluster_NodeConfig_NodeImageConfig {
            image: core::default::Default::default(),
            image_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_NodeImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_NodeImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_NodeImageConfigRef {
        ContainerCluster_NodeConfig_NodeImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_NodeImageConfigRef {
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
pub struct ContainerCluster_NodeConfig_ReservationAffinity {
    consume_reservation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerCluster_NodeConfig_ReservationAffinity {
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_ReservationAffinity {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_ReservationAffinity {
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_ReservationAffinity {
    pub fn build(self) -> ContainerCluster_NodeConfig_ReservationAffinity {
        ContainerCluster_NodeConfig_ReservationAffinity {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_ReservationAffinityRef {
        ContainerCluster_NodeConfig_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_ReservationAffinityRef {
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
pub struct ContainerCluster_NodeConfig_SandboxConfig {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_SandboxConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_SandboxConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_SandboxConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_SandboxConfig {
    #[doc = "Type of the sandbox to use for the node (e.g. 'GVISOR')."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_SandboxConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_SandboxConfig {
        ContainerCluster_NodeConfig_SandboxConfig { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_SandboxConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_SandboxConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_SandboxConfigRef {
        ContainerCluster_NodeConfig_SandboxConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_SandboxConfigRef {
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
pub struct ContainerCluster_NodeConfig_SecondaryBootDisks {
    disk_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_SecondaryBootDisks {
    #[doc = "Set the field `mode`.\nMode for how the secondary boot disk is used."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_SecondaryBootDisks {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_SecondaryBootDisks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_SecondaryBootDisks {
    #[doc = "Disk image to create the secondary boot disk from"]
    pub disk_image: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_SecondaryBootDisks {
    pub fn build(self) -> ContainerCluster_NodeConfig_SecondaryBootDisks {
        ContainerCluster_NodeConfig_SecondaryBootDisks {
            disk_image: self.disk_image,
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_SecondaryBootDisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_SecondaryBootDisksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_SecondaryBootDisksRef {
        ContainerCluster_NodeConfig_SecondaryBootDisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_SecondaryBootDisksRef {
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
pub struct ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    #[doc = "."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = "."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    pub fn build(self) -> ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
        ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
        ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
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
struct ContainerCluster_NodeConfig_SoleTenantConfigDynamic {
    node_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig_SoleTenantConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>>,
    dynamic: ContainerCluster_NodeConfig_SoleTenantConfigDynamic,
}

impl ContainerCluster_NodeConfig_SoleTenantConfig {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig_SoleTenantConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_SoleTenantConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_SoleTenantConfig {}

impl BuildContainerCluster_NodeConfig_SoleTenantConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_SoleTenantConfig {
        ContainerCluster_NodeConfig_SoleTenantConfig {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_SoleTenantConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_SoleTenantConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_SoleTenantConfigRef {
        ContainerCluster_NodeConfig_SoleTenantConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_SoleTenantConfigRef {
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
pub struct ContainerCluster_NodeConfig_Taint {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_Taint { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_Taint {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_Taint {
    #[doc = "Effect for taint."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "Key for taint."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Value for taint."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_Taint {
    pub fn build(self) -> ContainerCluster_NodeConfig_Taint {
        ContainerCluster_NodeConfig_Taint {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_TaintRef {
        ContainerCluster_NodeConfig_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_TaintRef {
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
pub struct ContainerCluster_NodeConfig_TaintConfig {
    architecture_taint_behavior: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_TaintConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_TaintConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_TaintConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_TaintConfig {
    #[doc = "Architecture taint behavior. Controls, how we apply taints based on the node architecture."]
    pub architecture_taint_behavior: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_TaintConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_TaintConfig {
        ContainerCluster_NodeConfig_TaintConfig { architecture_taint_behavior: self.architecture_taint_behavior }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_TaintConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_TaintConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfig_TaintConfigRef {
        ContainerCluster_NodeConfig_TaintConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_TaintConfigRef {
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
pub struct ContainerCluster_NodeConfig_WindowsNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    osversion: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodeConfig_WindowsNodeConfig {
    #[doc =
        "Set the field `osversion`.\nThe OS Version of the windows nodepool.Values are OS_VERSION_UNSPECIFIED,OS_VERSION_LTSC2019 and OS_VERSION_LTSC2022"]
    pub fn set_osversion(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.osversion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodeConfig_WindowsNodeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_WindowsNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_WindowsNodeConfig {}

impl BuildContainerCluster_NodeConfig_WindowsNodeConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_WindowsNodeConfig {
        ContainerCluster_NodeConfig_WindowsNodeConfig { osversion: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_WindowsNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_WindowsNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_WindowsNodeConfigRef {
        ContainerCluster_NodeConfig_WindowsNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_WindowsNodeConfigRef {
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
pub struct ContainerCluster_NodeConfig_WorkloadMetadataConfig {
    mode: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeConfig_WorkloadMetadataConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeConfig_WorkloadMetadataConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig_WorkloadMetadataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig_WorkloadMetadataConfig {
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeConfig_WorkloadMetadataConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig_WorkloadMetadataConfig {
        ContainerCluster_NodeConfig_WorkloadMetadataConfig { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
        ContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
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
struct ContainerCluster_NodeConfigDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_AdvancedMachineFeatures>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_BootDisk>>,
    confidential_nodes: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ConfidentialNodes>>,
    containerd_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig>>,
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    fast_socket: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_FastSocket>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_GcfsConfig>>,
    guest_accelerator: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_GuestAccelerator>>,
    gvnic: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_Gvnic>>,
    kubelet_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_KubeletConfig>>,
    linux_node_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig>>,
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>>,
    node_image_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_NodeImageConfig>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ReservationAffinity>>,
    sandbox_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_SandboxConfig>>,
    secondary_boot_disks: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_SecondaryBootDisks>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    sole_tenant_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_SoleTenantConfig>>,
    taint: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_Taint>>,
    taint_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_TaintConfig>>,
    windows_node_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_WindowsNodeConfig>>,
    workload_metadata_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_WorkloadMetadataConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeConfig {
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
    advanced_machine_features: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ConfidentialNodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containerd_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_FastSocket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_GcfsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_GuestAccelerator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_Gvnic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_KubeletConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_image_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_NodeImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_SandboxConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_boot_disks: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_SecondaryBootDisks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_SoleTenantConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_Taint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_TaintConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_node_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_WindowsNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_WorkloadMetadataConfig>>,
    dynamic: ContainerCluster_NodeConfigDynamic,
}

impl ContainerCluster_NodeConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_AdvancedMachineFeatures>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_BootDisk>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ConfidentialNodes>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig>>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_FastSocket>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_GcfsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_GuestAccelerator>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_Gvnic>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_KubeletConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_NodeImageConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ReservationAffinity>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_SandboxConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_SecondaryBootDisks>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_SoleTenantConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_Taint>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_TaintConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_WindowsNodeConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_WorkloadMetadataConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_NodeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeConfig {}

impl BuildContainerCluster_NodeConfig {
    pub fn build(self) -> ContainerCluster_NodeConfig {
        ContainerCluster_NodeConfig {
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
pub struct ContainerCluster_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeConfigRef {
        ContainerCluster_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeConfigRef {
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
    pub fn effective_taints(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_EffectiveTaintsRef> {
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
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containerd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_FastSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_GcfsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_GuestAcceleratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_GvnicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_KubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_image_config` after provisioning.\n"]
    pub fn node_image_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_NodeImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_image_config", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_SandboxConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sandbox_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_SecondaryBootDisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_boot_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_SoleTenantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_TaintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `taint_config` after provisioning.\n"]
    pub fn taint_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_TaintConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_WindowsNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_WorkloadMetadataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodeCreationConfig {
    node_creation_mode: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodeCreationConfig { }

impl terrars::ToListMappable for ContainerCluster_NodeCreationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodeCreationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodeCreationConfig {
    #[doc =
        "NodeCreationMode defines the settings of node creation mode.\n Accepted values are:\n* VIA_KUBELET: Kubelet registers itself.\n* VIA_CONTROL_PLANE: gcp-controller-manager automatically creates the node object after CSR approval."]
    pub node_creation_mode: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodeCreationConfig {
    pub fn build(self) -> ContainerCluster_NodeCreationConfig {
        ContainerCluster_NodeCreationConfig { node_creation_mode: self.node_creation_mode }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodeCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodeCreationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodeCreationConfigRef {
        ContainerCluster_NodeCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodeCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_creation_mode` after provisioning.\nNodeCreationMode defines the settings of node creation mode.\n Accepted values are:\n* VIA_KUBELET: Kubelet registers itself.\n* VIA_CONTROL_PLANE: gcp-controller-manager automatically creates the node object after CSR approval."]
    pub fn node_creation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_creation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_Autoscaling {
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

impl ContainerCluster_NodePool_Autoscaling {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_Autoscaling {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_Autoscaling {}

impl BuildContainerCluster_NodePool_Autoscaling {
    pub fn build(self) -> ContainerCluster_NodePool_Autoscaling {
        ContainerCluster_NodePool_Autoscaling {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_AutoscalingRef {
        ContainerCluster_NodePool_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_AutoscalingRef {
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
pub struct ContainerCluster_NodePool_BestEffortProvisioning {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_provision_nodes: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerCluster_NodePool_BestEffortProvisioning {
    #[doc =
        "Set the field `min_provision_nodes`.\nMinimum number of nodes that must be provisioned for the creation to be considered successful. Can only be set when enabled is true."]
    pub fn set_min_provision_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_provision_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodePool_BestEffortProvisioning {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_BestEffortProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_BestEffortProvisioning {
    #[doc =
        "Whether node pool creation ignores non-fatal errors like stockout to provision as many nodes as possible, eventually bringing up the target number of nodes."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodePool_BestEffortProvisioning {
    pub fn build(self) -> ContainerCluster_NodePool_BestEffortProvisioning {
        ContainerCluster_NodePool_BestEffortProvisioning {
            enabled: self.enabled,
            min_provision_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_BestEffortProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_BestEffortProvisioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_BestEffortProvisioningRef {
        ContainerCluster_NodePool_BestEffortProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_BestEffortProvisioningRef {
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
pub struct ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[doc =
        "Set the field `enabled`.\nWhether to enable the maintenance exclusion until the end of support for this NodePool."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {}

impl BuildContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    pub fn build(self) -> ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
        ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
        ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
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
struct ContainerCluster_NodePool_MaintenancePolicyDynamic {
    exclusion_until_end_of_support: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_until_end_of_support: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
    dynamic: ContainerCluster_NodePool_MaintenancePolicyDynamic,
}

impl ContainerCluster_NodePool_MaintenancePolicy {
    #[doc = "Set the field `exclusion_until_end_of_support`.\n"]
    pub fn set_exclusion_until_end_of_support(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>,
                    >,
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

impl terrars::ToListMappable for ContainerCluster_NodePool_MaintenancePolicy {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_MaintenancePolicy {}

impl BuildContainerCluster_NodePool_MaintenancePolicy {
    pub fn build(self) -> ContainerCluster_NodePool_MaintenancePolicy {
        ContainerCluster_NodePool_MaintenancePolicy {
            exclusion_until_end_of_support: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_MaintenancePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_MaintenancePolicyRef {
        ContainerCluster_NodePool_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_until_end_of_support` after provisioning.\n"]
    pub fn exclusion_until_end_of_support(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_until_end_of_support", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodePool_Management {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_Management {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_Management {}

impl BuildContainerCluster_NodePool_Management {
    pub fn build(self) -> ContainerCluster_NodePool_Management {
        ContainerCluster_NodePool_Management {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_ManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_ManagementRef {
        ContainerCluster_NodePool_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_ManagementRef {
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
pub struct ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {}

impl BuildContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    pub fn build(self) -> ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
        ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
        ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
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
pub struct ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {}

impl BuildContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    pub fn build(self) -> ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
        ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
            max_pods_per_node: core::default::Default::default(),
            secondary_pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
        ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
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
pub struct ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig { }

impl terrars::ToListMappable for ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
    #[doc =
        "Specifies the total network bandwidth tier for the NodePool. [Valid values](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.Tier) include: \"TIER_1\" and \"TIER_UNSPECIFIED\"."]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
    pub fn build(self) -> ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
        ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef {
        ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef {
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

#[derive(Serialize, Default)]
struct ContainerCluster_NodePool_NetworkConfigDynamic {
    additional_node_network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    additional_pod_network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig>>,
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_NetworkConfig {
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
    ::Option<std::vec::Vec<ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_network_configs: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    dynamic: ContainerCluster_NodePool_NetworkConfigDynamic,
}

impl ContainerCluster_NodePool_NetworkConfig {
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
                    ::Into<
                        terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>,
                    >,
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
                    ::Into<
                        terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>,
                    >,
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
                    ::Into<
                        terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfig>,
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_NodePool_NetworkConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_NetworkConfig {}

impl BuildContainerCluster_NodePool_NetworkConfig {
    pub fn build(self) -> ContainerCluster_NodePool_NetworkConfig {
        ContainerCluster_NodePool_NetworkConfig {
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
pub struct ContainerCluster_NodePool_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_NetworkConfigRef {
        ContainerCluster_NodePool_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_NetworkConfigRef {
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
    ) -> terrars::ListRef<ContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_node_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_network_configs` after provisioning.\n"]
    pub fn additional_pod_network_configs(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodePool_NetworkConfig_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<ContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_NodeDrainConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_termination_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pdb_timeout_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_pdb_during_node_pool_deletion: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_NodePool_NodeDrainConfig {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_NodeDrainConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_NodeDrainConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_NodeDrainConfig {}

impl BuildContainerCluster_NodePool_NodeDrainConfig {
    pub fn build(self) -> ContainerCluster_NodePool_NodeDrainConfig {
        ContainerCluster_NodePool_NodeDrainConfig {
            grace_termination_duration: core::default::Default::default(),
            pdb_timeout_duration: core::default::Default::default(),
            respect_pdb_during_node_pool_deletion: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_NodeDrainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_NodeDrainConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_NodeDrainConfigRef {
        ContainerCluster_NodePool_NodeDrainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_NodeDrainConfigRef {
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
pub struct ContainerCluster_NodePool_PlacementPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_NodePool_PlacementPolicy {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_PlacementPolicy {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_PlacementPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_PlacementPolicy {
    #[doc = "Type defines the type of placement policy"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_NodePool_PlacementPolicy {
    pub fn build(self) -> ContainerCluster_NodePool_PlacementPolicy {
        ContainerCluster_NodePool_PlacementPolicy {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_PlacementPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_PlacementPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_PlacementPolicyRef {
        ContainerCluster_NodePool_PlacementPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_PlacementPolicyRef {
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
pub struct ContainerCluster_NodePool_QueuedProvisioning {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_NodePool_QueuedProvisioning { }

impl terrars::ToListMappable for ContainerCluster_NodePool_QueuedProvisioning {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_QueuedProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_QueuedProvisioning {
    #[doc = "Whether nodes in this node pool are obtainable solely through the ProvisioningRequest API"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NodePool_QueuedProvisioning {
    pub fn build(self) -> ContainerCluster_NodePool_QueuedProvisioning {
        ContainerCluster_NodePool_QueuedProvisioning { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_QueuedProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_QueuedProvisioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_QueuedProvisioningRef {
        ContainerCluster_NodePool_QueuedProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_QueuedProvisioningRef {
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
pub struct ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
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

impl terrars::ToListMappable for ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    type O =
        terrars::BlockAssignable<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {}

impl BuildContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    pub fn build(self) -> ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
        ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
        ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
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
struct ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsDynamic {
    standard_rollout_policy: std
    ::option
    ::Option<
        terrars::DynamicBlock<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>,
    >,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>>,
    dynamic: ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsDynamic,
}

impl ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
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
                            ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
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

impl terrars::ToListMappable for ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {}

impl BuildContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    pub fn build(self) -> ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
        ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
        ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
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
    ) -> terrars::ListRef<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePool_UpgradeSettingsDynamic {
    blue_green_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool_UpgradeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>>,
    dynamic: ContainerCluster_NodePool_UpgradeSettingsDynamic,
}

impl ContainerCluster_NodePool_UpgradeSettings {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>>,
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

impl terrars::ToListMappable for ContainerCluster_NodePool_UpgradeSettings {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool_UpgradeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool_UpgradeSettings {}

impl BuildContainerCluster_NodePool_UpgradeSettings {
    pub fn build(self) -> ContainerCluster_NodePool_UpgradeSettings {
        ContainerCluster_NodePool_UpgradeSettings {
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
            blue_green_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePool_UpgradeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePool_UpgradeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePool_UpgradeSettingsRef {
        ContainerCluster_NodePool_UpgradeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePool_UpgradeSettingsRef {
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
    pub fn blue_green_settings(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePoolDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_Autoscaling>>,
    best_effort_provisioning: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePool_BestEffortProvisioning>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_MaintenancePolicy>>,
    management: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_Management>>,
    network_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_NetworkConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig>>,
    node_drain_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_NodeDrainConfig>>,
    placement_policy: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_PlacementPolicy>>,
    queued_provisioning: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_QueuedProvisioning>>,
    upgrade_settings: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePool_UpgradeSettings>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_node_count_changes: std::option::Option<terrars::PrimField<bool>>,
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
    autoscaling: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    best_effort_provisioning: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_BestEffortProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_drain_config: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_NodeDrainConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_PlacementPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queued_provisioning: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_QueuedProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<std::vec::Vec<ContainerCluster_NodePool_UpgradeSettings>>,
    dynamic: ContainerCluster_NodePoolDynamic,
}

impl ContainerCluster_NodePool {
    #[doc =
        "Set the field `ignore_node_count_changes`.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state."]
    pub fn set_ignore_node_count_changes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_node_count_changes = std::option::Option::Some(v.into());
        self
    }

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
        "Set the field `name_prefix`.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name. Max length is 31 characters. Prefixes with lengths longer than 14 characters will use a shortened UUID that will be more prone to collisions."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_Autoscaling>>,
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

    #[doc = "Set the field `best_effort_provisioning`.\n"]
    pub fn set_best_effort_provisioning(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_BestEffortProvisioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.best_effort_provisioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.best_effort_provisioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_MaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_Management>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_NetworkConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig>>,
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

    #[doc = "Set the field `node_drain_config`.\n"]
    pub fn set_node_drain_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_NodeDrainConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_drain_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_drain_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_PlacementPolicy>>,
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

    #[doc = "Set the field `queued_provisioning`.\n"]
    pub fn set_queued_provisioning(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_QueuedProvisioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.queued_provisioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.queued_provisioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePool_UpgradeSettings>>,
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

impl terrars::ToListMappable for ContainerCluster_NodePool {
    type O = terrars::BlockAssignable<ContainerCluster_NodePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePool {}

impl BuildContainerCluster_NodePool {
    pub fn build(self) -> ContainerCluster_NodePool {
        ContainerCluster_NodePool {
            ignore_node_count_changes: core::default::Default::default(),
            initial_node_count: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
            name: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_locations: core::default::Default::default(),
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
            upgrade_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePoolRef {
        ContainerCluster_NodePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\nWhen true, the provider ignores external changes (drift) to the node count by skipping GCE API queries to the Instance Group Managers. This is a performance optimization for large clusters that saves API quota. Setting this to true will result in missing managed_instance_group_urls in the state."]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.base))
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
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name for the node pool beginning with the specified prefix. Conflicts with name. Max length is 31 characters. Prefixes with lengths longer than 14 characters will use a shortened UUID that will be more prone to collisions."]
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
    pub fn autoscaling(&self) -> terrars::ListRef<ContainerCluster_NodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `best_effort_provisioning` after provisioning.\n"]
    pub fn best_effort_provisioning(&self) -> terrars::ListRef<ContainerCluster_NodePool_BestEffortProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_effort_provisioning", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<ContainerCluster_NodePool_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<ContainerCluster_NodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<ContainerCluster_NodePool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> terrars::ListRef<ContainerCluster_NodePool_NodeDrainConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_drain_config", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<ContainerCluster_NodePool_PlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> terrars::ListRef<ContainerCluster_NodePool_QueuedProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queued_provisioning", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<ContainerCluster_NodePool_UpgradeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigDynamic {
    node_kernel_module_loading: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kernel_module_loading: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    dynamic: ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigDynamic,
}

impl ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    #[doc = "Set the field `cgroup_mode`.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn set_cgroup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cgroup_mode = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>,
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
}

impl terrars::ToListMappable for ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {}

impl BuildContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    pub fn build(self) -> ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
        ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
            cgroup_mode: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
        ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cgroup_mode` after provisioning.\ncgroupMode specifies the cgroup mode to be used on the node."]
    pub fn cgroup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> terrars::ListRef<ContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kernel_module_loading", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolAutoConfig_NetworkTags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContainerCluster_NodePoolAutoConfig_NetworkTags {
    #[doc = "Set the field `tags`.\nList of network tags applied to auto-provisioned node pools."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodePoolAutoConfig_NetworkTags {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_NetworkTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolAutoConfig_NetworkTags {}

impl BuildContainerCluster_NodePoolAutoConfig_NetworkTags {
    pub fn build(self) -> ContainerCluster_NodePoolAutoConfig_NetworkTags {
        ContainerCluster_NodePoolAutoConfig_NetworkTags { tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
        ContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nList of network tags applied to auto-provisioned node pools."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_kubelet_readonly_port_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    #[doc =
        "Set the field `insecure_kubelet_readonly_port_enabled`.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {}

impl BuildContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    pub fn build(self) -> ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
        ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
        ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePoolAutoConfigDynamic {
    linux_node_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>>,
    network_tags: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodePoolAutoConfig_NetworkTags>>,
    node_kubelet_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolAutoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolAutoConfig_NetworkTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kubelet_config: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>>,
    dynamic: ContainerCluster_NodePoolAutoConfigDynamic,
}

impl ContainerCluster_NodePoolAutoConfig {
    #[doc =
        "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>>,
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

    #[doc = "Set the field `network_tags`.\n"]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_NetworkTags>>,
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

    #[doc = "Set the field `node_kubelet_config`.\n"]
    pub fn set_node_kubelet_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_kubelet_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_kubelet_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_NodePoolAutoConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolAutoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolAutoConfig {}

impl BuildContainerCluster_NodePoolAutoConfig {
    pub fn build(self) -> ContainerCluster_NodePoolAutoConfig {
        ContainerCluster_NodePoolAutoConfig {
            resource_manager_tags: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            node_kubelet_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolAutoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolAutoConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePoolAutoConfigRef {
        ContainerCluster_NodePoolAutoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolAutoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<ContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(&self) -> terrars::ListRef<ContainerCluster_NodePoolAutoConfig_NetworkTagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kubelet_config` after provisioning.\n"]
    pub fn node_kubelet_config(&self) -> terrars::ListRef<ContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kubelet_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePoolDefaults_NodeConfigDefaultsDynamic {
    containerd_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_ContainerdConfig>>,
    gcfs_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_NodeConfig_GcfsConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_kubelet_readonly_port_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containerd_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_ContainerdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<std::vec::Vec<ContainerCluster_NodeConfig_GcfsConfig>>,
    dynamic: ContainerCluster_NodePoolDefaults_NodeConfigDefaultsDynamic,
}

impl ContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    #[doc =
        "Set the field `insecure_kubelet_readonly_port_enabled`.\nControls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`."]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logging_variant`.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn set_logging_variant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_variant = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_ContainerdConfig>>,
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

    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodeConfig_GcfsConfig>>,
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
}

impl terrars::ToListMappable for ContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolDefaults_NodeConfigDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolDefaults_NodeConfigDefaults {}

impl BuildContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    pub fn build(self) -> ContainerCluster_NodePoolDefaults_NodeConfigDefaults {
        ContainerCluster_NodePoolDefaults_NodeConfigDefaults {
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
        ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `logging_variant` after provisioning.\nType of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT."]
    pub fn logging_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_ContainerdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containerd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<ContainerCluster_NodeConfig_GcfsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NodePoolDefaultsDynamic {
    node_config_defaults: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_NodePoolDefaults_NodeConfigDefaults>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NodePoolDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config_defaults: std::option::Option<std::vec::Vec<ContainerCluster_NodePoolDefaults_NodeConfigDefaults>>,
    dynamic: ContainerCluster_NodePoolDefaultsDynamic,
}

impl ContainerCluster_NodePoolDefaults {
    #[doc = "Set the field `node_config_defaults`.\n"]
    pub fn set_node_config_defaults(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NodePoolDefaults_NodeConfigDefaults>>,
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

impl terrars::ToListMappable for ContainerCluster_NodePoolDefaults {
    type O = terrars::BlockAssignable<ContainerCluster_NodePoolDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NodePoolDefaults {}

impl BuildContainerCluster_NodePoolDefaults {
    pub fn build(self) -> ContainerCluster_NodePoolDefaults {
        ContainerCluster_NodePoolDefaults {
            node_config_defaults: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NodePoolDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NodePoolDefaultsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NodePoolDefaultsRef {
        ContainerCluster_NodePoolDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NodePoolDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_config_defaults` after provisioning.\n"]
    pub fn node_config_defaults(&self) -> terrars::ListRef<ContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config_defaults", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_NotificationConfig_Pubsub_Filter {
    event_type: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_NotificationConfig_Pubsub_Filter { }

impl terrars::ToListMappable for ContainerCluster_NotificationConfig_Pubsub_Filter {
    type O = terrars::BlockAssignable<ContainerCluster_NotificationConfig_Pubsub_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NotificationConfig_Pubsub_Filter {
    #[doc =
        "Can be used to filter what notifications are sent. Valid values include include UPGRADE_AVAILABLE_EVENT, UPGRADE_EVENT, SECURITY_BULLETIN_EVENT, and UPGRADE_INFO_EVENT"]
    pub event_type: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerCluster_NotificationConfig_Pubsub_Filter {
    pub fn build(self) -> ContainerCluster_NotificationConfig_Pubsub_Filter {
        ContainerCluster_NotificationConfig_Pubsub_Filter { event_type: self.event_type }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NotificationConfig_Pubsub_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NotificationConfig_Pubsub_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_NotificationConfig_Pubsub_FilterRef {
        ContainerCluster_NotificationConfig_Pubsub_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NotificationConfig_Pubsub_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_type` after provisioning.\nCan be used to filter what notifications are sent. Valid values include include UPGRADE_AVAILABLE_EVENT, UPGRADE_EVENT, SECURITY_BULLETIN_EVENT, and UPGRADE_INFO_EVENT"]
    pub fn event_type(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NotificationConfig_PubsubDynamic {
    filter: std::option::Option<terrars::DynamicBlock<ContainerCluster_NotificationConfig_Pubsub_Filter>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NotificationConfig_Pubsub {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<ContainerCluster_NotificationConfig_Pubsub_Filter>>,
    dynamic: ContainerCluster_NotificationConfig_PubsubDynamic,
}

impl ContainerCluster_NotificationConfig_Pubsub {
    #[doc =
        "Set the field `topic`.\nThe pubsub topic to push upgrade notifications to. Must be in the same project as the cluster. Must be in the format: projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NotificationConfig_Pubsub_Filter>>,
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

impl terrars::ToListMappable for ContainerCluster_NotificationConfig_Pubsub {
    type O = terrars::BlockAssignable<ContainerCluster_NotificationConfig_Pubsub>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NotificationConfig_Pubsub {
    #[doc = "Whether or not the notification config is enabled"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_NotificationConfig_Pubsub {
    pub fn build(self) -> ContainerCluster_NotificationConfig_Pubsub {
        ContainerCluster_NotificationConfig_Pubsub {
            enabled: self.enabled,
            topic: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NotificationConfig_PubsubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NotificationConfig_PubsubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NotificationConfig_PubsubRef {
        ContainerCluster_NotificationConfig_PubsubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NotificationConfig_PubsubRef {
    #[allow(dead_code)]
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
    pub fn filter(&self) -> terrars::ListRef<ContainerCluster_NotificationConfig_Pubsub_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_NotificationConfigDynamic {
    pubsub: std::option::Option<terrars::DynamicBlock<ContainerCluster_NotificationConfig_Pubsub>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub: std::option::Option<std::vec::Vec<ContainerCluster_NotificationConfig_Pubsub>>,
    dynamic: ContainerCluster_NotificationConfigDynamic,
}

impl ContainerCluster_NotificationConfig {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_NotificationConfig_Pubsub>>,
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

impl terrars::ToListMappable for ContainerCluster_NotificationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_NotificationConfig {}

impl BuildContainerCluster_NotificationConfig {
    pub fn build(self) -> ContainerCluster_NotificationConfig {
        ContainerCluster_NotificationConfig {
            pubsub: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_NotificationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_NotificationConfigRef {
        ContainerCluster_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> terrars::ListRef<ContainerCluster_NotificationConfig_PubsubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_PodAutoscaling {
    hpa_profile: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_PodAutoscaling { }

impl terrars::ToListMappable for ContainerCluster_PodAutoscaling {
    type O = terrars::BlockAssignable<ContainerCluster_PodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_PodAutoscaling {
    #[doc =
        "\n\t\t\t\t\t\t\t\tHPA Profile is used to configure the Horizontal Pod Autoscaler (HPA) profile for the cluster.\n\t\t\t\t\t\t\t\tAvailable options include:\n\t\t\t\t\t\t\t\t- NONE: Customers explicitly opt-out of HPA profiles.\n\t\t\t\t\t\t\t\t- PERFORMANCE: PERFORMANCE is used when customers opt-in to the performance HPA profile. In this profile we support a higher number of HPAs per cluster and faster metrics collection for workload autoscaling.\n\t\t\t\t\t\t\t"]
    pub hpa_profile: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_PodAutoscaling {
    pub fn build(self) -> ContainerCluster_PodAutoscaling {
        ContainerCluster_PodAutoscaling { hpa_profile: self.hpa_profile }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_PodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_PodAutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_PodAutoscalingRef {
        ContainerCluster_PodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_PodAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hpa_profile` after provisioning.\n\n\t\t\t\t\t\t\t\tHPA Profile is used to configure the Horizontal Pod Autoscaler (HPA) profile for the cluster.\n\t\t\t\t\t\t\t\tAvailable options include:\n\t\t\t\t\t\t\t\t- NONE: Customers explicitly opt-out of HPA profiles.\n\t\t\t\t\t\t\t\t- PERFORMANCE: PERFORMANCE is used when customers opt-in to the performance HPA profile. In this profile we support a higher number of HPAs per cluster and faster metrics collection for workload autoscaling.\n\t\t\t\t\t\t\t"]
    pub fn hpa_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hpa_profile", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig { }

impl terrars::ToListMappable for ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    type O = terrars::BlockAssignable<ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    #[doc = "Whether the cluster master is accessible globally or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    pub fn build(self) -> ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
        ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
        ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
    #[allow(dead_code)]
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
struct ContainerCluster_PrivateClusterConfigDynamic {
    master_global_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_PrivateClusterConfig {
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
    ::Option<std::vec::Vec<ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>>,
    dynamic: ContainerCluster_PrivateClusterConfigDynamic,
}

impl ContainerCluster_PrivateClusterConfig {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>>,
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

impl terrars::ToListMappable for ContainerCluster_PrivateClusterConfig {
    type O = terrars::BlockAssignable<ContainerCluster_PrivateClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_PrivateClusterConfig {}

impl BuildContainerCluster_PrivateClusterConfig {
    pub fn build(self) -> ContainerCluster_PrivateClusterConfig {
        ContainerCluster_PrivateClusterConfig {
            enable_private_endpoint: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            private_endpoint_subnetwork: core::default::Default::default(),
            master_global_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_PrivateClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_PrivateClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_PrivateClusterConfigRef {
        ContainerCluster_PrivateClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_PrivateClusterConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_global_access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_RbacBindingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_insecure_binding_system_authenticated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_insecure_binding_system_unauthenticated: std::option::Option<terrars::PrimField<bool>>,
}

impl ContainerCluster_RbacBindingConfig {
    #[doc =
        "Set the field `enable_insecure_binding_system_authenticated`.\nSetting this to true will allow any ClusterRoleBinding and RoleBinding with subjects system:authenticated."]
    pub fn set_enable_insecure_binding_system_authenticated(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_authenticated = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_insecure_binding_system_unauthenticated`.\nSetting this to true will allow any ClusterRoleBinding and RoleBinding with subjects system:anonymous or system:unauthenticated."]
    pub fn set_enable_insecure_binding_system_unauthenticated(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_unauthenticated = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_RbacBindingConfig {
    type O = terrars::BlockAssignable<ContainerCluster_RbacBindingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_RbacBindingConfig {}

impl BuildContainerCluster_RbacBindingConfig {
    pub fn build(self) -> ContainerCluster_RbacBindingConfig {
        ContainerCluster_RbacBindingConfig {
            enable_insecure_binding_system_authenticated: core::default::Default::default(),
            enable_insecure_binding_system_unauthenticated: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_RbacBindingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_RbacBindingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_RbacBindingConfigRef {
        ContainerCluster_RbacBindingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_RbacBindingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_insecure_binding_system_authenticated` after provisioning.\nSetting this to true will allow any ClusterRoleBinding and RoleBinding with subjects system:authenticated."]
    pub fn enable_insecure_binding_system_authenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insecure_binding_system_authenticated", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_insecure_binding_system_unauthenticated` after provisioning.\nSetting this to true will allow any ClusterRoleBinding and RoleBinding with subjects system:anonymous or system:unauthenticated."]
    pub fn enable_insecure_binding_system_unauthenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insecure_binding_system_unauthenticated", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ReleaseChannel {
    channel: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_ReleaseChannel { }

impl terrars::ToListMappable for ContainerCluster_ReleaseChannel {
    type O = terrars::BlockAssignable<ContainerCluster_ReleaseChannel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ReleaseChannel {
    #[doc =
        "The selected release channel. Accepted values are:\n* UNSPECIFIED: Not set.\n* RAPID: Weekly upgrade cadence; Early testers and developers who requires new features.\n* REGULAR: Multiple per month upgrade cadence; Production users who need features not yet offered in the Stable channel.\n* STABLE: Every few months upgrade cadence; Production users who need stability above all else, and for whom frequent upgrades are too risky.\n* EXTENDED: GKE provides extended support for Kubernetes minor versions through the Extended channel. With this channel, you can stay on a minor version for up to 24 months."]
    pub channel: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_ReleaseChannel {
    pub fn build(self) -> ContainerCluster_ReleaseChannel {
        ContainerCluster_ReleaseChannel { channel: self.channel }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ReleaseChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ReleaseChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_ReleaseChannelRef {
        ContainerCluster_ReleaseChannelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ReleaseChannelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe selected release channel. Accepted values are:\n* UNSPECIFIED: Not set.\n* RAPID: Weekly upgrade cadence; Early testers and developers who requires new features.\n* REGULAR: Multiple per month upgrade cadence; Production users who need features not yet offered in the Stable channel.\n* STABLE: Every few months upgrade cadence; Production users who need stability above all else, and for whom frequent upgrades are too risky.\n* EXTENDED: GKE provides extended support for Kubernetes minor versions through the Extended channel. With this channel, you can stay on a minor version for up to 24 months."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    dataset_id: terrars::PrimField<std::string::String>,
}

impl ContainerCluster_ResourceUsageExportConfig_BigqueryDestination { }

impl terrars::ToListMappable for ContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    type O = terrars::BlockAssignable<ContainerCluster_ResourceUsageExportConfig_BigqueryDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    #[doc = "The ID of a BigQuery Dataset."]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    pub fn build(self) -> ContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
        ContainerCluster_ResourceUsageExportConfig_BigqueryDestination { dataset_id: self.dataset_id }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
        ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of a BigQuery Dataset."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_ResourceUsageExportConfigDynamic {
    bigquery_destination: std
    ::option
    ::Option<terrars::DynamicBlock<ContainerCluster_ResourceUsageExportConfig_BigqueryDestination>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_ResourceUsageExportConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_egress_metering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_consumption_metering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination: std
    ::option
    ::Option<std::vec::Vec<ContainerCluster_ResourceUsageExportConfig_BigqueryDestination>>,
    dynamic: ContainerCluster_ResourceUsageExportConfigDynamic,
}

impl ContainerCluster_ResourceUsageExportConfig {
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
                    ::Into<terrars::BlockAssignable<ContainerCluster_ResourceUsageExportConfig_BigqueryDestination>>,
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

impl terrars::ToListMappable for ContainerCluster_ResourceUsageExportConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ResourceUsageExportConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ResourceUsageExportConfig {}

impl BuildContainerCluster_ResourceUsageExportConfig {
    pub fn build(self) -> ContainerCluster_ResourceUsageExportConfig {
        ContainerCluster_ResourceUsageExportConfig {
            enable_network_egress_metering: core::default::Default::default(),
            enable_resource_consumption_metering: core::default::Default::default(),
            bigquery_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ResourceUsageExportConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ResourceUsageExportConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_ResourceUsageExportConfigRef {
        ContainerCluster_ResourceUsageExportConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ResourceUsageExportConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_RollbackSafeUpgrade {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_RollbackSafeUpgrade {
    #[doc =
        "Set the field `control_plane_soak_duration`.\nA user-defined period that the cluster remains in the rollbackable state. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"604800s\" for 7 days. Minimum is 6 hours, maximum is 7 days. If omitted, the two-step upgrade is skipped and a standard one-step upgrade is performed."]
    pub fn set_control_plane_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_soak_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_RollbackSafeUpgrade {
    type O = terrars::BlockAssignable<ContainerCluster_RollbackSafeUpgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_RollbackSafeUpgrade {}

impl BuildContainerCluster_RollbackSafeUpgrade {
    pub fn build(self) -> ContainerCluster_RollbackSafeUpgrade {
        ContainerCluster_RollbackSafeUpgrade { control_plane_soak_duration: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_RollbackSafeUpgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_RollbackSafeUpgradeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_RollbackSafeUpgradeRef {
        ContainerCluster_RollbackSafeUpgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_RollbackSafeUpgradeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_soak_duration` after provisioning.\nA user-defined period that the cluster remains in the rollbackable state. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"604800s\" for 7 days. Minimum is 6 hours, maximum is 7 days. If omitted, the two-step upgrade is skipped and a standard one-step upgrade is performed."]
    pub fn control_plane_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_soak_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_SecretManagerConfig_RotationConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_SecretManagerConfig_RotationConfig {
    #[doc =
        "Set the field `rotation_interval`.\nThe interval between two consecutive rotations. Default rotation interval is 2 minutes"]
    pub fn set_rotation_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_SecretManagerConfig_RotationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_SecretManagerConfig_RotationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_SecretManagerConfig_RotationConfig {
    #[doc = "Enable the Secret manager auto rotation."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_SecretManagerConfig_RotationConfig {
    pub fn build(self) -> ContainerCluster_SecretManagerConfig_RotationConfig {
        ContainerCluster_SecretManagerConfig_RotationConfig {
            enabled: self.enabled,
            rotation_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_SecretManagerConfig_RotationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_SecretManagerConfig_RotationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_SecretManagerConfig_RotationConfigRef {
        ContainerCluster_SecretManagerConfig_RotationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_SecretManagerConfig_RotationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable the Secret manager auto rotation."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rotation_interval` after provisioning.\nThe interval between two consecutive rotations. Default rotation interval is 2 minutes"]
    pub fn rotation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_SecretManagerConfigDynamic {
    rotation_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_SecretManagerConfig_RotationConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_SecretManagerConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_config: std::option::Option<std::vec::Vec<ContainerCluster_SecretManagerConfig_RotationConfig>>,
    dynamic: ContainerCluster_SecretManagerConfigDynamic,
}

impl ContainerCluster_SecretManagerConfig {
    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_SecretManagerConfig_RotationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rotation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rotation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_SecretManagerConfig {
    type O = terrars::BlockAssignable<ContainerCluster_SecretManagerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_SecretManagerConfig {
    #[doc = "Enable the Secret manager csi component."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_SecretManagerConfig {
    pub fn build(self) -> ContainerCluster_SecretManagerConfig {
        ContainerCluster_SecretManagerConfig {
            enabled: self.enabled,
            rotation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_SecretManagerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_SecretManagerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_SecretManagerConfigRef {
        ContainerCluster_SecretManagerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_SecretManagerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable the Secret manager csi component."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(&self) -> terrars::ListRef<ContainerCluster_SecretManagerConfig_RotationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_SecretSyncConfig_RotationConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_SecretSyncConfig_RotationConfig {
    #[doc =
        "Set the field `rotation_interval`.\nThe interval between two consecutive rotations. Default rotation interval is 2 minutes"]
    pub fn set_rotation_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_SecretSyncConfig_RotationConfig {
    type O = terrars::BlockAssignable<ContainerCluster_SecretSyncConfig_RotationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_SecretSyncConfig_RotationConfig {
    #[doc = "Enable the Secret sync auto rotation."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_SecretSyncConfig_RotationConfig {
    pub fn build(self) -> ContainerCluster_SecretSyncConfig_RotationConfig {
        ContainerCluster_SecretSyncConfig_RotationConfig {
            enabled: self.enabled,
            rotation_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_SecretSyncConfig_RotationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_SecretSyncConfig_RotationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerCluster_SecretSyncConfig_RotationConfigRef {
        ContainerCluster_SecretSyncConfig_RotationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_SecretSyncConfig_RotationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable the Secret sync auto rotation."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rotation_interval` after provisioning.\nThe interval between two consecutive rotations. Default rotation interval is 2 minutes"]
    pub fn rotation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerCluster_SecretSyncConfigDynamic {
    rotation_config: std::option::Option<terrars::DynamicBlock<ContainerCluster_SecretSyncConfig_RotationConfig>>,
}

#[derive(Serialize)]
pub struct ContainerCluster_SecretSyncConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_config: std::option::Option<std::vec::Vec<ContainerCluster_SecretSyncConfig_RotationConfig>>,
    dynamic: ContainerCluster_SecretSyncConfigDynamic,
}

impl ContainerCluster_SecretSyncConfig {
    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCluster_SecretSyncConfig_RotationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rotation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rotation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_SecretSyncConfig {
    type O = terrars::BlockAssignable<ContainerCluster_SecretSyncConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_SecretSyncConfig {
    #[doc = "Enable the Sync as k8s secret add-on."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_SecretSyncConfig {
    pub fn build(self) -> ContainerCluster_SecretSyncConfig {
        ContainerCluster_SecretSyncConfig {
            enabled: self.enabled,
            rotation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_SecretSyncConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_SecretSyncConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_SecretSyncConfigRef {
        ContainerCluster_SecretSyncConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_SecretSyncConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable the Sync as k8s secret add-on."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(&self) -> terrars::ListRef<ContainerCluster_SecretSyncConfig_RotationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_SecurityPostureConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_SecurityPostureConfig {
    #[doc =
        "Set the field `mode`.\nSets the mode of the Kubernetes security posture API's off-cluster features. Available options include DISABLED, BASIC, and ENTERPRISE."]
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

impl terrars::ToListMappable for ContainerCluster_SecurityPostureConfig {
    type O = terrars::BlockAssignable<ContainerCluster_SecurityPostureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_SecurityPostureConfig {}

impl BuildContainerCluster_SecurityPostureConfig {
    pub fn build(self) -> ContainerCluster_SecurityPostureConfig {
        ContainerCluster_SecurityPostureConfig {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_SecurityPostureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_SecurityPostureConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_SecurityPostureConfigRef {
        ContainerCluster_SecurityPostureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_SecurityPostureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSets the mode of the Kubernetes security posture API's off-cluster features. Available options include DISABLED, BASIC, and ENTERPRISE."]
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
pub struct ContainerCluster_ServiceExternalIpsConfig {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_ServiceExternalIpsConfig { }

impl terrars::ToListMappable for ContainerCluster_ServiceExternalIpsConfig {
    type O = terrars::BlockAssignable<ContainerCluster_ServiceExternalIpsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_ServiceExternalIpsConfig {
    #[doc = "When enabled, services with external ips specified will be allowed."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_ServiceExternalIpsConfig {
    pub fn build(self) -> ContainerCluster_ServiceExternalIpsConfig {
        ContainerCluster_ServiceExternalIpsConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_ServiceExternalIpsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_ServiceExternalIpsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_ServiceExternalIpsConfigRef {
        ContainerCluster_ServiceExternalIpsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_ServiceExternalIpsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, services with external ips specified will be allowed."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_Timeouts {
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

impl terrars::ToListMappable for ContainerCluster_Timeouts {
    type O = terrars::BlockAssignable<ContainerCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_Timeouts {}

impl BuildContainerCluster_Timeouts {
    pub fn build(self) -> ContainerCluster_Timeouts {
        ContainerCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_TimeoutsRef {
        ContainerCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_TimeoutsRef {
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
pub struct ContainerCluster_UserManagedKeysConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_disk_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_api_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_peer_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gkeops_etcd_backup_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_signing_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_verification_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerCluster_UserManagedKeysConfig {
    #[doc =
        "Set the field `aggregation_ca`.\nThe Certificate Authority Service caPool to use for the aggreation CA in this cluster."]
    pub fn set_aggregation_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_ca`.\nThe Certificate Authority Service caPool to use for the cluster CA in this cluster."]
    pub fn set_cluster_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `control_plane_disk_encryption_key`.\nThe Cloud KMS cryptoKey to use for Confidential Hyperdisk on the control plane nodes."]
    pub fn set_control_plane_disk_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_disk_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etcd_api_ca`.\nThe Certificate Authority Service caPool to use for the etcd API CA in this cluster."]
    pub fn set_etcd_api_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etcd_api_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etcd_peer_ca`.\nThe Certificate Authority Service caPool to use for the etcd peer CA in this cluster."]
    pub fn set_etcd_peer_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etcd_peer_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gkeops_etcd_backup_encryption_key`.\nResource path of the Cloud KMS cryptoKey to use for encryption of internal etcd backups."]
    pub fn set_gkeops_etcd_backup_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gkeops_etcd_backup_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_signing_keys`.\nThe Cloud KMS cryptoKeyVersions to use for signing service account JWTs issued by this cluster."]
    pub fn set_service_account_signing_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_signing_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_verification_keys`.\nThe Cloud KMS cryptoKeyVersions to use for verifying service account JWTs issued by this cluster."]
    pub fn set_service_account_verification_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_verification_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_UserManagedKeysConfig {
    type O = terrars::BlockAssignable<ContainerCluster_UserManagedKeysConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_UserManagedKeysConfig {}

impl BuildContainerCluster_UserManagedKeysConfig {
    pub fn build(self) -> ContainerCluster_UserManagedKeysConfig {
        ContainerCluster_UserManagedKeysConfig {
            aggregation_ca: core::default::Default::default(),
            cluster_ca: core::default::Default::default(),
            control_plane_disk_encryption_key: core::default::Default::default(),
            etcd_api_ca: core::default::Default::default(),
            etcd_peer_ca: core::default::Default::default(),
            gkeops_etcd_backup_encryption_key: core::default::Default::default(),
            service_account_signing_keys: core::default::Default::default(),
            service_account_verification_keys: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_UserManagedKeysConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_UserManagedKeysConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_UserManagedKeysConfigRef {
        ContainerCluster_UserManagedKeysConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_UserManagedKeysConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aggregation_ca` after provisioning.\nThe Certificate Authority Service caPool to use for the aggreation CA in this cluster."]
    pub fn aggregation_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ca` after provisioning.\nThe Certificate Authority Service caPool to use for the cluster CA in this cluster."]
    pub fn cluster_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_disk_encryption_key` after provisioning.\nThe Cloud KMS cryptoKey to use for Confidential Hyperdisk on the control plane nodes."]
    pub fn control_plane_disk_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_disk_encryption_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_disk_encryption_key_versions` after provisioning.\nThe Cloud KMS cryptoKey versions to use for Confidential Hyperdisk on the control plane nodes."]
    pub fn control_plane_disk_encryption_key_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.control_plane_disk_encryption_key_versions", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `etcd_api_ca` after provisioning.\nThe Certificate Authority Service caPool to use for the etcd API CA in this cluster."]
    pub fn etcd_api_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_api_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `etcd_peer_ca` after provisioning.\nThe Certificate Authority Service caPool to use for the etcd peer CA in this cluster."]
    pub fn etcd_peer_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_peer_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gkeops_etcd_backup_encryption_key` after provisioning.\nResource path of the Cloud KMS cryptoKey to use for encryption of internal etcd backups."]
    pub fn gkeops_etcd_backup_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gkeops_etcd_backup_encryption_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_signing_keys` after provisioning.\nThe Cloud KMS cryptoKeyVersions to use for signing service account JWTs issued by this cluster."]
    pub fn service_account_signing_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_account_signing_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_verification_keys` after provisioning.\nThe Cloud KMS cryptoKeyVersions to use for verifying service account JWTs issued by this cluster."]
    pub fn service_account_verification_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_account_verification_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_VerticalPodAutoscaling {
    enabled: terrars::PrimField<bool>,
}

impl ContainerCluster_VerticalPodAutoscaling { }

impl terrars::ToListMappable for ContainerCluster_VerticalPodAutoscaling {
    type O = terrars::BlockAssignable<ContainerCluster_VerticalPodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_VerticalPodAutoscaling {
    #[doc = "Enables vertical pod autoscaling."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildContainerCluster_VerticalPodAutoscaling {
    pub fn build(self) -> ContainerCluster_VerticalPodAutoscaling {
        ContainerCluster_VerticalPodAutoscaling { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_VerticalPodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_VerticalPodAutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_VerticalPodAutoscalingRef {
        ContainerCluster_VerticalPodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_VerticalPodAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables vertical pod autoscaling."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCluster_WorkloadIdentityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerCluster_WorkloadIdentityConfig {
    #[doc = "Set the field `workload_pool`.\nThe workload pool to attach all Kubernetes service accounts to."]
    pub fn set_workload_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerCluster_WorkloadIdentityConfig {
    type O = terrars::BlockAssignable<ContainerCluster_WorkloadIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCluster_WorkloadIdentityConfig {}

impl BuildContainerCluster_WorkloadIdentityConfig {
    pub fn build(self) -> ContainerCluster_WorkloadIdentityConfig {
        ContainerCluster_WorkloadIdentityConfig { workload_pool: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContainerCluster_WorkloadIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCluster_WorkloadIdentityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCluster_WorkloadIdentityConfigRef {
        ContainerCluster_WorkloadIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCluster_WorkloadIdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workload_pool` after provisioning.\nThe workload pool to attach all Kubernetes service accounts to."]
    pub fn workload_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_pool", self.base))
    }
}
