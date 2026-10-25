#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VmwareengineClusterData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_settings: std::option::Option<std::vec::Vec<VmwareengineCluster_AutoscalingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore_mount_config: std::option::Option<std::vec::Vec<VmwareengineCluster_DatastoreMountConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_configs: std::option::Option<std::vec::Vec<VmwareengineCluster_NodeTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VmwareengineCluster_Timeouts>,
    dynamic: VmwareengineClusterDynamic,
}

struct VmwareengineCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VmwareengineClusterData_>,
}

#[derive(Clone)]
pub struct VmwareengineCluster(std::rc::Rc<VmwareengineCluster_>);

impl VmwareengineCluster {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_settings`.\n"]
    pub fn set_autoscaling_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datastore_mount_config`.\n"]
    pub fn set_datastore_mount_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareengineCluster_DatastoreMountConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().datastore_mount_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.datastore_mount_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareengineCluster_NodeTypeConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_type_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_type_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VmwareengineCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(&self) -> terrars::ListRef<VmwareengineCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\n"]
    pub fn datastore_mount_config(&self) -> terrars::ListRef<VmwareengineCluster_DatastoreMountConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_mount_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineCluster_TimeoutsRef {
        VmwareengineCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VmwareengineCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VmwareengineCluster { }

impl terrars::ToListMappable for VmwareengineCluster {
    type O = terrars::ListRef<VmwareengineClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VmwareengineCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vmwareengine_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVmwareengineCluster {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Cluster."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildVmwareengineCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> VmwareengineCluster {
        let out = VmwareengineCluster(std::rc::Rc::new(VmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VmwareengineClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                autoscaling_settings: core::default::Default::default(),
                datastore_mount_config: core::default::Default::default(),
                node_type_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VmwareengineClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VmwareengineClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(&self) -> terrars::ListRef<VmwareengineCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\n"]
    pub fn datastore_mount_config(&self) -> terrars::ListRef<VmwareengineCluster_DatastoreMountConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_mount_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineCluster_TimeoutsRef {
        VmwareengineCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineClusterDynamic {
    autoscaling_settings: std::option::Option<terrars::DynamicBlock<VmwareengineCluster_AutoscalingSettings>>,
    datastore_mount_config: std::option::Option<terrars::DynamicBlock<VmwareengineCluster_DatastoreMountConfig>>,
    node_type_configs: std::option::Option<terrars::DynamicBlock<VmwareengineCluster_NodeTypeConfigs>>,
}

#[derive(Serialize)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds { }

impl terrars::ToListMappable for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    type O =
        terrars
        ::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    pub fn build(self) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds { }

impl terrars::ToListMappable for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    type O = terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    pub fn build(self) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    scale_in: terrars::PrimField<f64>,
    scale_out: terrars::PrimField<f64>,
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds { }

impl terrars::ToListMappable for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    type O = terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: terrars::PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: terrars::PrimField<f64>,
}

impl BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    pub fn build(self) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesDynamic {
    consumed_memory_thresholds: std
    ::option
    ::Option<
        terrars::DynamicBlock<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds>,
    >,
    cpu_thresholds: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>>,
    storage_thresholds: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>>,
}

#[derive(Serialize)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    autoscale_policy_id: terrars::PrimField<std::string::String>,
    node_type_id: terrars::PrimField<std::string::String>,
    scale_out_size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumed_memory_thresholds: std
    ::option
    ::Option<std::vec::Vec<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_thresholds: std
    ::option
    ::Option<std::vec::Vec<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_thresholds: std
    ::option
    ::Option<std::vec::Vec<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>>,
    dynamic: VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesDynamic,
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumed_memory_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumed_memory_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cpu_thresholds`.\n"]
    pub fn set_cpu_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_thresholds`.\n"]
    pub fn set_storage_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    type O = terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = ""]
    pub autoscale_policy_id: terrars::PrimField<std::string::String>,
    #[doc = "The canonical identifier of the node type to add or remove."]
    pub node_type_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Number of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub scale_out_size: terrars::PrimField<f64>,
}

impl BuildVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    pub fn build(self) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
            autoscale_policy_id: self.autoscale_policy_id,
            node_type_id: self.node_type_id,
            scale_out_size: self.scale_out_size,
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
        VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscale_policy_id` after provisioning.\n"]
    pub fn autoscale_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscale_policy_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_type_id` after provisioning.\nThe canonical identifier of the node type to add or remove."]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_out_size` after provisioning.\nNumber of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub fn scale_out_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_size", self.base))
    }

    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]
    pub fn consumed_memory_thresholds(
        &self,
    ) -> terrars::ListRef<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_memory_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> terrars::ListRef<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> terrars::ListRef<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_thresholds", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineCluster_AutoscalingSettingsDynamic {
    autoscaling_policies: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>>,
}

