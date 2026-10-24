use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EdgecontainerClusterData {
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
    authorization: std::option::Option<std::vec::Vec<EdgecontainerClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<EdgecontainerClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_encryption: std::option::Option<std::vec::Vec<EdgecontainerClusterControlPlaneEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<EdgecontainerClusterFleetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<EdgecontainerClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking: std::option::Option<std::vec::Vec<EdgecontainerClusterNetworkingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_addons_config: std::option::Option<std::vec::Vec<EdgecontainerClusterSystemAddonsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EdgecontainerClusterTimeoutsEl>,
    dynamic: EdgecontainerClusterDynamic,
}

struct EdgecontainerCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EdgecontainerClusterData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterAuthorizationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterControlPlaneEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterControlPlaneEncryptionEl>>,
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
    pub fn set_fleet(self, v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterFleetEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterNetworkingEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterSystemAddonsConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<EdgecontainerClusterTimeoutsEl>) -> Self {
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
    pub fn maintenance_events(&self) -> terrars::ListRef<EdgecontainerClusterMaintenanceEventsElRef> {
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
    pub fn authorization(&self) -> terrars::ListRef<EdgecontainerClusterAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<EdgecontainerClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<EdgecontainerClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<EdgecontainerClusterNetworkingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> terrars::ListRef<EdgecontainerClusterSystemAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(EdgecontainerClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_max_pods_per_node: core::default::Default::default(),
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
    pub fn maintenance_events(&self) -> terrars::ListRef<EdgecontainerClusterMaintenanceEventsElRef> {
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
    pub fn authorization(&self) -> terrars::ListRef<EdgecontainerClusterAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_encryption` after provisioning.\n"]
    pub fn control_plane_encryption(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<EdgecontainerClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<EdgecontainerClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> terrars::ListRef<EdgecontainerClusterNetworkingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_addons_config` after provisioning.\n"]
    pub fn system_addons_config(&self) -> terrars::ListRef<EdgecontainerClusterSystemAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenanceEventsEl {
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

impl EdgecontainerClusterMaintenanceEventsEl {
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

impl terrars::ToListMappable for EdgecontainerClusterMaintenanceEventsEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterMaintenanceEventsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterMaintenanceEventsEl {}

impl BuildEdgecontainerClusterMaintenanceEventsEl {
    pub fn build(self) -> EdgecontainerClusterMaintenanceEventsEl {
        EdgecontainerClusterMaintenanceEventsEl {
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

pub struct EdgecontainerClusterMaintenanceEventsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterMaintenanceEventsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterMaintenanceEventsElRef {
        EdgecontainerClusterMaintenanceEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterMaintenanceEventsElRef {
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
pub struct EdgecontainerClusterAuthorizationElAdminUsersEl {
    username: terrars::PrimField<std::string::String>,
}

impl EdgecontainerClusterAuthorizationElAdminUsersEl { }

impl terrars::ToListMappable for EdgecontainerClusterAuthorizationElAdminUsersEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterAuthorizationElAdminUsersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterAuthorizationElAdminUsersEl {
    #[doc = "An active Google username."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerClusterAuthorizationElAdminUsersEl {
    pub fn build(self) -> EdgecontainerClusterAuthorizationElAdminUsersEl {
        EdgecontainerClusterAuthorizationElAdminUsersEl { username: self.username }
    }
}

pub struct EdgecontainerClusterAuthorizationElAdminUsersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterAuthorizationElAdminUsersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterAuthorizationElAdminUsersElRef {
        EdgecontainerClusterAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nAn active Google username."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerClusterAuthorizationElDynamic {
    admin_users: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
    dynamic: EdgecontainerClusterAuthorizationElDynamic,
}

impl EdgecontainerClusterAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterAuthorizationElAdminUsersEl>>,
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

impl terrars::ToListMappable for EdgecontainerClusterAuthorizationEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterAuthorizationEl {}

impl BuildEdgecontainerClusterAuthorizationEl {
    pub fn build(self) -> EdgecontainerClusterAuthorizationEl {
        EdgecontainerClusterAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterAuthorizationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterAuthorizationElRef {
        EdgecontainerClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<EdgecontainerClusterAuthorizationElAdminUsersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneElLocalEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_deployment_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterControlPlaneElLocalEl {
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

impl terrars::ToListMappable for EdgecontainerClusterControlPlaneElLocalEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterControlPlaneElLocalEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterControlPlaneElLocalEl {}

impl BuildEdgecontainerClusterControlPlaneElLocalEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneElLocalEl {
        EdgecontainerClusterControlPlaneElLocalEl {
            machine_filter: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_location: core::default::Default::default(),
            shared_deployment_policy: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterControlPlaneElLocalElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterControlPlaneElLocalElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterControlPlaneElLocalElRef {
        EdgecontainerClusterControlPlaneElLocalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterControlPlaneElLocalElRef {
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
pub struct EdgecontainerClusterControlPlaneElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterControlPlaneElRemoteEl {
    #[doc =
        "Set the field `node_location`.\nName of the Google Distributed Cloud Edge zones where this node pool\nwill be created. For example: 'us-central1-edge-customer-a'."]
    pub fn set_node_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerClusterControlPlaneElRemoteEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterControlPlaneElRemoteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterControlPlaneElRemoteEl {}

impl BuildEdgecontainerClusterControlPlaneElRemoteEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneElRemoteEl {
        EdgecontainerClusterControlPlaneElRemoteEl { node_location: core::default::Default::default() }
    }
}

pub struct EdgecontainerClusterControlPlaneElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterControlPlaneElRemoteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterControlPlaneElRemoteElRef {
        EdgecontainerClusterControlPlaneElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterControlPlaneElRemoteElRef {
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
struct EdgecontainerClusterControlPlaneElDynamic {
    local: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterControlPlaneElLocalEl>>,
    remote: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterControlPlaneElRemoteEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local: std::option::Option<std::vec::Vec<EdgecontainerClusterControlPlaneElLocalEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std::option::Option<std::vec::Vec<EdgecontainerClusterControlPlaneElRemoteEl>>,
    dynamic: EdgecontainerClusterControlPlaneElDynamic,
}

impl EdgecontainerClusterControlPlaneEl {
    #[doc = "Set the field `local`.\n"]
    pub fn set_local(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterControlPlaneElLocalEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterControlPlaneElRemoteEl>>,
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

impl terrars::ToListMappable for EdgecontainerClusterControlPlaneEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterControlPlaneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterControlPlaneEl {}

impl BuildEdgecontainerClusterControlPlaneEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEl {
        EdgecontainerClusterControlPlaneEl {
            local: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterControlPlaneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterControlPlaneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterControlPlaneElRef {
        EdgecontainerClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterControlPlaneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local` after provisioning.\n"]
    pub fn local(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneElLocalElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
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

impl terrars::ToListMappable for EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {}

impl BuildEdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
        EdgecontainerClusterControlPlaneEncryptionElKmsStatusEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
        EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef {
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
pub struct EdgecontainerClusterControlPlaneEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterControlPlaneEncryptionEl {
    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS CryptoKey e.g.\nprojects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}\nto use for protecting control plane disks. If not specified, a\nGoogle-managed key will be used instead."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerClusterControlPlaneEncryptionEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterControlPlaneEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterControlPlaneEncryptionEl {}

impl BuildEdgecontainerClusterControlPlaneEncryptionEl {
    pub fn build(self) -> EdgecontainerClusterControlPlaneEncryptionEl {
        EdgecontainerClusterControlPlaneEncryptionEl { kms_key: core::default::Default::default() }
    }
}

pub struct EdgecontainerClusterControlPlaneEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterControlPlaneEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterControlPlaneEncryptionElRef {
        EdgecontainerClusterControlPlaneEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterControlPlaneEncryptionElRef {
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
    pub fn kms_status(&self) -> terrars::ListRef<EdgecontainerClusterControlPlaneEncryptionElKmsStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_status", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterFleetEl {
    project: terrars::PrimField<std::string::String>,
}

impl EdgecontainerClusterFleetEl { }

impl terrars::ToListMappable for EdgecontainerClusterFleetEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterFleetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterFleetEl {
    #[doc =
        "The name of the Fleet host project where this cluster will be registered.\nProject names are formatted as\n'projects/<project-number>'."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerClusterFleetEl {
    pub fn build(self) -> EdgecontainerClusterFleetEl {
        EdgecontainerClusterFleetEl { project: self.project }
    }
}

pub struct EdgecontainerClusterFleetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterFleetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterFleetElRef {
        EdgecontainerClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterFleetElRef {
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
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
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

impl terrars::ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {}

impl BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef {
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
struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElDynamic {
    window: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<std::vec::Vec<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElDynamic,
}

impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
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
                        terrars
                        ::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowEl>,
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

impl terrars::ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {}

impl BuildEdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl {
            recurrence: core::default::Default::default(),
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recurrence` after provisioning.\nAn RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how\nthis window recurs. They go on for the span of time between the start and\nend time."]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(
        &self,
    ) -> terrars::ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElWindowElDynamic {
    recurring_window: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyElWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_window: std
    ::option
    ::Option<std::vec::Vec<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElWindowElDynamic,
}

impl EdgecontainerClusterMaintenancePolicyElWindowEl {
    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowEl>,
                    >,
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

impl terrars::ToListMappable for EdgecontainerClusterMaintenancePolicyElWindowEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterMaintenancePolicyElWindowEl {}

impl BuildEdgecontainerClusterMaintenancePolicyElWindowEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyElWindowEl {
        EdgecontainerClusterMaintenancePolicyElWindowEl {
            recurring_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterMaintenancePolicyElWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterMaintenancePolicyElWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterMaintenancePolicyElWindowElRef {
        EdgecontainerClusterMaintenancePolicyElWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterMaintenancePolicyElWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(
        &self,
    ) -> terrars::ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRecurringWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_window", self.base))
    }
}

#[derive(Serialize, Default)]
struct EdgecontainerClusterMaintenancePolicyElDynamic {
    window: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<std::vec::Vec<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
    dynamic: EdgecontainerClusterMaintenancePolicyElDynamic,
}

impl EdgecontainerClusterMaintenancePolicyEl {
    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyElWindowEl>>,
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

impl terrars::ToListMappable for EdgecontainerClusterMaintenancePolicyEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterMaintenancePolicyEl {}

impl BuildEdgecontainerClusterMaintenancePolicyEl {
    pub fn build(self) -> EdgecontainerClusterMaintenancePolicyEl {
        EdgecontainerClusterMaintenancePolicyEl {
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterMaintenancePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterMaintenancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterMaintenancePolicyElRef {
        EdgecontainerClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterMaintenancePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::ListRef<EdgecontainerClusterMaintenancePolicyElWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterNetworkingEl {
    cluster_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    services_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services_ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl EdgecontainerClusterNetworkingEl {
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

impl terrars::ToListMappable for EdgecontainerClusterNetworkingEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterNetworkingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterNetworkingEl {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub cluster_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these\nblocks. Only a single block is supported. This field cannot be changed\nafter creation."]
    pub services_ipv4_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildEdgecontainerClusterNetworkingEl {
    pub fn build(self) -> EdgecontainerClusterNetworkingEl {
        EdgecontainerClusterNetworkingEl {
            cluster_ipv4_cidr_blocks: self.cluster_ipv4_cidr_blocks,
            cluster_ipv6_cidr_blocks: core::default::Default::default(),
            services_ipv4_cidr_blocks: self.services_ipv4_cidr_blocks,
            services_ipv6_cidr_blocks: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterNetworkingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterNetworkingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterNetworkingElRef {
        EdgecontainerClusterNetworkingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterNetworkingElRef {
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
pub struct EdgecontainerClusterSystemAddonsConfigElIngressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_vip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterSystemAddonsConfigElIngressEl {
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

impl terrars::ToListMappable for EdgecontainerClusterSystemAddonsConfigElIngressEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterSystemAddonsConfigElIngressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterSystemAddonsConfigElIngressEl {}

impl BuildEdgecontainerClusterSystemAddonsConfigElIngressEl {
    pub fn build(self) -> EdgecontainerClusterSystemAddonsConfigElIngressEl {
        EdgecontainerClusterSystemAddonsConfigElIngressEl {
            disabled: core::default::Default::default(),
            ipv4_vip: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterSystemAddonsConfigElIngressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterSystemAddonsConfigElIngressElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EdgecontainerClusterSystemAddonsConfigElIngressElRef {
        EdgecontainerClusterSystemAddonsConfigElIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterSystemAddonsConfigElIngressElRef {
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
struct EdgecontainerClusterSystemAddonsConfigElDynamic {
    ingress: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
}

#[derive(Serialize)]
pub struct EdgecontainerClusterSystemAddonsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
    dynamic: EdgecontainerClusterSystemAddonsConfigElDynamic,
}

impl EdgecontainerClusterSystemAddonsConfigEl {
    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerClusterSystemAddonsConfigElIngressEl>>,
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

impl terrars::ToListMappable for EdgecontainerClusterSystemAddonsConfigEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterSystemAddonsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterSystemAddonsConfigEl {}

impl BuildEdgecontainerClusterSystemAddonsConfigEl {
    pub fn build(self) -> EdgecontainerClusterSystemAddonsConfigEl {
        EdgecontainerClusterSystemAddonsConfigEl {
            ingress: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterSystemAddonsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterSystemAddonsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterSystemAddonsConfigElRef {
        EdgecontainerClusterSystemAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterSystemAddonsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<EdgecontainerClusterSystemAddonsConfigElIngressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerClusterTimeoutsEl {
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

impl terrars::ToListMappable for EdgecontainerClusterTimeoutsEl {
    type O = terrars::BlockAssignable<EdgecontainerClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerClusterTimeoutsEl {}

impl BuildEdgecontainerClusterTimeoutsEl {
    pub fn build(self) -> EdgecontainerClusterTimeoutsEl {
        EdgecontainerClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerClusterTimeoutsElRef {
        EdgecontainerClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerClusterTimeoutsElRef {
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
struct EdgecontainerClusterDynamic {
    authorization: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterAuthorizationEl>>,
    control_plane: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterControlPlaneEl>>,
    control_plane_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<EdgecontainerClusterControlPlaneEncryptionEl>>,
    fleet: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterFleetEl>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterMaintenancePolicyEl>>,
    networking: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterNetworkingEl>>,
    system_addons_config: std::option::Option<terrars::DynamicBlock<EdgecontainerClusterSystemAddonsConfigEl>>,
}
