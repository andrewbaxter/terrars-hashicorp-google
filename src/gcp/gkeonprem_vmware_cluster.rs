#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeonpremVmwareClusterData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_bundled_ingress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_advanced_cluster: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_control_plane_v2: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    on_prem_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_validations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_tracking_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anti_affinity_groups: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_AntiAffinityGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_Authorization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_AutoRepairConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_ControlPlaneNode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataplane_v2: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_DataplaneV2>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_Storage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeonpremVmwareCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_policy: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_UpgradePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcenter: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_Vcenter>>,
    dynamic: GkeonpremVmwareClusterDynamic,
}

struct GkeonpremVmwareCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeonpremVmwareClusterData_>,
}

#[derive(Clone)]
pub struct GkeonpremVmwareCluster(std::rc::Rc<GkeonpremVmwareCluster_>);

impl GkeonpremVmwareCluster {
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
        "Set the field `annotations`.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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

    #[doc = "Set the field `description`.\nA human readable description of this VMware User Cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_bundled_ingress`.\nDisable bundled ingress."]
    pub fn set_disable_bundled_ingress(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_bundled_ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_advanced_cluster`.\nEnable advanced cluster. Default to false."]
    pub fn set_enable_advanced_cluster(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_advanced_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_control_plane_v2`.\nEnable control plane V2. Default to false."]
    pub fn set_enable_control_plane_v2(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_control_plane_v2 = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `skip_validations`.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn set_skip_validations(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().skip_validations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_tracking_enabled`.\nEnable VM tracking."]
    pub fn set_vm_tracking_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().vm_tracking_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anti_affinity_groups`.\n"]
    pub fn set_anti_affinity_groups(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_AntiAffinityGroups>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_Authorization>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_AutoRepairConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_ControlPlaneNode>>,
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

    #[doc = "Set the field `dataplane_v2`.\n"]
    pub fn set_dataplane_v2(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_DataplaneV2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataplane_v2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataplane_v2 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig>>,
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

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_Storage>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeonpremVmwareCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_UpgradePolicy>>,
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

    #[doc = "Set the field `vcenter`.\n"]
    pub fn set_vcenter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_Vcenter>>,
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
        "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub fn admin_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which VMware User Cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time at which VMware User Cluster was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware User Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_bundled_ingress` after provisioning.\nDisable bundled ingress."]
    pub fn disable_bundled_ingress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_bundled_ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nEnable advanced cluster. Default to false."]
    pub fn enable_advanced_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_advanced_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_control_plane_v2` after provisioning.\nEnable control plane V2. Default to false."]
    pub fn enable_control_plane_v2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_control_plane_v2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware User Cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremVmwareCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMware OnPremUserCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\n\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for your user cluster."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware User Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_validations` after provisioning.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn skip_validations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skip_validations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware User Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which VMware User Cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nValidationCheck represents the result of the preflight check job."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vm_tracking_enabled` after provisioning.\nEnable VM tracking."]
    pub fn vm_tracking_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_tracking_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AntiAffinityGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anti_affinity_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AutoRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ControlPlaneNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplane_v2` after provisioning.\n"]
    pub fn dataplane_v2(&self) -> terrars::ListRef<GkeonpremVmwareCluster_DataplaneV2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplane_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremVmwareCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareCluster_TimeoutsRef {
        GkeonpremVmwareCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremVmwareCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> terrars::ListRef<GkeonpremVmwareCluster_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

impl terrars::Referable for GkeonpremVmwareCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeonpremVmwareCluster { }

impl terrars::ToListMappable for GkeonpremVmwareCluster {
    type O = terrars::ListRef<GkeonpremVmwareClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeonpremVmwareCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gkeonprem_vmware_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeonpremVmwareCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub admin_cluster_membership: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The VMware cluster name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The Anthos clusters on the VMware version for your user cluster."]
    pub on_prem_version: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeonpremVmwareCluster {
        let out = GkeonpremVmwareCluster(std::rc::Rc::new(GkeonpremVmwareCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeonpremVmwareClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_cluster_membership: self.admin_cluster_membership,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_bundled_ingress: core::default::Default::default(),
                enable_advanced_cluster: core::default::Default::default(),
                enable_control_plane_v2: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                on_prem_version: self.on_prem_version,
                project: core::default::Default::default(),
                skip_validations: core::default::Default::default(),
                vm_tracking_enabled: core::default::Default::default(),
                anti_affinity_groups: core::default::Default::default(),
                authorization: core::default::Default::default(),
                auto_repair_config: core::default::Default::default(),
                control_plane_node: core::default::Default::default(),
                dataplane_v2: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_config: core::default::Default::default(),
                storage: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                vcenter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeonpremVmwareClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeonpremVmwareClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_cluster_membership` after provisioning.\nThe admin cluster this VMware User Cluster belongs to.\nThis is the full resource name of the admin cluster's hub membership.\nIn the future, references to other resource types might be allowed if\nadmin clusters are modeled as their own resources."]
    pub fn admin_cluster_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_cluster_membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations on the VMware User Cluster.\nThis field has the same restrictions as Kubernetes annotations.\nThe total size of all keys and values combined is limited to 256k.\nKey can have 2 segments: prefix (optional) and name (required),\nseparated by a slash (/).\nPrefix must be a DNS subdomain.\nName must be 63 characters or less, begin and end with alphanumerics,\nwith dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which VMware User Cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time at which VMware User Cluster was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human readable description of this VMware User Cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_bundled_ingress` after provisioning.\nDisable bundled ingress."]
    pub fn disable_bundled_ingress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_bundled_ingress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_advanced_cluster` after provisioning.\nEnable advanced cluster. Default to false."]
    pub fn enable_advanced_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_advanced_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_control_plane_v2` after provisioning.\nEnable control plane V2. Default to false."]
    pub fn enable_control_plane_v2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_control_plane_v2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe DNS name of VMware User Cluster's API server."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding.\nAllows clients to perform consistent read-modify-writes\nthrough optimistic concurrency control."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration for the cluster."]
    pub fn fleet(&self) -> terrars::ListRef<GkeonpremVmwareCluster_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_name` after provisioning.\nThe object name of the VMware OnPremUserCluster custom resource on the\nassociated admin cluster. This field is used to support conflicting\nnames when enrolling existing clusters to the API. When used as a part of\ncluster enrollment, this field will differ from the ID in the resource\nname. For new clusters, this field will match the user provided cluster ID\nand be visible in the last component of the resource name. It is not\nmodifiable.\n\nAll users should use this name to access their cluster using gkectl or\nkubectl and should expect to see the local name when viewing admin\ncluster controller logs."]
    pub fn local_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe VMware cluster name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_prem_version` after provisioning.\nThe Anthos clusters on the VMware version for your user cluster."]
    pub fn on_prem_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_prem_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the VMware User Cluster."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_validations` after provisioning.\nA list of validations to skip during preflight checks. Possible values: [\"VALIDATION_SKIP_UNSPECIFIED\", \"ALL\", \"WORKSTATION\", \"CONFIG\", \"DOCKER\", \"INFRA\", \"LOAD_BALANCER\", \"VIPS\", \"NODE_IPS\", \"DNS\", \"TOD\", \"NET_CONFIG\", \"STORAGE_DRIVER\", \"PROXY\", \"INTERNET\", \"GCP\", \"GKEHUB\", \"RESERVED_IPS\", \"STACKDRIVER\", \"NODEPOOL_AUTOSCALING\", \"OS_IMAGES\", \"CLUSTER_VERSION\", \"CLUSTER_HEALTH\", \"WINDOWS\", \"HSM_SECRET_ENCRYPTION\", \"BACKUP_ADMIN\", \"CONNECTIVITY\", \"CLUSTER_SECRETS_CONFIG\", \"CSI_WORKLOAD\", \"VSPHERE_VERSION\", \"MIGRATION\"]"]
    pub fn skip_validations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skip_validations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of this cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nResourceStatus representing detailed cluster state."]
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareCluster_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of the VMware User Cluster."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which VMware User Cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_check` after provisioning.\nValidationCheck represents the result of the preflight check job."]
    pub fn validation_check(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ValidationCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vm_tracking_enabled` after provisioning.\nEnable VM tracking."]
    pub fn vm_tracking_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_tracking_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anti_affinity_groups` after provisioning.\n"]
    pub fn anti_affinity_groups(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AntiAffinityGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anti_affinity_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AuthorizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_repair_config` after provisioning.\n"]
    pub fn auto_repair_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_AutoRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_node` after provisioning.\n"]
    pub fn control_plane_node(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ControlPlaneNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplane_v2` after provisioning.\n"]
    pub fn dataplane_v2(&self) -> terrars::ListRef<GkeonpremVmwareCluster_DataplaneV2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataplane_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<GkeonpremVmwareCluster_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeonpremVmwareCluster_TimeoutsRef {
        GkeonpremVmwareCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<GkeonpremVmwareCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcenter` after provisioning.\n"]
    pub fn vcenter(&self) -> terrars::ListRef<GkeonpremVmwareCluster_VcenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcenter", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareClusterDynamic {
    anti_affinity_groups: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_AntiAffinityGroups>>,
    authorization: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_Authorization>>,
    auto_repair_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_AutoRepairConfig>>,
    control_plane_node: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_ControlPlaneNode>>,
    dataplane_v2: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_DataplaneV2>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer>>,
    network_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig>>,
    storage: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_Storage>>,
    upgrade_policy: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_UpgradePolicy>>,
    vcenter: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_Vcenter>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_Fleet {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_Fleet {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Fleet {}

impl BuildGkeonpremVmwareCluster_Fleet {
    pub fn build(self) -> GkeonpremVmwareCluster_Fleet {
        GkeonpremVmwareCluster_Fleet { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_FleetRef {
        GkeonpremVmwareCluster_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_FleetRef {
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
pub struct GkeonpremVmwareCluster_Status_Conditions {
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

impl GkeonpremVmwareCluster_Status_Conditions {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_Status_Conditions {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Status_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Status_Conditions {}

impl BuildGkeonpremVmwareCluster_Status_Conditions {
    pub fn build(self) -> GkeonpremVmwareCluster_Status_Conditions {
        GkeonpremVmwareCluster_Status_Conditions {
            last_transition_time: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_Status_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_Status_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_Status_ConditionsRef {
        GkeonpremVmwareCluster_Status_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_Status_ConditionsRef {
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
pub struct GkeonpremVmwareCluster_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<GkeonpremVmwareCluster_Status_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_Status {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareCluster_Status_Conditions>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_Status {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Status {}

impl BuildGkeonpremVmwareCluster_Status {
    pub fn build(self) -> GkeonpremVmwareCluster_Status {
        GkeonpremVmwareCluster_Status {
            conditions: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_StatusRef {
        GkeonpremVmwareCluster_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GkeonpremVmwareCluster_Status_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_ValidationCheck_Status_Result {
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

impl GkeonpremVmwareCluster_ValidationCheck_Status_Result {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_ValidationCheck_Status_Result {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ValidationCheck_Status_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ValidationCheck_Status_Result {}

impl BuildGkeonpremVmwareCluster_ValidationCheck_Status_Result {
    pub fn build(self) -> GkeonpremVmwareCluster_ValidationCheck_Status_Result {
        GkeonpremVmwareCluster_ValidationCheck_Status_Result {
            category: core::default::Default::default(),
            description: core::default::Default::default(),
            details: core::default::Default::default(),
            options: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef {
        GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef {
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
pub struct GkeonpremVmwareCluster_ValidationCheck_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<GkeonpremVmwareCluster_ValidationCheck_Status_Result>>,
}

impl GkeonpremVmwareCluster_ValidationCheck_Status {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareCluster_ValidationCheck_Status_Result>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_ValidationCheck_Status {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ValidationCheck_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ValidationCheck_Status {}

impl BuildGkeonpremVmwareCluster_ValidationCheck_Status {
    pub fn build(self) -> GkeonpremVmwareCluster_ValidationCheck_Status {
        GkeonpremVmwareCluster_ValidationCheck_Status { result: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ValidationCheck_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ValidationCheck_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_ValidationCheck_StatusRef {
        GkeonpremVmwareCluster_ValidationCheck_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ValidationCheck_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ValidationCheck_Status_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_ValidationCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<GkeonpremVmwareCluster_ValidationCheck_Status>>,
}

impl GkeonpremVmwareCluster_ValidationCheck {
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
        v: impl std::convert::Into<terrars::ListField<GkeonpremVmwareCluster_ValidationCheck_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_ValidationCheck {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ValidationCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ValidationCheck {}

impl BuildGkeonpremVmwareCluster_ValidationCheck {
    pub fn build(self) -> GkeonpremVmwareCluster_ValidationCheck {
        GkeonpremVmwareCluster_ValidationCheck {
            options: core::default::Default::default(),
            scenario: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ValidationCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ValidationCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_ValidationCheckRef {
        GkeonpremVmwareCluster_ValidationCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ValidationCheckRef {
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
    pub fn status(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ValidationCheck_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_AntiAffinityGroups {
    aag_config_disabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareCluster_AntiAffinityGroups { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_AntiAffinityGroups {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_AntiAffinityGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_AntiAffinityGroups {
    #[doc = "Spread nodes across at least three physical hosts (requires at least three\nhosts).\nEnabled by default."]
    pub aag_config_disabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareCluster_AntiAffinityGroups {
    pub fn build(self) -> GkeonpremVmwareCluster_AntiAffinityGroups {
        GkeonpremVmwareCluster_AntiAffinityGroups { aag_config_disabled: self.aag_config_disabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_AntiAffinityGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_AntiAffinityGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_AntiAffinityGroupsRef {
        GkeonpremVmwareCluster_AntiAffinityGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_AntiAffinityGroupsRef {
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
pub struct GkeonpremVmwareCluster_Authorization_AdminUsers {
    username: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareCluster_Authorization_AdminUsers { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_Authorization_AdminUsers {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Authorization_AdminUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Authorization_AdminUsers {
    #[doc = "The name of the user, e.g. 'my-gcp-id@gmail.com'."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareCluster_Authorization_AdminUsers {
    pub fn build(self) -> GkeonpremVmwareCluster_Authorization_AdminUsers {
        GkeonpremVmwareCluster_Authorization_AdminUsers { username: self.username }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_Authorization_AdminUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_Authorization_AdminUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_Authorization_AdminUsersRef {
        GkeonpremVmwareCluster_Authorization_AdminUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_Authorization_AdminUsersRef {
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
struct GkeonpremVmwareCluster_AuthorizationDynamic {
    admin_users: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_Authorization_AdminUsers>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_Authorization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_users: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_Authorization_AdminUsers>>,
    dynamic: GkeonpremVmwareCluster_AuthorizationDynamic,
}

impl GkeonpremVmwareCluster_Authorization {
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_Authorization_AdminUsers>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_Authorization {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Authorization {}

impl BuildGkeonpremVmwareCluster_Authorization {
    pub fn build(self) -> GkeonpremVmwareCluster_Authorization {
        GkeonpremVmwareCluster_Authorization {
            admin_users: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_AuthorizationRef {
        GkeonpremVmwareCluster_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> terrars::ListRef<GkeonpremVmwareCluster_Authorization_AdminUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_AutoRepairConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareCluster_AutoRepairConfig { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_AutoRepairConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_AutoRepairConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_AutoRepairConfig {
    #[doc = "Whether auto repair is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareCluster_AutoRepairConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_AutoRepairConfig {
        GkeonpremVmwareCluster_AutoRepairConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_AutoRepairConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_AutoRepairConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_AutoRepairConfigRef {
        GkeonpremVmwareCluster_AutoRepairConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_AutoRepairConfigRef {
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
pub struct GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
    #[doc = "Set the field `datastore`.\n"]
    pub fn set_datastore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datastore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_policy_name`.\n"]
    pub fn set_storage_policy_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_policy_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {}

impl BuildGkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
        GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfig {
            datastore: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef {
        GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datastore` after provisioning.\n"]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_policy_name` after provisioning.\n"]
    pub fn storage_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_policy_name", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig {
    #[doc = "Whether to enable control plane node auto resizing."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig {
        GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef {
        GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable control plane node auto resizing."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_ControlPlaneNodeDynamic {
    auto_resize_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_ControlPlaneNode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_resize_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig>>,
    dynamic: GkeonpremVmwareCluster_ControlPlaneNodeDynamic,
}

impl GkeonpremVmwareCluster_ControlPlaneNode {
    #[doc =
        "Set the field `cpus`.\nThe number of CPUs for each admin cluster node that serve as control planes\nfor this VMware User Cluster. (default: 4 CPUs)"]
    pub fn set_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory`.\nThe megabytes of memory for each admin cluster node that serves as a\ncontrol plane for this VMware User Cluster (default: 8192 MB memory)."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replicas`.\nThe number of control plane nodes for this VMware User Cluster.\n(default: 1 replica)."]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_resize_config`.\n"]
    pub fn set_auto_resize_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfig>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_ControlPlaneNode {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_ControlPlaneNode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_ControlPlaneNode {}

impl BuildGkeonpremVmwareCluster_ControlPlaneNode {
    pub fn build(self) -> GkeonpremVmwareCluster_ControlPlaneNode {
        GkeonpremVmwareCluster_ControlPlaneNode {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
            replicas: core::default::Default::default(),
            auto_resize_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_ControlPlaneNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_ControlPlaneNodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_ControlPlaneNodeRef {
        GkeonpremVmwareCluster_ControlPlaneNodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_ControlPlaneNodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpus` after provisioning.\nThe number of CPUs for each admin cluster node that serve as control planes\nfor this VMware User Cluster. (default: 4 CPUs)"]
    pub fn cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nThe megabytes of memory for each admin cluster node that serves as a\ncontrol plane for this VMware User Cluster (default: 8192 MB memory)."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nThe number of control plane nodes for this VMware User Cluster.\n(default: 1 replica)."]
    pub fn replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `vsphere_config` after provisioning.\nVsphere-specific config."]
    pub fn vsphere_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ControlPlaneNode_VsphereConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vsphere_config", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_resize_config` after provisioning.\n"]
    pub fn auto_resize_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_ControlPlaneNode_AutoResizeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_resize_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_DataplaneV2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataplane_v2_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_dataplane_v2_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremVmwareCluster_DataplaneV2 {
    #[doc =
        "Set the field `advanced_networking`.\nEnable advanced networking which requires dataplane_v2_enabled to be set true."]
    pub fn set_advanced_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.advanced_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataplane_v2_enabled`.\nEnables Dataplane V2."]
    pub fn set_dataplane_v2_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dataplane_v2_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `windows_dataplane_v2_enabled`.\nEnable Dataplane V2 for clusters with Windows nodes."]
    pub fn set_windows_dataplane_v2_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.windows_dataplane_v2_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_DataplaneV2 {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_DataplaneV2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_DataplaneV2 {}

impl BuildGkeonpremVmwareCluster_DataplaneV2 {
    pub fn build(self) -> GkeonpremVmwareCluster_DataplaneV2 {
        GkeonpremVmwareCluster_DataplaneV2 {
            advanced_networking: core::default::Default::default(),
            dataplane_v2_enabled: core::default::Default::default(),
            windows_dataplane_v2_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_DataplaneV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_DataplaneV2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_DataplaneV2Ref {
        GkeonpremVmwareCluster_DataplaneV2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_DataplaneV2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `advanced_networking` after provisioning.\nEnable advanced networking which requires dataplane_v2_enabled to be set true."]
    pub fn advanced_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `dataplane_v2_enabled` after provisioning.\nEnables Dataplane V2."]
    pub fn dataplane_v2_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataplane_v2_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `windows_dataplane_v2_enabled` after provisioning.\nEnable Dataplane V2 for clusters with Windows nodes."]
    pub fn windows_dataplane_v2_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.windows_dataplane_v2_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_LoadBalancer_F5Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snat_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_LoadBalancer_F5Config {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer_F5Config {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_F5Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer_F5Config {}

impl BuildGkeonpremVmwareCluster_LoadBalancer_F5Config {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer_F5Config {
        GkeonpremVmwareCluster_LoadBalancer_F5Config {
            address: core::default::Default::default(),
            partition: core::default::Default::default(),
            snat_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef {
        GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef {
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
pub struct GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_http_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_https_node_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    konnectivity_server_node_port: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {}

impl BuildGkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
        GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig {
            control_plane_node_port: core::default::Default::default(),
            ingress_http_node_port: core::default::Default::default(),
            ingress_https_node_port: core::default::Default::default(),
            konnectivity_server_node_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef {
        GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
    addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avoid_buggy_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_assign: std::option::Option<terrars::PrimField<bool>>,
    pool: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
    #[doc =
        "Set the field `avoid_buggy_ips`.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for\nthose special IP addresses."]
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
    #[doc =
        "The addresses that are part of this pool. Each address\nmust be either in the CIDR form (1.2.3.0/24) or range\nform (1.2.3.1-1.2.3.5)."]
    pub addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The name of the address pool."]
    pub pool: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
        GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools {
            addresses: self.addresses,
            avoid_buggy_ips: core::default::Default::default(),
            manual_assign: core::default::Default::default(),
            pool: self.pool,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef {
        GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `addresses` after provisioning.\nThe addresses that are part of this pool. Each address\nmust be either in the CIDR form (1.2.3.0/24) or range\nform (1.2.3.1-1.2.3.5)."]
    pub fn addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `avoid_buggy_ips` after provisioning.\nIf true, avoid using IPs ending in .0 or .255.\nThis avoids buggy consumer devices mistakenly dropping IPv4 traffic for\nthose special IP addresses."]
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

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigDynamic {
    address_pools: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_pools: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools>>,
    dynamic: GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigDynamic,
}

impl GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
    #[doc = "Set the field `address_pools`.\n"]
    pub fn set_address_pools(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPools>>,
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
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {}

impl BuildGkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
        GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig {
            address_pools: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef {
        GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_pools` after provisioning.\n"]
    pub fn address_pools(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig_AddressPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_LoadBalancer_VipConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_vip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_vip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_LoadBalancer_VipConfig {
    #[doc =
        "Set the field `control_plane_vip`.\nThe VIP which you previously set aside for the Kubernetes API of this cluster."]
    pub fn set_control_plane_vip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.control_plane_vip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress_vip`.\nThe VIP which you previously set aside for ingress traffic into this cluster."]
    pub fn set_ingress_vip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ingress_vip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer_VipConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_VipConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer_VipConfig {}

impl BuildGkeonpremVmwareCluster_LoadBalancer_VipConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer_VipConfig {
        GkeonpremVmwareCluster_LoadBalancer_VipConfig {
            control_plane_vip: core::default::Default::default(),
            ingress_vip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancer_VipConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancer_VipConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_LoadBalancer_VipConfigRef {
        GkeonpremVmwareCluster_LoadBalancer_VipConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancer_VipConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_vip` after provisioning.\nThe VIP which you previously set aside for the Kubernetes API of this cluster."]
    pub fn control_plane_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_vip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_vip` after provisioning.\nThe VIP which you previously set aside for ingress traffic into this cluster."]
    pub fn ingress_vip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_vip", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_LoadBalancerDynamic {
    f5_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer_F5Config>>,
    manual_lb_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig>>,
    metal_lb_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig>>,
    vip_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_LoadBalancer_VipConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_LoadBalancer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    f5_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer_F5Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_lb_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metal_lb_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_LoadBalancer_VipConfig>>,
    dynamic: GkeonpremVmwareCluster_LoadBalancerDynamic,
}

impl GkeonpremVmwareCluster_LoadBalancer {
    #[doc = "Set the field `f5_config`.\n"]
    pub fn set_f5_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_F5Config>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_ManualLbConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer_VipConfig>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_LoadBalancer {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_LoadBalancer {}

impl BuildGkeonpremVmwareCluster_LoadBalancer {
    pub fn build(self) -> GkeonpremVmwareCluster_LoadBalancer {
        GkeonpremVmwareCluster_LoadBalancer {
            f5_config: core::default::Default::default(),
            manual_lb_config: core::default::Default::default(),
            metal_lb_config: core::default::Default::default(),
            vip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_LoadBalancerRef {
        GkeonpremVmwareCluster_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `f5_config` after provisioning.\n"]
    pub fn f5_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancer_F5ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.f5_config", self.base))
    }

    #[doc = "Get a reference to the value of field `manual_lb_config` after provisioning.\n"]
    pub fn manual_lb_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancer_ManualLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `metal_lb_config` after provisioning.\n"]
    pub fn metal_lb_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancer_MetalLbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metal_lb_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vip_config` after provisioning.\n"]
    pub fn vip_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_LoadBalancer_VipConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\nIP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
    type O =
        terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {}

impl BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips {
            hostname: core::default::Default::default(),
            ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef {
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
struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockDynamic {
    ips: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips>,
    >,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netmask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std
    ::option
    ::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips>>,
    dynamic: GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockDynamic,
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
    #[doc = "Set the field `gateway`.\nThe network gateway used by the VMware User Cluster."]
    pub fn set_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `netmask`.\nThe netmask used by the VMware User Cluster."]
    pub fn set_netmask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.netmask = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_Ips,
                        >,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {}

impl BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock {
            gateway: core::default::Default::default(),
            netmask: core::default::Default::default(),
            ips: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware User Cluster."]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc =
        "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware User Cluster."]
    pub fn netmask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(
        &self,
    ) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigDynamic {
    control_plane_ip_block: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_ip_block: std
    ::option
    ::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock>>,
    dynamic: GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigDynamic,
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
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
                            GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlock,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {}

impl BuildGkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config {
            control_plane_ip_block: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef {
        GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_ip_block` after provisioning.\n"]
    pub fn control_plane_ip_block(
        &self,
    ) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config_ControlPlaneIpBlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_ip_block", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig {
    enabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig {
    #[doc = "enabled is a flag to mark if DHCP IP allocation is\nused for VMware user clusters."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig {
        GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef {
        GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nenabled is a flag to mark if DHCP IP allocation is\nused for VMware user clusters."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_HostConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_search_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ntp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeonpremVmwareCluster_NetworkConfig_HostConfig {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_HostConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_HostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_HostConfig {}

impl BuildGkeonpremVmwareCluster_NetworkConfig_HostConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_HostConfig {
        GkeonpremVmwareCluster_NetworkConfig_HostConfig {
            dns_search_domains: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            ntp_servers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_HostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_HostConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_HostConfigRef {
        GkeonpremVmwareCluster_NetworkConfig_HostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_HostConfigRef {
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

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    ip: terrars::PrimField<std::string::String>,
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[doc = "Set the field `hostname`.\nHostname of the machine. VM's name will be used if this field is empty."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    #[doc = "IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24)."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips {
            hostname: core::default::Default::default(),
            ip: self.ip,
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef {
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
struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksDynamic {
    ips: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    gateway: terrars::PrimField<std::string::String>,
    netmask: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>>,
    dynamic: GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksDynamic,
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    #[doc = "Set the field `ips`.\n"]
    pub fn set_ips(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_Ips>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    #[doc = "The network gateway used by the VMware User Cluster."]
    pub gateway: terrars::PrimField<std::string::String>,
    #[doc = "The netmask used by the VMware User Cluster."]
    pub netmask: terrars::PrimField<std::string::String>,
}

impl BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks {
            gateway: self.gateway,
            netmask: self.netmask,
            ips: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gateway` after provisioning.\nThe network gateway used by the VMware User Cluster."]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc =
        "Get a reference to the value of field `netmask` after provisioning.\nThe netmask used by the VMware User Cluster."]
    pub fn netmask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask", self.base))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\n"]
    pub fn ips(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigDynamic {
    ip_blocks: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks>>,
    dynamic: GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigDynamic,
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocks>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {}

impl BuildGkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig {
            ip_blocks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef {
        GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeonpremVmwareCluster_NetworkConfigDynamic {
    control_plane_v2_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config>>,
    dhcp_ip_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig>>,
    host_config: std::option::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_HostConfig>>,
    static_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig>>,
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_NetworkConfig {
    pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcenter_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_v2_config: std
    ::option
    ::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_ip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_HostConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_ip_config: std::option::Option<std::vec::Vec<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig>>,
    dynamic: GkeonpremVmwareCluster_NetworkConfigDynamic,
}

impl GkeonpremVmwareCluster_NetworkConfig {
    #[doc =
        "Set the field `vcenter_network`.\nvcenter_network specifies vCenter network name. Inherited from the admin cluster."]
    pub fn set_vcenter_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vcenter_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_v2_config`.\n"]
    pub fn set_control_plane_v2_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_plane_v2_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_v2_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dhcp_ip_config`.\n"]
    pub fn set_dhcp_ip_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfig>>,
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

    #[doc = "Set the field `host_config`.\n"]
    pub fn set_host_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_HostConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfig>>,
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_NetworkConfig {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_NetworkConfig {
    #[doc =
        "All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.\nOnly a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "All services in the cluster are assigned an RFC1918 IPv4 address\nfrom these ranges. Only a single range is supported.. This field\ncannot be changed after creation."]
    pub service_address_cidr_blocks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeonpremVmwareCluster_NetworkConfig {
    pub fn build(self) -> GkeonpremVmwareCluster_NetworkConfig {
        GkeonpremVmwareCluster_NetworkConfig {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            vcenter_network: core::default::Default::default(),
            control_plane_v2_config: core::default::Default::default(),
            dhcp_ip_config: core::default::Default::default(),
            host_config: core::default::Default::default(),
            static_ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_NetworkConfigRef {
        GkeonpremVmwareCluster_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_NetworkConfigRef {
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
        "Get a reference to the value of field `vcenter_network` after provisioning.\nvcenter_network specifies vCenter network name. Inherited from the admin cluster."]
    pub fn vcenter_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcenter_network", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_v2_config` after provisioning.\n"]
    pub fn control_plane_v2_config(
        &self,
    ) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_ControlPlaneV2ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_v2_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_ip_config` after provisioning.\n"]
    pub fn dhcp_ip_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_DhcpIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dhcp_ip_config", self.base))
    }

    #[doc = "Get a reference to the value of field `host_config` after provisioning.\n"]
    pub fn host_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_HostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_config", self.base))
    }

    #[doc = "Get a reference to the value of field `static_ip_config` after provisioning.\n"]
    pub fn static_ip_config(&self) -> terrars::ListRef<GkeonpremVmwareCluster_NetworkConfig_StaticIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_Storage {
    vsphere_csi_disabled: terrars::PrimField<bool>,
}

impl GkeonpremVmwareCluster_Storage { }

impl terrars::ToListMappable for GkeonpremVmwareCluster_Storage {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Storage {
    #[doc = "Whether or not to deploy vSphere CSI components in the VMware User Cluster.\nEnabled by default."]
    pub vsphere_csi_disabled: terrars::PrimField<bool>,
}

impl BuildGkeonpremVmwareCluster_Storage {
    pub fn build(self) -> GkeonpremVmwareCluster_Storage {
        GkeonpremVmwareCluster_Storage { vsphere_csi_disabled: self.vsphere_csi_disabled }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_StorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_StorageRef {
        GkeonpremVmwareCluster_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vsphere_csi_disabled` after provisioning.\nWhether or not to deploy vSphere CSI components in the VMware User Cluster.\nEnabled by default."]
    pub fn vsphere_csi_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vsphere_csi_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeonpremVmwareCluster_Timeouts {
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_Timeouts {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Timeouts {}

impl BuildGkeonpremVmwareCluster_Timeouts {
    pub fn build(self) -> GkeonpremVmwareCluster_Timeouts {
        GkeonpremVmwareCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_TimeoutsRef {
        GkeonpremVmwareCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_TimeoutsRef {
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
pub struct GkeonpremVmwareCluster_UpgradePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_only: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeonpremVmwareCluster_UpgradePolicy {
    #[doc = "Set the field `control_plane_only`.\nControls whether the upgrade applies to the control plane only."]
    pub fn set_control_plane_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.control_plane_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeonpremVmwareCluster_UpgradePolicy {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_UpgradePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_UpgradePolicy {}

impl BuildGkeonpremVmwareCluster_UpgradePolicy {
    pub fn build(self) -> GkeonpremVmwareCluster_UpgradePolicy {
        GkeonpremVmwareCluster_UpgradePolicy { control_plane_only: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_UpgradePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_UpgradePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_UpgradePolicyRef {
        GkeonpremVmwareCluster_UpgradePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_UpgradePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane_only` after provisioning.\nControls whether the upgrade applies to the control plane only."]
    pub fn control_plane_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_only", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeonpremVmwareCluster_Vcenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_cert_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
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

impl GkeonpremVmwareCluster_Vcenter {
    #[doc = "Set the field `ca_cert_data`.\nContains the vCenter CA certificate public key for SSL verification."]
    pub fn set_ca_cert_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_cert_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster`.\nThe name of the vCenter cluster for the user cluster."]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datacenter`.\nThe name of the vCenter datacenter for the user cluster."]
    pub fn set_datacenter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datacenter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datastore`.\nThe name of the vCenter datastore for the user cluster."]
    pub fn set_datastore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datastore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `folder`.\nThe name of the vCenter folder for the user cluster."]
    pub fn set_folder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_pool`.\nThe name of the vCenter resource pool for the user cluster."]
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

impl terrars::ToListMappable for GkeonpremVmwareCluster_Vcenter {
    type O = terrars::BlockAssignable<GkeonpremVmwareCluster_Vcenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeonpremVmwareCluster_Vcenter {}

impl BuildGkeonpremVmwareCluster_Vcenter {
    pub fn build(self) -> GkeonpremVmwareCluster_Vcenter {
        GkeonpremVmwareCluster_Vcenter {
            ca_cert_data: core::default::Default::default(),
            cluster: core::default::Default::default(),
            datacenter: core::default::Default::default(),
            datastore: core::default::Default::default(),
            folder: core::default::Default::default(),
            resource_pool: core::default::Default::default(),
            storage_policy_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeonpremVmwareCluster_VcenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeonpremVmwareCluster_VcenterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeonpremVmwareCluster_VcenterRef {
        GkeonpremVmwareCluster_VcenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeonpremVmwareCluster_VcenterRef {
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
        "Get a reference to the value of field `cluster` after provisioning.\nThe name of the vCenter cluster for the user cluster."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datacenter` after provisioning.\nThe name of the vCenter datacenter for the user cluster."]
    pub fn datacenter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datacenter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datastore` after provisioning.\nThe name of the vCenter datastore for the user cluster."]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe name of the vCenter folder for the user cluster."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool` after provisioning.\nThe name of the vCenter resource pool for the user cluster."]
    pub fn resource_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_policy_name` after provisioning.\nThe name of the vCenter storage policy for the user cluster."]
    pub fn storage_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_policy_name", self.base))
    }
}
