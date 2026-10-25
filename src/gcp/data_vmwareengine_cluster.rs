#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareengineClusterData_ {
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
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
}

struct DataVmwareengineCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareengineClusterData_>,
}

#[derive(Clone)]
pub struct DataVmwareengineCluster(std::rc::Rc<DataVmwareengineCluster_>);

impl DataVmwareengineCluster {
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
        "Get a reference to the value of field `autoscaling_settings` after provisioning.\nConfiguration of the autoscaling applied to this cluster"]
    pub fn autoscaling_settings(&self) -> terrars::ListRef<DataVmwareengineCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `datastore_mount_config` after provisioning.\nOptional. Configuration to mount a datastore.\nMount can be done along with cluster create or during cluster update\nSince service subnet is not configured with ip range on mgmt cluster creation, mount on management cluster is done as update only\nfor unmount remove 'datastore_mount_config' config from the update of cluster resource"]
    pub fn datastore_mount_config(&self) -> terrars::ListRef<DataVmwareengineCluster_DatastoreMountConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_mount_config", self.extract_ref()))
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
        "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> terrars::SetRef<DataVmwareengineCluster_NodeTypeConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
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
}

impl terrars::Referable for DataVmwareengineCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareengineCluster { }

impl terrars::ToListMappable for DataVmwareengineCluster {
    type O = terrars::ListRef<DataVmwareengineClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareengineCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareengineCluster {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Cluster."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareengineCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareengineCluster {
        let out = DataVmwareengineCluster(std::rc::Rc::new(DataVmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareengineClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareengineClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareengineClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_settings` after provisioning.\nConfiguration of the autoscaling applied to this cluster"]
    pub fn autoscaling_settings(&self) -> terrars::ListRef<DataVmwareengineCluster_AutoscalingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `datastore_mount_config` after provisioning.\nOptional. Configuration to mount a datastore.\nMount can be done along with cluster create or during cluster update\nSince service subnet is not configured with ip range on mgmt cluster creation, mount on management cluster is done as update only\nfor unmount remove 'datastore_mount_config' config from the update of cluster resource"]
    pub fn datastore_mount_config(&self) -> terrars::ListRef<DataVmwareengineCluster_DatastoreMountConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_mount_config", self.extract_ref()))
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
        "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> terrars::SetRef<DataVmwareengineCluster_NodeTypeConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_type_configs", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    type O =
        terrars
        ::BlockAssignable<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {}

impl BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
    pub fn build(self) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {}

impl BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
    pub fn build(self) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    type O =
        terrars::BlockAssignable<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {}

impl BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
    pub fn build(self) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscale_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumed_memory_thresholds: std
    ::option
    ::Option<
        terrars::ListField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_thresholds: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_thresholds: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>>,
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    #[doc = "Set the field `autoscale_policy_id`.\n"]
    pub fn set_autoscale_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autoscale_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholds,
                        >,
                    >,
    ) -> Self {
        self.consumed_memory_thresholds = std::option::Option::Some(v.into());
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
                        ::ListField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholds>,
                    >,
    ) -> Self {
        self.cpu_thresholds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_size`.\n"]
    pub fn set_scale_out_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out_size = std::option::Option::Some(v.into());
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
                        ::ListField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholds>,
                    >,
    ) -> Self {
        self.storage_thresholds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {}

impl BuildDataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
    pub fn build(self) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies {
            autoscale_policy_id: core::default::Default::default(),
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
            scale_out_size: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
        DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscale_policy_id` after provisioning.\n"]
    pub fn autoscale_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscale_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]
    pub fn consumed_memory_thresholds(
        &self,
    ) -> terrars
    ::ListRef<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_ConsumedMemoryThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_memory_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> terrars::ListRef<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_CpuThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_size` after provisioning.\n"]
    pub fn scale_out_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_size", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> terrars::ListRef<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies_StorageThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_thresholds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_AutoscalingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policies: std
    ::option
    ::Option<terrars::SetField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cool_down_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cluster_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineCluster_AutoscalingSettings {
    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPolicies>>,
    ) -> Self {
        self.autoscaling_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cool_down_period`.\n"]
    pub fn set_cool_down_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cool_down_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_cluster_node_count`.\n"]
    pub fn set_max_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cluster_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cluster_node_count`.\n"]
    pub fn set_min_cluster_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cluster_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_AutoscalingSettings {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_AutoscalingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_AutoscalingSettings {}

impl BuildDataVmwareengineCluster_AutoscalingSettings {
    pub fn build(self) -> DataVmwareengineCluster_AutoscalingSettings {
        DataVmwareengineCluster_AutoscalingSettings {
            autoscaling_policies: core::default::Default::default(),
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_AutoscalingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_AutoscalingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_AutoscalingSettingsRef {
        DataVmwareengineCluster_AutoscalingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_AutoscalingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_policies` after provisioning.\n"]
    pub fn autoscaling_policies(
        &self,
    ) -> terrars::SetRef<DataVmwareengineCluster_AutoscalingSettings_AutoscalingPoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.autoscaling_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `cool_down_period` after provisioning.\n"]
    pub fn cool_down_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cool_down_period", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cluster_node_count` after provisioning.\n"]
    pub fn max_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cluster_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cluster_node_count` after provisioning.\n"]
    pub fn min_cluster_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cluster_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_peering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    #[doc = "Set the field `connection_count`.\n"]
    pub fn set_connection_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtu`.\n"]
    pub fn set_mtu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mtu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_peering`.\n"]
    pub fn set_network_peering(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_peering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {}

impl BuildDataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
    pub fn build(self) -> DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
        DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork {
            connection_count: core::default::Default::default(),
            mtu: core::default::Default::default(),
            network_peering: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
        DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_count` after provisioning.\n"]
    pub fn connection_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_count", self.base))
    }

    #[doc = "Get a reference to the value of field `mtu` after provisioning.\n"]
    pub fn mtu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.base))
    }

