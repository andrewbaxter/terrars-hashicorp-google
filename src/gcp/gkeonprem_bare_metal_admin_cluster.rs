use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremBareMetalAdminClusterData {
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
    cluster_operations: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_access_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterProxyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterStorageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremBareMetalAdminClusterTimeoutsEl>,
    dynamic: GkeonpremBareMetalAdminClusterDynamic,
}

struct GkeonpremBareMetalAdminCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremBareMetalAdminClusterData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterProxyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremBareMetalAdminClusterTimeoutsEl>) -> Self {
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterFleetElRef> {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStatusElRef> {
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
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterValidationCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterClusterOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterMaintenanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNodeAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterProxyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(GkeonpremBareMetalAdminClusterData {
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterFleetElRef> {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStatusElRef> {
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
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterValidationCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterClusterOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterMaintenanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNodeAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterProxyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterFleetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterFleetEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterFleetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterFleetEl {}

impl BuildGkeonpremBareMetalAdminClusterFleetEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterFleetEl {
        GkeonpremBareMetalAdminClusterFleetEl { membership: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalAdminClusterFleetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterFleetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterFleetElRef {
        GkeonpremBareMetalAdminClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterFleetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStatusElConditionsEl {
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

impl GkeonpremBareMetalAdminClusterStatusElConditionsEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStatusElConditionsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStatusElConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStatusElConditionsEl {}

impl BuildGkeonpremBareMetalAdminClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStatusElConditionsEl {
        GkeonpremBareMetalAdminClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
        GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStatusElConditionsElRef {
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
pub struct GkeonpremBareMetalAdminClusterStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalAdminClusterStatusElConditionsEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStatusEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStatusEl {}

impl BuildGkeonpremBareMetalAdminClusterStatusEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStatusEl {
        GkeonpremBareMetalAdminClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterStatusElRef {
        GkeonpremBareMetalAdminClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStatusElConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
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

impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {}

impl BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef {
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
pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>>,
}

impl GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultEl>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusEl {}

impl BuildGkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusEl {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusEl { result: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterValidationCheckElStatusElResultElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterValidationCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>>,
}

impl GkeonpremBareMetalAdminClusterValidationCheckEl {
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
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalAdminClusterValidationCheckElStatusEl>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterValidationCheckEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterValidationCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterValidationCheckEl {}

impl BuildGkeonpremBareMetalAdminClusterValidationCheckEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterValidationCheckEl {
        GkeonpremBareMetalAdminClusterValidationCheckEl {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterValidationCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterValidationCheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterValidationCheckElRef {
        GkeonpremBareMetalAdminClusterValidationCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterValidationCheckElRef {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterValidationCheckElStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterClusterOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_application_logs: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalAdminClusterClusterOperationsEl {
    #[doc =
        "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_application_logs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterClusterOperationsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterClusterOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterClusterOperationsEl {}

impl BuildGkeonpremBareMetalAdminClusterClusterOperationsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterClusterOperationsEl {
        GkeonpremBareMetalAdminClusterClusterOperationsEl {
            enable_application_logs: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterClusterOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterClusterOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterClusterOperationsElRef {
        GkeonpremBareMetalAdminClusterClusterOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterClusterOperationsElRef {
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
pub struct GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    argument: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    #[doc =
        "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: terrars::PrimField<std::string::String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
        GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl {
            argument: self.argument,
            value: self.value,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef {
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
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
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
::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
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
struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
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
        ::Vec<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl>,
    >,
    dynamic: GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
                            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
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
                            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
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
::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    type O =
        terrars
        ::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
    ::ListRef<
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
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
                            GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterControlPlaneElDynamic {
    api_server_args: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
    control_plane_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterControlPlaneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_server_args: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterControlPlaneElDynamic,
}

impl GkeonpremBareMetalAdminClusterControlPlaneEl {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsEl>>,
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
                        ::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigEl>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterControlPlaneEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterControlPlaneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterControlPlaneEl {}

impl BuildGkeonpremBareMetalAdminClusterControlPlaneEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterControlPlaneEl {
        GkeonpremBareMetalAdminClusterControlPlaneEl {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterControlPlaneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterControlPlaneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterControlPlaneElRef {
        GkeonpremBareMetalAdminClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterControlPlaneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElApiServerArgsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_server_args", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminClusterControlPlaneElControlPlaneNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl { enabled: self.enabled }
    }
}

pub struct GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    control_plane_vip: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster."]
    pub control_plane_vip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl {
        GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl { control_plane_vip: self.control_plane_vip }
    }
}

pub struct GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef {
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
struct GkeonpremBareMetalAdminClusterLoadBalancerElDynamic {
    manual_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
    port_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterLoadBalancerElDynamic,
}

impl GkeonpremBareMetalAdminClusterLoadBalancerEl {
    #[doc = "Set the field `manual_lb_config`.\n"]
    pub fn set_manual_lb_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterLoadBalancerEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterLoadBalancerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterLoadBalancerEl {}

impl BuildGkeonpremBareMetalAdminClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterLoadBalancerEl {
        GkeonpremBareMetalAdminClusterLoadBalancerEl {
            manual_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterLoadBalancerElRef {
        GkeonpremBareMetalAdminClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterLoadBalancerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElManualLbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElPortConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterLoadBalancerElVipConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterMaintenanceConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    #[doc =
        "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalAdminClusterMaintenanceConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
        GkeonpremBareMetalAdminClusterMaintenanceConfigEl {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
        GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterMaintenanceConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
        GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
        GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef {
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

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterNetworkConfigElDynamic {
    island_mode_cidr: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    island_mode_cidr: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
    dynamic: GkeonpremBareMetalAdminClusterNetworkConfigElDynamic,
}

impl GkeonpremBareMetalAdminClusterNetworkConfigEl {
    #[doc = "Set the field `island_mode_cidr`.\n"]
    pub fn set_island_mode_cidr(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrEl>>,
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
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterNetworkConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterNetworkConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNetworkConfigEl {
        GkeonpremBareMetalAdminClusterNetworkConfigEl {
            island_mode_cidr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterNetworkConfigElRef {
        GkeonpremBareMetalAdminClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterNetworkConfigElIslandModeCidrElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.island_mode_cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    #[doc =
        "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterNodeAccessConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterNodeAccessConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNodeAccessConfigEl {
        GkeonpremBareMetalAdminClusterNodeAccessConfigEl { login_user: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
        GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterNodeAccessConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremBareMetalAdminClusterNodeConfigEl {
    #[doc =
        "Set the field `max_pods_per_node`.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterNodeConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterNodeConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterNodeConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterNodeConfigEl {
        GkeonpremBareMetalAdminClusterNodeConfigEl { max_pods_per_node: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalAdminClusterNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterNodeConfigElRef {
        GkeonpremBareMetalAdminClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterNodeConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterProxyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_proxy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterProxyEl {
    #[doc =
        "Set the field `no_proxy`.\nA list of IPs, hostnames, and domains that should skip the proxy.\nExamples: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn set_no_proxy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.no_proxy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterProxyEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterProxyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterProxyEl {
    #[doc =
        "Specifies the address of your proxy server.\nExamples: http://domain\nWARNING: Do not provide credentials in the format\nhttp://(username:password@)domain these will be rejected by the server."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterProxyEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterProxyEl {
        GkeonpremBareMetalAdminClusterProxyEl {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterProxyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterProxyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterProxyElRef {
        GkeonpremBareMetalAdminClusterProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterProxyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_proxy` after provisioning.\nA list of IPs, hostnames, and domains that should skip the proxy.\nExamples: [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn no_proxy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecifies the address of your proxy server.\nExamples: http://domain\nWARNING: Do not provide credentials in the format\nhttp://(username:password@)domain these will be rejected by the server."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl { username: self.username }
    }
}

pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef {
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
struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElDynamic {
    admin_users: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
    dynamic: GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElDynamic,
}

impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersEl>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {}

impl BuildGkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElAdminUsersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterSecurityConfigElDynamic {
    authorization: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterSecurityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
    dynamic: GkeonpremBareMetalAdminClusterSecurityConfigElDynamic,
}

impl GkeonpremBareMetalAdminClusterSecurityConfigEl {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterSecurityConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterSecurityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterSecurityConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterSecurityConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterSecurityConfigEl {
        GkeonpremBareMetalAdminClusterSecurityConfigEl {
            authorization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterSecurityConfigElRef {
        GkeonpremBareMetalAdminClusterSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterSecurityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterSecurityConfigElAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef {
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
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef {
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
struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElDynamic {
    lvp_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_path_pv_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElDynamic,
}

impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
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
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigEl>,
                    >,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {}

impl BuildGkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
        GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `shared_path_pv_count` after provisioning.\nThe number of subdirectories to create under path."]
    pub fn shared_path_pv_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_path_pv_count", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_config` after provisioning.\n"]
    pub fn lvp_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElLvpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalAdminClusterStorageElDynamic {
    lvp_node_mounts_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
    lvp_share_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterStorageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_node_mounts_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_share_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
    dynamic: GkeonpremBareMetalAdminClusterStorageElDynamic,
}

impl GkeonpremBareMetalAdminClusterStorageEl {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterStorageEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterStorageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterStorageEl {}

impl BuildGkeonpremBareMetalAdminClusterStorageEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterStorageEl {
        GkeonpremBareMetalAdminClusterStorageEl {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterStorageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterStorageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterStorageElRef {
        GkeonpremBareMetalAdminClusterStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterStorageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStorageElLvpNodeMountsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_node_mounts_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(&self) -> terrars::ListRef<GkeonpremBareMetalAdminClusterStorageElLvpShareConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_share_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalAdminClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalAdminClusterTimeoutsEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalAdminClusterTimeoutsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalAdminClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalAdminClusterTimeoutsEl {}

impl BuildGkeonpremBareMetalAdminClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremBareMetalAdminClusterTimeoutsEl {
        GkeonpremBareMetalAdminClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalAdminClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalAdminClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalAdminClusterTimeoutsElRef {
        GkeonpremBareMetalAdminClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalAdminClusterTimeoutsElRef {
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
struct GkeonpremBareMetalAdminClusterDynamic {
    cluster_operations: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterClusterOperationsEl>>,
    control_plane: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterControlPlaneEl>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterLoadBalancerEl>>,
    maintenance_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterMaintenanceConfigEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterNetworkConfigEl>>,
    node_access_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterNodeAccessConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterNodeConfigEl>>,
    proxy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterProxyEl>>,
    security_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterSecurityConfigEl>>,
    storage: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalAdminClusterStorageEl>>,
}
