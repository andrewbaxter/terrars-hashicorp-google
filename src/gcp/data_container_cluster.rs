#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataContainerClusterData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_node_pool_refresh: std::option::Option<terrars::PrimField<bool>>,
}

struct DataContainerCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataContainerClusterData_>,
}

#[derive(Clone)]
pub struct DataContainerCluster(std::rc::Rc<DataContainerCluster_>);

impl DataContainerCluster {
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
        "Set the field `location`.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_node_pool_refresh`.\nIf true, the provider will not refresh the inline node_pool state from the API during cluster reads. Set this to true only when all node pools are managed via separate google_container_node_pool resources; it substantially improves plan/apply performance on clusters with a high node pool count. Must not be set to true when inline node_pool blocks are defined on this resource."]
    pub fn set_skip_node_pool_refresh(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_node_pool_refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\nAnonymousAuthenticationConfig allows users to restrict or enable anonymous access to the cluster."]
    pub fn anonymous_authentication_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AnonymousAuthenticationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<DataContainerCluster_AuthenticatorGroupsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\nConfiguration for the cluster policy."]
    pub fn autopilot_cluster_policy_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AutopilotClusterPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
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
        "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataContainerCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_ClusterAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> terrars::ListRef<DataContainerCluster_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\nConfiguration for all of the cluster's control plane endpoints. Currently supports only DNS endpoint configuration and disable IP endpoint. Other IP endpoint configurations are available in private_cluster_config."]
    pub fn control_plane_endpoints_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ControlPlaneEndpointsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> terrars::ListRef<DataContainerCluster_CostManagementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> terrars::ListRef<DataContainerCluster_DatabaseEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
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
        "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> terrars::ListRef<DataContainerCluster_DefaultSnatStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
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
        "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> terrars::ListRef<DataContainerCluster_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
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
        "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<DataContainerCluster_EnableK8sBetaApisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `enterprise_config` after provisioning.\nDefines the config needed to enable/disable GKE Enterprise"]
    pub fn enterprise_config(&self) -> terrars::ListRef<DataContainerCluster_EnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<DataContainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> terrars::ListRef<DataContainerCluster_GatewayApiConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\nConfiguration options for the auto-upgrade patch type feature, which provide more control over the speed of automatic upgrades of your GKE clusters."]
    pub fn gke_auto_upgrade_config(&self) -> terrars::ListRef<DataContainerCluster_GkeAutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(&self) -> terrars::ListRef<DataContainerCluster_IdentityServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
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
        "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
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
        "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> terrars::ListRef<DataContainerCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataContainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_opentelemetry_config` after provisioning.\nThe configuration for the Managed OpenTelemetry pipeline inside the cluster."]
    pub fn managed_opentelemetry_config(&self) -> terrars::ListRef<DataContainerCluster_ManagedOpentelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_opentelemetry_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> terrars::ListRef<DataContainerCluster_MasterAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> terrars::ListRef<DataContainerCluster_MeshCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> terrars::ListRef<DataContainerCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
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
        "Get a reference to the value of field `network_performance_config` after provisioning.\nNetwork bandwidth tier configuration."]
    pub fn network_performance_config(&self) -> terrars::ListRef<DataContainerCluster_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> terrars::ListRef<DataContainerCluster_NetworkPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_creation_config` after provisioning.\nNodeCreationConfig defines the settings of node creation mode."]
    pub fn node_creation_config(&self) -> terrars::ListRef<DataContainerCluster_NodeCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_creation_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> terrars::ListRef<DataContainerCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<DataContainerCluster_NodePoolAutoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<DataContainerCluster_NodePoolDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> terrars::ListRef<DataContainerCluster_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pod_autoscaling` after provisioning.\nPodAutoscaling is used for configuration of parameters for workload autoscaling"]
    pub fn pod_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_PodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> terrars::ListRef<DataContainerCluster_PrivateClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
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
        "Get a reference to the value of field `rbac_binding_config` after provisioning.\nRBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created."]
    pub fn rbac_binding_config(&self) -> terrars::ListRef<DataContainerCluster_RbacBindingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rbac_binding_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> terrars::ListRef<DataContainerCluster_ReleaseChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
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
        "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<DataContainerCluster_ResourceUsageExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollback_safe_upgrade` after provisioning.\nConfiguration for rollback-safe (two-step) upgrades."]
    pub fn rollback_safe_upgrade(&self) -> terrars::ListRef<DataContainerCluster_RollbackSafeUpgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_safe_upgrade", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_config` after provisioning.\nConfiguration for the Secret Manager feature."]
    pub fn secret_manager_config(&self) -> terrars::ListRef<DataContainerCluster_SecretManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_sync_config` after provisioning.\nConfiguration for the Sync as k8s secrets feature."]
    pub fn secret_sync_config(&self) -> terrars::ListRef<DataContainerCluster_SecretSyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_sync_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(&self) -> terrars::ListRef<DataContainerCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<DataContainerCluster_ServiceExternalIpsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `user_managed_keys_config` after provisioning.\nThe custom keys configuration of the cluster."]
    pub fn user_managed_keys_config(&self) -> terrars::ListRef<DataContainerCluster_UserManagedKeysConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed_keys_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_VerticalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<DataContainerCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataContainerCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataContainerCluster { }

impl terrars::ToListMappable for DataContainerCluster {
    type O = terrars::ListRef<DataContainerClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataContainerCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_container_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataContainerCluster {
    pub tf_id: std::string::String,
    #[doc = "The name of the cluster, unique within the project and location."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataContainerCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataContainerCluster {
        let out = DataContainerCluster(std::rc::Rc::new(DataContainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataContainerClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                skip_node_pool_refresh: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataContainerClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataContainerClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_net_admin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\nAnonymousAuthenticationConfig allows users to restrict or enable anonymous access to the cluster."]
    pub fn anonymous_authentication_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AnonymousAuthenticationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(&self) -> terrars::ListRef<DataContainerCluster_AuthenticatorGroupsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticator_groups_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\nConfiguration for the cluster policy."]
    pub fn autopilot_cluster_policy_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AutopilotClusterPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
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
        "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> terrars::ListRef<DataContainerCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_ClusterAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> terrars::ListRef<DataContainerCluster_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\nConfiguration for all of the cluster's control plane endpoints. Currently supports only DNS endpoint configuration and disable IP endpoint. Other IP endpoint configurations are available in private_cluster_config."]
    pub fn control_plane_endpoints_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ControlPlaneEndpointsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> terrars::ListRef<DataContainerCluster_CostManagementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_management_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> terrars::ListRef<DataContainerCluster_DatabaseEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_encryption", self.extract_ref()))
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
        "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> terrars::ListRef<DataContainerCluster_DefaultSnatStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_snat_status", self.extract_ref()))
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
        "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> terrars::ListRef<DataContainerCluster_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
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
        "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> terrars::ListRef<DataContainerCluster_EnableK8sBetaApisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_k8s_beta_apis", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `enterprise_config` after provisioning.\nDefines the config needed to enable/disable GKE Enterprise"]
    pub fn enterprise_config(&self) -> terrars::ListRef<DataContainerCluster_EnterpriseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<DataContainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> terrars::ListRef<DataContainerCluster_GatewayApiConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_api_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\nConfiguration options for the auto-upgrade patch type feature, which provide more control over the speed of automatic upgrades of your GKE clusters."]
    pub fn gke_auto_upgrade_config(&self) -> terrars::ListRef<DataContainerCluster_GkeAutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(&self) -> terrars::ListRef<DataContainerCluster_IdentityServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_service_config", self.extract_ref()))
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
        "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_allocation_policy", self.extract_ref()))
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
        "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> terrars::ListRef<DataContainerCluster_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataContainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_opentelemetry_config` after provisioning.\nThe configuration for the Managed OpenTelemetry pipeline inside the cluster."]
    pub fn managed_opentelemetry_config(&self) -> terrars::ListRef<DataContainerCluster_ManagedOpentelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_opentelemetry_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> terrars::ListRef<DataContainerCluster_MasterAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_auth", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MasterAuthorizedNetworksConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> terrars::ListRef<DataContainerCluster_MeshCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh_certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_master_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> terrars::ListRef<DataContainerCluster_MonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_config", self.extract_ref()))
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
        "Get a reference to the value of field `network_performance_config` after provisioning.\nNetwork bandwidth tier configuration."]
    pub fn network_performance_config(&self) -> terrars::ListRef<DataContainerCluster_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> terrars::ListRef<DataContainerCluster_NetworkPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_creation_config` after provisioning.\nNodeCreationConfig defines the settings of node creation mode."]
    pub fn node_creation_config(&self) -> terrars::ListRef<DataContainerCluster_NodeCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_creation_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> terrars::ListRef<DataContainerCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> terrars::ListRef<DataContainerCluster_NodePoolAutoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_auto_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> terrars::ListRef<DataContainerCluster_NodePoolDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_defaults", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> terrars::ListRef<DataContainerCluster_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pod_autoscaling` after provisioning.\nPodAutoscaling is used for configuration of parameters for workload autoscaling"]
    pub fn pod_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_PodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> terrars::ListRef<DataContainerCluster_PrivateClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_cluster_config", self.extract_ref()))
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
        "Get a reference to the value of field `rbac_binding_config` after provisioning.\nRBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created."]
    pub fn rbac_binding_config(&self) -> terrars::ListRef<DataContainerCluster_RbacBindingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rbac_binding_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> terrars::ListRef<DataContainerCluster_ReleaseChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
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
        "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(&self) -> terrars::ListRef<DataContainerCluster_ResourceUsageExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_usage_export_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollback_safe_upgrade` after provisioning.\nConfiguration for rollback-safe (two-step) upgrades."]
    pub fn rollback_safe_upgrade(&self) -> terrars::ListRef<DataContainerCluster_RollbackSafeUpgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_safe_upgrade", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_config` after provisioning.\nConfiguration for the Secret Manager feature."]
    pub fn secret_manager_config(&self) -> terrars::ListRef<DataContainerCluster_SecretManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_manager_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_sync_config` after provisioning.\nConfiguration for the Sync as k8s secrets feature."]
    pub fn secret_sync_config(&self) -> terrars::ListRef<DataContainerCluster_SecretSyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_sync_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(&self) -> terrars::ListRef<DataContainerCluster_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(&self) -> terrars::ListRef<DataContainerCluster_ServiceExternalIpsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_external_ips_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `user_managed_keys_config` after provisioning.\nThe custom keys configuration of the cluster."]
    pub fn user_managed_keys_config(&self) -> terrars::ListRef<DataContainerCluster_UserManagedKeysConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed_keys_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(&self) -> terrars::ListRef<DataContainerCluster_VerticalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertical_pod_autoscaling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(&self) -> terrars::ListRef<DataContainerCluster_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AddonsConfig_AgentSandboxConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_AgentSandboxConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_AgentSandboxConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_AgentSandboxConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_AgentSandboxConfig {}

impl BuildDataContainerCluster_AddonsConfig_AgentSandboxConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_AgentSandboxConfig {
        DataContainerCluster_AddonsConfig_AgentSandboxConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_AgentSandboxConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_AgentSandboxConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_AgentSandboxConfigRef {
        DataContainerCluster_AddonsConfig_AgentSandboxConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_AgentSandboxConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_CloudrunConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_AddonsConfig_CloudrunConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.load_balancer_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_CloudrunConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_CloudrunConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_CloudrunConfig {}

impl BuildDataContainerCluster_AddonsConfig_CloudrunConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_CloudrunConfig {
        DataContainerCluster_AddonsConfig_CloudrunConfig {
            disabled: core::default::Default::default(),
            load_balancer_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_CloudrunConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_CloudrunConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_CloudrunConfigRef {
        DataContainerCluster_AddonsConfig_CloudrunConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_CloudrunConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_ConfigConnectorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_ConfigConnectorConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_ConfigConnectorConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_ConfigConnectorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_ConfigConnectorConfig {}

impl BuildDataContainerCluster_AddonsConfig_ConfigConnectorConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_ConfigConnectorConfig {
        DataContainerCluster_AddonsConfig_ConfigConnectorConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
        DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_DnsCacheConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_DnsCacheConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_DnsCacheConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_DnsCacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_DnsCacheConfig {}

impl BuildDataContainerCluster_AddonsConfig_DnsCacheConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_DnsCacheConfig {
        DataContainerCluster_AddonsConfig_DnsCacheConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_DnsCacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_DnsCacheConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_DnsCacheConfigRef {
        DataContainerCluster_AddonsConfig_DnsCacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_DnsCacheConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {}

impl BuildDataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
        DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
        DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {}

impl BuildDataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig {
        DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
        DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {}

impl BuildDataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig {
        DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
        DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_GkeBackupAgentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_GkeBackupAgentConfig {}

impl BuildDataContainerCluster_AddonsConfig_GkeBackupAgentConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_GkeBackupAgentConfig {
        DataContainerCluster_AddonsConfig_GkeBackupAgentConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
        DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {}

impl BuildDataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig {
        DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
        DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {}

impl BuildDataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling {
        DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
        DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef {
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
pub struct DataContainerCluster_AddonsConfig_HttpLoadBalancing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_HttpLoadBalancing {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_HttpLoadBalancing {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_HttpLoadBalancing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_HttpLoadBalancing {}

impl BuildDataContainerCluster_AddonsConfig_HttpLoadBalancing {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_HttpLoadBalancing {
        DataContainerCluster_AddonsConfig_HttpLoadBalancing { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_HttpLoadBalancingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_HttpLoadBalancingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_HttpLoadBalancingRef {
        DataContainerCluster_AddonsConfig_HttpLoadBalancingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_HttpLoadBalancingRef {
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
pub struct DataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_multi_nic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_legacy_lustre_port: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    #[doc = "Set the field `disable_multi_nic`.\n"]
    pub fn set_disable_multi_nic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_multi_nic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_legacy_lustre_port`.\n"]
    pub fn set_enable_legacy_lustre_port(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_legacy_lustre_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_LustreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_LustreCsiDriverConfig {}

impl BuildDataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
        DataContainerCluster_AddonsConfig_LustreCsiDriverConfig {
            disable_multi_nic: core::default::Default::default(),
            enable_legacy_lustre_port: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
        DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_multi_nic` after provisioning.\n"]
    pub fn disable_multi_nic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_multi_nic", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_legacy_lustre_port` after provisioning.\n"]
    pub fn enable_legacy_lustre_port(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_legacy_lustre_port", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AddonsConfig_NetworkPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_NetworkPolicyConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_NetworkPolicyConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_NetworkPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_NetworkPolicyConfig {}

impl BuildDataContainerCluster_AddonsConfig_NetworkPolicyConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_NetworkPolicyConfig {
        DataContainerCluster_AddonsConfig_NetworkPolicyConfig { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
        DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_NodeReadinessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_NodeReadinessConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_NodeReadinessConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_NodeReadinessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_NodeReadinessConfig {}

impl BuildDataContainerCluster_AddonsConfig_NodeReadinessConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_NodeReadinessConfig {
        DataContainerCluster_AddonsConfig_NodeReadinessConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_NodeReadinessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_NodeReadinessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_NodeReadinessConfigRef {
        DataContainerCluster_AddonsConfig_NodeReadinessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_NodeReadinessConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {}

impl BuildDataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig {
        DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
        DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_PodSnapshotConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_PodSnapshotConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_PodSnapshotConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_PodSnapshotConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_PodSnapshotConfig {}

impl BuildDataContainerCluster_AddonsConfig_PodSnapshotConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_PodSnapshotConfig {
        DataContainerCluster_AddonsConfig_PodSnapshotConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_PodSnapshotConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_PodSnapshotConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_PodSnapshotConfigRef {
        DataContainerCluster_AddonsConfig_PodSnapshotConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_PodSnapshotConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {}

impl BuildDataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
        DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
        DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    type O =
        terrars::BlockAssignable<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {}

impl BuildDataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
        DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
        DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_cluster_logging_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_cluster_monitoring_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>>,
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ray_cluster_logging_config`.\n"]
    pub fn set_ray_cluster_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfig>,
                    >,
    ) -> Self {
        self.ray_cluster_logging_config = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfig>,
                    >,
    ) -> Self {
        self.ray_cluster_monitoring_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_RayOperatorConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_RayOperatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_RayOperatorConfig {}

impl BuildDataContainerCluster_AddonsConfig_RayOperatorConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_RayOperatorConfig {
        DataContainerCluster_AddonsConfig_RayOperatorConfig {
            enabled: core::default::Default::default(),
            ray_cluster_logging_config: core::default::Default::default(),
            ray_cluster_monitoring_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_RayOperatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_RayOperatorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_RayOperatorConfigRef {
        DataContainerCluster_AddonsConfig_RayOperatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_RayOperatorConfigRef {
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
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_cluster_logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ray_cluster_monitoring_config` after provisioning.\n"]
    pub fn ray_cluster_monitoring_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_RayOperatorConfig_RayClusterMonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_cluster_monitoring_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AddonsConfig_SliceControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_SliceControllerConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_SliceControllerConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_SliceControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_SliceControllerConfig {}

impl BuildDataContainerCluster_AddonsConfig_SliceControllerConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_SliceControllerConfig {
        DataContainerCluster_AddonsConfig_SliceControllerConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_SliceControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_SliceControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_SliceControllerConfigRef {
        DataContainerCluster_AddonsConfig_SliceControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_SliceControllerConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_SlurmOperatorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_SlurmOperatorConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_SlurmOperatorConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_SlurmOperatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_SlurmOperatorConfig {}

impl BuildDataContainerCluster_AddonsConfig_SlurmOperatorConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_SlurmOperatorConfig {
        DataContainerCluster_AddonsConfig_SlurmOperatorConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
        DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef {
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
pub struct DataContainerCluster_AddonsConfig_StatefulHaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AddonsConfig_StatefulHaConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig_StatefulHaConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig_StatefulHaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig_StatefulHaConfig {}

impl BuildDataContainerCluster_AddonsConfig_StatefulHaConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig_StatefulHaConfig {
        DataContainerCluster_AddonsConfig_StatefulHaConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfig_StatefulHaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfig_StatefulHaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AddonsConfig_StatefulHaConfigRef {
        DataContainerCluster_AddonsConfig_StatefulHaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfig_StatefulHaConfigRef {
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
pub struct DataContainerCluster_AddonsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_sandbox_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_AgentSandboxConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudrun_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_CloudrunConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_connector_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_ConfigConnectorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_cache_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_DnsCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_persistent_disk_csi_driver_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_filestore_csi_driver_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_fuse_csi_driver_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_backup_agent_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_GkeBackupAgentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_scale_checkpointing_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaling: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_load_balancing: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_HttpLoadBalancing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lustre_csi_driver_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_LustreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_policy_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_NetworkPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_readiness_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_NodeReadinessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelstore_csi_driver_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_snapshot_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_PodSnapshotConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_operator_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slice_controller_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_AddonsConfig_SliceControllerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slurm_operator_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_SlurmOperatorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_ha_config: std::option::Option<terrars::ListField<DataContainerCluster_AddonsConfig_StatefulHaConfig>>,
}

impl DataContainerCluster_AddonsConfig {
    #[doc = "Set the field `agent_sandbox_config`.\n"]
    pub fn set_agent_sandbox_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_AgentSandboxConfig>>,
    ) -> Self {
        self.agent_sandbox_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudrun_config`.\n"]
    pub fn set_cloudrun_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_CloudrunConfig>>,
    ) -> Self {
        self.cloudrun_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config_connector_config`.\n"]
    pub fn set_config_connector_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_ConfigConnectorConfig>>,
    ) -> Self {
        self.config_connector_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_cache_config`.\n"]
    pub fn set_dns_cache_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_DnsCacheConfig>>,
    ) -> Self {
        self.dns_cache_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gce_persistent_disk_csi_driver_config`.\n"]
    pub fn set_gce_persistent_disk_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfig>>,
    ) -> Self {
        self.gce_persistent_disk_csi_driver_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_filestore_csi_driver_config`.\n"]
    pub fn set_gcp_filestore_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfig>>,
    ) -> Self {
        self.gcp_filestore_csi_driver_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_fuse_csi_driver_config`.\n"]
    pub fn set_gcs_fuse_csi_driver_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfig>>,
    ) -> Self {
        self.gcs_fuse_csi_driver_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_backup_agent_config`.\n"]
    pub fn set_gke_backup_agent_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_GkeBackupAgentConfig>>,
    ) -> Self {
        self.gke_backup_agent_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `high_scale_checkpointing_config`.\n"]
    pub fn set_high_scale_checkpointing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfig>>,
    ) -> Self {
        self.high_scale_checkpointing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `horizontal_pod_autoscaling`.\n"]
    pub fn set_horizontal_pod_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_HorizontalPodAutoscaling>>,
    ) -> Self {
        self.horizontal_pod_autoscaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_load_balancing`.\n"]
    pub fn set_http_load_balancing(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_HttpLoadBalancing>>,
    ) -> Self {
        self.http_load_balancing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lustre_csi_driver_config`.\n"]
    pub fn set_lustre_csi_driver_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_LustreCsiDriverConfig>>,
    ) -> Self {
        self.lustre_csi_driver_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_policy_config`.\n"]
    pub fn set_network_policy_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_NetworkPolicyConfig>>,
    ) -> Self {
        self.network_policy_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_readiness_config`.\n"]
    pub fn set_node_readiness_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_NodeReadinessConfig>>,
    ) -> Self {
        self.node_readiness_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelstore_csi_driver_config`.\n"]
    pub fn set_parallelstore_csi_driver_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfig>>,
    ) -> Self {
        self.parallelstore_csi_driver_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_snapshot_config`.\n"]
    pub fn set_pod_snapshot_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_PodSnapshotConfig>>,
    ) -> Self {
        self.pod_snapshot_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ray_operator_config`.\n"]
    pub fn set_ray_operator_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_RayOperatorConfig>>,
    ) -> Self {
        self.ray_operator_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slice_controller_config`.\n"]
    pub fn set_slice_controller_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_SliceControllerConfig>>,
    ) -> Self {
        self.slice_controller_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slurm_operator_config`.\n"]
    pub fn set_slurm_operator_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_SlurmOperatorConfig>>,
    ) -> Self {
        self.slurm_operator_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_ha_config`.\n"]
    pub fn set_stateful_ha_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_AddonsConfig_StatefulHaConfig>>,
    ) -> Self {
        self.stateful_ha_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AddonsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AddonsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AddonsConfig {}

impl BuildDataContainerCluster_AddonsConfig {
    pub fn build(self) -> DataContainerCluster_AddonsConfig {
        DataContainerCluster_AddonsConfig {
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
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AddonsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AddonsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_AddonsConfigRef {
        DataContainerCluster_AddonsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AddonsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_sandbox_config` after provisioning.\n"]
    pub fn agent_sandbox_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_AgentSandboxConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_sandbox_config", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudrun_config` after provisioning.\n"]
    pub fn cloudrun_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_CloudrunConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudrun_config", self.base))
    }

    #[doc = "Get a reference to the value of field `config_connector_config` after provisioning.\n"]
    pub fn config_connector_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_ConfigConnectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_connector_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_cache_config` after provisioning.\n"]
    pub fn dns_cache_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_DnsCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_persistent_disk_csi_driver_config` after provisioning.\n"]
    pub fn gce_persistent_disk_csi_driver_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_GcePersistentDiskCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_persistent_disk_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_filestore_csi_driver_config` after provisioning.\n"]
    pub fn gcp_filestore_csi_driver_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_GcpFilestoreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_filestore_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_fuse_csi_driver_config` after provisioning.\n"]
    pub fn gcs_fuse_csi_driver_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_GcsFuseCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_fuse_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_backup_agent_config` after provisioning.\n"]
    pub fn gke_backup_agent_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_GkeBackupAgentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_backup_agent_config", self.base))
    }

    #[doc = "Get a reference to the value of field `high_scale_checkpointing_config` after provisioning.\n"]
    pub fn high_scale_checkpointing_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_HighScaleCheckpointingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.high_scale_checkpointing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaling` after provisioning.\n"]
    pub fn horizontal_pod_autoscaling(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_HorizontalPodAutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `http_load_balancing` after provisioning.\n"]
    pub fn http_load_balancing(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_HttpLoadBalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_load_balancing", self.base))
    }

    #[doc = "Get a reference to the value of field `lustre_csi_driver_config` after provisioning.\n"]
    pub fn lustre_csi_driver_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_LustreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lustre_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `network_policy_config` after provisioning.\n"]
    pub fn network_policy_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_NetworkPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_policy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_readiness_config` after provisioning.\n"]
    pub fn node_readiness_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_NodeReadinessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_readiness_config", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelstore_csi_driver_config` after provisioning.\n"]
    pub fn parallelstore_csi_driver_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_ParallelstoreCsiDriverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelstore_csi_driver_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_snapshot_config` after provisioning.\n"]
    pub fn pod_snapshot_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_PodSnapshotConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_snapshot_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ray_operator_config` after provisioning.\n"]
    pub fn ray_operator_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_RayOperatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_operator_config", self.base))
    }

    #[doc = "Get a reference to the value of field `slice_controller_config` after provisioning.\n"]
    pub fn slice_controller_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_AddonsConfig_SliceControllerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slice_controller_config", self.base))
    }

    #[doc = "Get a reference to the value of field `slurm_operator_config` after provisioning.\n"]
    pub fn slurm_operator_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_SlurmOperatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slurm_operator_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_ha_config` after provisioning.\n"]
    pub fn stateful_ha_config(&self) -> terrars::ListRef<DataContainerCluster_AddonsConfig_StatefulHaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_ha_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AnonymousAuthenticationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_AnonymousAuthenticationConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AnonymousAuthenticationConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AnonymousAuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AnonymousAuthenticationConfig {}

impl BuildDataContainerCluster_AnonymousAuthenticationConfig {
    pub fn build(self) -> DataContainerCluster_AnonymousAuthenticationConfig {
        DataContainerCluster_AnonymousAuthenticationConfig { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AnonymousAuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AnonymousAuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AnonymousAuthenticationConfigRef {
        DataContainerCluster_AnonymousAuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AnonymousAuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AuthenticatorGroupsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_AuthenticatorGroupsConfig {
    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AuthenticatorGroupsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AuthenticatorGroupsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AuthenticatorGroupsConfig {}

impl BuildDataContainerCluster_AuthenticatorGroupsConfig {
    pub fn build(self) -> DataContainerCluster_AuthenticatorGroupsConfig {
        DataContainerCluster_AuthenticatorGroupsConfig { security_group: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AuthenticatorGroupsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AuthenticatorGroupsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AuthenticatorGroupsConfigRef {
        DataContainerCluster_AuthenticatorGroupsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AuthenticatorGroupsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_AutopilotClusterPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_standard_node_pools: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_system_impersonation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_system_mutation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_unsafe_webhooks: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_AutopilotClusterPolicyConfig {
    #[doc = "Set the field `no_standard_node_pools`.\n"]
    pub fn set_no_standard_node_pools(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_standard_node_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_system_impersonation`.\n"]
    pub fn set_no_system_impersonation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_system_impersonation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_system_mutation`.\n"]
    pub fn set_no_system_mutation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_system_mutation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_unsafe_webhooks`.\n"]
    pub fn set_no_unsafe_webhooks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_unsafe_webhooks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_AutopilotClusterPolicyConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_AutopilotClusterPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_AutopilotClusterPolicyConfig {}

impl BuildDataContainerCluster_AutopilotClusterPolicyConfig {
    pub fn build(self) -> DataContainerCluster_AutopilotClusterPolicyConfig {
        DataContainerCluster_AutopilotClusterPolicyConfig {
            no_standard_node_pools: core::default::Default::default(),
            no_system_impersonation: core::default::Default::default(),
            no_system_mutation: core::default::Default::default(),
            no_unsafe_webhooks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_AutopilotClusterPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_AutopilotClusterPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_AutopilotClusterPolicyConfigRef {
        DataContainerCluster_AutopilotClusterPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_AutopilotClusterPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `no_standard_node_pools` after provisioning.\n"]
    pub fn no_standard_node_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_standard_node_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `no_system_impersonation` after provisioning.\n"]
    pub fn no_system_impersonation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_system_impersonation", self.base))
    }

    #[doc = "Get a reference to the value of field `no_system_mutation` after provisioning.\n"]
    pub fn no_system_mutation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_system_mutation", self.base))
    }

    #[doc = "Get a reference to the value of field `no_unsafe_webhooks` after provisioning.\n"]
    pub fn no_unsafe_webhooks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_unsafe_webhooks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_BinaryAuthorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_BinaryAuthorization {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_mode`.\n"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_BinaryAuthorization {
    type O = terrars::BlockAssignable<DataContainerCluster_BinaryAuthorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_BinaryAuthorization {}

impl BuildDataContainerCluster_BinaryAuthorization {
    pub fn build(self) -> DataContainerCluster_BinaryAuthorization {
        DataContainerCluster_BinaryAuthorization {
            enabled: core::default::Default::default(),
            evaluation_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_BinaryAuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_BinaryAuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_BinaryAuthorizationRef {
        DataContainerCluster_BinaryAuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_BinaryAuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\n"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
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
::ToListMappable for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    type O =
        terrars
        ::BlockAssignable<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {}

impl BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
    pub fn build(self) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions {
            auto_upgrade_start_time: core::default::Default::default(),
            description: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef {
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
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_options: std
    ::option
    ::Option<
        terrars::ListField<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions>,
    >,
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    #[doc = "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_options`.\n"]
    pub fn set_upgrade_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptions,
                        >,
                    >,
    ) -> Self {
        self.upgrade_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    type O = terrars::BlockAssignable<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {}

impl BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
    pub fn build(self) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
            upgrade_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_options` after provisioning.\n"]
    pub fn upgrade_options(
        &self,
    ) -> terrars
    ::ListRef<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management_UpgradeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_ShieldedInstanceConfig {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ShieldedInstanceConfig {}

impl BuildDataContainerCluster_NodeConfig_ShieldedInstanceConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ShieldedInstanceConfig {
        DataContainerCluster_NodeConfig_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
        DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    #[doc = "Set the field `batch_node_count`.\n"]
    pub fn set_batch_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_percentage`.\n"]
    pub fn set_batch_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_soak_duration`.\n"]
    pub fn set_batch_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.batch_soak_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    type O =
        terrars
        ::BlockAssignable<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {}

impl BuildDataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
    pub fn build(self) -> DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
        DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
        DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_node_count` after provisioning.\n"]
    pub fn batch_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_percentage` after provisioning.\n"]
    pub fn batch_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_soak_duration` after provisioning.\n"]
    pub fn batch_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_soak_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_rollout_policy: std
    ::option
    ::Option<
        terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy>,
    >,
}

impl DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    #[doc = "Set the field `node_pool_soak_duration`.\n"]
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
                        ::ListField<
                            DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicy,
                        >,
                    >,
    ) -> Self {
        self.standard_rollout_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {}

impl BuildDataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
    pub fn build(self) -> DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
        DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
        DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\n"]
    pub fn node_pool_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_pool_soak_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]
    pub fn standard_rollout_policy(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings_StandardRolloutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_rollout_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_UpgradeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_settings: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_UpgradeSettings {
    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettings>>,
    ) -> Self {
        self.blue_green_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_surge`.\n"]
    pub fn set_max_surge(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_UpgradeSettings {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_UpgradeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_UpgradeSettings {}

impl BuildDataContainerCluster_NodePool_UpgradeSettings {
    pub fn build(self) -> DataContainerCluster_NodePool_UpgradeSettings {
        DataContainerCluster_NodePool_UpgradeSettings {
            blue_green_settings: core::default::Default::default(),
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_UpgradeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_UpgradeSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_UpgradeSettingsRef {
        DataContainerCluster_NodePool_UpgradeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_UpgradeSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodePool_UpgradeSettings_BlueGreenSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `max_surge` after provisioning.\n"]
    pub fn max_surge(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings>>,
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    #[doc = "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size`.\n"]
    pub fn set_disk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_type = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_Management>,
                    >,
    ) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    ) -> Self {
        self.shielded_instance_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings>>,
    ) -> Self {
        self.upgrade_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    type O = terrars::BlockAssignable<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {}

impl BuildDataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
    pub fn build(self) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults {
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

#[allow(dead_code)]
pub struct DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
        DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<DataContainerCluster_NodePool_UpgradeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ClusterAutoscaling_ResourceLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ClusterAutoscaling_ResourceLimits {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ClusterAutoscaling_ResourceLimits {
    type O = terrars::BlockAssignable<DataContainerCluster_ClusterAutoscaling_ResourceLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ClusterAutoscaling_ResourceLimits {}

impl BuildDataContainerCluster_ClusterAutoscaling_ResourceLimits {
    pub fn build(self) -> DataContainerCluster_ClusterAutoscaling_ResourceLimits {
        DataContainerCluster_ClusterAutoscaling_ResourceLimits {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
        DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ClusterAutoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_provisioning_defaults: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_provisioning_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_compute_class_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_limits: std::option::Option<terrars::ListField<DataContainerCluster_ClusterAutoscaling_ResourceLimits>>,
}

impl DataContainerCluster_ClusterAutoscaling {
    #[doc = "Set the field `auto_provisioning_defaults`.\n"]
    pub fn set_auto_provisioning_defaults(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaults>>,
    ) -> Self {
        self.auto_provisioning_defaults = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_provisioning_locations`.\n"]
    pub fn set_auto_provisioning_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.auto_provisioning_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_profile`.\n"]
    pub fn set_autoscaling_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autoscaling_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_compute_class_enabled`.\n"]
    pub fn set_default_compute_class_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_compute_class_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_limits`.\n"]
    pub fn set_resource_limits(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_ClusterAutoscaling_ResourceLimits>>,
    ) -> Self {
        self.resource_limits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ClusterAutoscaling {
    type O = terrars::BlockAssignable<DataContainerCluster_ClusterAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ClusterAutoscaling {}

impl BuildDataContainerCluster_ClusterAutoscaling {
    pub fn build(self) -> DataContainerCluster_ClusterAutoscaling {
        DataContainerCluster_ClusterAutoscaling {
            auto_provisioning_defaults: core::default::Default::default(),
            auto_provisioning_locations: core::default::Default::default(),
            autoscaling_profile: core::default::Default::default(),
            default_compute_class_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ClusterAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ClusterAutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_ClusterAutoscalingRef {
        DataContainerCluster_ClusterAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ClusterAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_provisioning_defaults` after provisioning.\n"]
    pub fn auto_provisioning_defaults(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ClusterAutoscaling_AutoProvisioningDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_provisioning_locations` after provisioning.\n"]
    pub fn auto_provisioning_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_provisioning_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_profile` after provisioning.\n"]
    pub fn autoscaling_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `default_compute_class_enabled` after provisioning.\n"]
    pub fn default_compute_class_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_compute_class_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(&self) -> terrars::ListRef<DataContainerCluster_ClusterAutoscaling_ResourceLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ConfidentialNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_ConfidentialNodes {
    #[doc = "Set the field `confidential_instance_type`.\n"]
    pub fn set_confidential_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.confidential_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ConfidentialNodes {
    type O = terrars::BlockAssignable<DataContainerCluster_ConfidentialNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ConfidentialNodes {}

impl BuildDataContainerCluster_ConfidentialNodes {
    pub fn build(self) -> DataContainerCluster_ConfidentialNodes {
        DataContainerCluster_ConfidentialNodes {
            confidential_instance_type: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ConfidentialNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ConfidentialNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_ConfidentialNodesRef {
        DataContainerCluster_ConfidentialNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ConfidentialNodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n"]
    pub fn confidential_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidential_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_external_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_certs_via_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_k8s_tokens_via_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    #[doc = "Set the field `allow_external_traffic`.\n"]
    pub fn set_allow_external_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_external_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_k8s_certs_via_dns`.\n"]
    pub fn set_enable_k8s_certs_via_dns(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_k8s_certs_via_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_k8s_tokens_via_dns`.\n"]
    pub fn set_enable_k8s_tokens_via_dns(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_k8s_tokens_via_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {}

impl BuildDataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
    pub fn build(self) -> DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
        DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig {
            allow_external_traffic: core::default::Default::default(),
            enable_k8s_certs_via_dns: core::default::Default::default(),
            enable_k8s_tokens_via_dns: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
        DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_external_traffic` after provisioning.\n"]
    pub fn allow_external_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_external_traffic", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_certs_via_dns` after provisioning.\n"]
    pub fn enable_k8s_certs_via_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_k8s_certs_via_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_k8s_tokens_via_dns` after provisioning.\n"]
    pub fn enable_k8s_tokens_via_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_k8s_tokens_via_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {}

impl BuildDataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
    pub fn build(self) -> DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
        DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
        DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef {
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
pub struct DataContainerCluster_ControlPlaneEndpointsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_endpoint_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_endpoints_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>>,
}

impl DataContainerCluster_ControlPlaneEndpointsConfig {
    #[doc = "Set the field `dns_endpoint_config`.\n"]
    pub fn set_dns_endpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfig>>,
    ) -> Self {
        self.dns_endpoint_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_endpoints_config`.\n"]
    pub fn set_ip_endpoints_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfig>>,
    ) -> Self {
        self.ip_endpoints_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ControlPlaneEndpointsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ControlPlaneEndpointsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ControlPlaneEndpointsConfig {}

impl BuildDataContainerCluster_ControlPlaneEndpointsConfig {
    pub fn build(self) -> DataContainerCluster_ControlPlaneEndpointsConfig {
        DataContainerCluster_ControlPlaneEndpointsConfig {
            dns_endpoint_config: core::default::Default::default(),
            ip_endpoints_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ControlPlaneEndpointsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ControlPlaneEndpointsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ControlPlaneEndpointsConfigRef {
        DataContainerCluster_ControlPlaneEndpointsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ControlPlaneEndpointsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_endpoint_config` after provisioning.\n"]
    pub fn dns_endpoint_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ControlPlaneEndpointsConfig_DnsEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_endpoints_config` after provisioning.\n"]
    pub fn ip_endpoints_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ControlPlaneEndpointsConfig_IpEndpointsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_endpoints_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_CostManagementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_CostManagementConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_CostManagementConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_CostManagementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_CostManagementConfig {}

impl BuildDataContainerCluster_CostManagementConfig {
    pub fn build(self) -> DataContainerCluster_CostManagementConfig {
        DataContainerCluster_CostManagementConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_CostManagementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_CostManagementConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_CostManagementConfigRef {
        DataContainerCluster_CostManagementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_CostManagementConfigRef {
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
pub struct DataContainerCluster_DatabaseEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_DatabaseEncryption {
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_DatabaseEncryption {
    type O = terrars::BlockAssignable<DataContainerCluster_DatabaseEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_DatabaseEncryption {}

impl BuildDataContainerCluster_DatabaseEncryption {
    pub fn build(self) -> DataContainerCluster_DatabaseEncryption {
        DataContainerCluster_DatabaseEncryption {
            key_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_DatabaseEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_DatabaseEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_DatabaseEncryptionRef {
        DataContainerCluster_DatabaseEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_DatabaseEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_DefaultSnatStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_DefaultSnatStatus {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_DefaultSnatStatus {
    type O = terrars::BlockAssignable<DataContainerCluster_DefaultSnatStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_DefaultSnatStatus {}

impl BuildDataContainerCluster_DefaultSnatStatus {
    pub fn build(self) -> DataContainerCluster_DefaultSnatStatus {
        DataContainerCluster_DefaultSnatStatus { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_DefaultSnatStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_DefaultSnatStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_DefaultSnatStatusRef {
        DataContainerCluster_DefaultSnatStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_DefaultSnatStatusRef {
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
pub struct DataContainerCluster_DnsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additive_vpc_scope_dns_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_dns_scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_DnsConfig {
    #[doc = "Set the field `additive_vpc_scope_dns_domain`.\n"]
    pub fn set_additive_vpc_scope_dns_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additive_vpc_scope_dns_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_dns`.\n"]
    pub fn set_cluster_dns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_dns_domain`.\n"]
    pub fn set_cluster_dns_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_dns_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_dns_scope`.\n"]
    pub fn set_cluster_dns_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_dns_scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_DnsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_DnsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_DnsConfig {}

impl BuildDataContainerCluster_DnsConfig {
    pub fn build(self) -> DataContainerCluster_DnsConfig {
        DataContainerCluster_DnsConfig {
            additive_vpc_scope_dns_domain: core::default::Default::default(),
            cluster_dns: core::default::Default::default(),
            cluster_dns_domain: core::default::Default::default(),
            cluster_dns_scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_DnsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_DnsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_DnsConfigRef {
        DataContainerCluster_DnsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_DnsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additive_vpc_scope_dns_domain` after provisioning.\n"]
    pub fn additive_vpc_scope_dns_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additive_vpc_scope_dns_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_dns` after provisioning.\n"]
    pub fn cluster_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_dns_domain` after provisioning.\n"]
    pub fn cluster_dns_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_dns_scope` after provisioning.\n"]
    pub fn cluster_dns_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_EnableK8sBetaApis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_apis: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_EnableK8sBetaApis {
    #[doc = "Set the field `enabled_apis`.\n"]
    pub fn set_enabled_apis(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_apis = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_EnableK8sBetaApis {
    type O = terrars::BlockAssignable<DataContainerCluster_EnableK8sBetaApis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_EnableK8sBetaApis {}

impl BuildDataContainerCluster_EnableK8sBetaApis {
    pub fn build(self) -> DataContainerCluster_EnableK8sBetaApis {
        DataContainerCluster_EnableK8sBetaApis { enabled_apis: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_EnableK8sBetaApisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_EnableK8sBetaApisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_EnableK8sBetaApisRef {
        DataContainerCluster_EnableK8sBetaApisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_EnableK8sBetaApisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_apis` after provisioning.\n"]
    pub fn enabled_apis(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_apis", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_EnterpriseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_EnterpriseConfig {
    #[doc = "Set the field `cluster_tier`.\n"]
    pub fn set_cluster_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_tier`.\n"]
    pub fn set_desired_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desired_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_EnterpriseConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_EnterpriseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_EnterpriseConfig {}

impl BuildDataContainerCluster_EnterpriseConfig {
    pub fn build(self) -> DataContainerCluster_EnterpriseConfig {
        DataContainerCluster_EnterpriseConfig {
            cluster_tier: core::default::Default::default(),
            desired_tier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_EnterpriseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_EnterpriseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_EnterpriseConfigRef {
        DataContainerCluster_EnterpriseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_EnterpriseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_tier` after provisioning.\n"]
    pub fn cluster_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_tier` after provisioning.\n"]
    pub fn desired_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_registered: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_Fleet {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership_id`.\n"]
    pub fn set_membership_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership_location`.\n"]
    pub fn set_membership_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.membership_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership_type`.\n"]
    pub fn set_membership_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_registered`.\n"]
    pub fn set_pre_registered(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pre_registered = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_Fleet {
    type O = terrars::BlockAssignable<DataContainerCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_Fleet {}

impl BuildDataContainerCluster_Fleet {
    pub fn build(self) -> DataContainerCluster_Fleet {
        DataContainerCluster_Fleet {
            membership: core::default::Default::default(),
            membership_id: core::default::Default::default(),
            membership_location: core::default::Default::default(),
            membership_type: core::default::Default::default(),
            pre_registered: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_FleetRef {
        DataContainerCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\n"]
    pub fn membership_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_id", self.base))
    }

    #[doc = "Get a reference to the value of field `membership_location` after provisioning.\n"]
    pub fn membership_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.base))
    }

    #[doc = "Get a reference to the value of field `membership_type` after provisioning.\n"]
    pub fn membership_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_type", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_registered` after provisioning.\n"]
    pub fn pre_registered(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_registered", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_GatewayApiConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_GatewayApiConfig {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_GatewayApiConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_GatewayApiConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_GatewayApiConfig {}

impl BuildDataContainerCluster_GatewayApiConfig {
    pub fn build(self) -> DataContainerCluster_GatewayApiConfig {
        DataContainerCluster_GatewayApiConfig { channel: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_GatewayApiConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_GatewayApiConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_GatewayApiConfigRef {
        DataContainerCluster_GatewayApiConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_GatewayApiConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_GkeAutoUpgradeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_GkeAutoUpgradeConfig {
    #[doc = "Set the field `patch_mode`.\n"]
    pub fn set_patch_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.patch_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_GkeAutoUpgradeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_GkeAutoUpgradeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_GkeAutoUpgradeConfig {}

impl BuildDataContainerCluster_GkeAutoUpgradeConfig {
    pub fn build(self) -> DataContainerCluster_GkeAutoUpgradeConfig {
        DataContainerCluster_GkeAutoUpgradeConfig { patch_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_GkeAutoUpgradeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_GkeAutoUpgradeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_GkeAutoUpgradeConfigRef {
        DataContainerCluster_GkeAutoUpgradeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_GkeAutoUpgradeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `patch_mode` after provisioning.\n"]
    pub fn patch_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_IdentityServiceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_IdentityServiceConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IdentityServiceConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IdentityServiceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IdentityServiceConfig {}

impl BuildDataContainerCluster_IdentityServiceConfig {
    pub fn build(self) -> DataContainerCluster_IdentityServiceConfig {
        DataContainerCluster_IdentityServiceConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IdentityServiceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IdentityServiceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_IdentityServiceConfigRef {
        DataContainerCluster_IdentityServiceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IdentityServiceConfigRef {
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
pub struct DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_ipv4_range_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    #[doc = "Set the field `pod_ipv4_range_names`.\n"]
    pub fn set_pod_ipv4_range_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pod_ipv4_range_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {}

impl BuildDataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
        DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig {
            pod_ipv4_range_names: core::default::Default::default(),
            status: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
        DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pod_ipv4_range_names` after provisioning.\n"]
    pub fn pod_ipv4_range_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_ipv4_range_names", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_range_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    #[doc = "Set the field `pod_range_names`.\n"]
    pub fn set_pod_range_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pod_range_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {}

impl BuildDataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
        DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig {
            pod_range_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
        DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pod_range_names` after provisioning.\n"]
    pub fn pod_range_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.pod_range_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy_AutoIpamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy_AutoIpamConfig {}

impl BuildDataContainerCluster_IpAllocationPolicy_AutoIpamConfig {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy_AutoIpamConfig {
        DataContainerCluster_IpAllocationPolicy_AutoIpamConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
        DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef {
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
pub struct DataContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy_NetworkTierConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy_NetworkTierConfig {}

impl BuildDataContainerCluster_IpAllocationPolicy_NetworkTierConfig {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy_NetworkTierConfig {
        DataContainerCluster_IpAllocationPolicy_NetworkTierConfig { network_tier: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
        DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {}

impl BuildDataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
        DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig {
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
        DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef {
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
pub struct DataContainerCluster_IpAllocationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_ip_ranges_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_ranges_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_ipam_config: std::option::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AutoIpamConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_NetworkTierConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_secondary_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_IpAllocationPolicy {
    #[doc = "Set the field `additional_ip_ranges_config`.\n"]
    pub fn set_additional_ip_ranges_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfig>>,
    ) -> Self {
        self.additional_ip_ranges_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_pod_ranges_config`.\n"]
    pub fn set_additional_pod_ranges_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfig>>,
    ) -> Self {
        self.additional_pod_ranges_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_ipam_config`.\n"]
    pub fn set_auto_ipam_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_AutoIpamConfig>>,
    ) -> Self {
        self.auto_ipam_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tier_config`.\n"]
    pub fn set_network_tier_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_NetworkTierConfig>>,
    ) -> Self {
        self.network_tier_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    ) -> Self {
        self.pod_cidr_overprovision_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.services_secondary_range_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_IpAllocationPolicy {
    type O = terrars::BlockAssignable<DataContainerCluster_IpAllocationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_IpAllocationPolicy {}

impl BuildDataContainerCluster_IpAllocationPolicy {
    pub fn build(self) -> DataContainerCluster_IpAllocationPolicy {
        DataContainerCluster_IpAllocationPolicy {
            additional_ip_ranges_config: core::default::Default::default(),
            additional_pod_ranges_config: core::default::Default::default(),
            auto_ipam_config: core::default::Default::default(),
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            network_tier_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            stack_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_IpAllocationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_IpAllocationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_IpAllocationPolicyRef {
        DataContainerCluster_IpAllocationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_IpAllocationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_ip_ranges_config` after provisioning.\n"]
    pub fn additional_ip_ranges_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_AdditionalIpRangesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_ip_ranges_config", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_ranges_config` after provisioning.\n"]
    pub fn additional_pod_ranges_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_AdditionalPodRangesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_ranges_config", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_ipam_config` after provisioning.\n"]
    pub fn auto_ipam_config(&self) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_AutoIpamConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_ipam_config", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `network_tier_config` after provisioning.\n"]
    pub fn network_tier_config(&self) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_NetworkTierConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tier_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }

    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.services_secondary_range_name", self.base))
    }

    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_components: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_LoggingConfig {
    #[doc = "Set the field `enable_components`.\n"]
    pub fn set_enable_components(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enable_components = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_LoggingConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_LoggingConfig {}

impl BuildDataContainerCluster_LoggingConfig {
    pub fn build(self) -> DataContainerCluster_LoggingConfig {
        DataContainerCluster_LoggingConfig { enable_components: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_LoggingConfigRef {
        DataContainerCluster_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {}

impl BuildDataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
        DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow {
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
        DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef {
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
pub struct DataContainerCluster_MaintenancePolicy_DisruptionBudget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_disruption_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_minor_version_disruption_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor_version_disruption_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_version_disruption_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MaintenancePolicy_DisruptionBudget {
    #[doc = "Set the field `last_disruption_time`.\n"]
    pub fn set_last_disruption_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_disruption_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_minor_version_disruption_time`.\n"]
    pub fn set_last_minor_version_disruption_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_minor_version_disruption_time = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_DisruptionBudget {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_DisruptionBudget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_DisruptionBudget {}

impl BuildDataContainerCluster_MaintenancePolicy_DisruptionBudget {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_DisruptionBudget {
        DataContainerCluster_MaintenancePolicy_DisruptionBudget {
            last_disruption_time: core::default::Default::default(),
            last_minor_version_disruption_time: core::default::Default::default(),
            minor_version_disruption_interval: core::default::Default::default(),
            patch_version_disruption_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
        DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef {
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
pub struct DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    #[doc = "Set the field `end_time_behavior`.\n"]
    pub fn set_end_time_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {}

impl BuildDataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
        DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions {
            end_time_behavior: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
        DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time_behavior` after provisioning.\n"]
    pub fn end_time_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_options: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusion_name`.\n"]
    pub fn set_exclusion_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exclusion_name = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptions>,
                    >,
    ) -> Self {
        self.exclusion_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_MaintenanceExclusion {}

impl BuildDataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
        DataContainerCluster_MaintenancePolicy_MaintenanceExclusion {
            end_time: core::default::Default::default(),
            exclusion_name: core::default::Default::default(),
            exclusion_options: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
        DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef {
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

    #[doc = "Get a reference to the value of field `exclusion_options` after provisioning.\n"]
    pub fn exclusion_options(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion_ExclusionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_options", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `month`.\n"]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\n"]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {}

impl BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef {
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
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
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

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    type O =
        terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {}

impl BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef {
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

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay_until: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_start_time: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>>,
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    #[doc = "Set the field `delay_until`.\n"]
    pub fn set_delay_until(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntil>,
                    >,
    ) -> Self {
        self.delay_until = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recurrence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window_duration`.\n"]
    pub fn set_window_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window_duration = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTime>,
                    >,
    ) -> Self {
        self.window_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {}

impl BuildDataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow {
            delay_until: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            window_duration: core::default::Default::default(),
            window_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
        DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delay_until` after provisioning.\n"]
    pub fn delay_until(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_DelayUntilRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay_until", self.base))
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `window_duration` after provisioning.\n"]
    pub fn window_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow_WindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MaintenancePolicy_RecurringWindow {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recurrence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy_RecurringWindow {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy_RecurringWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy_RecurringWindow {}

impl BuildDataContainerCluster_MaintenancePolicy_RecurringWindow {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy_RecurringWindow {
        DataContainerCluster_MaintenancePolicy_RecurringWindow {
            end_time: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicy_RecurringWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicy_RecurringWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MaintenancePolicy_RecurringWindowRef {
        DataContainerCluster_MaintenancePolicy_RecurringWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicy_RecurringWindowRef {
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

#[derive(Serialize)]
pub struct DataContainerCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disruption_budget: std::option::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_DisruptionBudget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusion: std
    ::option
    ::Option<terrars::SetField<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_window: std::option::Option<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringWindow>>,
}

impl DataContainerCluster_MaintenancePolicy {
    #[doc = "Set the field `daily_maintenance_window`.\n"]
    pub fn set_daily_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindow>>,
    ) -> Self {
        self.daily_maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_MaintenancePolicy_DisruptionBudget>>,
    ) -> Self {
        self.disruption_budget = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_exclusion`.\n"]
    pub fn set_maintenance_exclusion(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataContainerCluster_MaintenancePolicy_MaintenanceExclusion>>,
    ) -> Self {
        self.maintenance_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurring_maintenance_window`.\n"]
    pub fn set_recurring_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindow>>,
    ) -> Self {
        self.recurring_maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_MaintenancePolicy_RecurringWindow>>,
    ) -> Self {
        self.recurring_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataContainerCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MaintenancePolicy {}

impl BuildDataContainerCluster_MaintenancePolicy {
    pub fn build(self) -> DataContainerCluster_MaintenancePolicy {
        DataContainerCluster_MaintenancePolicy {
            daily_maintenance_window: core::default::Default::default(),
            disruption_budget: core::default::Default::default(),
            maintenance_exclusion: core::default::Default::default(),
            recurring_maintenance_window: core::default::Default::default(),
            recurring_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_MaintenancePolicyRef {
        DataContainerCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_maintenance_window` after provisioning.\n"]
    pub fn daily_maintenance_window(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_DailyMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(&self) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_DisruptionBudgetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disruption_budget", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_exclusion` after provisioning.\n"]
    pub fn maintenance_exclusion(
        &self,
    ) -> terrars::SetRef<DataContainerCluster_MaintenancePolicy_MaintenanceExclusionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.maintenance_exclusion", self.base))
    }

    #[doc = "Get a reference to the value of field `recurring_maintenance_window` after provisioning.\n"]
    pub fn recurring_maintenance_window(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_RecurringMaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(&self) -> terrars::ListRef<DataContainerCluster_MaintenancePolicy_RecurringWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ManagedOpentelemetryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ManagedOpentelemetryConfig {
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ManagedOpentelemetryConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ManagedOpentelemetryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ManagedOpentelemetryConfig {}

impl BuildDataContainerCluster_ManagedOpentelemetryConfig {
    pub fn build(self) -> DataContainerCluster_ManagedOpentelemetryConfig {
        DataContainerCluster_ManagedOpentelemetryConfig { scope: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ManagedOpentelemetryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ManagedOpentelemetryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ManagedOpentelemetryConfigRef {
        DataContainerCluster_ManagedOpentelemetryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ManagedOpentelemetryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MasterAuth_ClientCertificateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issue_client_certificate: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_MasterAuth_ClientCertificateConfig {
    #[doc = "Set the field `issue_client_certificate`.\n"]
    pub fn set_issue_client_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.issue_client_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MasterAuth_ClientCertificateConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_MasterAuth_ClientCertificateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MasterAuth_ClientCertificateConfig {}

impl BuildDataContainerCluster_MasterAuth_ClientCertificateConfig {
    pub fn build(self) -> DataContainerCluster_MasterAuth_ClientCertificateConfig {
        DataContainerCluster_MasterAuth_ClientCertificateConfig {
            issue_client_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MasterAuth_ClientCertificateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MasterAuth_ClientCertificateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MasterAuth_ClientCertificateConfigRef {
        DataContainerCluster_MasterAuth_ClientCertificateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MasterAuth_ClientCertificateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `issue_client_certificate` after provisioning.\n"]
    pub fn issue_client_certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issue_client_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MasterAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MasterAuth_ClientCertificateConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MasterAuth {
    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_config`.\n"]
    pub fn set_client_certificate_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_MasterAuth_ClientCertificateConfig>>,
    ) -> Self {
        self.client_certificate_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_ca_certificate`.\n"]
    pub fn set_cluster_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ca_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MasterAuth {
    type O = terrars::BlockAssignable<DataContainerCluster_MasterAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MasterAuth {}

impl BuildDataContainerCluster_MasterAuth {
    pub fn build(self) -> DataContainerCluster_MasterAuth {
        DataContainerCluster_MasterAuth {
            client_certificate: core::default::Default::default(),
            client_certificate_config: core::default::Default::default(),
            client_key: core::default::Default::default(),
            cluster_ca_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MasterAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MasterAuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_MasterAuthRef {
        DataContainerCluster_MasterAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MasterAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate_config` after provisioning.\n"]
    pub fn client_certificate_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MasterAuth_ClientCertificateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_config", self.base))
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\n"]
    pub fn cluster_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    type O = terrars::BlockAssignable<DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {}

impl BuildDataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
    pub fn build(self) -> DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
        DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks {
            cidr_block: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
        DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MasterAuthorizedNetworksConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<terrars::SetField<DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_public_cidrs_access_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_enforcement_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_MasterAuthorizedNetworksConfig {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocks>>,
    ) -> Self {
        self.cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_public_cidrs_access_enabled`.\n"]
    pub fn set_gcp_public_cidrs_access_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.gcp_public_cidrs_access_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_enforcement_enabled`.\n"]
    pub fn set_private_endpoint_enforcement_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.private_endpoint_enforcement_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MasterAuthorizedNetworksConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_MasterAuthorizedNetworksConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MasterAuthorizedNetworksConfig {}

impl BuildDataContainerCluster_MasterAuthorizedNetworksConfig {
    pub fn build(self) -> DataContainerCluster_MasterAuthorizedNetworksConfig {
        DataContainerCluster_MasterAuthorizedNetworksConfig {
            cidr_blocks: core::default::Default::default(),
            gcp_public_cidrs_access_enabled: core::default::Default::default(),
            private_endpoint_enforcement_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MasterAuthorizedNetworksConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MasterAuthorizedNetworksConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MasterAuthorizedNetworksConfigRef {
        DataContainerCluster_MasterAuthorizedNetworksConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MasterAuthorizedNetworksConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> terrars::SetRef<DataContainerCluster_MasterAuthorizedNetworksConfig_CidrBlocksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_public_cidrs_access_enabled` after provisioning.\n"]
    pub fn gcp_public_cidrs_access_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_public_cidrs_access_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_enforcement_enabled` after provisioning.\n"]
    pub fn private_endpoint_enforcement_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_enforcement_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MeshCertificates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_certificates: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_MeshCertificates {
    #[doc = "Set the field `enable_certificates`.\n"]
    pub fn set_enable_certificates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MeshCertificates {
    type O = terrars::BlockAssignable<DataContainerCluster_MeshCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MeshCertificates {}

impl BuildDataContainerCluster_MeshCertificates {
    pub fn build(self) -> DataContainerCluster_MeshCertificates {
        DataContainerCluster_MeshCertificates { enable_certificates: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MeshCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MeshCertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_MeshCertificatesRef {
        DataContainerCluster_MeshCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MeshCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_certificates` after provisioning.\n"]
    pub fn enable_certificates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_metrics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_relay: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    #[doc = "Set the field `enable_metrics`.\n"]
    pub fn set_enable_metrics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_relay`.\n"]
    pub fn set_enable_relay(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_relay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {}

impl BuildDataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
    pub fn build(self) -> DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
        DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig {
            enable_metrics: core::default::Default::default(),
            enable_relay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
        DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_metrics` after provisioning.\n"]
    pub fn enable_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_relay` after provisioning.\n"]
    pub fn enable_relay(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_relay", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {}

impl BuildDataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
    pub fn build(self) -> DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
        DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig {
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
        DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MonitoringConfig_ManagedPrometheus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_monitoring_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_MonitoringConfig_ManagedPrometheus {
    #[doc = "Set the field `auto_monitoring_config`.\n"]
    pub fn set_auto_monitoring_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfig>,
                    >,
    ) -> Self {
        self.auto_monitoring_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MonitoringConfig_ManagedPrometheus {
    type O = terrars::BlockAssignable<DataContainerCluster_MonitoringConfig_ManagedPrometheus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MonitoringConfig_ManagedPrometheus {}

impl BuildDataContainerCluster_MonitoringConfig_ManagedPrometheus {
    pub fn build(self) -> DataContainerCluster_MonitoringConfig_ManagedPrometheus {
        DataContainerCluster_MonitoringConfig_ManagedPrometheus {
            auto_monitoring_config: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_MonitoringConfig_ManagedPrometheusRef {
        DataContainerCluster_MonitoringConfig_ManagedPrometheusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MonitoringConfig_ManagedPrometheusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_monitoring_config` after provisioning.\n"]
    pub fn auto_monitoring_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MonitoringConfig_ManagedPrometheus_AutoMonitoringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_monitoring_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_MonitoringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_datapath_observability_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_components: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_prometheus: std::option::Option<terrars::ListField<DataContainerCluster_MonitoringConfig_ManagedPrometheus>>,
}

impl DataContainerCluster_MonitoringConfig {
    #[doc = "Set the field `advanced_datapath_observability_config`.\n"]
    pub fn set_advanced_datapath_observability_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfig>,
                    >,
    ) -> Self {
        self.advanced_datapath_observability_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_components`.\n"]
    pub fn set_enable_components(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enable_components = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_prometheus`.\n"]
    pub fn set_managed_prometheus(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_MonitoringConfig_ManagedPrometheus>>,
    ) -> Self {
        self.managed_prometheus = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_MonitoringConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_MonitoringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_MonitoringConfig {}

impl BuildDataContainerCluster_MonitoringConfig {
    pub fn build(self) -> DataContainerCluster_MonitoringConfig {
        DataContainerCluster_MonitoringConfig {
            advanced_datapath_observability_config: core::default::Default::default(),
            enable_components: core::default::Default::default(),
            managed_prometheus: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_MonitoringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_MonitoringConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_MonitoringConfigRef {
        DataContainerCluster_MonitoringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_MonitoringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_datapath_observability_config` after provisioning.\n"]
    pub fn advanced_datapath_observability_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_MonitoringConfig_AdvancedDatapathObservabilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_datapath_observability_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enable_components", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_prometheus` after provisioning.\n"]
    pub fn managed_prometheus(&self) -> terrars::ListRef<DataContainerCluster_MonitoringConfig_ManagedPrometheusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NetworkPerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bandwidth_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NetworkPerformanceConfig {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bandwidth_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NetworkPerformanceConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NetworkPerformanceConfig {}

impl BuildDataContainerCluster_NetworkPerformanceConfig {
    pub fn build(self) -> DataContainerCluster_NetworkPerformanceConfig {
        DataContainerCluster_NetworkPerformanceConfig {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NetworkPerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NetworkPerformanceConfigRef {
        DataContainerCluster_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NetworkPerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NetworkPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NetworkPolicy {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NetworkPolicy {
    type O = terrars::BlockAssignable<DataContainerCluster_NetworkPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NetworkPolicy {}

impl BuildDataContainerCluster_NetworkPolicy {
    pub fn build(self) -> DataContainerCluster_NetworkPolicy {
        DataContainerCluster_NetworkPolicy {
            enabled: core::default::Default::default(),
            provider: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NetworkPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NetworkPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NetworkPolicyRef {
        DataContainerCluster_NetworkPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NetworkPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_monitoring_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_AdvancedMachineFeatures {
    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_monitoring_unit`.\n"]
    pub fn set_performance_monitoring_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.performance_monitoring_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_AdvancedMachineFeatures {}

impl BuildDataContainerCluster_NodeConfig_AdvancedMachineFeatures {
    pub fn build(self) -> DataContainerCluster_NodeConfig_AdvancedMachineFeatures {
        DataContainerCluster_NodeConfig_AdvancedMachineFeatures {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
        DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\n"]
    pub fn performance_monitoring_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_monitoring_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_BootDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_BootDisk {
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_iops`.\n"]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_BootDisk {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_BootDisk {}

impl BuildDataContainerCluster_NodeConfig_BootDisk {
    pub fn build(self) -> DataContainerCluster_NodeConfig_BootDisk {
        DataContainerCluster_NodeConfig_BootDisk {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_BootDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfig_BootDiskRef {
        DataContainerCluster_NodeConfig_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_BootDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\n"]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    #[doc = "Set the field `secret_uri`.\n"]
    pub fn set_secret_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
    pub fn build(
        self,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig {
            secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_uri` after provisioning.\n"]
    pub fn secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_certificate_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
        >,
    >,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    #[doc = "Set the field `fqdns`.\n"]
    pub fn set_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_secret_manager_certificate_config`.\n"]
    pub fn set_gcp_secret_manager_certificate_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfig,
                        >,
                    >,
    ) -> Self {
        self.gcp_secret_manager_certificate_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
    pub fn build(
        self,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig {
            fqdns: core::default::Default::default(),
            gcp_secret_manager_certificate_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fqdns` after provisioning.\n"]
    pub fn fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fqdns", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"]
    pub fn gcp_secret_manager_certificate_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig_GcpSecretManagerCertificateConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_secret_manager_certificate_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_domain_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    #[doc = "Set the field `certificate_authority_domain_config`.\n"]
    pub fn set_certificate_authority_domain_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfig,
                        >,
                    >,
    ) -> Self {
        self.certificate_authority_domain_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig {
            certificate_authority_domain_config: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"]
    pub fn certificate_authority_domain_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig_CertificateAuthorityDomainConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_domain_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    type O =
        terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    type O =
        terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Cert>,
                    >,
    ) -> Self {
        self.cert = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_Key>,
                    >,
    ) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_CertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client_KeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dial_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_path: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    #[doc = "Set the field `ca`.\n"]
    pub fn set_ca(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Ca>,
                    >,
    ) -> Self {
        self.ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capabilities = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Client>,
                    >,
    ) -> Self {
        self.client = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dial_timeout`.\n"]
    pub fn set_dial_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dial_timeout = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_Header>,
                    >,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_path`.\n"]
    pub fn set_override_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts {
            ca: core::default::Default::default(),
            capabilities: core::default::Default::default(),
            client: core::default::Default::default(),
            dial_timeout: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            override_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca` after provisioning.\n"]
    pub fn ca(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_CaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_ClientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }

    #[doc = "Get a reference to the value of field `dial_timeout` after provisioning.\n"]
    pub fn dial_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dial_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `override_path` after provisioning.\n"]
    pub fn override_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_Hosts>>,
    ) -> Self {
        self.hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server`.\n"]
    pub fn set_server(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts {
            hosts: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts_HostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `server` after provisioning.\n"]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
        DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
        DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef {
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
pub struct DataContainerCluster_NodeConfig_ContainerdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_registry_access_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_hosts: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    writable_cgroups: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>>,
}

impl DataContainerCluster_NodeConfig_ContainerdConfig {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfig>,
                    >,
    ) -> Self {
        self.private_registry_access_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHosts>>,
    ) -> Self {
        self.registry_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroups>>,
    ) -> Self {
        self.writable_cgroups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ContainerdConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ContainerdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ContainerdConfig {}

impl BuildDataContainerCluster_NodeConfig_ContainerdConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ContainerdConfig {
        DataContainerCluster_NodeConfig_ContainerdConfig {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ContainerdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ContainerdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ContainerdConfigRef {
        DataContainerCluster_NodeConfig_ContainerdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ContainerdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]
    pub fn private_registry_access_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_PrivateRegistryAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_RegistryHostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registry_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfig_WritableCgroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.writable_cgroups", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_EffectiveTaints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_EffectiveTaints {
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

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_EffectiveTaints {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_EffectiveTaints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_EffectiveTaints {}

impl BuildDataContainerCluster_NodeConfig_EffectiveTaints {
    pub fn build(self) -> DataContainerCluster_NodeConfig_EffectiveTaints {
        DataContainerCluster_NodeConfig_EffectiveTaints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_EffectiveTaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_EffectiveTaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_EffectiveTaintsRef {
        DataContainerCluster_NodeConfig_EffectiveTaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_EffectiveTaintsRef {
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
pub struct DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    #[doc = "Set the field `data_cache_count`.\n"]
    pub fn set_data_cache_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_cache_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.local_ssd_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {}

impl BuildDataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
        DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
        DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_cache_count` after provisioning.\n"]
    pub fn data_cache_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_cache_count", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_FastSocket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_FastSocket {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_FastSocket {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_FastSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_FastSocket {}

impl BuildDataContainerCluster_NodeConfig_FastSocket {
    pub fn build(self) -> DataContainerCluster_NodeConfig_FastSocket {
        DataContainerCluster_NodeConfig_FastSocket { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_FastSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_FastSocketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfig_FastSocketRef {
        DataContainerCluster_NodeConfig_FastSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_FastSocketRef {
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
pub struct DataContainerCluster_NodeConfig_GcfsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_GcfsConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_GcfsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_GcfsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_GcfsConfig {}

impl BuildDataContainerCluster_NodeConfig_GcfsConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_GcfsConfig {
        DataContainerCluster_NodeConfig_GcfsConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_GcfsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_GcfsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfig_GcfsConfigRef {
        DataContainerCluster_NodeConfig_GcfsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_GcfsConfigRef {
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
pub struct DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_driver_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {}

impl BuildDataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
        DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
        DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_driver_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_shared_clients_per_gpu: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
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

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {}

impl BuildDataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
        DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
        DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef {
    #[allow(dead_code)]
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
pub struct DataContainerCluster_NodeConfig_GuestAccelerator {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_driver_installation_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_sharing_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_GuestAccelerator {
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
                        ::ListField<DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfig>,
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
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfig>>,
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

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_GuestAccelerator {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_GuestAccelerator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_GuestAccelerator {}

impl BuildDataContainerCluster_NodeConfig_GuestAccelerator {
    pub fn build(self) -> DataContainerCluster_NodeConfig_GuestAccelerator {
        DataContainerCluster_NodeConfig_GuestAccelerator {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_GuestAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_GuestAcceleratorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_GuestAcceleratorRef {
        DataContainerCluster_NodeConfig_GuestAcceleratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_GuestAcceleratorRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_GuestAccelerator_GpuDriverInstallationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_driver_installation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_GuestAccelerator_GpuSharingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_sharing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_Gvnic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_Gvnic {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_Gvnic {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_Gvnic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_Gvnic {}

impl BuildDataContainerCluster_NodeConfig_Gvnic {
    pub fn build(self) -> DataContainerCluster_NodeConfig_Gvnic {
        DataContainerCluster_NodeConfig_Gvnic { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_GvnicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_GvnicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfig_GvnicRef {
        DataContainerCluster_NodeConfig_GvnicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_GvnicRef {
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
pub struct DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_container_restart_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    #[doc = "Set the field `max_container_restart_period`.\n"]
    pub fn set_max_container_restart_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_container_restart_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
        DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff {
            max_container_restart_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
        DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_container_restart_period` after provisioning.\n"]
    pub fn max_container_restart_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_container_restart_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
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

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim {
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
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
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

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft {
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
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
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

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.imagefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.imagefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodefs_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nodefs_inodes_free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pid_available = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod {
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
pub struct DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
        DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imagefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_available", self.base))
    }

    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodefs_inodes_free", self.base))
    }

    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_available", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager {
        DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager { policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
        DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
        DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
        DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_KubeletConfig {
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
    crash_loop_back_off: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_max_pod_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_minimum_reclaim: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eviction_soft_grace_period: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
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
    memory_manager: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_pids_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shutdown_grace_period_critical_pods_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shutdown_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_process_oom_kill: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_manager: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager>>,
}

impl DataContainerCluster_NodeConfig_KubeletConfig {
    #[doc = "Set the field `allowed_unsafe_sysctls`.\n"]
    pub fn set_allowed_unsafe_sysctls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_unsafe_sysctls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_log_max_files`.\n"]
    pub fn set_container_log_max_files(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_log_max_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_log_max_size`.\n"]
    pub fn set_container_log_max_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_log_max_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_cfs_quota`.\n"]
    pub fn set_cpu_cfs_quota(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_cfs_quota_period`.\n"]
    pub fn set_cpu_cfs_quota_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_cfs_quota_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manager_policy`.\n"]
    pub fn set_cpu_manager_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cpu_manager_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crash_loop_back_off`.\n"]
    pub fn set_crash_loop_back_off(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOff>>,
    ) -> Self {
        self.crash_loop_back_off = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eviction_max_pod_grace_period_seconds`.\n"]
    pub fn set_eviction_max_pod_grace_period_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.eviction_max_pod_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eviction_minimum_reclaim`.\n"]
    pub fn set_eviction_minimum_reclaim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaim>>,
    ) -> Self {
        self.eviction_minimum_reclaim = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eviction_soft`.\n"]
    pub fn set_eviction_soft(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoft>>,
    ) -> Self {
        self.eviction_soft = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eviction_soft_grace_period`.\n"]
    pub fn set_eviction_soft_grace_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriod>>,
    ) -> Self {
        self.eviction_soft_grace_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_gc_high_threshold_percent`.\n"]
    pub fn set_image_gc_high_threshold_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_gc_high_threshold_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_gc_low_threshold_percent`.\n"]
    pub fn set_image_gc_low_threshold_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_gc_low_threshold_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_maximum_gc_age`.\n"]
    pub fn set_image_maximum_gc_age(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_maximum_gc_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_minimum_gc_age`.\n"]
    pub fn set_image_minimum_gc_age(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_minimum_gc_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_image_pulls`.\n"]
    pub fn set_max_parallel_image_pulls(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallel_image_pulls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_manager`.\n"]
    pub fn set_memory_manager(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_MemoryManager>>,
    ) -> Self {
        self.memory_manager = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_pids_limit`.\n"]
    pub fn set_pod_pids_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pod_pids_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shutdown_grace_period_critical_pods_seconds`.\n"]
    pub fn set_shutdown_grace_period_critical_pods_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.shutdown_grace_period_critical_pods_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shutdown_grace_period_seconds`.\n"]
    pub fn set_shutdown_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shutdown_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_process_oom_kill`.\n"]
    pub fn set_single_process_oom_kill(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_process_oom_kill = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topology_manager`.\n"]
    pub fn set_topology_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig_TopologyManager>>,
    ) -> Self {
        self.topology_manager = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_KubeletConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_KubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_KubeletConfig {}

impl BuildDataContainerCluster_NodeConfig_KubeletConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_KubeletConfig {
        DataContainerCluster_NodeConfig_KubeletConfig {
            allowed_unsafe_sysctls: core::default::Default::default(),
            container_log_max_files: core::default::Default::default(),
            container_log_max_size: core::default::Default::default(),
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            crash_loop_back_off: core::default::Default::default(),
            eviction_max_pod_grace_period_seconds: core::default::Default::default(),
            eviction_minimum_reclaim: core::default::Default::default(),
            eviction_soft: core::default::Default::default(),
            eviction_soft_grace_period: core::default::Default::default(),
            image_gc_high_threshold_percent: core::default::Default::default(),
            image_gc_low_threshold_percent: core::default::Default::default(),
            image_maximum_gc_age: core::default::Default::default(),
            image_minimum_gc_age: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            max_parallel_image_pulls: core::default::Default::default(),
            memory_manager: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
            shutdown_grace_period_critical_pods_seconds: core::default::Default::default(),
            shutdown_grace_period_seconds: core::default::Default::default(),
            single_process_oom_kill: core::default::Default::default(),
            topology_manager: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_KubeletConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_KubeletConfigRef {
        DataContainerCluster_NodeConfig_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_KubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_unsafe_sysctls` after provisioning.\n"]
    pub fn allowed_unsafe_sysctls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_unsafe_sysctls", self.base))
    }

    #[doc = "Get a reference to the value of field `container_log_max_files` after provisioning.\n"]
    pub fn container_log_max_files(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_log_max_files", self.base))
    }

    #[doc = "Get a reference to the value of field `container_log_max_size` after provisioning.\n"]
    pub fn container_log_max_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_log_max_size", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\n"]
    pub fn cpu_cfs_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\n"]
    pub fn cpu_cfs_quota_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_cfs_quota_period", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_manager_policy` after provisioning.\n"]
    pub fn cpu_manager_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_manager_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `crash_loop_back_off` after provisioning.\n"]
    pub fn crash_loop_back_off(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_CrashLoopBackOffRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crash_loop_back_off", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_max_pod_grace_period_seconds` after provisioning.\n"]
    pub fn eviction_max_pod_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eviction_max_pod_grace_period_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_EvictionMinimumReclaimRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_minimum_reclaim", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft", self.base))
    }

    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_EvictionSoftGracePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eviction_soft_grace_period", self.base))
    }

    #[doc = "Get a reference to the value of field `image_gc_high_threshold_percent` after provisioning.\n"]
    pub fn image_gc_high_threshold_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_gc_high_threshold_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `image_gc_low_threshold_percent` after provisioning.\n"]
    pub fn image_gc_low_threshold_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_gc_low_threshold_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `image_maximum_gc_age` after provisioning.\n"]
    pub fn image_maximum_gc_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_maximum_gc_age", self.base))
    }

    #[doc = "Get a reference to the value of field `image_minimum_gc_age` after provisioning.\n"]
    pub fn image_minimum_gc_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_minimum_gc_age", self.base))
    }

    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_parallel_image_pulls` after provisioning.\n"]
    pub fn max_parallel_image_pulls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_image_pulls", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_MemoryManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_manager", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_pids_limit` after provisioning.\n"]
    pub fn pod_pids_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_pids_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `shutdown_grace_period_critical_pods_seconds` after provisioning.\n"]
    pub fn shutdown_grace_period_critical_pods_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.shutdown_grace_period_critical_pods_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `shutdown_grace_period_seconds` after provisioning.\n"]
    pub fn shutdown_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shutdown_grace_period_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `single_process_oom_kill` after provisioning.\n"]
    pub fn single_process_oom_kill(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_process_oom_kill", self.base))
    }

    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfig_TopologyManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology_manager", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ptp_kvm_time_sync: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\n"]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_ptp_kvm_time_sync` after provisioning.\n"]
    pub fn enable_ptp_kvm_time_sync(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ptp_kvm_time_sync", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_secret_manager_secret_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_secret_manager_secret_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_generation`.\n"]
    pub fn set_gcs_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gcs_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_uri`.\n"]
    pub fn set_gcs_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
            gcs_generation: core::default::Default::default(),
            gcs_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_secret_manager_secret_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_generation` after provisioning.\n"]
    pub fn gcs_generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_uri` after provisioning.\n"]
    pub fn gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_script: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    #[doc = "Set the field `init_script`.\n"]
    pub fn set_init_script(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScript>,
                    >,
    ) -> Self {
        self.init_script = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit {
            init_script: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `init_script` after provisioning.\n"]
    pub fn init_script(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit_InitScriptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.init_script", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_1g: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepage_size_2m: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    #[doc = "Set the field `hugepage_size_1g`.\n"]
    pub fn set_hugepage_size_1g(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hugepage_size_1g = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hugepage_size_2m`.\n"]
    pub fn set_hugepage_size_2m(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hugepage_size_2m = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hugepage_size_1g` after provisioning.\n"]
    pub fn hugepage_size_1g(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugepage_size_1g", self.base))
    }

    #[doc = "Get a reference to the value of field `hugepage_size_2m` after provisioning.\n"]
    pub fn hugepage_size_2m(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugepage_size_2m", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading {
            policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    #[doc = "Set the field `swap_size_gib`.\n"]
    pub fn set_swap_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `swap_size_percent`.\n"]
    pub fn set_swap_size_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"]
    pub fn swap_size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_gib", self.base))
    }

    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"]
    pub fn swap_size_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    type O =
        terrars
        ::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile {
            disk_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig {
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef {
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
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_size_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    #[doc = "Set the field `swap_size_gib`.\n"]
    pub fn set_swap_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `swap_size_percent`.\n"]
    pub fn set_swap_size_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swap_size_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    type O =
        terrars
        ::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"]
    pub fn swap_size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_gib", self.base))
    }

    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"]
    pub fn swap_size_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swap_size_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_profile: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_local_ssd_profile: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_local_ssd_profile: std
    ::option
    ::Option<
        terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile>,
    >,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    #[doc = "Set the field `boot_disk_profile`.\n"]
    pub fn set_boot_disk_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfile>,
                    >,
    ) -> Self {
        self.boot_disk_profile = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfile,
                        >,
                    >,
    ) -> Self {
        self.dedicated_local_ssd_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
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
                        ::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfig>,
                    >,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfile,
                        >,
                    >,
    ) -> Self {
        self.ephemeral_local_ssd_profile = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig {
            boot_disk_profile: core::default::Default::default(),
            dedicated_local_ssd_profile: core::default::Default::default(),
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            ephemeral_local_ssd_profile: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boot_disk_profile` after provisioning.\n"]
    pub fn boot_disk_profile(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_BootDiskProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]
    pub fn dedicated_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_DedicatedLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_local_ssd_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]
    pub fn ephemeral_local_ssd_profile(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig_EphemeralLocalSsdProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_local_ssd_profile", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accurate_time_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_node_init: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugepages_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kernel_module_loading: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swap_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transparent_hugepage_defrag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transparent_hugepage_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfig {
    #[doc = "Set the field `accurate_time_config`.\n"]
    pub fn set_accurate_time_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfig>>,
    ) -> Self {
        self.accurate_time_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cgroup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_node_init`.\n"]
    pub fn set_custom_node_init(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInit>>,
    ) -> Self {
        self.custom_node_init = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hugepages_config`.\n"]
    pub fn set_hugepages_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfig>>,
    ) -> Self {
        self.hugepages_config = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>,
                    >,
    ) -> Self {
        self.node_kernel_module_loading = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `swap_config`.\n"]
    pub fn set_swap_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfig>>,
    ) -> Self {
        self.swap_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sysctls`.\n"]
    pub fn set_sysctls(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sysctls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transparent_hugepage_defrag`.\n"]
    pub fn set_transparent_hugepage_defrag(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transparent_hugepage_defrag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transparent_hugepage_enabled`.\n"]
    pub fn set_transparent_hugepage_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transparent_hugepage_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LinuxNodeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LinuxNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LinuxNodeConfig {}

impl BuildDataContainerCluster_NodeConfig_LinuxNodeConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LinuxNodeConfig {
        DataContainerCluster_NodeConfig_LinuxNodeConfig {
            accurate_time_config: core::default::Default::default(),
            cgroup_mode: core::default::Default::default(),
            custom_node_init: core::default::Default::default(),
            hugepages_config: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            swap_config: core::default::Default::default(),
            sysctls: core::default::Default::default(),
            transparent_hugepage_defrag: core::default::Default::default(),
            transparent_hugepage_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LinuxNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LinuxNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LinuxNodeConfigRef {
        DataContainerCluster_NodeConfig_LinuxNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LinuxNodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accurate_time_config` after provisioning.\n"]
    pub fn accurate_time_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_AccurateTimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accurate_time_config", self.base))
    }

    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_node_init` after provisioning.\n"]
    pub fn custom_node_init(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_CustomNodeInitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_node_init", self.base))
    }

    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_HugepagesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hugepages_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kernel_module_loading", self.base))
    }

    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_SwapConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sysctls` after provisioning.\n"]
    pub fn sysctls(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }

    #[doc = "Get a reference to the value of field `transparent_hugepage_defrag` after provisioning.\n"]
    pub fn transparent_hugepage_defrag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transparent_hugepage_defrag", self.base))
    }

    #[doc = "Get a reference to the value of field `transparent_hugepage_enabled` after provisioning.\n"]
    pub fn transparent_hugepage_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transparent_hugepage_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.local_ssd_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {}

impl BuildDataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
        DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig {
            local_ssd_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
        DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_NodeImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_NodeImageConfig {
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_project`.\n"]
    pub fn set_image_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_NodeImageConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_NodeImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_NodeImageConfig {}

impl BuildDataContainerCluster_NodeConfig_NodeImageConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_NodeImageConfig {
        DataContainerCluster_NodeConfig_NodeImageConfig {
            image: core::default::Default::default(),
            image_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_NodeImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_NodeImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_NodeImageConfigRef {
        DataContainerCluster_NodeConfig_NodeImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_NodeImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_project` after provisioning.\n"]
    pub fn image_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_reservation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NodeConfig_ReservationAffinity {
    #[doc = "Set the field `consume_reservation_type`.\n"]
    pub fn set_consume_reservation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consume_reservation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_ReservationAffinity {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_ReservationAffinity {}

impl BuildDataContainerCluster_NodeConfig_ReservationAffinity {
    pub fn build(self) -> DataContainerCluster_NodeConfig_ReservationAffinity {
        DataContainerCluster_NodeConfig_ReservationAffinity {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_ReservationAffinityRef {
        DataContainerCluster_NodeConfig_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\n"]
    pub fn consume_reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_SandboxConfig {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_SandboxConfig {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_SandboxConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_SandboxConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_SandboxConfig {}

impl BuildDataContainerCluster_NodeConfig_SandboxConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_SandboxConfig {
        DataContainerCluster_NodeConfig_SandboxConfig { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_SandboxConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_SandboxConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_SandboxConfigRef {
        DataContainerCluster_NodeConfig_SandboxConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_SandboxConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_SecondaryBootDisks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_SecondaryBootDisks {
    #[doc = "Set the field `disk_image`.\n"]
    pub fn set_disk_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_SecondaryBootDisks {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_SecondaryBootDisks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_SecondaryBootDisks {}

impl BuildDataContainerCluster_NodeConfig_SecondaryBootDisks {
    pub fn build(self) -> DataContainerCluster_NodeConfig_SecondaryBootDisks {
        DataContainerCluster_NodeConfig_SecondaryBootDisks {
            disk_image: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_SecondaryBootDisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_SecondaryBootDisksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_SecondaryBootDisksRef {
        DataContainerCluster_NodeConfig_SecondaryBootDisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_SecondaryBootDisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_image` after provisioning.\n"]
    pub fn disk_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_image", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {}

impl BuildDataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
    pub fn build(self) -> DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
        DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
        DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_SoleTenantConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity: std::option::Option<terrars::SetField<DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>>,
}

impl DataContainerCluster_NodeConfig_SoleTenantConfig {
    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinity>>,
    ) -> Self {
        self.node_affinity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_SoleTenantConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_SoleTenantConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_SoleTenantConfig {}

impl BuildDataContainerCluster_NodeConfig_SoleTenantConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_SoleTenantConfig {
        DataContainerCluster_NodeConfig_SoleTenantConfig {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_SoleTenantConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_SoleTenantConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_SoleTenantConfigRef {
        DataContainerCluster_NodeConfig_SoleTenantConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_SoleTenantConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `node_affinity` after provisioning.\n"]
    pub fn node_affinity(&self) -> terrars::SetRef<DataContainerCluster_NodeConfig_SoleTenantConfig_NodeAffinityRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_affinity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_Taint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_Taint {
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

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_Taint {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_Taint {}

impl BuildDataContainerCluster_NodeConfig_Taint {
    pub fn build(self) -> DataContainerCluster_NodeConfig_Taint {
        DataContainerCluster_NodeConfig_Taint {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfig_TaintRef {
        DataContainerCluster_NodeConfig_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_TaintRef {
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
pub struct DataContainerCluster_NodeConfig_TaintConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture_taint_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_TaintConfig {
    #[doc = "Set the field `architecture_taint_behavior`.\n"]
    pub fn set_architecture_taint_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.architecture_taint_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_TaintConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_TaintConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_TaintConfig {}

impl BuildDataContainerCluster_NodeConfig_TaintConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_TaintConfig {
        DataContainerCluster_NodeConfig_TaintConfig {
            architecture_taint_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_TaintConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_TaintConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_TaintConfigRef {
        DataContainerCluster_NodeConfig_TaintConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_TaintConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture_taint_behavior` after provisioning.\n"]
    pub fn architecture_taint_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture_taint_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_WindowsNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    osversion: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_WindowsNodeConfig {
    #[doc = "Set the field `osversion`.\n"]
    pub fn set_osversion(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.osversion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_WindowsNodeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_WindowsNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_WindowsNodeConfig {}

impl BuildDataContainerCluster_NodeConfig_WindowsNodeConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_WindowsNodeConfig {
        DataContainerCluster_NodeConfig_WindowsNodeConfig { osversion: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_WindowsNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_WindowsNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_WindowsNodeConfigRef {
        DataContainerCluster_NodeConfig_WindowsNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_WindowsNodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `osversion` after provisioning.\n"]
    pub fn osversion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.osversion", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig_WorkloadMetadataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeConfig_WorkloadMetadataConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig_WorkloadMetadataConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig_WorkloadMetadataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig_WorkloadMetadataConfig {}

impl BuildDataContainerCluster_NodeConfig_WorkloadMetadataConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig_WorkloadMetadataConfig {
        DataContainerCluster_NodeConfig_WorkloadMetadataConfig { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
        DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_nodes: std::option::Option<terrars::ListField<DataContainerCluster_ConfidentialNodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containerd_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_taints: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_EffectiveTaints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_storage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage_local_ssd_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_socket: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_FastSocket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flex_start: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_GcfsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpudirect_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_GuestAccelerator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gvnic: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_Gvnic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_nvme_ssd_block_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>>,
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
    node_image_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_NodeImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_SandboxConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_boot_disks: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_SecondaryBootDisks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_SoleTenantConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_pools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_Taint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_TaintConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_node_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_WindowsNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_metadata_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_WorkloadMetadataConfig>>,
}

impl DataContainerCluster_NodeConfig {
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_AdvancedMachineFeatures>>,
    ) -> Self {
        self.advanced_machine_features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_BootDisk>>,
    ) -> Self {
        self.boot_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_ConfidentialNodes>>,
    ) -> Self {
        self.confidential_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig>>,
    ) -> Self {
        self.containerd_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_taints`.\n"]
    pub fn set_effective_taints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_EffectiveTaints>>,
    ) -> Self {
        self.effective_taints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_confidential_storage`.\n"]
    pub fn set_enable_confidential_storage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfig>>,
    ) -> Self {
        self.ephemeral_storage_local_ssd_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_FastSocket>>,
    ) -> Self {
        self.fast_socket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flex_start`.\n"]
    pub fn set_flex_start(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flex_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_GcfsConfig>>,
    ) -> Self {
        self.gcfs_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpudirect_strategy`.\n"]
    pub fn set_gpudirect_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpudirect_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_GuestAccelerator>>,
    ) -> Self {
        self.guest_accelerator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_Gvnic>>,
    ) -> Self {
        self.gvnic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_KubeletConfig>>,
    ) -> Self {
        self.kubelet_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig>>,
    ) -> Self {
        self.linux_node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfig>>,
    ) -> Self {
        self.local_nvme_ssd_block_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.local_ssd_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_encryption_mode`.\n"]
    pub fn set_local_ssd_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_variant = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_run_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_group`.\n"]
    pub fn set_node_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_image_config`.\n"]
    pub fn set_node_image_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_NodeImageConfig>>,
    ) -> Self {
        self.node_image_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_ReservationAffinity>>,
    ) -> Self {
        self.reservation_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sandbox_config`.\n"]
    pub fn set_sandbox_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_SandboxConfig>>,
    ) -> Self {
        self.sandbox_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_boot_disks`.\n"]
    pub fn set_secondary_boot_disks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_SecondaryBootDisks>>,
    ) -> Self {
        self.secondary_boot_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_ShieldedInstanceConfig>>,
    ) -> Self {
        self.shielded_instance_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_SoleTenantConfig>>,
    ) -> Self {
        self.sole_tenant_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot`.\n"]
    pub fn set_spot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.spot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_pools`.\n"]
    pub fn set_storage_pools(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.storage_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_Taint>>,
    ) -> Self {
        self.taint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `taint_config`.\n"]
    pub fn set_taint_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_TaintConfig>>,
    ) -> Self {
        self.taint_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `windows_node_config`.\n"]
    pub fn set_windows_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_WindowsNodeConfig>>,
    ) -> Self {
        self.windows_node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_WorkloadMetadataConfig>>,
    ) -> Self {
        self.workload_metadata_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeConfig {}

impl BuildDataContainerCluster_NodeConfig {
    pub fn build(self) -> DataContainerCluster_NodeConfig {
        DataContainerCluster_NodeConfig {
            advanced_machine_features: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            boot_disk_kms_key: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            effective_taints: core::default::Default::default(),
            enable_confidential_storage: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            flex_start: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            gpudirect_strategy: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            image_type: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            local_ssd_encryption_mode: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            node_image_config: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            sandbox_config: core::default::Default::default(),
            secondary_boot_disks: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            spot: core::default::Default::default(),
            storage_pools: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            taint_config: core::default::Default::default(),
            windows_node_config: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeConfigRef {
        DataContainerCluster_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(&self) -> terrars::ListRef<DataContainerCluster_ConfidentialNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containerd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_taints` after provisioning.\n"]
    pub fn effective_taints(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_EffectiveTaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_taints", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_confidential_storage` after provisioning.\n"]
    pub fn enable_confidential_storage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_EphemeralStorageLocalSsdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage_local_ssd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_FastSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }

    #[doc = "Get a reference to the value of field `flex_start` after provisioning.\n"]
    pub fn flex_start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flex_start", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_GcfsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gpudirect_strategy` after provisioning.\n"]
    pub fn gpudirect_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpudirect_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_GuestAcceleratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.base))
    }

    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_GvnicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }

    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_KubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LocalNvmeSsdBlockConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_nvme_ssd_block_config", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_encryption_mode` after provisioning.\n"]
    pub fn local_ssd_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_run_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc = "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }

    #[doc = "Get a reference to the value of field `node_image_config` after provisioning.\n"]
    pub fn node_image_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_NodeImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_image_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_SandboxConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sandbox_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_SecondaryBootDisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_boot_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_SoleTenantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_config", self.base))
    }

    #[doc = "Get a reference to the value of field `spot` after provisioning.\n"]
    pub fn spot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_pools` after provisioning.\n"]
    pub fn storage_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_TaintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }

    #[doc = "Get a reference to the value of field `taint_config` after provisioning.\n"]
    pub fn taint_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_TaintConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_WindowsNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_WorkloadMetadataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_metadata_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodeCreationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_creation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodeCreationConfig {
    #[doc = "Set the field `node_creation_mode`.\n"]
    pub fn set_node_creation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.node_creation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodeCreationConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodeCreationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodeCreationConfig {}

impl BuildDataContainerCluster_NodeCreationConfig {
    pub fn build(self) -> DataContainerCluster_NodeCreationConfig {
        DataContainerCluster_NodeCreationConfig { node_creation_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodeCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodeCreationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodeCreationConfigRef {
        DataContainerCluster_NodeCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodeCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_creation_mode` after provisioning.\n"]
    pub fn node_creation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_creation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_Autoscaling {
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

impl DataContainerCluster_NodePool_Autoscaling {
    #[doc = "Set the field `location_policy`.\n"]
    pub fn set_location_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_node_count`.\n"]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_count`.\n"]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_max_node_count`.\n"]
    pub fn set_total_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_min_node_count`.\n"]
    pub fn set_total_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_min_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_Autoscaling {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_Autoscaling {}

impl BuildDataContainerCluster_NodePool_Autoscaling {
    pub fn build(self) -> DataContainerCluster_NodePool_Autoscaling {
        DataContainerCluster_NodePool_Autoscaling {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodePool_AutoscalingRef {
        DataContainerCluster_NodePool_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_policy` after provisioning.\n"]
    pub fn location_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_max_node_count` after provisioning.\n"]
    pub fn total_max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_max_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_min_node_count` after provisioning.\n"]
    pub fn total_min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_BestEffortProvisioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_provision_nodes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataContainerCluster_NodePool_BestEffortProvisioning {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_provision_nodes`.\n"]
    pub fn set_min_provision_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_provision_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_BestEffortProvisioning {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_BestEffortProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_BestEffortProvisioning {}

impl BuildDataContainerCluster_NodePool_BestEffortProvisioning {
    pub fn build(self) -> DataContainerCluster_NodePool_BestEffortProvisioning {
        DataContainerCluster_NodePool_BestEffortProvisioning {
            enabled: core::default::Default::default(),
            min_provision_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_BestEffortProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_BestEffortProvisioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_BestEffortProvisioningRef {
        DataContainerCluster_NodePool_BestEffortProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_BestEffortProvisioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `min_provision_nodes` after provisioning.\n"]
    pub fn min_provision_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_provision_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {}

impl BuildDataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
    pub fn build(self) -> DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
        DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport {
            enabled: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
        DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_until_end_of_support: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>>,
}

impl DataContainerCluster_NodePool_MaintenancePolicy {
    #[doc = "Set the field `exclusion_until_end_of_support`.\n"]
    pub fn set_exclusion_until_end_of_support(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupport>,
                    >,
    ) -> Self {
        self.exclusion_until_end_of_support = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_MaintenancePolicy {}

impl BuildDataContainerCluster_NodePool_MaintenancePolicy {
    pub fn build(self) -> DataContainerCluster_NodePool_MaintenancePolicy {
        DataContainerCluster_NodePool_MaintenancePolicy {
            exclusion_until_end_of_support: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_MaintenancePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_MaintenancePolicyRef {
        DataContainerCluster_NodePool_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_until_end_of_support` after provisioning.\n"]
    pub fn exclusion_until_end_of_support(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodePool_MaintenancePolicy_ExclusionUntilEndOfSupportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_until_end_of_support", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodePool_Management {
    #[doc = "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_upgrade = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_Management {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_Management {}

impl BuildDataContainerCluster_NodePool_Management {
    pub fn build(self) -> DataContainerCluster_NodePool_Management {
        DataContainerCluster_NodePool_Management {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_ManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodePool_ManagementRef {
        DataContainerCluster_NodePool_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_ManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {}

impl BuildDataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
    pub fn build(self) -> DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
        DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
        DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    #[doc = "Set the field `max_pods_per_node`.\n"]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_pod_range`.\n"]
    pub fn set_secondary_pod_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {}

impl BuildDataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
    pub fn build(self) -> DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
        DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs {
            max_pods_per_node: core::default::Default::default(),
            secondary_pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
        DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\n"]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_pod_range` after provisioning.\n"]
    pub fn secondary_pod_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_pod_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_network_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_node_network_configs: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_pod_network_configs: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_pod_range: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std::option::Option<terrars::ListField<DataContainerCluster_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_cidr_overprovision_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_NetworkConfig {
    #[doc = "Set the field `accelerator_network_profile`.\n"]
    pub fn set_accelerator_network_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.accelerator_network_profile = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigs>,
                    >,
    ) -> Self {
        self.additional_node_network_configs = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigs>,
                    >,
    ) -> Self {
        self.additional_pod_network_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_pod_range`.\n"]
    pub fn set_create_pod_range(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NetworkPerformanceConfig>>,
    ) -> Self {
        self.network_performance_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfig>>,
    ) -> Self {
        self.pod_cidr_overprovision_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_ipv4_cidr_block`.\n"]
    pub fn set_pod_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pod_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_range`.\n"]
    pub fn set_pod_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pod_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_NetworkConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_NetworkConfig {}

impl BuildDataContainerCluster_NodePool_NetworkConfig {
    pub fn build(self) -> DataContainerCluster_NodePool_NetworkConfig {
        DataContainerCluster_NodePool_NetworkConfig {
            accelerator_network_profile: core::default::Default::default(),
            additional_node_network_configs: core::default::Default::default(),
            additional_pod_network_configs: core::default::Default::default(),
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_NetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_NetworkConfigRef {
        DataContainerCluster_NodePool_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_network_profile` after provisioning.\n"]
    pub fn accelerator_network_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_network_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_node_network_configs` after provisioning.\n"]
    pub fn additional_node_network_configs(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodePool_NetworkConfig_AdditionalNodeNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_node_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_pod_network_configs` after provisioning.\n"]
    pub fn additional_pod_network_configs(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodePool_NetworkConfig_AdditionalPodNetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_pod_network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `create_pod_range` after provisioning.\n"]
    pub fn create_pod_range(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_pod_range", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> terrars::ListRef<DataContainerCluster_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_IpAllocationPolicy_PodCidrOverprovisionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_cidr_overprovision_config", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\n"]
    pub fn pod_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_range` after provisioning.\n"]
    pub fn pod_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_NodeDrainConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_termination_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pdb_timeout_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_pdb_during_node_pool_deletion: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodePool_NodeDrainConfig {
    #[doc = "Set the field `grace_termination_duration`.\n"]
    pub fn set_grace_termination_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.grace_termination_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pdb_timeout_duration`.\n"]
    pub fn set_pdb_timeout_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pdb_timeout_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respect_pdb_during_node_pool_deletion`.\n"]
    pub fn set_respect_pdb_during_node_pool_deletion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.respect_pdb_during_node_pool_deletion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_NodeDrainConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_NodeDrainConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_NodeDrainConfig {}

impl BuildDataContainerCluster_NodePool_NodeDrainConfig {
    pub fn build(self) -> DataContainerCluster_NodePool_NodeDrainConfig {
        DataContainerCluster_NodePool_NodeDrainConfig {
            grace_termination_duration: core::default::Default::default(),
            pdb_timeout_duration: core::default::Default::default(),
            respect_pdb_during_node_pool_deletion: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_NodeDrainConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_NodeDrainConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_NodeDrainConfigRef {
        DataContainerCluster_NodePool_NodeDrainConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_NodeDrainConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grace_termination_duration` after provisioning.\n"]
    pub fn grace_termination_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_termination_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `pdb_timeout_duration` after provisioning.\n"]
    pub fn pdb_timeout_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pdb_timeout_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `respect_pdb_during_node_pool_deletion` after provisioning.\n"]
    pub fn respect_pdb_during_node_pool_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_pdb_during_node_pool_deletion", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_PlacementPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool_PlacementPolicy {
    #[doc = "Set the field `policy_name`.\n"]
    pub fn set_policy_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tpu_topology`.\n"]
    pub fn set_tpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tpu_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_PlacementPolicy {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_PlacementPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_PlacementPolicy {}

impl BuildDataContainerCluster_NodePool_PlacementPolicy {
    pub fn build(self) -> DataContainerCluster_NodePool_PlacementPolicy {
        DataContainerCluster_NodePool_PlacementPolicy {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_PlacementPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_PlacementPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_PlacementPolicyRef {
        DataContainerCluster_NodePool_PlacementPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_PlacementPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_name` after provisioning.\n"]
    pub fn policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tpu_topology` after provisioning.\n"]
    pub fn tpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePool_QueuedProvisioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_NodePool_QueuedProvisioning {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool_QueuedProvisioning {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool_QueuedProvisioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool_QueuedProvisioning {}

impl BuildDataContainerCluster_NodePool_QueuedProvisioning {
    pub fn build(self) -> DataContainerCluster_NodePool_QueuedProvisioning {
        DataContainerCluster_NodePool_QueuedProvisioning { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePool_QueuedProvisioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePool_QueuedProvisioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePool_QueuedProvisioningRef {
        DataContainerCluster_NodePool_QueuedProvisioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePool_QueuedProvisioningRef {
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
pub struct DataContainerCluster_NodePool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    best_effort_provisioning: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePool_BestEffortProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_node_count_changes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_instance_group_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_drain_config: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_NodeDrainConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_PlacementPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queued_provisioning: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_QueuedProvisioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_settings: std::option::Option<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePool {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_Autoscaling>>,
    ) -> Self {
        self.autoscaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `best_effort_provisioning`.\n"]
    pub fn set_best_effort_provisioning(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_BestEffortProvisioning>>,
    ) -> Self {
        self.best_effort_provisioning = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_node_count_changes`.\n"]
    pub fn set_ignore_node_count_changes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_node_count_changes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_node_count`.\n"]
    pub fn set_initial_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_group_urls`.\n"]
    pub fn set_instance_group_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_group_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_MaintenancePolicy>>,
    ) -> Self {
        self.maintenance_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_instance_group_urls`.\n"]
    pub fn set_managed_instance_group_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.managed_instance_group_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_Management>>,
    ) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_pods_per_node`.\n"]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_NetworkConfig>>,
    ) -> Self {
        self.network_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig>>,
    ) -> Self {
        self.node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_drain_config`.\n"]
    pub fn set_node_drain_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_NodeDrainConfig>>,
    ) -> Self {
        self.node_drain_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_locations`.\n"]
    pub fn set_node_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.node_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_PlacementPolicy>>,
    ) -> Self {
        self.placement_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queued_provisioning`.\n"]
    pub fn set_queued_provisioning(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_QueuedProvisioning>>,
    ) -> Self {
        self.queued_provisioning = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePool_UpgradeSettings>>,
    ) -> Self {
        self.upgrade_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePool {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePool {}

impl BuildDataContainerCluster_NodePool {
    pub fn build(self) -> DataContainerCluster_NodePool {
        DataContainerCluster_NodePool {
            autoscaling: core::default::Default::default(),
            best_effort_provisioning: core::default::Default::default(),
            ignore_node_count_changes: core::default::Default::default(),
            initial_node_count: core::default::Default::default(),
            instance_group_urls: core::default::Default::default(),
            maintenance_policy: core::default::Default::default(),
            managed_instance_group_urls: core::default::Default::default(),
            management: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
            name: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
            network_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_drain_config: core::default::Default::default(),
            node_locations: core::default::Default::default(),
            placement_policy: core::default::Default::default(),
            queued_provisioning: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodePoolRef {
        DataContainerCluster_NodePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<DataContainerCluster_NodePool_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `best_effort_provisioning` after provisioning.\n"]
    pub fn best_effort_provisioning(&self) -> terrars::ListRef<DataContainerCluster_NodePool_BestEffortProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_effort_provisioning", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_node_count_changes` after provisioning.\n"]
    pub fn ignore_node_count_changes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_node_count_changes", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\n"]
    pub fn initial_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_urls` after provisioning.\n"]
    pub fn instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataContainerCluster_NodePool_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\n"]
    pub fn managed_instance_group_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_group_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<DataContainerCluster_NodePool_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\n"]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataContainerCluster_NodePool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> terrars::ListRef<DataContainerCluster_NodePool_NodeDrainConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_drain_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\n"]
    pub fn node_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> terrars::ListRef<DataContainerCluster_NodePool_PlacementPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(&self) -> terrars::ListRef<DataContainerCluster_NodePool_QueuedProvisioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queued_provisioning", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> terrars::ListRef<DataContainerCluster_NodePool_UpgradeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kernel_module_loading: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>>,
}

impl DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    #[doc = "Set the field `cgroup_mode`.\n"]
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
                        terrars::ListField<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoading>,
                    >,
    ) -> Self {
        self.node_kernel_module_loading = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {}

impl BuildDataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
    pub fn build(self) -> DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
        DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig {
            cgroup_mode: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
        DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_NodeConfig_LinuxNodeConfig_NodeKernelModuleLoadingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kernel_module_loading", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolAutoConfig_NetworkTags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NodePoolAutoConfig_NetworkTags {
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolAutoConfig_NetworkTags {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolAutoConfig_NetworkTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolAutoConfig_NetworkTags {}

impl BuildDataContainerCluster_NodePoolAutoConfig_NetworkTags {
    pub fn build(self) -> DataContainerCluster_NodePoolAutoConfig_NetworkTags {
        DataContainerCluster_NodePoolAutoConfig_NetworkTags { tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
        DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_kubelet_readonly_port_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {}

impl BuildDataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
    pub fn build(self) -> DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
        DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig {
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
        DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolAutoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_node_config: std::option::Option<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_NetworkTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_kubelet_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NodePoolAutoConfig {
    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfig>>,
    ) -> Self {
        self.linux_node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\n"]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_NetworkTags>>,
    ) -> Self {
        self.network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_kubelet_config`.\n"]
    pub fn set_node_kubelet_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfig>>,
    ) -> Self {
        self.node_kubelet_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolAutoConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolAutoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolAutoConfig {}

impl BuildDataContainerCluster_NodePoolAutoConfig {
    pub fn build(self) -> DataContainerCluster_NodePoolAutoConfig {
        DataContainerCluster_NodePoolAutoConfig {
            linux_node_config: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            node_kubelet_config: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolAutoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolAutoConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodePoolAutoConfigRef {
        DataContainerCluster_NodePoolAutoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolAutoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(&self) -> terrars::ListRef<DataContainerCluster_NodePoolAutoConfig_LinuxNodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(&self) -> terrars::ListRef<DataContainerCluster_NodePoolAutoConfig_NetworkTagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `node_kubelet_config` after provisioning.\n"]
    pub fn node_kubelet_config(&self) -> terrars::ListRef<DataContainerCluster_NodePoolAutoConfig_NodeKubeletConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containerd_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcfs_config: std::option::Option<terrars::ListField<DataContainerCluster_NodeConfig_GcfsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_kubelet_readonly_port_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_variant: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_ContainerdConfig>>,
    ) -> Self {
        self.containerd_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodeConfig_GcfsConfig>>,
    ) -> Self {
        self.gcfs_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_variant = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolDefaults_NodeConfigDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolDefaults_NodeConfigDefaults {}

impl BuildDataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
    pub fn build(self) -> DataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
        DataContainerCluster_NodePoolDefaults_NodeConfigDefaults {
            containerd_config: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
        DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_ContainerdConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containerd_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> terrars::ListRef<DataContainerCluster_NodeConfig_GcfsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }

    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_variant", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NodePoolDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config_defaults: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_NodePoolDefaults_NodeConfigDefaults>>,
}

impl DataContainerCluster_NodePoolDefaults {
    #[doc = "Set the field `node_config_defaults`.\n"]
    pub fn set_node_config_defaults(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NodePoolDefaults_NodeConfigDefaults>>,
    ) -> Self {
        self.node_config_defaults = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NodePoolDefaults {
    type O = terrars::BlockAssignable<DataContainerCluster_NodePoolDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NodePoolDefaults {}

impl BuildDataContainerCluster_NodePoolDefaults {
    pub fn build(self) -> DataContainerCluster_NodePoolDefaults {
        DataContainerCluster_NodePoolDefaults { node_config_defaults: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NodePoolDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NodePoolDefaultsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NodePoolDefaultsRef {
        DataContainerCluster_NodePoolDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NodePoolDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_config_defaults` after provisioning.\n"]
    pub fn node_config_defaults(&self) -> terrars::ListRef<DataContainerCluster_NodePoolDefaults_NodeConfigDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config_defaults", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NotificationConfig_Pubsub_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_type: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataContainerCluster_NotificationConfig_Pubsub_Filter {
    #[doc = "Set the field `event_type`.\n"]
    pub fn set_event_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NotificationConfig_Pubsub_Filter {
    type O = terrars::BlockAssignable<DataContainerCluster_NotificationConfig_Pubsub_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NotificationConfig_Pubsub_Filter {}

impl BuildDataContainerCluster_NotificationConfig_Pubsub_Filter {
    pub fn build(self) -> DataContainerCluster_NotificationConfig_Pubsub_Filter {
        DataContainerCluster_NotificationConfig_Pubsub_Filter { event_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NotificationConfig_Pubsub_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NotificationConfig_Pubsub_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NotificationConfig_Pubsub_FilterRef {
        DataContainerCluster_NotificationConfig_Pubsub_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NotificationConfig_Pubsub_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NotificationConfig_Pubsub {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::ListField<DataContainerCluster_NotificationConfig_Pubsub_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_NotificationConfig_Pubsub {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NotificationConfig_Pubsub_Filter>>,
    ) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NotificationConfig_Pubsub {
    type O = terrars::BlockAssignable<DataContainerCluster_NotificationConfig_Pubsub>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NotificationConfig_Pubsub {}

impl BuildDataContainerCluster_NotificationConfig_Pubsub {
    pub fn build(self) -> DataContainerCluster_NotificationConfig_Pubsub {
        DataContainerCluster_NotificationConfig_Pubsub {
            enabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NotificationConfig_PubsubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NotificationConfig_PubsubRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_NotificationConfig_PubsubRef {
        DataContainerCluster_NotificationConfig_PubsubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NotificationConfig_PubsubRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataContainerCluster_NotificationConfig_Pubsub_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub: std::option::Option<terrars::ListField<DataContainerCluster_NotificationConfig_Pubsub>>,
}

impl DataContainerCluster_NotificationConfig {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_NotificationConfig_Pubsub>>,
    ) -> Self {
        self.pubsub = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_NotificationConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_NotificationConfig {}

impl BuildDataContainerCluster_NotificationConfig {
    pub fn build(self) -> DataContainerCluster_NotificationConfig {
        DataContainerCluster_NotificationConfig { pubsub: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_NotificationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_NotificationConfigRef {
        DataContainerCluster_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> terrars::ListRef<DataContainerCluster_NotificationConfig_PubsubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_PodAutoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hpa_profile: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_PodAutoscaling {
    #[doc = "Set the field `hpa_profile`.\n"]
    pub fn set_hpa_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hpa_profile = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_PodAutoscaling {
    type O = terrars::BlockAssignable<DataContainerCluster_PodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_PodAutoscaling {}

impl BuildDataContainerCluster_PodAutoscaling {
    pub fn build(self) -> DataContainerCluster_PodAutoscaling {
        DataContainerCluster_PodAutoscaling { hpa_profile: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_PodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_PodAutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_PodAutoscalingRef {
        DataContainerCluster_PodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_PodAutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hpa_profile` after provisioning.\n"]
    pub fn hpa_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hpa_profile", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {}

impl BuildDataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
    pub fn build(self) -> DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
        DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
        DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef {
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
pub struct DataContainerCluster_PrivateClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_nodes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_global_access_config: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_ipv4_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peering_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_PrivateClusterConfig {
    #[doc = "Set the field `enable_private_endpoint`.\n"]
    pub fn set_enable_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_global_access_config`.\n"]
    pub fn set_master_global_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfig>>,
    ) -> Self {
        self.master_global_access_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_ipv4_cidr_block`.\n"]
    pub fn set_master_ipv4_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_ipv4_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peering_name`.\n"]
    pub fn set_peering_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peering_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_subnetwork`.\n"]
    pub fn set_private_endpoint_subnetwork(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_endpoint_subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_endpoint`.\n"]
    pub fn set_public_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_PrivateClusterConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_PrivateClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_PrivateClusterConfig {}

impl BuildDataContainerCluster_PrivateClusterConfig {
    pub fn build(self) -> DataContainerCluster_PrivateClusterConfig {
        DataContainerCluster_PrivateClusterConfig {
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

#[allow(dead_code)]
pub struct DataContainerCluster_PrivateClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_PrivateClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_PrivateClusterConfigRef {
        DataContainerCluster_PrivateClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_PrivateClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\n"]
    pub fn enable_private_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `master_global_access_config` after provisioning.\n"]
    pub fn master_global_access_config(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_PrivateClusterConfig_MasterGlobalAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_global_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\n"]
    pub fn master_ipv4_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_ipv4_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `peering_name` after provisioning.\n"]
    pub fn peering_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peering_name", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_subnetwork` after provisioning.\n"]
    pub fn private_endpoint_subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_RbacBindingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_insecure_binding_system_authenticated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_insecure_binding_system_unauthenticated: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_RbacBindingConfig {
    #[doc = "Set the field `enable_insecure_binding_system_authenticated`.\n"]
    pub fn set_enable_insecure_binding_system_authenticated(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_authenticated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_insecure_binding_system_unauthenticated`.\n"]
    pub fn set_enable_insecure_binding_system_unauthenticated(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_unauthenticated = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_RbacBindingConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_RbacBindingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_RbacBindingConfig {}

impl BuildDataContainerCluster_RbacBindingConfig {
    pub fn build(self) -> DataContainerCluster_RbacBindingConfig {
        DataContainerCluster_RbacBindingConfig {
            enable_insecure_binding_system_authenticated: core::default::Default::default(),
            enable_insecure_binding_system_unauthenticated: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_RbacBindingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_RbacBindingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_RbacBindingConfigRef {
        DataContainerCluster_RbacBindingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_RbacBindingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_insecure_binding_system_authenticated` after provisioning.\n"]
    pub fn enable_insecure_binding_system_authenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insecure_binding_system_authenticated", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_insecure_binding_system_unauthenticated` after provisioning.\n"]
    pub fn enable_insecure_binding_system_unauthenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insecure_binding_system_unauthenticated", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ReleaseChannel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ReleaseChannel {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ReleaseChannel {
    type O = terrars::BlockAssignable<DataContainerCluster_ReleaseChannel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ReleaseChannel {}

impl BuildDataContainerCluster_ReleaseChannel {
    pub fn build(self) -> DataContainerCluster_ReleaseChannel {
        DataContainerCluster_ReleaseChannel { channel: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ReleaseChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ReleaseChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_ReleaseChannelRef {
        DataContainerCluster_ReleaseChannelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ReleaseChannelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    type O = terrars::BlockAssignable<DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {}

impl BuildDataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
    pub fn build(self) -> DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
        DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination {
            dataset_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
        DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ResourceUsageExportConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination: std
    ::option
    ::Option<terrars::ListField<DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_egress_metering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_consumption_metering: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_ResourceUsageExportConfig {
    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataContainerCluster_ResourceUsageExportConfig_BigqueryDestination>>,
    ) -> Self {
        self.bigquery_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_egress_metering`.\n"]
    pub fn set_enable_network_egress_metering(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_egress_metering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_consumption_metering`.\n"]
    pub fn set_enable_resource_consumption_metering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_consumption_metering = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ResourceUsageExportConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ResourceUsageExportConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ResourceUsageExportConfig {}

impl BuildDataContainerCluster_ResourceUsageExportConfig {
    pub fn build(self) -> DataContainerCluster_ResourceUsageExportConfig {
        DataContainerCluster_ResourceUsageExportConfig {
            bigquery_destination: core::default::Default::default(),
            enable_network_egress_metering: core::default::Default::default(),
            enable_resource_consumption_metering: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ResourceUsageExportConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ResourceUsageExportConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ResourceUsageExportConfigRef {
        DataContainerCluster_ResourceUsageExportConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ResourceUsageExportConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> terrars::ListRef<DataContainerCluster_ResourceUsageExportConfig_BigqueryDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_network_egress_metering` after provisioning.\n"]
    pub fn enable_network_egress_metering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_egress_metering", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_resource_consumption_metering` after provisioning.\n"]
    pub fn enable_resource_consumption_metering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_consumption_metering", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_RollbackSafeUpgrade {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_RollbackSafeUpgrade {
    #[doc = "Set the field `control_plane_soak_duration`.\n"]
    pub fn set_control_plane_soak_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_soak_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_RollbackSafeUpgrade {
    type O = terrars::BlockAssignable<DataContainerCluster_RollbackSafeUpgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_RollbackSafeUpgrade {}

impl BuildDataContainerCluster_RollbackSafeUpgrade {
    pub fn build(self) -> DataContainerCluster_RollbackSafeUpgrade {
        DataContainerCluster_RollbackSafeUpgrade { control_plane_soak_duration: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_RollbackSafeUpgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_RollbackSafeUpgradeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_RollbackSafeUpgradeRef {
        DataContainerCluster_RollbackSafeUpgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_RollbackSafeUpgradeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_soak_duration` after provisioning.\n"]
    pub fn control_plane_soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_soak_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_SecretSyncConfig_RotationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_SecretSyncConfig_RotationConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_interval`.\n"]
    pub fn set_rotation_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_SecretSyncConfig_RotationConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_SecretSyncConfig_RotationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_SecretSyncConfig_RotationConfig {}

impl BuildDataContainerCluster_SecretSyncConfig_RotationConfig {
    pub fn build(self) -> DataContainerCluster_SecretSyncConfig_RotationConfig {
        DataContainerCluster_SecretSyncConfig_RotationConfig {
            enabled: core::default::Default::default(),
            rotation_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_SecretSyncConfig_RotationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_SecretSyncConfig_RotationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_SecretSyncConfig_RotationConfigRef {
        DataContainerCluster_SecretSyncConfig_RotationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_SecretSyncConfig_RotationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_interval` after provisioning.\n"]
    pub fn rotation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_SecretManagerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_config: std::option::Option<terrars::ListField<DataContainerCluster_SecretSyncConfig_RotationConfig>>,
}

impl DataContainerCluster_SecretManagerConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_SecretSyncConfig_RotationConfig>>,
    ) -> Self {
        self.rotation_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_SecretManagerConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_SecretManagerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_SecretManagerConfig {}

impl BuildDataContainerCluster_SecretManagerConfig {
    pub fn build(self) -> DataContainerCluster_SecretManagerConfig {
        DataContainerCluster_SecretManagerConfig {
            enabled: core::default::Default::default(),
            rotation_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_SecretManagerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_SecretManagerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_SecretManagerConfigRef {
        DataContainerCluster_SecretManagerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_SecretManagerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(&self) -> terrars::ListRef<DataContainerCluster_SecretSyncConfig_RotationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_SecretSyncConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_config: std::option::Option<terrars::ListField<DataContainerCluster_SecretSyncConfig_RotationConfig>>,
}

impl DataContainerCluster_SecretSyncConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataContainerCluster_SecretSyncConfig_RotationConfig>>,
    ) -> Self {
        self.rotation_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_SecretSyncConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_SecretSyncConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_SecretSyncConfig {}

impl BuildDataContainerCluster_SecretSyncConfig {
    pub fn build(self) -> DataContainerCluster_SecretSyncConfig {
        DataContainerCluster_SecretSyncConfig {
            enabled: core::default::Default::default(),
            rotation_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_SecretSyncConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_SecretSyncConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_SecretSyncConfigRef {
        DataContainerCluster_SecretSyncConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_SecretSyncConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(&self) -> terrars::ListRef<DataContainerCluster_SecretSyncConfig_RotationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_SecurityPostureConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_SecurityPostureConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vulnerability_mode`.\n"]
    pub fn set_vulnerability_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vulnerability_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_SecurityPostureConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_SecurityPostureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_SecurityPostureConfig {}

impl BuildDataContainerCluster_SecurityPostureConfig {
    pub fn build(self) -> DataContainerCluster_SecurityPostureConfig {
        DataContainerCluster_SecurityPostureConfig {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_SecurityPostureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_SecurityPostureConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_SecurityPostureConfigRef {
        DataContainerCluster_SecurityPostureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_SecurityPostureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `vulnerability_mode` after provisioning.\n"]
    pub fn vulnerability_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_ServiceExternalIpsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_ServiceExternalIpsConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_ServiceExternalIpsConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_ServiceExternalIpsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_ServiceExternalIpsConfig {}

impl BuildDataContainerCluster_ServiceExternalIpsConfig {
    pub fn build(self) -> DataContainerCluster_ServiceExternalIpsConfig {
        DataContainerCluster_ServiceExternalIpsConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_ServiceExternalIpsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_ServiceExternalIpsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_ServiceExternalIpsConfigRef {
        DataContainerCluster_ServiceExternalIpsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_ServiceExternalIpsConfigRef {
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
pub struct DataContainerCluster_UserManagedKeysConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ca: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_disk_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_disk_encryption_key_versions: std
    ::option
    ::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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

impl DataContainerCluster_UserManagedKeysConfig {
    #[doc = "Set the field `aggregation_ca`.\n"]
    pub fn set_aggregation_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_ca`.\n"]
    pub fn set_cluster_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_disk_encryption_key`.\n"]
    pub fn set_control_plane_disk_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_disk_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_disk_encryption_key_versions`.\n"]
    pub fn set_control_plane_disk_encryption_key_versions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.control_plane_disk_encryption_key_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etcd_api_ca`.\n"]
    pub fn set_etcd_api_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etcd_api_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etcd_peer_ca`.\n"]
    pub fn set_etcd_peer_ca(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etcd_peer_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gkeops_etcd_backup_encryption_key`.\n"]
    pub fn set_gkeops_etcd_backup_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gkeops_etcd_backup_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_signing_keys`.\n"]
    pub fn set_service_account_signing_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_signing_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_verification_keys`.\n"]
    pub fn set_service_account_verification_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_verification_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_UserManagedKeysConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_UserManagedKeysConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_UserManagedKeysConfig {}

impl BuildDataContainerCluster_UserManagedKeysConfig {
    pub fn build(self) -> DataContainerCluster_UserManagedKeysConfig {
        DataContainerCluster_UserManagedKeysConfig {
            aggregation_ca: core::default::Default::default(),
            cluster_ca: core::default::Default::default(),
            control_plane_disk_encryption_key: core::default::Default::default(),
            control_plane_disk_encryption_key_versions: core::default::Default::default(),
            etcd_api_ca: core::default::Default::default(),
            etcd_peer_ca: core::default::Default::default(),
            gkeops_etcd_backup_encryption_key: core::default::Default::default(),
            service_account_signing_keys: core::default::Default::default(),
            service_account_verification_keys: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_UserManagedKeysConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_UserManagedKeysConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainerCluster_UserManagedKeysConfigRef {
        DataContainerCluster_UserManagedKeysConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_UserManagedKeysConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_ca` after provisioning.\n"]
    pub fn aggregation_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_ca` after provisioning.\n"]
    pub fn cluster_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_disk_encryption_key` after provisioning.\n"]
    pub fn control_plane_disk_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_disk_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_disk_encryption_key_versions` after provisioning.\n"]
    pub fn control_plane_disk_encryption_key_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.control_plane_disk_encryption_key_versions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `etcd_api_ca` after provisioning.\n"]
    pub fn etcd_api_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_api_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `etcd_peer_ca` after provisioning.\n"]
    pub fn etcd_peer_ca(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_peer_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `gkeops_etcd_backup_encryption_key` after provisioning.\n"]
    pub fn gkeops_etcd_backup_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gkeops_etcd_backup_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_signing_keys` after provisioning.\n"]
    pub fn service_account_signing_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_account_signing_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_verification_keys` after provisioning.\n"]
    pub fn service_account_verification_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_account_verification_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataContainerCluster_VerticalPodAutoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataContainerCluster_VerticalPodAutoscaling {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_VerticalPodAutoscaling {
    type O = terrars::BlockAssignable<DataContainerCluster_VerticalPodAutoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_VerticalPodAutoscaling {}

impl BuildDataContainerCluster_VerticalPodAutoscaling {
    pub fn build(self) -> DataContainerCluster_VerticalPodAutoscaling {
        DataContainerCluster_VerticalPodAutoscaling { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_VerticalPodAutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_VerticalPodAutoscalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_VerticalPodAutoscalingRef {
        DataContainerCluster_VerticalPodAutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_VerticalPodAutoscalingRef {
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
pub struct DataContainerCluster_WorkloadIdentityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataContainerCluster_WorkloadIdentityConfig {
    #[doc = "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataContainerCluster_WorkloadIdentityConfig {
    type O = terrars::BlockAssignable<DataContainerCluster_WorkloadIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainerCluster_WorkloadIdentityConfig {}

impl BuildDataContainerCluster_WorkloadIdentityConfig {
    pub fn build(self) -> DataContainerCluster_WorkloadIdentityConfig {
        DataContainerCluster_WorkloadIdentityConfig { workload_pool: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataContainerCluster_WorkloadIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainerCluster_WorkloadIdentityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataContainerCluster_WorkloadIdentityConfigRef {
        DataContainerCluster_WorkloadIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainerCluster_WorkloadIdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_pool", self.base))
    }
}
