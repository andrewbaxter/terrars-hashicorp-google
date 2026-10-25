#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremVmwareAdminClusterData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_cluster_membership: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_advanced_cluster: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_prem_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addon_node: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_AddonNode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anti_affinity_groups: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_AntiAffinityGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_Authorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_AutoRepairConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_ControlPlaneNode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_LoadBalancer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_PlatformConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_registry_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_PrivateRegistryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_Proxy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremVmwareAdminCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcenter: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_Vcenter>>,
    dynamic: GkeonpremVmwareAdminClusterDynamic,
}

struct GkeonpremVmwareAdminCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremVmwareAdminClusterData_>,
}

#[derive(Clone)]
pub struct GkeonpremVmwareAdminCluster(std::rc::Rc<GkeonpremVmwareAdminCluster_>);

impl GkeonpremVmwareAdminCluster {
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
        "Set the field `annotations`.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bootstrap_cluster_membership`.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn set_bootstrap_cluster_membership(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().bootstrap_cluster_membership = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human readable description of this VMware admin cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_advanced_cluster`.\nIf set, the advanced cluster feature is enabled."]
    pub fn set_enable_advanced_cluster(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_advanced_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_type`.\nThe OS image type for the VMware admin cluster."]
    pub fn set_image_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_prem_version`.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn set_on_prem_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().on_prem_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `addon_node`.\n"]
    pub fn set_addon_node(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AddonNode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addon_node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addon_node = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `anti_affinity_groups`.\n"]
    pub fn set_anti_affinity_groups(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AntiAffinityGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anti_affinity_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anti_affinity_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Authorization>>,
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

    #[doc = "Set the field `auto_repair_config`.\n"]
    pub fn set_auto_repair_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AutoRepairConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_repair_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_repair_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane_node`.\n"]
    pub fn set_control_plane_node(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_ControlPlaneNode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_node = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig>>,
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

    #[doc = "Set the field `platform_config`.\n"]
    pub fn set_platform_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_PlatformConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().platform_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.platform_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_registry_config`.\n"]
    pub fn set_private_registry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_PrivateRegistryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_registry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_registry_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Proxy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremVmwareAdminCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcenter`.\n"]
    pub fn set_vcenter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Vcenter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vcenter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vcenter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_cluster_membership` after provisioning.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn bootstrap_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware admin cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nIf set, the advanced cluster feature is enabled."]
    pub fn enable_advanced_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_advanced_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware admin cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image_type` after provisioning.\nThe OS image type for the VMware admin cluster."]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMwareAdminCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware admin cluster resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware admin cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe lifecycle state of the VMware admin cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware Admin Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addon_node` after provisioning.\n"]
    pub fn addon_node(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AddonNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addon_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anti_affinity_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AutoRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_ControlPlaneNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_config` after provisioning.\n"]
    pub fn platform_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_PlatformConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_registry_config` after provisioning.\n"]
    pub fn private_registry_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareAdminCluster_TimeoutsRef {
        GkeonpremVmwareAdminCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

impl terrars::Referable for GkeonpremVmwareAdminCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeonpremVmwareAdminCluster { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster {
    type O = terrars::ListRef<GkeonpremVmwareAdminClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeonpremVmwareAdminCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gkeonprem_vmware_admin_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeonpremVmwareAdminCluster {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The VMware admin cluster resource name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeonpremVmwareAdminCluster {
        let out = GkeonpremVmwareAdminCluster(std::rc::Rc::new(GkeonpremVmwareAdminCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeonpremVmwareAdminClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                bootstrap_cluster_membership: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_advanced_cluster: core::default::Default::default(),
                id: core::default::Default::default(),
                image_type: core::default::Default::default(),
                location: self.location,
                name: self.name,
                on_prem_version: core::default::Default::default(),
                project: core::default::Default::default(),
                addon_node: core::default::Default::default(),
                anti_affinity_groups: core::default::Default::default(),
                authorization: core::default::Default::default(),
                auto_repair_config: core::default::Default::default(),
                control_plane_node: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_config: core::default::Default::default(),
                platform_config: core::default::Default::default(),
                private_registry_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vcenter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeonpremVmwareAdminClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeonpremVmwareAdminClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_cluster_membership` after provisioning.\nThe bootstrap cluster this VMware admin cluster belongs to."]
    pub fn bootstrap_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware admin cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nIf set, the advanced cluster feature is enabled."]
    pub fn enable_advanced_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_advanced_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware admin cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image_type` after provisioning.\nThe OS image type for the VMware admin cluster."]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMwareAdminCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware admin cluster resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for the admin cluster."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware admin cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe lifecycle state of the VMware admin cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware Admin Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addon_node` after provisioning.\n"]
    pub fn addon_node(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AddonNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addon_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anti_affinity_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AutoRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_ControlPlaneNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_config` after provisioning.\n"]
    pub fn platform_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_PlatformConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_registry_config` after provisioning.\n"]
    pub fn private_registry_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareAdminCluster_TimeoutsRef {
        GkeonpremVmwareAdminCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminClusterDynamic {
    addon_node: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_AddonNode>>,
    anti_affinity_groups: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_AntiAffinityGroups>>,
    authorization: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_Authorization>>,
    auto_repair_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_AutoRepairConfig>>,
    control_plane_node: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_ControlPlaneNode>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_LoadBalancer>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig>>,
    platform_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_PlatformConfig>>,
    private_registry_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_PrivateRegistryConfig>>,
    proxy: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_Proxy>>,
    vcenter: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_Vcenter>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_Fleet {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Fleet {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Fleet {}

impl BuildGkeonpremVmwareAdminCluster_Fleet {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Fleet {
        GkeonpremVmwareAdminCluster_Fleet { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_FleetRef {
        GkeonpremVmwareAdminCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Status_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_transition_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_Status_Conditions {
    #[doc = "Set the field `last_transition_time`.\n"]
    pub fn set_last_transition_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_transition_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Status_Conditions {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Status_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Status_Conditions {}

impl BuildGkeonpremVmwareAdminCluster_Status_Conditions {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Status_Conditions {
        GkeonpremVmwareAdminCluster_Status_Conditions {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_Status_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_Status_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_Status_ConditionsRef {
        GkeonpremVmwareAdminCluster_Status_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_Status_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_transition_time` after provisioning.\n"]
    pub fn last_transition_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_transition_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremVmwareAdminCluster_Status_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_Status {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareAdminCluster_Status_Conditions>>,
    ) -> Self {
        self.conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Status {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Status {}

impl BuildGkeonpremVmwareAdminCluster_Status {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Status {
        GkeonpremVmwareAdminCluster_Status {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_StatusRef {
        GkeonpremVmwareAdminCluster_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_Status_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig {
    #[doc = "Whether to enable controle plane node auto resizing."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig {
        GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef {
        GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable controle plane node auto resizing."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_AddonNodeDynamic {
    auto_resize_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_AddonNode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_resize_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig>>,
    dynamic: GkeonpremVmwareAdminCluster_AddonNodeDynamic,
}

impl GkeonpremVmwareAdminCluster_AddonNode {
    #[doc = "Set the field `auto_resize_config`.\n"]
    pub fn set_auto_resize_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_resize_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_resize_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_AddonNode {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AddonNode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_AddonNode {}

impl BuildGkeonpremVmwareAdminCluster_AddonNode {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_AddonNode {
        GkeonpremVmwareAdminCluster_AddonNode {
            auto_resize_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_AddonNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_AddonNodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_AddonNodeRef {
        GkeonpremVmwareAdminCluster_AddonNodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_AddonNodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_resize_config` after provisioning.\n"]
    pub fn auto_resize_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_AddonNode_AutoResizeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_resize_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_AntiAffinityGroups {
    aag_config_disabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareAdminCluster_AntiAffinityGroups { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_AntiAffinityGroups {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AntiAffinityGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_AntiAffinityGroups {
    #[doc = "Spread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub aag_config_disabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareAdminCluster_AntiAffinityGroups {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_AntiAffinityGroups {
        GkeonpremVmwareAdminCluster_AntiAffinityGroups { aag_config_disabled: self.aag_config_disabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef {
        GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_AntiAffinityGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aag_config_disabled` after provisioning.\nSpread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub fn aag_config_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aag_config_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Authorization_ViewerUsers {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareAdminCluster_Authorization_ViewerUsers { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Authorization_ViewerUsers {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Authorization_ViewerUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Authorization_ViewerUsers {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_Authorization_ViewerUsers {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Authorization_ViewerUsers {
        GkeonpremVmwareAdminCluster_Authorization_ViewerUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef {
        GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_AuthorizationDynamic {
    viewer_users: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_Authorization_ViewerUsers>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    viewer_users: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_Authorization_ViewerUsers>>,
    dynamic: GkeonpremVmwareAdminCluster_AuthorizationDynamic,
}

impl GkeonpremVmwareAdminCluster_Authorization {
    #[doc = "Set the field `viewer_users`.\n"]
    pub fn set_viewer_users(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Authorization_ViewerUsers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.viewer_users = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.viewer_users = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Authorization {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Authorization {}

impl BuildGkeonpremVmwareAdminCluster_Authorization {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Authorization {
        GkeonpremVmwareAdminCluster_Authorization {
            viewer_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_AuthorizationRef {
        GkeonpremVmwareAdminCluster_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `viewer_users` after provisioning.\n"]
    pub fn viewer_users(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_Authorization_ViewerUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_users", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_AutoRepairConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareAdminCluster_AutoRepairConfig { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_AutoRepairConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_AutoRepairConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_AutoRepairConfig {
    #[doc = "Whether auto repair is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareAdminCluster_AutoRepairConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_AutoRepairConfig {
        GkeonpremVmwareAdminCluster_AutoRepairConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_AutoRepairConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_AutoRepairConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_AutoRepairConfigRef {
        GkeonpremVmwareAdminCluster_AutoRepairConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_AutoRepairConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether auto repair is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_ControlPlaneNode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremVmwareAdminCluster_ControlPlaneNode {
    #[doc = "Set the field `cpus`.\nThe number of vCPUs for the control-plane node of the admin cluster."]
    pub fn set_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory`.\nThe number of mebibytes of memory for the control-plane node of the admin cluster."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas`.\nThe number of control plane nodes for this VMware admin cluster."]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_ControlPlaneNode {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_ControlPlaneNode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_ControlPlaneNode {}

impl BuildGkeonpremVmwareAdminCluster_ControlPlaneNode {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_ControlPlaneNode {
        GkeonpremVmwareAdminCluster_ControlPlaneNode {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
            replicas: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_ControlPlaneNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_ControlPlaneNodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_ControlPlaneNodeRef {
        GkeonpremVmwareAdminCluster_ControlPlaneNodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_ControlPlaneNodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpus` after provisioning.\nThe number of vCPUs for the control-plane node of the admin cluster."]
    pub fn cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nThe number of mebibytes of memory for the control-plane node of the admin cluster."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nThe number of control plane nodes for this VMware admin cluster."]
    pub fn replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snat_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
    #[doc = "Set the field `address`.\nThe load balancer's IP address."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `partition`.\nhe preexisting partition to be used by the load balancer. T\nhis partition is usually created for the admin cluster for example:\n'my-f5-admin-partition'."]
    pub fn set_partition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.partition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snat_pool`.\nThe pool name. Only necessary, if using SNAT."]
    pub fn set_snat_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snat_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_F5Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_LoadBalancer_F5Config {}

impl BuildGkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
        GkeonpremVmwareAdminCluster_LoadBalancer_F5Config {
            address: core::default::Default::default(),
            partition: core::default::Default::default(),
            snat_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef {
        GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe load balancer's IP address."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `partition` after provisioning.\nhe preexisting partition to be used by the load balancer. T\nhis partition is usually created for the admin cluster for example:\n'my-f5-admin-partition'."]
    pub fn partition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snat_pool` after provisioning.\nThe pool name. Only necessary, if using SNAT."]
    pub fn snat_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snat_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_http_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_https_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    konnectivity_server_node_port: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
    #[doc = "Set the field `addons_node_port`.\nNodePort for add-ons server in the admin cluster."]
    pub fn set_addons_node_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.addons_node_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `control_plane_node_port`.\nNodePort for control plane service. The Kubernetes API server in the admin\ncluster is implemented as a Service of type NodePort (ex. 30968)."]
    pub fn set_control_plane_node_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.control_plane_node_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress_http_node_port`.\nNodePort for ingress service's http. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 32527)."]
    pub fn set_ingress_http_node_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ingress_http_node_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress_https_node_port`.\nNodePort for ingress service's https. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 30139)."]
    pub fn set_ingress_https_node_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ingress_https_node_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `konnectivity_server_node_port`.\nNodePort for konnectivity server service running as a sidecar in each\nkube-apiserver pod (ex. 30564)."]
    pub fn set_konnectivity_server_node_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.konnectivity_server_node_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {}

impl BuildGkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
        GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig {
            addons_node_port: core::default::Default::default(),
            control_plane_node_port: core::default::Default::default(),
            ingress_http_node_port: core::default::Default::default(),
            ingress_https_node_port: core::default::Default::default(),
            konnectivity_server_node_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef {
        GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addons_node_port` after provisioning.\nNodePort for add-ons server in the admin cluster."]
    pub fn addons_node_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.addons_node_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_node_port` after provisioning.\nNodePort for control plane service. The Kubernetes API server in the admin\ncluster is implemented as a Service of type NodePort (ex. 30968)."]
    pub fn control_plane_node_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_node_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_http_node_port` after provisioning.\nNodePort for ingress service's http. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 32527)."]
    pub fn ingress_http_node_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_http_node_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_https_node_port` after provisioning.\nNodePort for ingress service's https. The ingress service in the admin\ncluster is implemented as a Service of type NodePort (ex. 30139)."]
    pub fn ingress_https_node_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_https_node_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `konnectivity_server_node_port` after provisioning.\nNodePort for konnectivity server service running as a sidecar in each\nkube-apiserver pod (ex. 30564)."]
    pub fn konnectivity_server_node_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.konnectivity_server_node_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {
    #[doc = "Set the field `enabled`.\nMetal LB is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {}

impl BuildGkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig {
        GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef {
        GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nMetal LB is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_vip: std::option::Option<terrars::PrimField<std::string::String>>,
    control_plane_vip: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
    #[doc = "Set the field `addons_vip`.\nThe VIP to configure the load balancer for add-ons."]
    pub fn set_addons_vip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.addons_vip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
    #[doc = "The VIP which you previously set aside for the Kubernetes\nAPI of this VMware Admin Cluster."]
    pub control_plane_vip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
        GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig {
            addons_vip: core::default::Default::default(),
            control_plane_vip: self.control_plane_vip,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef {
        GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addons_vip` after provisioning.\nThe VIP to configure the load balancer for add-ons."]
    pub fn addons_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.addons_vip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes\nAPI of this VMware Admin Cluster."]
    pub fn control_plane_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_vip", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_LoadBalancerDynamic {
    f5_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_LoadBalancer_F5Config>>,
    manual_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig>>,
    metal_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    f5_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_LoadBalancer_F5Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metal_lb_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig>>,
    dynamic: GkeonpremVmwareAdminCluster_LoadBalancerDynamic,
}

impl GkeonpremVmwareAdminCluster_LoadBalancer {
    #[doc = "Set the field `f5_config`.\n"]
    pub fn set_f5_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_F5Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.f5_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.f5_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `manual_lb_config`.\n"]
    pub fn set_manual_lb_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manual_lb_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manual_lb_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metal_lb_config`.\n"]
    pub fn set_metal_lb_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metal_lb_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metal_lb_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vip_config`.\n"]
    pub fn set_vip_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer_VipConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_LoadBalancer {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_LoadBalancer {}

impl BuildGkeonpremVmwareAdminCluster_LoadBalancer {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_LoadBalancer {
        GkeonpremVmwareAdminCluster_LoadBalancer {
            f5_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_LoadBalancerRef {
        GkeonpremVmwareAdminCluster_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `f5_config` after provisioning.\n"]
    pub fn f5_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancer_F5ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.f5_config", self.base))
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancer_ManualLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancer_MetalLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metal_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_LoadBalancer_VipConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig { }

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig {
    #[doc = "enabled is a flag to mark if DHCP IP allocation is\nused for VMware admin clusters."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig {
        GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nenabled is a flag to mark if DHCP IP allocation is\nused for VMware admin clusters."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    ip: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[doc = "IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
            hostname: core::default::Default::default(),
            ip: self.ip,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nIP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockDynamic {
    ips: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
    gateway: terrars::PrimField<std::string::String>,
    netmask: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
    dynamic: GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockDynamic,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ips = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ips = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
    type O =
        terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
    #[doc = "The network gateway used by the VMware Admin Cluster."]
    pub gateway: terrars::PrimField<std::string::String>,
    #[doc = "The netmask used by the VMware Admin Cluster."]
    pub netmask: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
        GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware Admin Cluster."]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc =
        "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware Admin Cluster."]
    pub fn netmask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigDynamic {
    control_plane_ip_block: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_ip_block: std
    ::option
    ::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock>>,
    dynamic: GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigDynamic,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
    #[doc = "Set the field `control_plane_ip_block`.\n"]
    pub fn set_control_plane_ip_block(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlock,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_plane_ip_block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_ip_block = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
        GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig {
            control_plane_ip_block: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_ip_block` after provisioning.\n"]
    pub fn control_plane_ip_block(
        &self,
    ) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig_ControlPlaneIpBlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_ip_block", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_search_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ntp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
    #[doc = "Set the field `dns_search_domains`.\nDNS search domains."]
    pub fn set_dns_search_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_search_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\nDNS servers."]
    pub fn set_dns_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ntp_servers`.\nNTP servers."]
    pub fn set_ntp_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ntp_servers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
        GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig {
            dns_search_domains: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            ntp_servers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_search_domains` after provisioning.\nDNS search domains."]
    pub fn dns_search_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_search_domains", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\nDNS servers."]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `ntp_servers` after provisioning.\nNTP servers."]
    pub fn ntp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ntp_servers", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksDynamic {
    ips: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    gateway: terrars::PrimField<std::string::String>,
    netmask: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
    dynamic: GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksDynamic,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ips = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ips = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    #[doc = "The network gateway used by the VMware Admin Cluster."]
    pub gateway: terrars::PrimField<std::string::String>,
    #[doc = "The netmask used by the VMware Admin Cluster."]
    pub netmask: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware Admin Cluster."]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc =
        "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware Admin Cluster."]
    pub fn netmask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigDynamic {
    ip_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks>>,
    dynamic: GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigDynamic,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocks>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig {
            ip_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef {
        GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareAdminCluster_NetworkConfigDynamic {
    dhcp_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig>>,
    ha_control_plane_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig>>,
    host_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig>>,
    static_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfig {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcenter_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_ip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ha_control_plane_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_ip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig>>,
    dynamic: GkeonpremVmwareAdminCluster_NetworkConfigDynamic,
}

impl GkeonpremVmwareAdminCluster_NetworkConfig {
    #[doc = "Set the field `vcenter_network`.\nvcenter_network specifies vCenter network name."]
    pub fn set_vcenter_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vcenter_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dhcp_ip_config`.\n"]
    pub fn set_dhcp_ip_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dhcp_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dhcp_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ha_control_plane_config`.\n"]
    pub fn set_ha_control_plane_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ha_control_plane_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ha_control_plane_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host_config`.\n"]
    pub fn set_host_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_HostConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_ip_config`.\n"]
    pub fn set_static_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_NetworkConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_NetworkConfig {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremVmwareAdminCluster_NetworkConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_NetworkConfig {
        GkeonpremVmwareAdminCluster_NetworkConfig {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            vcenter_network: core::default::Default::default(),
            dhcp_ip_config: core::default::Default::default(),
            ha_control_plane_config: core::default::Default::default(),
            host_config: core::default::Default::default(),
            static_ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_NetworkConfigRef {
        GkeonpremVmwareAdminCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub fn service_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vcenter_network` after provisioning.\nvcenter_network specifies vCenter network name."]
    pub fn vcenter_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcenter_network", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_ip_config` after provisioning.\n"]
    pub fn dhcp_ip_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_DhcpIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dhcp_ip_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ha_control_plane_config` after provisioning.\n"]
    pub fn ha_control_plane_config(
        &self,
    ) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_HaControlPlaneConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ha_control_plane_config", self.base))
    }

    #[doc = "Get a reference to the value of field `host_config` after provisioning.\n"]
    pub fn host_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_HostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_config", self.base))
    }

    #[doc = "Get a reference to the value of field `static_ip_config` after provisioning.\n"]
    pub fn static_ip_config(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_NetworkConfig_StaticIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremVmwareAdminCluster_Status>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareAdminCluster_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_PlatformConfig_Bundles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_PlatformConfig_Bundles {}

impl BuildGkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
        GkeonpremVmwareAdminCluster_PlatformConfig_Bundles {
            status: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef {
        GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_PlatformConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_PlatformConfig {
    #[doc =
        "Set the field `required_platform_version`.\nThe required platform version e.g. 1.13.1.\nIf the current platform version is lower than the target version,\nthe platform version will be updated to the target version.\nIf the target version is not installed in the platform\n(bundle versions), download the target version bundle."]
    pub fn set_required_platform_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.required_platform_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_PlatformConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_PlatformConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_PlatformConfig {}

impl BuildGkeonpremVmwareAdminCluster_PlatformConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_PlatformConfig {
        GkeonpremVmwareAdminCluster_PlatformConfig { required_platform_version: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_PlatformConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_PlatformConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_PlatformConfigRef {
        GkeonpremVmwareAdminCluster_PlatformConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_PlatformConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundles` after provisioning.\nThe list of bundles installed in the admin cluster."]
    pub fn bundles(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_PlatformConfig_BundlesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bundles", self.base))
    }

    #[doc =
        "Get a reference to the value of field `platform_version` after provisioning.\nThe platform version e.g. 1.13.2."]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_platform_version` after provisioning.\nThe required platform version e.g. 1.13.1.\nIf the current platform version is lower than the target version,\nthe platform version will be updated to the target version.\nIf the target version is not installed in the platform\n(bundle versions), download the target version bundle."]
    pub fn required_platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_platform_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_PrivateRegistryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_cert: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_PrivateRegistryConfig {
    #[doc = "Set the field `address`.\nThe registry address."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ca_cert`.\nThe CA certificate public key for private registry."]
    pub fn set_ca_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_cert = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_PrivateRegistryConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_PrivateRegistryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_PrivateRegistryConfig {}

impl BuildGkeonpremVmwareAdminCluster_PrivateRegistryConfig {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_PrivateRegistryConfig {
        GkeonpremVmwareAdminCluster_PrivateRegistryConfig {
            address: core::default::Default::default(),
            ca_cert: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef {
        GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_PrivateRegistryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe registry address."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_cert` after provisioning.\nThe CA certificate public key for private registry."]
    pub fn ca_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Proxy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_proxy: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareAdminCluster_Proxy {
    #[doc =
        "Set the field `no_proxy`.\nA comma-separated list of IP addresses, IP address ranges,\nhost names, and domain names that should not go through the proxy server."]
    pub fn set_no_proxy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.no_proxy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Proxy {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Proxy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Proxy {
    #[doc = "The proxy url."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareAdminCluster_Proxy {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Proxy {
        GkeonpremVmwareAdminCluster_Proxy {
            no_proxy: core::default::Default::default(),
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_ProxyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_ProxyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_ProxyRef {
        GkeonpremVmwareAdminCluster_ProxyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_ProxyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_proxy` after provisioning.\nA comma-separated list of IP addresses, IP address ranges,\nhost names, and domain names that should not go through the proxy server."]
    pub fn no_proxy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe proxy url."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareAdminCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_Timeouts {
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

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Timeouts {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Timeouts {}

impl BuildGkeonpremVmwareAdminCluster_Timeouts {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Timeouts {
        GkeonpremVmwareAdminCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_TimeoutsRef {
        GkeonpremVmwareAdminCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_TimeoutsRef {
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
pub struct GkeonpremVmwareAdminCluster_Vcenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_cert_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datacenter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareAdminCluster_Vcenter {
    #[doc = "Set the field `address`.\nThe vCenter IP address."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ca_cert_data`.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn set_ca_cert_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_cert_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster`.\nThe name of the vCenter cluster for the admin cluster."]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_disk`.\nThe name of the virtual machine disk (VMDK) for the admin cluster."]
    pub fn set_data_disk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datacenter`.\nThe name of the vCenter datacenter for the admin cluster."]
    pub fn set_datacenter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datacenter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datastore`.\nThe name of the vCenter datastore for the admin cluster."]
    pub fn set_datastore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datastore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `folder`.\nThe name of the vCenter folder for the admin cluster."]
    pub fn set_folder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_pool`.\nThe name of the vCenter resource pool for the admin cluster."]
    pub fn set_resource_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_policy_name`.\nThe name of the vCenter storage policy for the user cluster."]
    pub fn set_storage_policy_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_policy_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareAdminCluster_Vcenter {
    type O = terrars::BlockAssignable<GkeonpremVmwareAdminCluster_Vcenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareAdminCluster_Vcenter {}

impl BuildGkeonpremVmwareAdminCluster_Vcenter {
    pub fn build(self) -> GkeonpremVmwareAdminCluster_Vcenter {
        GkeonpremVmwareAdminCluster_Vcenter {
            address: core::default::Default::default(),
            ca_cert_data: core::default::Default::default(),
            cluster: core::default::Default::default(),
            data_disk: core::default::Default::default(),
            datacenter: core::default::Default::default(),
            datastore: core::default::Default::default(),
            folder: core::default::Default::default(),
            resource_pool: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareAdminCluster_VcenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareAdminCluster_VcenterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareAdminCluster_VcenterRef {
        GkeonpremVmwareAdminCluster_VcenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareAdminCluster_VcenterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe vCenter IP address."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_cert_data` after provisioning.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn ca_cert_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe name of the vCenter cluster for the admin cluster."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_disk` after provisioning.\nThe name of the virtual machine disk (VMDK) for the admin cluster."]
    pub fn data_disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datacenter` after provisioning.\nThe name of the vCenter datacenter for the admin cluster."]
    pub fn datacenter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datacenter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datastore` after provisioning.\nThe name of the vCenter datastore for the admin cluster."]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe name of the vCenter folder for the admin cluster."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool` after provisioning.\nThe name of the vCenter resource pool for the admin cluster."]
    pub fn resource_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_policy_name` after provisioning.\nThe name of the vCenter storage policy for the user cluster."]
    pub fn storage_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_policy_name", self.base))
    }
}
