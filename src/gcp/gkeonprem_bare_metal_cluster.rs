use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremBareMetalClusterData {
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
    admin_cluster_membership: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    bare_metal_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_operations: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterClusterOperationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_access_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_environment_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterProxyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterSecurityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterStorageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremBareMetalClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_policy: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterUpgradePolicyEl>>,
    dynamic: GkeonpremBareMetalClusterDynamic,
}

struct GkeonpremBareMetalCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremBareMetalClusterData>,
}

#[derive(Clone)]
pub struct GkeonpremBareMetalCluster(std::rc::Rc<GkeonpremBareMetalCluster_>);

impl GkeonpremBareMetalCluster {
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
        "Set the field `annotations`.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human readable description of this Bare Metal User Cluster."]
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

    #[doc = "Set the field `binary_authorization`.\n"]
    pub fn set_binary_authorization(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
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

    #[doc = "Set the field `cluster_operations`.\n"]
    pub fn set_cluster_operations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterClusterOperationsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterNodeConfigEl>>,
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

    #[doc = "Set the field `os_environment_config`.\n"]
    pub fn set_os_environment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().os_environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.os_environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy`.\n"]
    pub fn set_proxy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterProxyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterStorageEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremBareMetalClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterUpgradePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub fn admin_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
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
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal User Cluster's API server."]
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal User Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal User Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal User Cluster."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalClusterValidationCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<GkeonpremBareMetalClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalClusterClusterOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterMaintenanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNodeAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterOsEnvironmentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalClusterProxyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremBareMetalClusterUpgradePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }
}

impl terrars::Referable for GkeonpremBareMetalCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeonpremBareMetalCluster { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster {
    type O = terrars::ListRef<GkeonpremBareMetalClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeonpremBareMetalCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gkeonprem_bare_metal_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeonpremBareMetalCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub admin_cluster_membership: terrars::PrimField<std::string::String>,
    #[doc = "A human readable description of this Bare Metal User Cluster."]
    pub bare_metal_version: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The bare metal cluster name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeonpremBareMetalCluster {
        let out = GkeonpremBareMetalCluster(std::rc::Rc::new(GkeonpremBareMetalCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeonpremBareMetalClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_cluster_membership: self.admin_cluster_membership,
                annotations: core::default::Default::default(),
                bare_metal_version: self.bare_metal_version,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                binary_authorization: core::default::Default::default(),
                cluster_operations: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                maintenance_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                node_access_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                os_environment_config: core::default::Default::default(),
                proxy: core::default::Default::default(),
                security_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeonpremBareMetalClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeonpremBareMetalClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe Admin Cluster this Bare Metal User Cluster belongs to.\nThis is the full resource name of the Admin Cluster's hub membership."]
    pub fn admin_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the Bare Metal User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bare_metal_version` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
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
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this Bare Metal User Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address name of Bare Metal User Cluster's API server."]
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalClusterFleetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the Bare Metal Cluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe bare metal cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the Bare Metal User Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nSpecifies detailed cluster status."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the Bare Metal User Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the cluster was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nSpecifies the security related settings for the Bare Metal User Cluster."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalClusterValidationCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<GkeonpremBareMetalClusterBinaryAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalClusterClusterOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalClusterControlPlaneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterMaintenanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNodeAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterOsEnvironmentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalClusterProxyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremBareMetalClusterUpgradePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterFleetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterFleetEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterFleetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterFleetEl {}

impl BuildGkeonpremBareMetalClusterFleetEl {
    pub fn build(self) -> GkeonpremBareMetalClusterFleetEl {
        GkeonpremBareMetalClusterFleetEl { membership: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterFleetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterFleetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterFleetElRef {
        GkeonpremBareMetalClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterFleetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStatusElConditionsEl {
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

impl GkeonpremBareMetalClusterStatusElConditionsEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterStatusElConditionsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStatusElConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStatusElConditionsEl {}

impl BuildGkeonpremBareMetalClusterStatusElConditionsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStatusElConditionsEl {
        GkeonpremBareMetalClusterStatusElConditionsEl {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterStatusElConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStatusElConditionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterStatusElConditionsElRef {
        GkeonpremBareMetalClusterStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStatusElConditionsElRef {
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
pub struct GkeonpremBareMetalClusterStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremBareMetalClusterStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalClusterStatusElConditionsEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterStatusEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStatusEl {}

impl BuildGkeonpremBareMetalClusterStatusEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStatusEl {
        GkeonpremBareMetalClusterStatusEl {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterStatusElRef {
        GkeonpremBareMetalClusterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStatusElConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
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

impl GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterValidationCheckElStatusElResultEl {}

impl BuildGkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
        GkeonpremBareMetalClusterValidationCheckElStatusElResultEl {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
        GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef {
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
pub struct GkeonpremBareMetalClusterValidationCheckElStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>>,
}

impl GkeonpremBareMetalClusterValidationCheckElStatusEl {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalClusterValidationCheckElStatusElResultEl>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterValidationCheckElStatusEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterValidationCheckElStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterValidationCheckElStatusEl {}

impl BuildGkeonpremBareMetalClusterValidationCheckElStatusEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckElStatusEl {
        GkeonpremBareMetalClusterValidationCheckElStatusEl { result: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterValidationCheckElStatusElRef {
        GkeonpremBareMetalClusterValidationCheckElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterValidationCheckElStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<GkeonpremBareMetalClusterValidationCheckElStatusElResultElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterValidationCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremBareMetalClusterValidationCheckElStatusEl>>,
}

impl GkeonpremBareMetalClusterValidationCheckEl {
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
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalClusterValidationCheckElStatusEl>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterValidationCheckEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterValidationCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterValidationCheckEl {}

impl BuildGkeonpremBareMetalClusterValidationCheckEl {
    pub fn build(self) -> GkeonpremBareMetalClusterValidationCheckEl {
        GkeonpremBareMetalClusterValidationCheckEl {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterValidationCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterValidationCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterValidationCheckElRef {
        GkeonpremBareMetalClusterValidationCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterValidationCheckElRef {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalClusterValidationCheckElStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterBinaryAuthorizationEl {
    #[doc =
        "Set the field `evaluation_mode`.\nMode of operation for binauthz policy evaluation. If unspecified,\ndefaults to DISABLED. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterBinaryAuthorizationEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterBinaryAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterBinaryAuthorizationEl {}

impl BuildGkeonpremBareMetalClusterBinaryAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalClusterBinaryAuthorizationEl {
        GkeonpremBareMetalClusterBinaryAuthorizationEl { evaluation_mode: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterBinaryAuthorizationElRef {
        GkeonpremBareMetalClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for binauthz policy evaluation. If unspecified,\ndefaults to DISABLED. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterClusterOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_application_logs: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalClusterClusterOperationsEl {
    #[doc =
        "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_application_logs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterClusterOperationsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterClusterOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterClusterOperationsEl {}

impl BuildGkeonpremBareMetalClusterClusterOperationsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterClusterOperationsEl {
        GkeonpremBareMetalClusterClusterOperationsEl { enable_application_logs: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterClusterOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterClusterOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterClusterOperationsElRef {
        GkeonpremBareMetalClusterClusterOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterClusterOperationsElRef {
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
pub struct GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    argument: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    #[doc =
        "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: terrars::PrimField<std::string::String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
        GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl {
            argument: self.argument,
            value: self.value,
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
        GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef {
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
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
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
::ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {}

impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
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
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
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
::ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {}

impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef {
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
struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
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
        ::Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std::vec::Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl>,
    >,
    dynamic: GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsEl,
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
                            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    type O =
        terrars
        ::BlockAssignable<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef {
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
    ::ListRef<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElNodeConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
        GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterControlPlaneElDynamic {
    api_server_args: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
    control_plane_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterControlPlaneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_server_args: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalClusterControlPlaneElDynamic,
}

impl GkeonpremBareMetalClusterControlPlaneEl {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneElApiServerArgsEl>>,
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
                        terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigEl>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterControlPlaneEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterControlPlaneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterControlPlaneEl {}

impl BuildGkeonpremBareMetalClusterControlPlaneEl {
    pub fn build(self) -> GkeonpremBareMetalClusterControlPlaneEl {
        GkeonpremBareMetalClusterControlPlaneEl {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterControlPlaneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterControlPlaneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterControlPlaneElRef {
        GkeonpremBareMetalClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterControlPlaneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(&self) -> terrars::ListRef<GkeonpremBareMetalClusterControlPlaneElApiServerArgsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_server_args", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterControlPlaneElControlPlaneNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avoid_buggy_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_assign: std::option::Option<terrars::PrimField<std::string::String>>,
    pool: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[doc =
        "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
    pub fn set_avoid_buggy_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.avoid_buggy_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manual_assign`.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn set_manual_assign(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manual_assign = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    #[doc =
        "The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the address pool."]
    pub pool: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
    pub fn avoid_buggy_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.avoid_buggy_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `manual_assign` after provisioning.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn manual_assign(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manual_assign", self.base))
    }

    #[doc = "Get a reference to the value of field `pool` after provisioning.\nThe name of the address pool."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_nodes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ip_address: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[doc =
        "Set the field `control_plane_nodes`.\nThe IP address of the control plane node that connects to the external\npeer.\nIf you don't specify any control plane nodes, all control plane nodes\ncan connect to the external peer. If you specify one or more IP addresses,\nonly the nodes specified participate in peering sessions."]
    pub fn set_control_plane_nodes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.control_plane_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    #[doc = "BGP autonomous system number (ASN) for the network that contains the\nexternal peer device."]
    pub asn: terrars::PrimField<f64>,
    #[doc = "The IP address of the external peer device."]
    pub ip_address: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl {
            asn: self.asn,
            control_plane_nodes: core::default::Default::default(),
            ip_address: self.ip_address,
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) for the network that contains the\nexternal peer device."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_nodes` after provisioning.\nThe IP address of the control plane node that connects to the external\npeer.\nIf you don't specify any control plane nodes, all control plane nodes\ncan connect to the external peer. If you specify one or more IP addresses,\nonly the nodes specified participate in peering sessions."]
    pub fn control_plane_nodes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the external peer device."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_burst: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_pull_qps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialize_image_pulls_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
    #[doc =
        "Set the field `registry_burst`.\nThe maximum size of bursty pulls, temporarily allows pulls to burst to this\nnumber, while still not exceeding registry_pull_qps.\nThe value must not be a negative number.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 10."]
    pub fn set_registry_burst(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.registry_burst = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `registry_pull_qps`.\nThe limit of registry pulls per second.\nSetting this value to 0 means no limit.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 5."]
    pub fn set_registry_pull_qps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.registry_pull_qps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `serialize_image_pulls_disabled`.\nPrevents the Kubelet from pulling multiple images at a time.\nWe recommend *not* changing the default value on nodes that run docker\ndaemon with version  < 1.9 or an Another Union File System (Aufs) storage\nbackend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has\nmore details."]
    pub fn set_serialize_image_pulls_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.serialize_image_pulls_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl {
            registry_burst: core::default::Default::default(),
            registry_pull_qps: core::default::Default::default(),
            serialize_image_pulls_disabled: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `registry_burst` after provisioning.\nThe maximum size of bursty pulls, temporarily allows pulls to burst to this\nnumber, while still not exceeding registry_pull_qps.\nThe value must not be a negative number.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 10."]
    pub fn registry_burst(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_burst", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry_pull_qps` after provisioning.\nThe limit of registry pulls per second.\nSetting this value to 0 means no limit.\nUpdating this field may impact scalability by changing the amount of\ntraffic produced by image pulls.\nDefaults to 5."]
    pub fn registry_pull_qps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_pull_qps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serialize_image_pulls_disabled` after provisioning.\nPrevents the Kubelet from pulling multiple images at a time.\nWe recommend *not* changing the default value on nodes that run docker\ndaemon with version  < 1.9 or an Another Union File System (Aufs) storage\nbackend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has\nmore details."]
    pub fn serialize_image_pulls_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialize_image_pulls_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
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
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
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
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic {
    kubelet_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl,
        >,
    >,
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
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
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigEl,
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
                            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
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
                            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
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

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElKubeletConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]
    pub fn node_configs(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    type O =
        terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>>,
    bgp_peer_configs: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>>,
    load_balancer_node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_peer_configs: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsEl>,
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
                        terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsEl>,
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
                            GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    #[doc = "BGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub asn: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl {
            asn: self.asn,
            address_pools: core::default::Default::default(),
            bgp_peer_configs: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElAddressPoolsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_peer_configs` after provisioning.\n"]
    pub fn bgp_peer_configs(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElBgpPeerConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_peer_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElLoadBalancerNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl { enabled: self.enabled }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef {
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
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avoid_buggy_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_assign: std::option::Option<terrars::PrimField<bool>>,
    pool: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc =
        "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
    pub fn set_avoid_buggy_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.avoid_buggy_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manual_assign`.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn set_manual_assign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.manual_assign = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    #[doc =
        "The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the address pool."]
    pub pool: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses."]
    pub fn avoid_buggy_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.avoid_buggy_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `manual_assign` after provisioning.\nIf true, prevent IP addresses from being automatically assigned."]
    pub fn manual_assign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manual_assign", self.base))
    }

    #[doc = "Get a reference to the value of field `pool` after provisioning.\nThe name of the address pool."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef {
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
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef {
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
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
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
        ::Vec<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
        >,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsEl,
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
                            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsEl,
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
::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef {
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
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElNodeConfigsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElTaintsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl>,
    >,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
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
                            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
    type O =
        terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    load_balancer_node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl>>,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsEl>,
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
                            GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigEl,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl {
            address_pools: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElAddressPoolsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElLoadBalancerNodePoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl GkeonpremBareMetalClusterLoadBalancerElPortConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElPortConfigEl {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef {
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
pub struct GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    control_plane_vip: terrars::PrimField<std::string::String>,
    ingress_vip: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterLoadBalancerElVipConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster."]
    pub control_plane_vip: terrars::PrimField<std::string::String>,
    #[doc = "The VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster."]
    pub ingress_vip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
        GkeonpremBareMetalClusterLoadBalancerElVipConfigEl {
            control_plane_vip: self.control_plane_vip,
            ingress_vip: self.ingress_vip,
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
        GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster."]
    pub fn control_plane_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_vip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_vip` after provisioning.\nThe VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster."]
    pub fn ingress_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_vip", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterLoadBalancerElDynamic {
    bgp_lb_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
    manual_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
    metal_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
    port_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterLoadBalancerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metal_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
    dynamic: GkeonpremBareMetalClusterLoadBalancerElDynamic,
}

impl GkeonpremBareMetalClusterLoadBalancerEl {
    #[doc = "Set the field `bgp_lb_config`.\n"]
    pub fn set_bgp_lb_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigEl>>,
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
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigEl>>,
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

    #[doc = "Set the field `port_config`.\n"]
    pub fn set_port_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElPortConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerElVipConfigEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterLoadBalancerEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterLoadBalancerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterLoadBalancerEl {}

impl BuildGkeonpremBareMetalClusterLoadBalancerEl {
    pub fn build(self) -> GkeonpremBareMetalClusterLoadBalancerEl {
        GkeonpremBareMetalClusterLoadBalancerEl {
            bgp_lb_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterLoadBalancerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterLoadBalancerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterLoadBalancerElRef {
        GkeonpremBareMetalClusterLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterLoadBalancerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_lb_config` after provisioning.\n"]
    pub fn bgp_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElBgpLbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElManualLbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElMetalLbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metal_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElPortConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterLoadBalancerElVipConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterMaintenanceConfigEl {
    maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterMaintenanceConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterMaintenanceConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterMaintenanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterMaintenanceConfigEl {
    #[doc =
        "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalClusterMaintenanceConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterMaintenanceConfigEl {
        GkeonpremBareMetalClusterMaintenanceConfigEl {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}

pub struct GkeonpremBareMetalClusterMaintenanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterMaintenanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterMaintenanceConfigElRef {
        GkeonpremBareMetalClusterMaintenanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterMaintenanceConfigElRef {
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
pub struct GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
        GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}

pub struct GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
        GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef {
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
pub struct GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    #[doc =
        "Set the field `enabled`.\nWhether to enable multiple network interfaces for your pods.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {}

impl BuildGkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
        GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable multiple network interfaces for your pods.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    #[doc = "Set the field `enabled`.\nWhether to install the SR-IOV operator."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {}

impl BuildGkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl {
        GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to install the SR-IOV operator."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterNetworkConfigElDynamic {
    island_mode_cidr: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
    multiple_network_interfaces_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>>,
    sr_iov_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    island_mode_cidr: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiple_network_interfaces_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sr_iov_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
    dynamic: GkeonpremBareMetalClusterNetworkConfigElDynamic,
}

impl GkeonpremBareMetalClusterNetworkConfigEl {
    #[doc =
        "Set the field `advanced_networking`.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
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
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrEl>>,
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
                        ::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigEl>,
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

    #[doc = "Set the field `sr_iov_config`.\n"]
    pub fn set_sr_iov_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sr_iov_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sr_iov_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterNetworkConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNetworkConfigEl {}

impl BuildGkeonpremBareMetalClusterNetworkConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNetworkConfigEl {
        GkeonpremBareMetalClusterNetworkConfigEl {
            advanced_networking: core::default::Default::default(),
            island_mode_cidr: core::default::Default::default(),
            multiple_network_interfaces_config: core::default::Default::default(),
            sr_iov_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterNetworkConfigElRef {
        GkeonpremBareMetalClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `advanced_networking` after provisioning.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
    pub fn advanced_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNetworkConfigElIslandModeCidrElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.island_mode_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `multiple_network_interfaces_config` after provisioning.\n"]
    pub fn multiple_network_interfaces_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterNetworkConfigElMultipleNetworkInterfacesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiple_network_interfaces_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sr_iov_config` after provisioning.\n"]
    pub fn sr_iov_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterNetworkConfigElSrIovConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sr_iov_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterNodeAccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterNodeAccessConfigEl {
    #[doc =
        "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterNodeAccessConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNodeAccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNodeAccessConfigEl {}

impl BuildGkeonpremBareMetalClusterNodeAccessConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNodeAccessConfigEl {
        GkeonpremBareMetalClusterNodeAccessConfigEl { login_user: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterNodeAccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNodeAccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterNodeAccessConfigElRef {
        GkeonpremBareMetalClusterNodeAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNodeAccessConfigElRef {
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
pub struct GkeonpremBareMetalClusterNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremBareMetalClusterNodeConfigEl {
    #[doc =
        "Set the field `container_runtime`.\nThe available runtimes that can be used to run containers in a Bare Metal User Cluster. Possible values: [\"CONTAINER_RUNTIME_UNSPECIFIED\", \"DOCKER\", \"CONTAINERD\"]"]
    pub fn set_container_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_pods_per_node`.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn set_max_pods_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pods_per_node = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterNodeConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterNodeConfigEl {}

impl BuildGkeonpremBareMetalClusterNodeConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterNodeConfigEl {
        GkeonpremBareMetalClusterNodeConfigEl {
            container_runtime: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterNodeConfigElRef {
        GkeonpremBareMetalClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_runtime` after provisioning.\nThe available runtimes that can be used to run containers in a Bare Metal User Cluster. Possible values: [\"CONTAINER_RUNTIME_UNSPECIFIED\", \"DOCKER\", \"CONTAINERD\"]"]
    pub fn container_runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_runtime", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_pods_per_node` after provisioning.\nThe maximum number of pods a node can run. The size of the CIDR range\nassigned to the node will be derived from this parameter."]
    pub fn max_pods_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pods_per_node", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterOsEnvironmentConfigEl {
    package_repo_excluded: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalClusterOsEnvironmentConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterOsEnvironmentConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterOsEnvironmentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterOsEnvironmentConfigEl {
    #[doc = "Whether the package repo should not be included when initializing\nbare metal machines."]
    pub package_repo_excluded: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalClusterOsEnvironmentConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterOsEnvironmentConfigEl {
        GkeonpremBareMetalClusterOsEnvironmentConfigEl { package_repo_excluded: self.package_repo_excluded }
    }
}

pub struct GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
        GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterOsEnvironmentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `package_repo_excluded` after provisioning.\nWhether the package repo should not be included when initializing\nbare metal machines."]
    pub fn package_repo_excluded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_repo_excluded", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterProxyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_proxy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterProxyEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterProxyEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterProxyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterProxyEl {
    #[doc =
        "Specifies the address of your proxy server.\nExamples: http://domain\nWARNING: Do not provide credentials in the format\nhttp://(username:password@)domain these will be rejected by the server."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterProxyEl {
    pub fn build(self) -> GkeonpremBareMetalClusterProxyEl {
        GkeonpremBareMetalClusterProxyEl {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct GkeonpremBareMetalClusterProxyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterProxyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterProxyElRef {
        GkeonpremBareMetalClusterProxyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterProxyElRef {
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
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl { username: self.username }
    }
}

pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef {
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
struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElDynamic {
    admin_users: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>>,
    dynamic: GkeonpremBareMetalClusterSecurityConfigElAuthorizationElDynamic,
}

impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersEl>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {}

impl BuildGkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
        GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElAdminUsersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterSecurityConfigElDynamic {
    authorization: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterSecurityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
    dynamic: GkeonpremBareMetalClusterSecurityConfigElDynamic,
}

impl GkeonpremBareMetalClusterSecurityConfigEl {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigElAuthorizationEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterSecurityConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterSecurityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterSecurityConfigEl {}

impl BuildGkeonpremBareMetalClusterSecurityConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterSecurityConfigEl {
        GkeonpremBareMetalClusterSecurityConfigEl {
            authorization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterSecurityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterSecurityConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterSecurityConfigElRef {
        GkeonpremBareMetalClusterSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterSecurityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremBareMetalClusterSecurityConfigElAuthorizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
        GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

pub struct GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef {
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
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl { }

impl terrars::ToListMappable for GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef {
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
struct GkeonpremBareMetalClusterStorageElLvpShareConfigElDynamic {
    lvp_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_path_pv_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
    dynamic: GkeonpremBareMetalClusterStorageElLvpShareConfigElDynamic,
}

impl GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
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
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStorageElLvpShareConfigEl {}

impl BuildGkeonpremBareMetalClusterStorageElLvpShareConfigEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
        GkeonpremBareMetalClusterStorageElLvpShareConfigEl {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
        GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStorageElLvpShareConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `shared_path_pv_count` after provisioning.\nThe number of subdirectories to create under path."]
    pub fn shared_path_pv_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_path_pv_count", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_config` after provisioning.\n"]
    pub fn lvp_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStorageElLvpShareConfigElLvpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterStorageElDynamic {
    lvp_node_mounts_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
    lvp_share_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterStorageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_node_mounts_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_share_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
    dynamic: GkeonpremBareMetalClusterStorageElDynamic,
}

impl GkeonpremBareMetalClusterStorageEl {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalClusterStorageElLvpShareConfigEl>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterStorageEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterStorageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterStorageEl {}

impl BuildGkeonpremBareMetalClusterStorageEl {
    pub fn build(self) -> GkeonpremBareMetalClusterStorageEl {
        GkeonpremBareMetalClusterStorageEl {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterStorageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterStorageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterStorageElRef {
        GkeonpremBareMetalClusterStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterStorageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStorageElLvpNodeMountsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_node_mounts_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(&self) -> terrars::ListRef<GkeonpremBareMetalClusterStorageElLvpShareConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_share_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterTimeoutsEl {
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

impl terrars::ToListMappable for GkeonpremBareMetalClusterTimeoutsEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterTimeoutsEl {}

impl BuildGkeonpremBareMetalClusterTimeoutsEl {
    pub fn build(self) -> GkeonpremBareMetalClusterTimeoutsEl {
        GkeonpremBareMetalClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeonpremBareMetalClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterTimeoutsElRef {
        GkeonpremBareMetalClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterTimeoutsElRef {
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
pub struct GkeonpremBareMetalClusterUpgradePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalClusterUpgradePolicyEl {
    #[doc =
        "Set the field `policy`.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalClusterUpgradePolicyEl {
    type O = terrars::BlockAssignable<GkeonpremBareMetalClusterUpgradePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalClusterUpgradePolicyEl {}

impl BuildGkeonpremBareMetalClusterUpgradePolicyEl {
    pub fn build(self) -> GkeonpremBareMetalClusterUpgradePolicyEl {
        GkeonpremBareMetalClusterUpgradePolicyEl { policy: core::default::Default::default() }
    }
}

pub struct GkeonpremBareMetalClusterUpgradePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalClusterUpgradePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalClusterUpgradePolicyElRef {
        GkeonpremBareMetalClusterUpgradePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalClusterUpgradePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterDynamic {
    binary_authorization: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterBinaryAuthorizationEl>>,
    cluster_operations: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterClusterOperationsEl>>,
    control_plane: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterControlPlaneEl>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterLoadBalancerEl>>,
    maintenance_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterMaintenanceConfigEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNetworkConfigEl>>,
    node_access_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNodeAccessConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterNodeConfigEl>>,
    os_environment_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterOsEnvironmentConfigEl>>,
    proxy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterProxyEl>>,
    security_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterSecurityConfigEl>>,
    storage: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterStorageEl>>,
    upgrade_policy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalClusterUpgradePolicyEl>>,
}