#[derive(Serialize)]
pub struct VmwareengineCluster_AutoscalingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cool_down_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policies: std::option::Option<std::vec::Vec<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>>,
    dynamic: VmwareengineCluster_AutoscalingSettingsDynamic,
}

impl VmwareengineCluster_AutoscalingSettings {
    #[doc =
        "Set the field `cool_down_period`.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_cool_down_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cool_down_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_cluster_node_count`.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_max_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cluster_node_count`.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_min_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_policies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareengineCluster_AutoscalingSettings {
    type O = terrars::BlockAssignable<VmwareengineCluster_AutoscalingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_AutoscalingSettings {}

impl BuildVmwareengineCluster_AutoscalingSettings {
    pub fn build(self) -> VmwareengineCluster_AutoscalingSettings {
        VmwareengineCluster_AutoscalingSettings {
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
            autoscaling_policies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_AutoscalingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_AutoscalingSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineCluster_AutoscalingSettingsRef {
        VmwareengineCluster_AutoscalingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_AutoscalingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cool_down_period` after provisioning.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn cool_down_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cool_down_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_cluster_node_count` after provisioning.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn max_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cluster_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cluster_node_count` after provisioning.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn min_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cluster_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtu: std::option::Option<terrars::PrimField<f64>>,
    subnet: terrars::PrimField<std::string::String>,
}

impl VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    #[doc =
        "Set the field `connection_count`.\nOptional. The number of connections of the NFS volume.\nSupported from vsphere 8.0u1. Possible values are 1-4.\nDefault value is 4."]
    pub fn set_connection_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mtu`.\nOptional. The Maximal Transmission Unit (MTU) of the datastore.\nMTU value can range from 1330-9000. If not set, system sets\ndefault MTU size to 1500."]
    pub fn set_mtu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mtu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    type O = terrars::BlockAssignable<VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    #[doc =
        "The resource name of the subnet\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g. projects/my-project/locations/us-central1/subnets/my-subnet"]
    pub subnet: terrars::PrimField<std::string::String>,
}

impl BuildVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    pub fn build(self) -> VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
        VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
            connection_count: core::default::Default::default(),
            mtu: core::default::Default::default(),
            subnet: self.subnet,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
        VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_count` after provisioning.\nOptional. The number of connections of the NFS volume.\nSupported from vsphere 8.0u1. Possible values are 1-4.\nDefault value is 4."]
    pub fn connection_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtu` after provisioning.\nOptional. The Maximal Transmission Unit (MTU) of the datastore.\nMTU value can range from 1330-9000. If not set, system sets\ndefault MTU size to 1500."]
    pub fn mtu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_peering` after provisioning.\nThe resource name of the network peering, used to access the\nfile share by clients on private cloud. Resource names are schemeless\nURIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g.\nprojects/my-project/locations/us-central1/networkPeerings/my-network-peering"]
    pub fn network_peering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_peering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnet` after provisioning.\nThe resource name of the subnet\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g. projects/my-project/locations/us-central1/subnets/my-subnet"]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineCluster_DatastoreMountConfigDynamic {
    datastore_network: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>>,
}

#[derive(Serialize)]
pub struct VmwareengineCluster_DatastoreMountConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    datastore: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_colocation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore_network: std::option::Option<std::vec::Vec<VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>>,
    dynamic: VmwareengineCluster_DatastoreMountConfigDynamic,
}

impl VmwareengineCluster_DatastoreMountConfig {
    #[doc =
        "Set the field `access_mode`.\nOptional. NFS is accessed by hosts in either read or read_write mode\nDefault value used will be READ_WRITE\nPossible values:\nREAD_ONLY\nREAD_WRITE"]
    pub fn set_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_colocation`.\nOptional. If set to true, the colocation requirement will be ignored.\nIf set to false, the colocation requirement will be enforced.\nColocation requirement is the requirement that the cluster must be in the\nsame region/zone of datastore."]
    pub fn set_ignore_colocation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_colocation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nfs_version`.\nOptional. The NFS protocol supported by the NFS volume.\nDefault value used will be NFS_V3\nPossible values:\nNFS_V3"]
    pub fn set_nfs_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nfs_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datastore_network`.\n"]
    pub fn set_datastore_network(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datastore_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datastore_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareengineCluster_DatastoreMountConfig {
    type O = terrars::BlockAssignable<VmwareengineCluster_DatastoreMountConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_DatastoreMountConfig {
    #[doc =
        "The resource name of the datastore to unmount.\nThe datastore requested to be mounted should be in same region/zone as the\ncluster.\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\nFor example:\n'projects/my-project/locations/us-central1/datastores/my-datastore'"]
    pub datastore: terrars::PrimField<std::string::String>,
}

impl BuildVmwareengineCluster_DatastoreMountConfig {
    pub fn build(self) -> VmwareengineCluster_DatastoreMountConfig {
        VmwareengineCluster_DatastoreMountConfig {
            access_mode: core::default::Default::default(),
            datastore: self.datastore,
            ignore_colocation: core::default::Default::default(),
            nfs_version: core::default::Default::default(),
            datastore_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_DatastoreMountConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_DatastoreMountConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineCluster_DatastoreMountConfigRef {
        VmwareengineCluster_DatastoreMountConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_DatastoreMountConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_mode` after provisioning.\nOptional. NFS is accessed by hosts in either read or read_write mode\nDefault value used will be READ_WRITE\nPossible values:\nREAD_ONLY\nREAD_WRITE"]
    pub fn access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `datastore` after provisioning.\nThe resource name of the datastore to unmount.\nThe datastore requested to be mounted should be in same region/zone as the\ncluster.\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\nFor example:\n'projects/my-project/locations/us-central1/datastores/my-datastore'"]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nFile share name."]
    pub fn file_share(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_colocation` after provisioning.\nOptional. If set to true, the colocation requirement will be ignored.\nIf set to false, the colocation requirement will be enforced.\nColocation requirement is the requirement that the cluster must be in the\nsame region/zone of datastore."]
    pub fn ignore_colocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_colocation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nfs_version` after provisioning.\nOptional. The NFS protocol supported by the NFS volume.\nDefault value used will be NFS_V3\nPossible values:\nNFS_V3"]
    pub fn nfs_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nfs_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nServer IP addresses of the NFS volume.\nFor NFS 3, you can only provide a single\nserver IP address or DNS names."]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }

    #[doc = "Get a reference to the value of field `datastore_network` after provisioning.\n"]
    pub fn datastore_network(&self) -> terrars::ListRef<VmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_network", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineCluster_NodeTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    node_count: terrars::PrimField<f64>,
    node_type_id: terrars::PrimField<std::string::String>,
}

