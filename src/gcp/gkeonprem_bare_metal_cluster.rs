#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremBareMetalClusterData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_BinaryAuthorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_operations: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_ClusterOperations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_MaintenanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_access_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_NodeAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_environment_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_OsEnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_Proxy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_SecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_Storage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremBareMetalCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_policy: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_UpgradePolicy>>,
    dynamic: GkeonpremBareMetalClusterDynamic,
}

struct GkeonpremBareMetalCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremBareMetalClusterData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_BinaryAuthorization>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_ClusterOperations>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_MaintenanceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_NodeAccessConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_NodeConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_OsEnvironmentConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_Proxy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremBareMetalCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_UpgradePolicy>>,
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_FleetRef> {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_StatusRef> {
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
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ClusterOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_MaintenanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NodeAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_OsEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalCluster_TimeoutsRef {
        GkeonpremBareMetalCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_UpgradePolicyRef> {
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
            data: std::cell::RefCell::new(GkeonpremBareMetalClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_cluster_membership: self.admin_cluster_membership,
                annotations: core::default::Default::default(),
                bare_metal_version: self.bare_metal_version,
                deletion_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_FleetRef> {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_StatusRef> {
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
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\n"]
    pub fn binary_authorization(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_BinaryAuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_operations` after provisioning.\n"]
    pub fn cluster_operations(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ClusterOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlaneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_MaintenanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_access_config` after provisioning.\n"]
    pub fn node_access_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NodeAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_environment_config` after provisioning.\n"]
    pub fn os_environment_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_OsEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy` after provisioning.\n"]
    pub fn proxy(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ProxyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremBareMetalCluster_TimeoutsRef {
        GkeonpremBareMetalCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalClusterDynamic {
    binary_authorization: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_BinaryAuthorization>>,
    cluster_operations: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_ClusterOperations>>,
    control_plane: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer>>,
    maintenance_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_MaintenanceConfig>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NetworkConfig>>,
    node_access_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NodeAccessConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NodeConfig>>,
    os_environment_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_OsEnvironmentConfig>>,
    proxy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_Proxy>>,
    security_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_SecurityConfig>>,
    storage: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_Storage>>,
    upgrade_policy: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_UpgradePolicy>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_Fleet {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Fleet {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Fleet {}

impl BuildGkeonpremBareMetalCluster_Fleet {
    pub fn build(self) -> GkeonpremBareMetalCluster_Fleet {
        GkeonpremBareMetalCluster_Fleet { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_FleetRef {
        GkeonpremBareMetalCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_FleetRef {
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
pub struct GkeonpremBareMetalCluster_Status_Conditions {
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

impl GkeonpremBareMetalCluster_Status_Conditions {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Status_Conditions {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Status_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Status_Conditions {}

impl BuildGkeonpremBareMetalCluster_Status_Conditions {
    pub fn build(self) -> GkeonpremBareMetalCluster_Status_Conditions {
        GkeonpremBareMetalCluster_Status_Conditions {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_Status_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_Status_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_Status_ConditionsRef {
        GkeonpremBareMetalCluster_Status_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_Status_ConditionsRef {
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
pub struct GkeonpremBareMetalCluster_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremBareMetalCluster_Status_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_Status {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalCluster_Status_Conditions>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Status {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Status {}

impl BuildGkeonpremBareMetalCluster_Status {
    pub fn build(self) -> GkeonpremBareMetalCluster_Status {
        GkeonpremBareMetalCluster_Status {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_StatusRef {
        GkeonpremBareMetalCluster_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_Status_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_ValidationCheck_Status_Result {
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

impl GkeonpremBareMetalCluster_ValidationCheck_Status_Result {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ValidationCheck_Status_Result {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ValidationCheck_Status_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ValidationCheck_Status_Result {}

impl BuildGkeonpremBareMetalCluster_ValidationCheck_Status_Result {
    pub fn build(self) -> GkeonpremBareMetalCluster_ValidationCheck_Status_Result {
        GkeonpremBareMetalCluster_ValidationCheck_Status_Result {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef {
        GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef {
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
pub struct GkeonpremBareMetalCluster_ValidationCheck_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<GkeonpremBareMetalCluster_ValidationCheck_Status_Result>>,
}

impl GkeonpremBareMetalCluster_ValidationCheck_Status {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalCluster_ValidationCheck_Status_Result>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ValidationCheck_Status {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ValidationCheck_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ValidationCheck_Status {}

impl BuildGkeonpremBareMetalCluster_ValidationCheck_Status {
    pub fn build(self) -> GkeonpremBareMetalCluster_ValidationCheck_Status {
        GkeonpremBareMetalCluster_ValidationCheck_Status { result: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ValidationCheck_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ValidationCheck_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ValidationCheck_StatusRef {
        GkeonpremBareMetalCluster_ValidationCheck_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ValidationCheck_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ValidationCheck_Status_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_ValidationCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremBareMetalCluster_ValidationCheck_Status>>,
}

impl GkeonpremBareMetalCluster_ValidationCheck {
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
        v: impl std::convert::Into<terrars::ListField<GkeonpremBareMetalCluster_ValidationCheck_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ValidationCheck {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ValidationCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ValidationCheck {}

impl BuildGkeonpremBareMetalCluster_ValidationCheck {
    pub fn build(self) -> GkeonpremBareMetalCluster_ValidationCheck {
        GkeonpremBareMetalCluster_ValidationCheck {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ValidationCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ValidationCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_ValidationCheckRef {
        GkeonpremBareMetalCluster_ValidationCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ValidationCheckRef {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ValidationCheck_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_BinaryAuthorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_BinaryAuthorization {
    #[doc =
        "Set the field `evaluation_mode`.\nMode of operation for binauthz policy evaluation. If unspecified,\ndefaults to DISABLED. Possible values: [\"DISABLED\", \"PROJECT_SINGLETON_POLICY_ENFORCE\"]"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_BinaryAuthorization {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_BinaryAuthorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_BinaryAuthorization {}

impl BuildGkeonpremBareMetalCluster_BinaryAuthorization {
    pub fn build(self) -> GkeonpremBareMetalCluster_BinaryAuthorization {
        GkeonpremBareMetalCluster_BinaryAuthorization { evaluation_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_BinaryAuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_BinaryAuthorizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_BinaryAuthorizationRef {
        GkeonpremBareMetalCluster_BinaryAuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_BinaryAuthorizationRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_ClusterOperations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_application_logs: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalCluster_ClusterOperations {
    #[doc =
        "Set the field `enable_application_logs`.\nWhether collection of application logs/metrics should be enabled (in addition to system logs/metrics)."]
    pub fn set_enable_application_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_application_logs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ClusterOperations {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ClusterOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ClusterOperations {}

impl BuildGkeonpremBareMetalCluster_ClusterOperations {
    pub fn build(self) -> GkeonpremBareMetalCluster_ClusterOperations {
        GkeonpremBareMetalCluster_ClusterOperations { enable_application_logs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ClusterOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ClusterOperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ClusterOperationsRef {
        GkeonpremBareMetalCluster_ClusterOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ClusterOperationsRef {
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
pub struct GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
    argument: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
    #[doc =
        "The argument name as it appears on the API Server command line please make sure to remove the leading dashes."]
    pub argument: terrars::PrimField<std::string::String>,
    #[doc = "The value of the arg as it will be passed to the API Server command line."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
    pub fn build(self) -> GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
        GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs {
            argument: self.argument,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef {
        GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef {
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
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[doc =
        "Set the field `labels`.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
::ToListMappable for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {}

impl BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe map of Kubernetes labels (key/value pairs) to be applied to\neach node. These will added in addition to any default label(s)\nthat Kubernetes may apply to the node. In case of conflict in\nlabel keys, the applied set may differ depending on the Kubernetes\nversion -- it's best to assume the behavior is undefined and\nconflicts should be avoided. For more information, including usage\nand the valid values, see:\n  - http://kubernetes.io/v1.1/docs/user-guide/labels.html\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
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
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
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
::ToListMappable for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    type O =
        terrars
        ::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {}

impl BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
    pub fn build(self) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef {
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
struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs>,
    >,
    taints: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_configs: std
    ::option
    ::Option<
        std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>>,
    dynamic: GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
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
                            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigs,
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
                            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    type O =
        terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef {
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
    ::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_NodeConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig>>,
    dynamic: GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
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
                            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
        GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_ControlPlaneDynamic {
    api_server_args: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs>>,
    control_plane_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_ControlPlane {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_server_args: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig>>,
    dynamic: GkeonpremBareMetalCluster_ControlPlaneDynamic,
}

impl GkeonpremBareMetalCluster_ControlPlane {
    #[doc = "Set the field `api_server_args`.\n"]
    pub fn set_api_server_args(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ApiServerArgs>>,
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
                        terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_ControlPlane {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_ControlPlane>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_ControlPlane {}

impl BuildGkeonpremBareMetalCluster_ControlPlane {
    pub fn build(self) -> GkeonpremBareMetalCluster_ControlPlane {
        GkeonpremBareMetalCluster_ControlPlane {
            api_server_args: core::default::Default::default(),
            control_plane_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ControlPlaneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ControlPlaneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_ControlPlaneRef {
        GkeonpremBareMetalCluster_ControlPlaneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ControlPlaneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_server_args` after provisioning.\n"]
    pub fn api_server_args(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ApiServerArgsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_server_args", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_node_pool_config` after provisioning.\n"]
    pub fn control_plane_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
    addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avoid_buggy_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_assign: std::option::Option<terrars::PrimField<bool>>,
    pool: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
    #[doc =
        "The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5)."]
    pub addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the address pool."]
    pub pool: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_nodes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ip_address: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    #[doc = "BGP autonomous system number (ASN) for the network that contains the\nexternal peer device."]
    pub asn: terrars::PrimField<f64>,
    #[doc = "The IP address of the external peer device."]
    pub ip_address: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs {
            asn: self.asn,
            control_plane_nodes: core::default::Default::default(),
            ip_address: self.ip_address,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_burst: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_pull_qps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialize_image_pulls_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
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
::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig {
            registry_burst: core::default::Default::default(),
            registry_pull_qps: core::default::Default::default(),
            serialize_image_pulls_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
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
::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs {
            labels: core::default::Default::default(),
            node_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef {
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

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic {
    kubelet_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >,
    >,
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
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
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>>,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
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
                            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfig,
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
                            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
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
                            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints,
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
::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
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
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
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

    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_KubeletConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubelet_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_configs` after provisioning.\n"]
    pub fn node_configs(
        &self,
    ) -> terrars
    ::ListRef<
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig>,
    >,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
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
                            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    bgp_peer_configs: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>>,
    load_balancer_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
    asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_peer_configs: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig>>,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
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
                        terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigs>,
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
                            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
    #[doc = "BGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub asn: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig {
            asn: self.asn,
            address_pools: core::default::Default::default(),
            bgp_peer_configs: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nBGP autonomous system number (ASN) of the cluster.\nThis field can be updated after cluster creation."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_peer_configs` after provisioning.\n"]
    pub fn bgp_peer_configs(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_BgpPeerConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_peer_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig {
    #[doc = "Whether manual load balancing is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig {
        GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef {
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

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic {
    node_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    taints: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
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
            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taints: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints>>,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
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
                            GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigs,
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
                            GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_Taints,
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
::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    type O =
        terrars
        ::BlockAssignable<
            GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
    pub fn build(
        self,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig {
            labels: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            node_configs: core::default::Default::default(),
            taints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef {
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
    ::ListRef<
        GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig_NodeConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_ControlPlane_ControlPlaneNodePoolConfig_NodePoolConfig_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig>,
    >,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
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
                            GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfig,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
    type O =
        terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig {
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig_NodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    load_balancer_node_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_node_pool_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig>>,
    dynamic: GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPools>>,
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
                            GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfig,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig {
            address_pools: core::default::Default::default(),
            load_balancer_node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig_AddressPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_node_pool_config` after provisioning.\n"]
    pub fn load_balancer_node_pool_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig_LoadBalancerNodePoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_node_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_PortConfig {
    control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_PortConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_PortConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_PortConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_PortConfig {
    #[doc = "The port that control plane hosted load balancers will listen on."]
    pub control_plane_load_balancer_port: terrars::PrimField<f64>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_PortConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_PortConfig {
        GkeonpremBareMetalCluster_LoadBalancer_PortConfig {
            control_plane_load_balancer_port: self.control_plane_load_balancer_port,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef {
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
pub struct GkeonpremBareMetalCluster_LoadBalancer_VipConfig {
    control_plane_vip: terrars::PrimField<std::string::String>,
    ingress_vip: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_LoadBalancer_VipConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer_VipConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_VipConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer_VipConfig {
    #[doc = "The VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster."]
    pub control_plane_vip: terrars::PrimField<std::string::String>,
    #[doc = "The VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster."]
    pub ingress_vip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_LoadBalancer_VipConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer_VipConfig {
        GkeonpremBareMetalCluster_LoadBalancer_VipConfig {
            control_plane_vip: self.control_plane_vip,
            ingress_vip: self.ingress_vip,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef {
        GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef {
    #[allow(dead_code)]
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
struct GkeonpremBareMetalCluster_LoadBalancerDynamic {
    bgp_lb_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig>>,
    manual_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig>>,
    metal_lb_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig>>,
    port_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_PortConfig>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_LoadBalancer_VipConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_LoadBalancer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metal_lb_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_PortConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_LoadBalancer_VipConfig>>,
    dynamic: GkeonpremBareMetalCluster_LoadBalancerDynamic,
}

impl GkeonpremBareMetalCluster_LoadBalancer {
    #[doc = "Set the field `bgp_lb_config`.\n"]
    pub fn set_bgp_lb_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_PortConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer_VipConfig>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_LoadBalancer {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_LoadBalancer {}

impl BuildGkeonpremBareMetalCluster_LoadBalancer {
    pub fn build(self) -> GkeonpremBareMetalCluster_LoadBalancer {
        GkeonpremBareMetalCluster_LoadBalancer {
            bgp_lb_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            port_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_LoadBalancerRef {
        GkeonpremBareMetalCluster_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_lb_config` after provisioning.\n"]
    pub fn bgp_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_BgpLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_ManualLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_MetalLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metal_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `port_config` after provisioning.\n"]
    pub fn port_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_PortConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_LoadBalancer_VipConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_MaintenanceConfig {
    maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_MaintenanceConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_MaintenanceConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_MaintenanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_MaintenanceConfig {
    #[doc =
        "All IPv4 address from these ranges will be placed into maintenance mode.\nNodes in maintenance mode will be cordoned and drained. When both of these\nare true, the \"baremetal.cluster.gke.io/maintenance\" annotation will be set\non the node resource."]
    pub maintenance_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalCluster_MaintenanceConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_MaintenanceConfig {
        GkeonpremBareMetalCluster_MaintenanceConfig {
            maintenance_address_cidr_blocks: self.maintenance_address_cidr_blocks,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_MaintenanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_MaintenanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_MaintenanceConfigRef {
        GkeonpremBareMetalCluster_MaintenanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_MaintenanceConfigRef {
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
pub struct GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. This field cannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
    pub fn build(self) -> GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
        GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef {
        GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef {
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
pub struct GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    #[doc =
        "Set the field `enabled`.\nWhether to enable multiple network interfaces for your pods.\nWhen set network_config.advanced_networking is automatically\nset to true."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {}

impl BuildGkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
        GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
        GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {
    #[doc = "Set the field `enabled`.\nWhether to install the SR-IOV operator."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {}

impl BuildGkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig {
        GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef {
        GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef {
    #[allow(dead_code)]
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
struct GkeonpremBareMetalCluster_NetworkConfigDynamic {
    island_mode_cidr: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr>>,
    multiple_network_interfaces_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig>>,
    sr_iov_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    island_mode_cidr: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiple_network_interfaces_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sr_iov_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig>>,
    dynamic: GkeonpremBareMetalCluster_NetworkConfigDynamic,
}

impl GkeonpremBareMetalCluster_NetworkConfig {
    #[doc =
        "Set the field `advanced_networking`.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
    pub fn set_advanced_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.advanced_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `island_mode_cidr`.\n"]
    pub fn set_island_mode_cidr(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidr>>,
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
                        ::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfig>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig_SrIovConfig>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NetworkConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NetworkConfig {}

impl BuildGkeonpremBareMetalCluster_NetworkConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_NetworkConfig {
        GkeonpremBareMetalCluster_NetworkConfig {
            advanced_networking: core::default::Default::default(),
            island_mode_cidr: core::default::Default::default(),
            multiple_network_interfaces_config: core::default::Default::default(),
            sr_iov_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_NetworkConfigRef {
        GkeonpremBareMetalCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `advanced_networking` after provisioning.\nEnables the use of advanced Anthos networking features, such as Bundled\nLoad Balancing with BGP or the egress NAT gateway.\nSetting configuration for advanced networking features will automatically\nset this flag."]
    pub fn advanced_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `island_mode_cidr` after provisioning.\n"]
    pub fn island_mode_cidr(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NetworkConfig_IslandModeCidrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.island_mode_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `multiple_network_interfaces_config` after provisioning.\n"]
    pub fn multiple_network_interfaces_config(
        &self,
    ) -> terrars::ListRef<GkeonpremBareMetalCluster_NetworkConfig_MultipleNetworkInterfacesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiple_network_interfaces_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sr_iov_config` after provisioning.\n"]
    pub fn sr_iov_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_NetworkConfig_SrIovConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sr_iov_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_NodeAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_NodeAccessConfig {
    #[doc =
        "Set the field `login_user`.\nLoginUser is the user name used to access node machines.\nIt defaults to \"root\" if not set."]
    pub fn set_login_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NodeAccessConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NodeAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NodeAccessConfig {}

impl BuildGkeonpremBareMetalCluster_NodeAccessConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_NodeAccessConfig {
        GkeonpremBareMetalCluster_NodeAccessConfig { login_user: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NodeAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NodeAccessConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_NodeAccessConfigRef {
        GkeonpremBareMetalCluster_NodeAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NodeAccessConfigRef {
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
pub struct GkeonpremBareMetalCluster_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pods_per_node: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremBareMetalCluster_NodeConfig {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_NodeConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_NodeConfig {}

impl BuildGkeonpremBareMetalCluster_NodeConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_NodeConfig {
        GkeonpremBareMetalCluster_NodeConfig {
            container_runtime: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_NodeConfigRef {
        GkeonpremBareMetalCluster_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_NodeConfigRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_OsEnvironmentConfig {
    package_repo_excluded: terrars::PrimField<bool>,
}

impl GkeonpremBareMetalCluster_OsEnvironmentConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_OsEnvironmentConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_OsEnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_OsEnvironmentConfig {
    #[doc = "Whether the package repo should not be included when initializing\nbare metal machines."]
    pub package_repo_excluded: terrars::PrimField<bool>,
}

impl BuildGkeonpremBareMetalCluster_OsEnvironmentConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_OsEnvironmentConfig {
        GkeonpremBareMetalCluster_OsEnvironmentConfig { package_repo_excluded: self.package_repo_excluded }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_OsEnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_OsEnvironmentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_OsEnvironmentConfigRef {
        GkeonpremBareMetalCluster_OsEnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_OsEnvironmentConfigRef {
    #[allow(dead_code)]
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
pub struct GkeonpremBareMetalCluster_Proxy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_proxy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_Proxy {
    #[doc =
        "Set the field `no_proxy`.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn set_no_proxy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.no_proxy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Proxy {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Proxy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Proxy {
    #[doc =
        "Specifies the address of your proxy server.\nFor example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_Proxy {
    pub fn build(self) -> GkeonpremBareMetalCluster_Proxy {
        GkeonpremBareMetalCluster_Proxy {
            no_proxy: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_ProxyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_ProxyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_ProxyRef {
        GkeonpremBareMetalCluster_ProxyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_ProxyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_proxy` after provisioning.\nA list of IPs, hostnames, and domains that should skip the proxy.\nFor example [\"127.0.0.1\", \"example.com\", \".corp\", \"localhost\"]."]
    pub fn no_proxy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_proxy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nSpecifies the address of your proxy server.\nFor example: http://domain\nWARNING: Do not provide credentials in the format\nof http://(username:password@)domain these will be rejected by the server."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers {
    pub fn build(self) -> GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers {
        GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef {
        GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef {
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
struct GkeonpremBareMetalCluster_SecurityConfig_AuthorizationDynamic {
    admin_users: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_SecurityConfig_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers>>,
    dynamic: GkeonpremBareMetalCluster_SecurityConfig_AuthorizationDynamic,
}

impl GkeonpremBareMetalCluster_SecurityConfig_Authorization {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsers>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_SecurityConfig_Authorization {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_SecurityConfig_Authorization {}

impl BuildGkeonpremBareMetalCluster_SecurityConfig_Authorization {
    pub fn build(self) -> GkeonpremBareMetalCluster_SecurityConfig_Authorization {
        GkeonpremBareMetalCluster_SecurityConfig_Authorization {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef {
        GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_SecurityConfig_Authorization_AdminUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_SecurityConfigDynamic {
    authorization: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_SecurityConfig_Authorization>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_SecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_SecurityConfig_Authorization>>,
    dynamic: GkeonpremBareMetalCluster_SecurityConfigDynamic,
}

impl GkeonpremBareMetalCluster_SecurityConfig {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig_Authorization>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_SecurityConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_SecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_SecurityConfig {}

impl BuildGkeonpremBareMetalCluster_SecurityConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_SecurityConfig {
        GkeonpremBareMetalCluster_SecurityConfig {
            authorization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_SecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_SecurityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_SecurityConfigRef {
        GkeonpremBareMetalCluster_SecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_SecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_SecurityConfig_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
        GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef {
        GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef {
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
pub struct GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
    path: terrars::PrimField<std::string::String>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig { }

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
    #[doc = "The host machine path."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The StorageClass name that PVs will be created with."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
        GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig {
            path: self.path,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef {
        GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef {
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
struct GkeonpremBareMetalCluster_Storage_LvpShareConfigDynamic {
    lvp_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_Storage_LvpShareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_path_pv_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig>>,
    dynamic: GkeonpremBareMetalCluster_Storage_LvpShareConfigDynamic,
}

impl GkeonpremBareMetalCluster_Storage_LvpShareConfig {
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
                    ::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfig>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Storage_LvpShareConfig {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpShareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Storage_LvpShareConfig {}

impl BuildGkeonpremBareMetalCluster_Storage_LvpShareConfig {
    pub fn build(self) -> GkeonpremBareMetalCluster_Storage_LvpShareConfig {
        GkeonpremBareMetalCluster_Storage_LvpShareConfig {
            shared_path_pv_count: core::default::Default::default(),
            lvp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_Storage_LvpShareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_Storage_LvpShareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremBareMetalCluster_Storage_LvpShareConfigRef {
        GkeonpremBareMetalCluster_Storage_LvpShareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_Storage_LvpShareConfigRef {
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
    pub fn lvp_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_Storage_LvpShareConfig_LvpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremBareMetalCluster_StorageDynamic {
    lvp_node_mounts_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig>>,
    lvp_share_config: std::option::Option<terrars::DynamicBlock<GkeonpremBareMetalCluster_Storage_LvpShareConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_Storage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_node_mounts_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lvp_share_config: std::option::Option<std::vec::Vec<GkeonpremBareMetalCluster_Storage_LvpShareConfig>>,
    dynamic: GkeonpremBareMetalCluster_StorageDynamic,
}

impl GkeonpremBareMetalCluster_Storage {
    #[doc = "Set the field `lvp_node_mounts_config`.\n"]
    pub fn set_lvp_node_mounts_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage_LvpShareConfig>>,
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Storage {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Storage {}

impl BuildGkeonpremBareMetalCluster_Storage {
    pub fn build(self) -> GkeonpremBareMetalCluster_Storage {
        GkeonpremBareMetalCluster_Storage {
            lvp_node_mounts_config: core::default::Default::default(),
            lvp_share_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_StorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_StorageRef {
        GkeonpremBareMetalCluster_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lvp_node_mounts_config` after provisioning.\n"]
    pub fn lvp_node_mounts_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_Storage_LvpNodeMountsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_node_mounts_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lvp_share_config` after provisioning.\n"]
    pub fn lvp_share_config(&self) -> terrars::ListRef<GkeonpremBareMetalCluster_Storage_LvpShareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lvp_share_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremBareMetalCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_Timeouts {
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

impl terrars::ToListMappable for GkeonpremBareMetalCluster_Timeouts {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_Timeouts {}

impl BuildGkeonpremBareMetalCluster_Timeouts {
    pub fn build(self) -> GkeonpremBareMetalCluster_Timeouts {
        GkeonpremBareMetalCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_TimeoutsRef {
        GkeonpremBareMetalCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_TimeoutsRef {
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
pub struct GkeonpremBareMetalCluster_UpgradePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremBareMetalCluster_UpgradePolicy {
    #[doc =
        "Set the field `policy`.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremBareMetalCluster_UpgradePolicy {
    type O = terrars::BlockAssignable<GkeonpremBareMetalCluster_UpgradePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremBareMetalCluster_UpgradePolicy {}

impl BuildGkeonpremBareMetalCluster_UpgradePolicy {
    pub fn build(self) -> GkeonpremBareMetalCluster_UpgradePolicy {
        GkeonpremBareMetalCluster_UpgradePolicy { policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremBareMetalCluster_UpgradePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremBareMetalCluster_UpgradePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremBareMetalCluster_UpgradePolicyRef {
        GkeonpremBareMetalCluster_UpgradePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremBareMetalCluster_UpgradePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nSpecifies which upgrade policy to use. Possible values: [\"SERIAL\", \"CONCURRENT\"]"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
