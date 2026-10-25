#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremBareMetalAdminClusterData_ {
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
    bare_metal_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_operations: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_ClusterOperations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_ControlPlane>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_MaintenanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_access_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_NodeAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_Proxy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_SecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_Storage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremBareMetalAdminCluster_Timeouts>,
    dynamic: GkeonpremBareMetalAdminClusterDynamic,
}

struct GkeonpremBareMetalAdminCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremBareMetalAdminClusterData_>,
}

#[derive(Clone)]
pub struct GkeonpremBareMetalAdminCluster(std::rc::Rc<GkeonpremBareMetalAdminCluster_>);

impl GkeonpremBareMetalAdminCluster {
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
        "Set the field `annotations`.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal_version`.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn set_bare_metal_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bare_metal_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_operations`.\n"]
    pub fn set_cluster_operations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ClusterOperations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane>>,
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

    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer>>,
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

    #[doc = "Set the field `maintenance_config`.\n"]
    pub fn set_maintenance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_MaintenanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NetworkConfig>>,
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

    #[doc = "Set the field `node_access_config`.\n"]
    pub fn set_node_access_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NodeAccessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NodeConfig>>,
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

    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Proxy>>,
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

    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremBareMetalAdminCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn bare_metal_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal Admin Cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fleet` after provisioning.\nFleet related configuration.\nFleets are a Google Cloud concept for logically organizing clusters,\nletting you use and manage multi-cluster capabilities and apply\nconsistent policies across your systems.\nSee [Anthos Fleets](https://cloud.google.com/anthos/multicluster-management/fleets) for\nmore details on Anthos multi-cluster capabilities using Fleets."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Admin Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal admin cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal Admin Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Admin Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal Admin Cluster."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ClusterOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_MaintenanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NodeAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminCluster_TimeoutsRef {
        GkeonpremBareMetalAdminCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GkeonpremBareMetalAdminCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeonpremBareMetalAdminCluster { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster {
    type O = terrars::ListRef<GkeonpremBareMetalAdminClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeonpremBareMetalAdminCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gkeonprem_bare_metal_admin_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The bare metal admin cluster name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeonpremBareMetalAdminCluster {
        let out = GkeonpremBareMetalAdminCluster(std::rc::Rc::new(GkeonpremBareMetalAdminCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeonpremBareMetalAdminClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                bare_metal_version: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                cluster_operations: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                maintenance_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_access_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                security_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeonpremBareMetalAdminClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeonpremBareMetalAdminClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal Admin Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn bare_metal_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the cluster was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the cluster was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal Admin Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal Admin Cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fleet` after provisioning.\nFleet related configuration.\nFleets are a Google Cloud concept for logically organizing clusters,\nletting you use and manage multi-cluster capabilities and apply\nconsistent policies across your systems.\nSee [Anthos Fleets](https://cloud.google.com/anthos/multicluster-management/fleets) for\nmore details on Anthos multi-cluster capabilities using Fleets."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Admin Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal admin cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal Admin Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal Admin Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal Admin Cluster."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ClusterOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_MaintenanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NodeAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminCluster_TimeoutsRef {
        GkeonpremBareMetalAdminCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterDynamic {
    cluster_operations: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_ClusterOperations>>,
    control_plane: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_ControlPlane>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer>>,
    maintenance_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_MaintenanceConfig>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_NetworkConfig>>,
    node_access_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_NodeAccessConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_NodeConfig>>,
    proxy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_Proxy>>,
    security_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_SecurityConfig>>,
    storage: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_Storage>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_Fleet {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Fleet {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Fleet {}

impl BuildGkeonpremBareMetalAdminCluster_Fleet {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Fleet {
        GkeonpremBareMetalAdminCluster_Fleet { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_FleetRef {
        GkeonpremBareMetalAdminCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_FleetRef {
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
pub struct GkeonpremBareMetalAdminCluster_Status_Conditions {
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

impl GkeonpremBareMetalAdminCluster_Status_Conditions {
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Status_Conditions {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Status_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Status_Conditions {}

impl BuildGkeonpremBareMetalAdminCluster_Status_Conditions {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Status_Conditions {
        GkeonpremBareMetalAdminCluster_Status_Conditions {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_Status_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_Status_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_Status_ConditionsRef {
        GkeonpremBareMetalAdminCluster_Status_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_Status_ConditionsRef {
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
pub struct GkeonpremBareMetalAdminCluster_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminCluster_Status_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_Status {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalAdminCluster_Status_Conditions>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Status {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Status {}

impl BuildGkeonpremBareMetalAdminCluster_Status {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Status {
        GkeonpremBareMetalAdminCluster_Status {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_StatusRef {
        GkeonpremBareMetalAdminCluster_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_Status_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {}

impl BuildGkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
        GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef {
        GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheck_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result>>,
}

impl GkeonpremBareMetalAdminCluster_ValidationCheck_Status {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalAdminCluster_ValidationCheck_Status_Result>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ValidationCheck_Status {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ValidationCheck_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ValidationCheck_Status {}

impl BuildGkeonpremBareMetalAdminCluster_ValidationCheck_Status {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ValidationCheck_Status {
        GkeonpremBareMetalAdminCluster_ValidationCheck_Status { result: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef {
        GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ValidationCheck_Status_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminCluster_ValidationCheck_Status>>,
}

impl GkeonpremBareMetalAdminCluster_ValidationCheck {
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scenario`.\n"]
    pub fn set_scenario(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scenario = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalAdminCluster_ValidationCheck_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ValidationCheck {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ValidationCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ValidationCheck {}

impl BuildGkeonpremBareMetalAdminCluster_ValidationCheck {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ValidationCheck {
        GkeonpremBareMetalAdminCluster_ValidationCheck {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ValidationCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ValidationCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ValidationCheckRef {
        GkeonpremBareMetalAdminCluster_ValidationCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ValidationCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `scenario` after provisioning.\n"]
    pub fn scenario(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scenario", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ValidationCheck_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ClusterOperations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_application_logs: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalAdminCluster_ClusterOperations {
    #[doc =
        "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_application_logs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ClusterOperations {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ClusterOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ClusterOperations {}

impl BuildGkeonpremBareMetalAdminCluster_ClusterOperations {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ClusterOperations {
        GkeonpremBareMetalAdminCluster_ClusterOperations {
            enable_application_logs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ClusterOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ClusterOperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ClusterOperationsRef {
        GkeonpremBareMetalAdminCluster_ClusterOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ClusterOperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_application_logs` after provisioning.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn enable_application_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_application_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
    argument: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
    #[doc =
        "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: terrars::PrimField<std::string::String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
        GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs {
            argument: self.argument,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef {
        GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `argument` after provisioning.\nThe argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub fn argument(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argument", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value of the arg as it will be passed to the API Server command line."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_ip`.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"]
    pub fn set_node_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_ip` after provisioning.\nThe default IPv4 address for SSH access and Kubernetes node.\nExample: 192.168.0.1"]
    pub fn node_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    #[doc =
        "Set the field `effect`.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nKey associated with the effect."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue associated with the effect."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nSpecifies the nodes operating system (default: LINUX). Possible values: [\"EFFECT_UNSPECIFIED\", \"PREFER_NO_SCHEDULE\", \"NO_EXECUTE\"]"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey associated with the effect."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue associated with the effect."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std::vec::Vec<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>,
    >,
    dynamic: GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\nSpecifies the nodes operating system (default: LINUX)."]
    pub fn set_operating_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_configs`.\n"]
    pub fn set_node_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    type O =
        terrars
        ::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nFor example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operating_system` after provisioning.\nSpecifies the nodes operating system (default: LINUX)."]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }

    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]
    pub fn node_configs(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
        GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_ControlPlaneDynamic {
    api_server_args: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs>>,
    control_plane_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlane {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_server_args: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_ControlPlaneDynamic,
}

impl GkeonpremBareMetalAdminCluster_ControlPlane {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_server_args = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_server_args = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane_node_pool_config`.\n"]
    pub fn set_control_plane_node_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_plane_node_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_node_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_ControlPlane {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_ControlPlane>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_ControlPlane {}

impl BuildGkeonpremBareMetalAdminCluster_ControlPlane {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_ControlPlane {
        GkeonpremBareMetalAdminCluster_ControlPlane {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ControlPlaneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ControlPlaneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_ControlPlaneRef {
        GkeonpremBareMetalAdminCluster_ControlPlaneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ControlPlaneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ControlPlane_ApiServerArgsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_server_args", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_ControlPlane_ControlPlaneNodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avoid_buggy_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_assign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
    #[doc = "Set the field `addresses`.\nThe addresses that are part of this pool."]
    pub fn set_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `avoid_buggy_ips`.\nThis avoids buggy consumer devices mistakenly\ndropping IPv4 traffic for those special IP addresses."]
    pub fn set_avoid_buggy_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.avoid_buggy_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manual_assign`.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn set_manual_assign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.manual_assign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool`.\n"]
    pub fn set_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools {
            addresses: core::default::Default::default(),
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool."]
    pub fn addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nThis avoids buggy consumer devices mistakenly\ndropping IPv4 traffic for those special IP addresses."]
    pub fn avoid_buggy_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.avoid_buggy_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `manual_assign` after provisioning.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn manual_assign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manual_assign", self.base))
    }

    #[doc = "Get a reference to the value of field `pool` after provisioning.\n"]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_nodes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `control_plane_nodes`.\nThe IP address of the control plane node that\nconnects to the external peer."]
    pub fn set_control_plane_nodes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.control_plane_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
            asn: core::default::Default::default(),
            control_plane_nodes: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_nodes` after provisioning.\nThe IP address of the control plane node that\nconnects to the external peer."]
    pub fn control_plane_nodes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_burst: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_pull_qps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialize_image_pulls_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    #[doc = "Set the field `registry_burst`.\n"]
    pub fn set_registry_burst(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.registry_burst = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_pull_qps`.\n"]
    pub fn set_registry_pull_qps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.registry_pull_qps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serialize_image_pulls_disabled`.\n"]
    pub fn set_serialize_image_pulls_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.serialize_image_pulls_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
            registry_burst: core::default::Default::default(),
            registry_pull_qps: core::default::Default::default(),
            serialize_image_pulls_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `registry_burst` after provisioning.\n"]
    pub fn registry_burst(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_burst", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_pull_qps` after provisioning.\n"]
    pub fn registry_pull_qps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_pull_qps", self.base))
    }

    #[doc = "Get a reference to the value of field `serialize_image_pulls_disabled` after provisioning.\n"]
    pub fn serialize_image_pulls_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialize_image_pulls_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[doc =
        "Set the field `labels`.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_ip`.\n"]
    pub fn set_node_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `node_ip` after provisioning.\n"]
    pub fn node_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
    #[doc = "Set the field `effect`.\nAvailable taint effects."]
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

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\nAvailable taint effects."]
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

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic {
    kubelet_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >,
    >,
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubelet_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints,
        >,
    >,
    dynamic: GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    #[doc =
        "Set the field `labels`.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\nThe available Operating Systems to be run in a Node."]
    pub fn set_operating_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
                        >,
                    >,
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

    #[doc = "Set the field `node_configs`.\n"]
    pub fn set_node_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `taints`.\n"]
    pub fn set_taints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_Taints,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to nodes of this node pool.\nAn object containing a list of key/value pairs.\nExample:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operating_system` after provisioning.\nThe available Operating Systems to be run in a Node."]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]
    pub fn node_configs(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_TaintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig>,
    >,
    dynamic: GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    type O =
        terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    bgp_peer_configs: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>>,
    load_balancer_node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_peer_configs: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
    #[doc = "Set the field `asn`.\nBGP autonomous system number (ASN) of the cluster."]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPools>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.address_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.address_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bgp_peer_configs`.\n"]
    pub fn set_bgp_peer_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bgp_peer_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bgp_peer_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer_node_pool_config`.\n"]
    pub fn set_load_balancer_node_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.load_balancer_node_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.load_balancer_node_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig {
            asn: core::default::Default::default(),
            address_pools: core::default::Default::default(),
            bgp_peer_configs: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) of the cluster."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_peer_configs` after provisioning.\n"]
    pub fn bgp_peer_configs(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_peer_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether manual load balancing is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
    control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_load_balancer_port` after provisioning.\nThe port that control plane hosted load balancers will listen on."]
    pub fn control_plane_load_balancer_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_load_balancer_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig {
    control_plane_vip: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster."]
    pub control_plane_vip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig {
        GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig { control_plane_vip: self.control_plane_vip }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef {
        GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster."]
    pub fn control_plane_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_vip", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_LoadBalancerDynamic {
    bgp_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig>>,
    manual_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig>>,
    port_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_LoadBalancerDynamic,
}

impl GkeonpremBareMetalAdminCluster_LoadBalancer {
    #[doc = "Set the field `bgp_lb_config`.\n"]
    pub fn set_bgp_lb_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bgp_lb_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bgp_lb_config = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfig>>,
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

    #[doc = "Set the field `port_config`.\n"]
    pub fn set_port_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vip_config`.\n"]
    pub fn set_vip_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfig>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_LoadBalancer {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_LoadBalancer {}

impl BuildGkeonpremBareMetalAdminCluster_LoadBalancer {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_LoadBalancer {
        GkeonpremBareMetalAdminCluster_LoadBalancer {
            bgp_lb_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_LoadBalancerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_LoadBalancerRef {
        GkeonpremBareMetalAdminCluster_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_lb_config` after provisioning.\n"]
    pub fn bgp_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_BgpLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_ManualLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_PortConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_LoadBalancer_VipConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_MaintenanceConfig {
    maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_MaintenanceConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_MaintenanceConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_MaintenanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_MaintenanceConfig {
    #[doc =
        "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalAdminCluster_MaintenanceConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_MaintenanceConfig {
        GkeonpremBareMetalAdminCluster_MaintenanceConfig {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_MaintenanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_MaintenanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_MaintenanceConfigRef {
        GkeonpremBareMetalAdminCluster_MaintenanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_MaintenanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `maintenance_address_cidr_blocks` after provisioning.\nAll IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub fn maintenance_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_address_cidr_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
        GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef {
        GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nAll pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_address_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nAll services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub fn service_address_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_address_cidr_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    #[doc = "Set the field `enabled`.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {}

impl BuildGkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
        GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
        GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_NetworkConfigDynamic {
    island_mode_cidr: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr>>,
    multiple_network_interfaces_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    island_mode_cidr: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiple_network_interfaces_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_NetworkConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_NetworkConfig {
    #[doc = "Set the field `advanced_networking`.\nEnables the use of advanced Anthos networking features."]
    pub fn set_advanced_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.advanced_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `island_mode_cidr`.\n"]
    pub fn set_island_mode_cidr(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidr>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.island_mode_cidr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.island_mode_cidr = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multiple_network_interfaces_config`.\n"]
    pub fn set_multiple_network_interfaces_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multiple_network_interfaces_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multiple_network_interfaces_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_NetworkConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_NetworkConfig {}

impl BuildGkeonpremBareMetalAdminCluster_NetworkConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_NetworkConfig {
        GkeonpremBareMetalAdminCluster_NetworkConfig {
            advanced_networking: core::default::Default::default(),
            island_mode_cidr: core::default::Default::default(),
            multiple_network_interfaces_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_NetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_NetworkConfigRef {
        GkeonpremBareMetalAdminCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `advanced_networking` after provisioning.\nEnables the use of advanced Anthos networking features."]
    pub fn advanced_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NetworkConfig_IslandModeCidrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.island_mode_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `multiple_network_interfaces_config` after provisioning.\n"]
    pub fn multiple_network_interfaces_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiple_network_interfaces_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_NodeAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_NodeAccessConfig {
    #[doc =
        "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_NodeAccessConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NodeAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_NodeAccessConfig {}

impl BuildGkeonpremBareMetalAdminCluster_NodeAccessConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_NodeAccessConfig {
        GkeonpremBareMetalAdminCluster_NodeAccessConfig { login_user: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_NodeAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_NodeAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_NodeAccessConfigRef {
        GkeonpremBareMetalAdminCluster_NodeAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_NodeAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `login_user` after provisioning.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn login_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_user", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremBareMetalAdminCluster_NodeConfig {
    #[doc =
        "Set the field `max_pods_per_node`.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_NodeConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_NodeConfig {}

impl BuildGkeonpremBareMetalAdminCluster_NodeConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_NodeConfig {
        GkeonpremBareMetalAdminCluster_NodeConfig { max_pods_per_node: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_NodeConfigRef {
        GkeonpremBareMetalAdminCluster_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Proxy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_proxy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_Proxy {
    #[doc =
        "Set the field `no_proxy`.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn set_no_proxy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.no_proxy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Proxy {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Proxy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Proxy {
    #[doc =
        "Specifies the address of your proxy server.\nFor Example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_Proxy {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Proxy {
        GkeonpremBareMetalAdminCluster_Proxy {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_ProxyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_ProxyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_ProxyRef {
        GkeonpremBareMetalAdminCluster_ProxyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_ProxyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_proxy` after provisioning.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn no_proxy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecifies the address of your proxy server.\nFor Example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers {
        GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef {
        GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef {
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
struct GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationDynamic {
    admin_users: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers>>,
    dynamic: GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationDynamic,
}

impl GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsers>,
                    >,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {}

impl BuildGkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
        GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef {
        GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization_AdminUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_SecurityConfigDynamic {
    authorization: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization>>,
    dynamic: GkeonpremBareMetalAdminCluster_SecurityConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_SecurityConfig {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig_Authorization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_SecurityConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_SecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_SecurityConfig {}

impl BuildGkeonpremBareMetalAdminCluster_SecurityConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_SecurityConfig {
        GkeonpremBareMetalAdminCluster_SecurityConfig {
            authorization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_SecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_SecurityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_SecurityConfigRef {
        GkeonpremBareMetalAdminCluster_SecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_SecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_SecurityConfig_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
        GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef {
        GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe host machine path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass name that PVs will be created with."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
        GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef {
        GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe host machine path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass name that PVs will be created with."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigDynamic {
    lvp_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_path_pv_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigDynamic,
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
    #[doc = "Set the field `shared_path_pv_count`.\nThe number of subdirectories to create under path."]
    pub fn set_shared_path_pv_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shared_path_pv_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lvp_config`.\n"]
    pub fn set_lvp_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lvp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {}

impl BuildGkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
        GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef {
        GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `shared_path_pv_count` after provisioning.\nThe number of subdirectories to create under path."]
    pub fn shared_path_pv_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_path_pv_count", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_config` after provisioning.\n"]
    pub fn lvp_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig_LvpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminCluster_StorageDynamic {
    lvp_node_mounts_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig>>,
    lvp_share_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Storage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_node_mounts_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_share_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig>>,
    dynamic: GkeonpremBareMetalAdminCluster_StorageDynamic,
}

impl GkeonpremBareMetalAdminCluster_Storage {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lvp_node_mounts_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_node_mounts_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lvp_share_config`.\n"]
    pub fn set_lvp_share_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lvp_share_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lvp_share_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Storage {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Storage {}

impl BuildGkeonpremBareMetalAdminCluster_Storage {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Storage {
        GkeonpremBareMetalAdminCluster_Storage {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_StorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_StorageRef {
        GkeonpremBareMetalAdminCluster_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_Storage_LvpNodeMountsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_node_mounts_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminCluster_Storage_LvpShareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_share_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminCluster_Timeouts {
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminCluster_Timeouts {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminCluster_Timeouts {}

impl BuildGkeonpremBareMetalAdminCluster_Timeouts {
    pub fn build(self) -> GkeonpremBareMetalAdminCluster_Timeouts {
        GkeonpremBareMetalAdminCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalAdminCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminCluster_TimeoutsRef {
        GkeonpremBareMetalAdminCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminCluster_TimeoutsRef {
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