impl VmwareengineCluster_NodeTypeConfigs {
    #[doc =
        "Set the field `custom_core_count`.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareengineCluster_NodeTypeConfigs {
    type O = terrars::BlockAssignable<VmwareengineCluster_NodeTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_NodeTypeConfigs {
    #[doc = "The number of nodes of this type in the cluster."]
    pub node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub node_type_id: terrars::PrimField<std::string::String>,
}

impl BuildVmwareengineCluster_NodeTypeConfigs {
    pub fn build(self) -> VmwareengineCluster_NodeTypeConfigs {
        VmwareengineCluster_NodeTypeConfigs {
            custom_core_count: core::default::Default::default(),
            node_count: self.node_count,
            node_type_id: self.node_type_id,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_NodeTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_NodeTypeConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineCluster_NodeTypeConfigsRef {
        VmwareengineCluster_NodeTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_NodeTypeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_core_count` after provisioning.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes of this type in the cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareengineCluster_Timeouts {
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

impl terrars::ToListMappable for VmwareengineCluster_Timeouts {
    type O = terrars::BlockAssignable<VmwareengineCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineCluster_Timeouts {}

impl BuildVmwareengineCluster_Timeouts {
    pub fn build(self) -> VmwareengineCluster_Timeouts {
        VmwareengineCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineCluster_TimeoutsRef {
        VmwareengineCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineCluster_TimeoutsRef {
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