    #[doc = "Get a reference to the value of field `network_peering` after provisioning.\n"]
    pub fn network_peering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_peering", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_DatastoreMountConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datastore_network: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_share: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_colocation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataVmwareengineCluster_DatastoreMountConfig {
    #[doc = "Set the field `access_mode`.\n"]
    pub fn set_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datastore`.\n"]
    pub fn set_datastore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datastore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datastore_network`.\n"]
    pub fn set_datastore_network(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetwork>>,
    ) -> Self {
        self.datastore_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_share`.\n"]
    pub fn set_file_share(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_share = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_colocation`.\n"]
    pub fn set_ignore_colocation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_colocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_version`.\n"]
    pub fn set_nfs_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nfs_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.servers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_DatastoreMountConfig {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_DatastoreMountConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_DatastoreMountConfig {}

impl BuildDataVmwareengineCluster_DatastoreMountConfig {
    pub fn build(self) -> DataVmwareengineCluster_DatastoreMountConfig {
        DataVmwareengineCluster_DatastoreMountConfig {
            access_mode: core::default::Default::default(),
            datastore: core::default::Default::default(),
            datastore_network: core::default::Default::default(),
            file_share: core::default::Default::default(),
            ignore_colocation: core::default::Default::default(),
            nfs_version: core::default::Default::default(),
            servers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_DatastoreMountConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_DatastoreMountConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineCluster_DatastoreMountConfigRef {
        DataVmwareengineCluster_DatastoreMountConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_DatastoreMountConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\n"]
    pub fn access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `datastore` after provisioning.\n"]
    pub fn datastore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }

    #[doc = "Get a reference to the value of field `datastore_network` after provisioning.\n"]
    pub fn datastore_network(
        &self,
    ) -> terrars::ListRef<DataVmwareengineCluster_DatastoreMountConfig_DatastoreNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datastore_network", self.base))
    }

    #[doc = "Get a reference to the value of field `file_share` after provisioning.\n"]
    pub fn file_share(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_colocation` after provisioning.\n"]
    pub fn ignore_colocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_colocation", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs_version` after provisioning.\n"]
    pub fn nfs_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nfs_version", self.base))
    }

    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineCluster_NodeTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineCluster_NodeTypeConfigs {
    #[doc = "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineCluster_NodeTypeConfigs {
    type O = terrars::BlockAssignable<DataVmwareengineCluster_NodeTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineCluster_NodeTypeConfigs {}

impl BuildDataVmwareengineCluster_NodeTypeConfigs {
    pub fn build(self) -> DataVmwareengineCluster_NodeTypeConfigs {
        DataVmwareengineCluster_NodeTypeConfigs {
            custom_core_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineCluster_NodeTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineCluster_NodeTypeConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareengineCluster_NodeTypeConfigsRef {
        DataVmwareengineCluster_NodeTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineCluster_NodeTypeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}
