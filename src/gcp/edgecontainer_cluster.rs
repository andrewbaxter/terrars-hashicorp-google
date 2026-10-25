#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EdgecontainerClusterData_ {
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
    default_max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_load_balancer_ipv4_address_pools: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<EdgecontainerCluster_Authorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<EdgecontainerCluster_ControlPlane>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_encryption: std::option::Option<std::vec::Vec<EdgecontainerCluster_ControlPlaneEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<EdgecontainerCluster_Fleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking: std::option::Option<std::vec::Vec<EdgecontainerCluster_Networking>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_addons_config: std::option::Option<std::vec::Vec<EdgecontainerCluster_SystemAddonsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EdgecontainerCluster_Timeouts>,
    dynamic: EdgecontainerClusterDynamic,
}

struct EdgecontainerCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EdgecontainerClusterData_>,
}

#[derive(Clone)]
pub struct EdgecontainerCluster(std::rc::Rc<EdgecontainerCluster_>);

impl EdgecontainerCluster {
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
        "Set the field `default_max_pods_per_node`.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
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
        "Set the field `external_load_balancer_ipv4_address_pools`.\nAddress pools for cluster data plane external load balancing."]
    pub fn set_external_load_balancer_ipv4_address_pools(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().external_load_balancer_ipv4_address_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `release_channel`.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn set_release_channel(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().release_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_version`.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn set_target_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_Authorization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_ControlPlane>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane_encryption`.\n"]
    pub fn set_control_plane_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_ControlPlaneEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_Fleet>>) -> Self {
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

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy>>,
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

    #[doc = "Set the field `networking`.\n"]
    pub fn set_networking(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_Networking>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networking = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networking = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `system_addons_config`.\n"]
    pub fn set_system_addons_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_SystemAddonsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().system_addons_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.system_addons_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EdgecontainerCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\nThe PEM-encoded public certificate of the cluster's CA."]
    pub fn cluster_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_version` after provisioning.\nThe control plane release version."]
    pub fn control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of the Kubernetes API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_load_balancer_ipv4_address_pools` after provisioning.\nAddress pools for cluster data plane external load balancing."]
    pub fn external_load_balancer_ipv4_address_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_load_balancer_ipv4_address_pools", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_events` after provisioning.\nAll the maintenance events scheduled for the cluster, including the ones\nongoing, planned for the future and done in the past (up to 90 days)."]
    pub fn maintenance_events(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenanceEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe GDCE cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes. This field can be empty\nif the cluster does not have any worker nodes."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number of the Kubernetes API server."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn release_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<EdgecontainerCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlaneEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<EdgecontainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<EdgecontainerCluster_NetworkingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> terrars::ListRef<EdgecontainerCluster_SystemAddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerCluster_TimeoutsRef {
        EdgecontainerCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EdgecontainerCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EdgecontainerCluster { }

impl terrars::ToListMappable for EdgecontainerCluster {
    type O = terrars::ListRef<EdgecontainerClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EdgecontainerCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_edgecontainer_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEdgecontainerCluster {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The GDCE cluster name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EdgecontainerCluster {
        let out = EdgecontainerCluster(std::rc::Rc::new(EdgecontainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EdgecontainerClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_max_pods_per_node: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                external_load_balancer_ipv4_address_pools: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                target_version: core::default::Default::default(),
                authorization: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                control_plane_encryption: core::default::Default::default(),
                fleet: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                networking: core::default::Default::default(),
                system_addons_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EdgecontainerClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EdgecontainerClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\nThe PEM-encoded public certificate of the cluster's CA."]
    pub fn cluster_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_version` after provisioning.\nThe control plane release version."]
    pub fn control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node used if a maximum value is not\nspecified explicitly for a node pool in this cluster. If unspecified, the\nKubernetes default value will be used."]
    pub fn default_max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_max_pods_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of the Kubernetes API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_load_balancer_ipv4_address_pools` after provisioning.\nAddress pools for cluster data plane external load balancing."]
    pub fn external_load_balancer_ipv4_address_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_load_balancer_ipv4_address_pools", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the edgecloud cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_events` after provisioning.\nAll the maintenance events scheduled for the cluster, including the ones\nongoing, planned for the future and done in the past (up to 90 days)."]
    pub fn maintenance_events(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenanceEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe GDCE cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes. This field can be empty\nif the cluster does not have any worker nodes."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number of the Kubernetes API server."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel a cluster is subscribed to. Possible values: [\"RELEASE_CHANNEL_UNSPECIFIED\", \"NONE\", \"REGULAR\"]"]
    pub fn release_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe target cluster version. For example: \"1.5.0\"."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<EdgecontainerCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlaneEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<EdgecontainerCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<EdgecontainerCluster_NetworkingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> terrars::ListRef<EdgecontainerCluster_SystemAddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerCluster_TimeoutsRef {
        EdgecontainerCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerClusterDynamic {
    authorization: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_Authorization>>,
    control_plane: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_ControlPlane>>,
    control_plane_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerCluster_ControlPlaneEncryption>>,
    fleet: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_Fleet>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy>>,
    networking: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_Networking>>,
    system_addons_config: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_SystemAddonsConfig>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenanceEvents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_MaintenanceEvents {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_MaintenanceEvents {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenanceEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenanceEvents {}

impl BuildEdgecontainerCluster_MaintenanceEvents {
    pub fn build(self) -> EdgecontainerCluster_MaintenanceEvents {
        EdgecontainerCluster_MaintenanceEvents {
            create_time: core::default::Default::default(),
            end_time: core::default::Default::default(),
            operation: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenanceEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenanceEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_MaintenanceEventsRef {
        EdgecontainerCluster_MaintenanceEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenanceEventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_Authorization_AdminUsers {
    username: terrars::PrimField<std::string::String>,
}

impl EdgecontainerCluster_Authorization_AdminUsers { }

impl terrars::ToListMappable for EdgecontainerCluster_Authorization_AdminUsers {
    type O = terrars::BlockAssignable<EdgecontainerCluster_Authorization_AdminUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_Authorization_AdminUsers {
    #[doc = "An active Google username."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerCluster_Authorization_AdminUsers {
    pub fn build(self) -> EdgecontainerCluster_Authorization_AdminUsers {
        EdgecontainerCluster_Authorization_AdminUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_Authorization_AdminUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_Authorization_AdminUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_Authorization_AdminUsersRef {
        EdgecontainerCluster_Authorization_AdminUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_Authorization_AdminUsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nAn active Google username."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_AuthorizationDynamic {
    admin_users: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_Authorization_AdminUsers>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<EdgecontainerCluster_Authorization_AdminUsers>>,
    dynamic: EdgecontainerCluster_AuthorizationDynamic,
}

impl EdgecontainerCluster_Authorization {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_Authorization_AdminUsers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.admin_users = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_users = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_Authorization {
    type O = terrars::BlockAssignable<EdgecontainerCluster_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_Authorization {}

impl BuildEdgecontainerCluster_Authorization {
    pub fn build(self) -> EdgecontainerCluster_Authorization {
        EdgecontainerCluster_Authorization {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_AuthorizationRef {
        EdgecontainerCluster_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<EdgecontainerCluster_Authorization_AdminUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_ControlPlane_Local {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_deployment_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_ControlPlane_Local {
    #[doc =
        "Set the field `machine_filter`.\nOnly machines matching this filter will be allowed to host control\nplane nodes. The filtering language accepts strings like \"name=<name>\",\nand is documented here: [AIP-160](https://google.aip.dev/160)."]
    pub fn set_machine_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_count`.\nThe number of nodes to serve as replicas of the Control Plane.\nOnly 1 and 3 are supported."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_location`.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn set_node_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_deployment_policy`.\nPolicy configuration about how user applications are deployed. Possible values: [\"SHARED_DEPLOYMENT_POLICY_UNSPECIFIED\", \"ALLOWED\", \"DISALLOWED\"]"]
    pub fn set_shared_deployment_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.shared_deployment_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_ControlPlane_Local {
    type O = terrars::BlockAssignable<EdgecontainerCluster_ControlPlane_Local>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_ControlPlane_Local {}

impl BuildEdgecontainerCluster_ControlPlane_Local {
    pub fn build(self) -> EdgecontainerCluster_ControlPlane_Local {
        EdgecontainerCluster_ControlPlane_Local {
            machine_filter: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_location: core::default::Default::default(),
            shared_deployment_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_ControlPlane_LocalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_ControlPlane_LocalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_ControlPlane_LocalRef {
        EdgecontainerCluster_ControlPlane_LocalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_ControlPlane_LocalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_filter` after provisioning.\nOnly machines matching this filter will be allowed to host control\nplane nodes. The filtering language accepts strings like \"name=<name>\",\nand is documented here: [AIP-160](https://google.aip.dev/160)."]
    pub fn machine_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes to serve as replicas of the Control Plane.\nOnly 1 and 3 are supported."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shared_deployment_policy` after provisioning.\nPolicy configuration about how user applications are deployed. Possible values: [\"SHARED_DEPLOYMENT_POLICY_UNSPECIFIED\", \"ALLOWED\", \"DISALLOWED\"]"]
    pub fn shared_deployment_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_deployment_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_ControlPlane_Remote {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_ControlPlane_Remote {
    #[doc =
        "Set the field `node_location`.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn set_node_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_ControlPlane_Remote {
    type O = terrars::BlockAssignable<EdgecontainerCluster_ControlPlane_Remote>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_ControlPlane_Remote {}

impl BuildEdgecontainerCluster_ControlPlane_Remote {
    pub fn build(self) -> EdgecontainerCluster_ControlPlane_Remote {
        EdgecontainerCluster_ControlPlane_Remote { node_location: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_ControlPlane_RemoteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_ControlPlane_RemoteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_ControlPlane_RemoteRef {
        EdgecontainerCluster_ControlPlane_RemoteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_ControlPlane_RemoteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_ControlPlaneDynamic {
    local: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_ControlPlane_Local>>,
    remote: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_ControlPlane_Remote>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_ControlPlane {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local: std::option::Option<std::vec::Vec<EdgecontainerCluster_ControlPlane_Local>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std::option::Option<std::vec::Vec<EdgecontainerCluster_ControlPlane_Remote>>,
    dynamic: EdgecontainerCluster_ControlPlaneDynamic,
}

impl EdgecontainerCluster_ControlPlane {
    #[doc = "Set the field `local`.\n"]
    pub fn set_local(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_ControlPlane_Local>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_ControlPlane_Remote>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_ControlPlane {
    type O = terrars::BlockAssignable<EdgecontainerCluster_ControlPlane>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_ControlPlane {}

impl BuildEdgecontainerCluster_ControlPlane {
    pub fn build(self) -> EdgecontainerCluster_ControlPlane {
        EdgecontainerCluster_ControlPlane {
            local: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_ControlPlaneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_ControlPlaneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_ControlPlaneRef {
        EdgecontainerCluster_ControlPlaneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_ControlPlaneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local` after provisioning.\n"]
    pub fn local(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlane_LocalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlane_RemoteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
    type O = terrars::BlockAssignable<EdgecontainerCluster_ControlPlaneEncryption_KmsStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_ControlPlaneEncryption_KmsStatus {}

impl BuildEdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
    pub fn build(self) -> EdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
        EdgecontainerCluster_ControlPlaneEncryption_KmsStatus {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef {
        EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_ControlPlaneEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_ControlPlaneEncryption {
    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS CryptoKey e.g.\nprojects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}\nto use for protecting control plane disks. If not specified, a\nGoogle-managed key will be used instead."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_ControlPlaneEncryption {
    type O = terrars::BlockAssignable<EdgecontainerCluster_ControlPlaneEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_ControlPlaneEncryption {}

impl BuildEdgecontainerCluster_ControlPlaneEncryption {
    pub fn build(self) -> EdgecontainerCluster_ControlPlaneEncryption {
        EdgecontainerCluster_ControlPlaneEncryption { kms_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_ControlPlaneEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_ControlPlaneEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_ControlPlaneEncryptionRef {
        EdgecontainerCluster_ControlPlaneEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_ControlPlaneEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS CryptoKey e.g.\nprojects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}\nto use for protecting control plane disks. If not specified, a\nGoogle-managed key will be used instead."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_active_version` after provisioning.\nThe Cloud KMS CryptoKeyVersion currently in use for protecting control\nplane disks. Only applicable if kms_key is set."]
    pub fn kms_key_active_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_active_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_state` after provisioning.\nAvailability of the Cloud KMS CryptoKey. If not 'KEY_AVAILABLE', then\nnodes may go offline as they cannot access their local data. This can be\ncaused by a lack of permissions to use the key, or if the key is disabled\nor deleted."]
    pub fn kms_key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_status` after provisioning.\nError status returned by Cloud KMS when using this key. This field may be\npopulated only if 'kms_key_state' is not 'KMS_KEY_STATE_KEY_AVAILABLE'.\nIf populated, this field contains the error status reported by Cloud KMS."]
    pub fn kms_status(&self) -> terrars::ListRef<EdgecontainerCluster_ControlPlaneEncryption_KmsStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_status", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_Fleet {
    project: terrars::PrimField<std::string::String>,
}

impl EdgecontainerCluster_Fleet { }

impl terrars::ToListMappable for EdgecontainerCluster_Fleet {
    type O = terrars::BlockAssignable<EdgecontainerCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_Fleet {
    #[doc =
        "The name of the Fleet host project where this cluster will be registered.\nProject names are formatted as\n'projects/<project-number>'."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerCluster_Fleet {
    pub fn build(self) -> EdgecontainerCluster_Fleet {
        EdgecontainerCluster_Fleet { project: self.project }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_FleetRef {
        EdgecontainerCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this cluster.\nMembership names are formatted as\n'projects/<project-number>/locations/global/membership/<cluster-id>'."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe name of the Fleet host project where this cluster will be registered.\nProject names are formatted as\n'projects/<project-number>'."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
    #[doc =
        "Set the field `end_time`.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\nThe time that the window first starts."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {}

impl BuildEdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
    pub fn build(self) -> EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
        EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef {
        EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nThe time that the window ends. The end time must take place after the\nstart time."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe time that the window first starts."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsDynamic {
    window: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>>,
    dynamic: EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsDynamic,
}

impl EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
    #[doc = "Set the field `id`.\nA unique (per cluster) id for the window."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {}

impl BuildEdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
    pub fn build(self) -> EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
        EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions {
            id: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef {
        EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA unique (per cluster) id for the window."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowDynamic {
    window: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>>,
    dynamic: EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowDynamic,
}

impl EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
    #[doc =
        "Set the field `recurrence`.\nAn RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how\nthis window recurs. They go on for the span of time between the start and\nend time."]
    pub fn set_recurrence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recurrence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_Window>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {}

impl BuildEdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
    pub fn build(self) -> EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
        EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow {
            recurrence: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef {
        EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recurrence` after provisioning.\nAn RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how\nthis window recurs. They go on for the span of time between the start and\nend time."]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions_WindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_MaintenancePolicy_WindowDynamic {
    recurring_window: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenancePolicy_Window {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_window: std::option::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow>>,
    dynamic: EdgecontainerCluster_MaintenancePolicy_WindowDynamic,
}

impl EdgecontainerCluster_MaintenancePolicy_Window {
    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindow>>,
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

impl terrars::ToListMappable for EdgecontainerCluster_MaintenancePolicy_Window {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_Window>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenancePolicy_Window {}

impl BuildEdgecontainerCluster_MaintenancePolicy_Window {
    pub fn build(self) -> EdgecontainerCluster_MaintenancePolicy_Window {
        EdgecontainerCluster_MaintenancePolicy_Window {
            recurring_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenancePolicy_WindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenancePolicy_WindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_MaintenancePolicy_WindowRef {
        EdgecontainerCluster_MaintenancePolicy_WindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenancePolicy_WindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicy_Window_RecurringWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_MaintenancePolicyDynamic {
    maintenance_exclusions: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions>>,
    window: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_MaintenancePolicy_Window>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusions: std
    ::option
    ::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<std::vec::Vec<EdgecontainerCluster_MaintenancePolicy_Window>>,
    dynamic: EdgecontainerCluster_MaintenancePolicyDynamic,
}

impl EdgecontainerCluster_MaintenancePolicy {
    #[doc = "Set the field `maintenance_exclusions`.\n"]
    pub fn set_maintenance_exclusions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_exclusions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_exclusions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy_Window>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<EdgecontainerCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_MaintenancePolicy {}

impl BuildEdgecontainerCluster_MaintenancePolicy {
    pub fn build(self) -> EdgecontainerCluster_MaintenancePolicy {
        EdgecontainerCluster_MaintenancePolicy {
            maintenance_exclusions: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_MaintenancePolicyRef {
        EdgecontainerCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_exclusions` after provisioning.\n"]
    pub fn maintenance_exclusions(
        &self,
    ) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicy_MaintenanceExclusionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::ListRef<EdgecontainerCluster_MaintenancePolicy_WindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_Networking {
    cluster_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    services_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl EdgecontainerCluster_Networking {
    #[doc =
        "Set the field `cluster_ipv6_cidr_blocks`.\nIf specified, dual stack mode is enabled and all pods in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn set_cluster_ipv6_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cluster_ipv6_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services_ipv6_cidr_blocks`.\nIf specified, dual stack mode is enabled and all services in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn set_services_ipv6_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.services_ipv6_cidr_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_Networking {
    type O = terrars::BlockAssignable<EdgecontainerCluster_Networking>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_Networking {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub cluster_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub services_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildEdgecontainerCluster_Networking {
    pub fn build(self) -> EdgecontainerCluster_Networking {
        EdgecontainerCluster_Networking {
            cluster_ipv4_cidr_blocks: self.cluster_ipv4_cidr_blocks,
            cluster_ipv6_cidr_blocks: core::default::Default::default(),
            services_ipv4_cidr_blocks: self.services_ipv4_cidr_blocks,
            services_ipv6_cidr_blocks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_NetworkingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_NetworkingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_NetworkingRef {
        EdgecontainerCluster_NetworkingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_NetworkingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv4_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn cluster_ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_ipv4_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_ipv6_cidr_blocks` after provisioning.\nIf specified, dual stack mode is enabled and all pods in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn cluster_ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_ipv6_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_type` after provisioning.\nIP addressing type of this cluster i.e. SINGLESTACK_V4 vs DUALSTACK_V4_V6."]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv4_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn services_ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services_ipv4_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services_ipv6_cidr_blocks` after provisioning.\nIf specified, dual stack mode is enabled and all services in the cluster are\nassigned an IPv6 address from these blocks alongside from an IPv4\naddress. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub fn services_ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services_ipv6_cidr_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_SystemAddonsConfig_Ingress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_vip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_SystemAddonsConfig_Ingress {
    #[doc = "Set the field `disabled`.\nWhether Ingress is disabled."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_vip`.\nIngress VIP."]
    pub fn set_ipv4_vip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_vip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_SystemAddonsConfig_Ingress {
    type O = terrars::BlockAssignable<EdgecontainerCluster_SystemAddonsConfig_Ingress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_SystemAddonsConfig_Ingress {}

impl BuildEdgecontainerCluster_SystemAddonsConfig_Ingress {
    pub fn build(self) -> EdgecontainerCluster_SystemAddonsConfig_Ingress {
        EdgecontainerCluster_SystemAddonsConfig_Ingress {
            disabled: core::default::Default::default(),
            ipv4_vip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_SystemAddonsConfig_IngressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_SystemAddonsConfig_IngressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerCluster_SystemAddonsConfig_IngressRef {
        EdgecontainerCluster_SystemAddonsConfig_IngressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_SystemAddonsConfig_IngressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether Ingress is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_vip` after provisioning.\nIngress VIP."]
    pub fn ipv4_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_vip", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerCluster_SystemAddonsConfigDynamic {
    ingress: std::option::Option<terrars::DynamicBlock<EdgecontainerCluster_SystemAddonsConfig_Ingress>>,
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_SystemAddonsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<EdgecontainerCluster_SystemAddonsConfig_Ingress>>,
    dynamic: EdgecontainerCluster_SystemAddonsConfigDynamic,
}

impl EdgecontainerCluster_SystemAddonsConfig {
    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerCluster_SystemAddonsConfig_Ingress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EdgecontainerCluster_SystemAddonsConfig {
    type O = terrars::BlockAssignable<EdgecontainerCluster_SystemAddonsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_SystemAddonsConfig {}

impl BuildEdgecontainerCluster_SystemAddonsConfig {
    pub fn build(self) -> EdgecontainerCluster_SystemAddonsConfig {
        EdgecontainerCluster_SystemAddonsConfig {
            ingress: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_SystemAddonsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_SystemAddonsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_SystemAddonsConfigRef {
        EdgecontainerCluster_SystemAddonsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_SystemAddonsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<EdgecontainerCluster_SystemAddonsConfig_IngressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerCluster_Timeouts {
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

impl terrars::ToListMappable for EdgecontainerCluster_Timeouts {
    type O = terrars::BlockAssignable<EdgecontainerCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerCluster_Timeouts {}

impl BuildEdgecontainerCluster_Timeouts {
    pub fn build(self) -> EdgecontainerCluster_Timeouts {
        EdgecontainerCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EdgecontainerCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerCluster_TimeoutsRef {
        EdgecontainerCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerCluster_TimeoutsRef {
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
