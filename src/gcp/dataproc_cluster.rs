#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocClusterData_ {
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
    graceful_decommission_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_cluster_config: std::option::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig>>,
    dynamic: DataprocClusterDynamic,
}

struct DataprocCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocClusterData_>,
}

#[derive(Clone)]
pub struct DataprocCluster(std::rc::Rc<DataprocCluster_>);

impl DataprocCluster {
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

    #[doc =
        "Set the field `graceful_decommission_timeout`.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn set_graceful_decommission_timeout(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().graceful_decommission_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_cluster_config`.\n"]
    pub fn set_virtual_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_cluster_config = std::option::Option::Some(d);
            },
        }
        self
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
        "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn graceful_decommission_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.graceful_decommission_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and zone."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocCluster_TimeoutsRef {
        DataprocCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_config` after provisioning.\n"]
    pub fn virtual_cluster_config(&self) -> terrars::ListRef<DataprocCluster_VirtualClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_cluster_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocCluster { }

impl terrars::ToListMappable for DataprocCluster {
    type O = terrars::ListRef<DataprocClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocCluster {
    pub tf_id: std::string::String,
    #[doc = "The name of the cluster, unique within the project and zone."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocCluster {
        let out = DataprocCluster(std::rc::Rc::new(DataprocCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                graceful_decommission_timeout: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_cluster_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn graceful_decommission_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.graceful_decommission_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and zone."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocCluster_TimeoutsRef {
        DataprocCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_config` after provisioning.\n"]
    pub fn virtual_cluster_config(&self) -> terrars::ListRef<DataprocCluster_VirtualClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_cluster_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocClusterDynamic {
    cluster_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig>>,
    virtual_cluster_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_AutoscalingConfig {
    policy_uri: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_AutoscalingConfig { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_AutoscalingConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_AutoscalingConfig {
    #[doc = "The autoscaling policy used by the cluster."]
    pub policy_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_AutoscalingConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_AutoscalingConfig {
        DataprocCluster_ClusterConfig_AutoscalingConfig { policy_uri: self.policy_uri }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_AutoscalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_AutoscalingConfigRef {
        DataprocCluster_ClusterConfig_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_AutoscalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy_uri` after provisioning.\nThe autoscaling policy used by the cluster."]
    pub fn policy_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_Accelerators {
    accelerator_count: terrars::PrimField<f64>,
    accelerator_type: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_MasterConfig_Accelerators { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_Accelerators {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_Accelerators {
    #[doc =
        "The number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub accelerator_count: terrars::PrimField<f64>,
    #[doc = "The short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub accelerator_type: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_Accelerators {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig_Accelerators {
        DataprocCluster_ClusterConfig_MasterConfig_Accelerators {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfig_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_AcceleratorsRef {
        DataprocCluster_ClusterConfig_MasterConfig_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
    #[doc =
        "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {}

impl BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    disk_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig>>,
    dynamic: DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigDynamic,
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the auxiliary node group. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nSpecifies the number of auxiliary nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {}

impl BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig {
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nList of auxiliary node group instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the auxiliary node group. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of auxiliary nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupDynamic {
    node_group_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
    roles: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig>>,
    dynamic: DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupDynamic,
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
    #[doc = "Set the field `node_group_config`.\n"]
    pub fn set_node_group_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_group_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
    #[doc = "Node group roles."]
    pub roles: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
    pub fn build(self) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup {
            roles: self.roles,
            node_group_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe Node group resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\nNode group roles."]
    pub fn roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `node_group_config` after provisioning.\n"]
    pub fn node_group_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup_NodeGroupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_group_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsDynamic {
    node_group: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup>>,
    dynamic: DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsDynamic,
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
    #[doc =
        "Set the field `node_group_id`.\nA node group ID. Generated if not specified. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters."]
    pub fn set_node_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_group`.\n"]
    pub fn set_node_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups {}

impl BuildDataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
    pub fn build(self) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroups {
            node_group_id: core::default::Default::default(),
            node_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef {
        DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_group_id` after provisioning.\nA node group ID. Generated if not specified. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters."]
    pub fn node_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups_NodeGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_overrides: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    metric_source: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
    #[doc =
        "Set the field `metric_overrides`.\nSpecify one or more [available OSS metrics] (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect."]
    pub fn set_metric_overrides(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metric_overrides = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
    #[doc =
        "A source for the collection of Dataproc OSS metrics (see [available OSS metrics] (https://cloud.google.com//dataproc/docs/guides/monitoring#available_oss_metrics))."]
    pub metric_source: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
    pub fn build(self) -> DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
        DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics {
            metric_overrides: core::default::Default::default(),
            metric_source: self.metric_source,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef {
        DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metric_overrides` after provisioning.\nSpecify one or more [available OSS metrics] (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect."]
    pub fn metric_overrides(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.metric_overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_source` after provisioning.\nA source for the collection of Dataproc OSS metrics (see [available OSS metrics] (https://cloud.google.com//dataproc/docs/guides/monitoring#available_oss_metrics))."]
    pub fn metric_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_DataprocMetricConfigDynamic {
    metrics: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_DataprocMetricConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics>>,
    dynamic: DataprocCluster_ClusterConfig_DataprocMetricConfigDynamic,
}

impl DataprocCluster_ClusterConfig_DataprocMetricConfig {
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_DataprocMetricConfig_Metrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_DataprocMetricConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_DataprocMetricConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_DataprocMetricConfig {}

impl BuildDataprocCluster_ClusterConfig_DataprocMetricConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_DataprocMetricConfig {
        DataprocCluster_ClusterConfig_DataprocMetricConfig {
            metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_DataprocMetricConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_DataprocMetricConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_DataprocMetricConfigRef {
        DataprocCluster_ClusterConfig_DataprocMetricConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_DataprocMetricConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_DataprocMetricConfig_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_EncryptionConfig {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_EncryptionConfig { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_EncryptionConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_EncryptionConfig {
    #[doc = "The Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_EncryptionConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_EncryptionConfig {
        DataprocCluster_ClusterConfig_EncryptionConfig { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_EncryptionConfigRef {
        DataprocCluster_ClusterConfig_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_EndpointConfig {
    enable_http_port_access: terrars::PrimField<bool>,
}

impl DataprocCluster_ClusterConfig_EndpointConfig { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_EndpointConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_EndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_EndpointConfig {
    #[doc =
        "The flag to enable http access to specific ports on the cluster from external sources (aka Component Gateway). Defaults to false."]
    pub enable_http_port_access: terrars::PrimField<bool>,
}

impl BuildDataprocCluster_ClusterConfig_EndpointConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_EndpointConfig {
        DataprocCluster_ClusterConfig_EndpointConfig { enable_http_port_access: self.enable_http_port_access }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_EndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_EndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_EndpointConfigRef {
        DataprocCluster_ClusterConfig_EndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_EndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_http_port_access` after provisioning.\nThe flag to enable http access to specific ports on the cluster from external sources (aka Component Gateway). Defaults to false."]
    pub fn enable_http_port_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http_port_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_ports` after provisioning.\nThe map of port descriptions to URLs. Will only be populated if enable_http_port_access is true."]
    pub fn http_ports(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.http_ports", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
    #[doc = "Set the field `confidential_instance_type`.\nDefines the type of Confidential Compute technology to use."]
    pub fn set_confidential_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.confidential_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_confidential_compute`.\nDefines whether the instance should have confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {}

impl BuildDataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
        DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig {
            confidential_instance_type: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef {
        DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_type` after provisioning.\nDefines the type of Confidential Compute technology to use."]
    pub fn confidential_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidential_instance_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity {
    node_group_uri: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity {
    #[doc = "The URI of a sole-tenant that the cluster will be created on."]
    pub node_group_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity {
    pub fn build(self) -> DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity {
        DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity { node_group_uri: self.node_group_uri }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef {
        DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_group_uri` after provisioning.\nThe URI of a sole-tenant that the cluster will be created on."]
    pub fn node_group_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_reservation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
    #[doc = "Set the field `consume_reservation_type`.\nType of reservation to consume."]
    pub fn set_consume_reservation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consume_reservation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nCorresponds to the label key of reservation resource."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nCorresponds to the label values of reservation resource."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {}

impl BuildDataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
    pub fn build(self) -> DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
        DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef {
        DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consume_reservation_type` after provisioning.\nType of reservation to consume."]
    pub fn consume_reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of reservation resource."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of reservation resource."]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
    #[doc =
        "Set the field `enable_integrity_monitoring`.\nDefines whether instances have integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\nDefines whether instances have Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\nDefines whether instances have the vTPM enabled."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {}

impl BuildDataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
        DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef {
        DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether instances have integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether instances have Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_vtpm` after provisioning.\nDefines whether instances have the vTPM enabled."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_GceClusterConfigDynamic {
    confidential_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig>>,
    node_group_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_affinity: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig>>,
    dynamic: DataprocCluster_ClusterConfig_GceClusterConfigDynamic,
}

impl DataprocCluster_ClusterConfig_GceClusterConfig {
    #[doc =
        "Set the field `internal_ip_only`.\nBy default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. If set to true, all instances in the cluster will only have internal IP addresses. Note: Private Google Access (also known as privateIpGoogleAccess) must be enabled on the subnetwork that the cluster will be launched in."]
    pub fn set_internal_ip_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_ip_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\nA map of the Compute Engine metadata entries to add to all instances"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe name or self_link of the Google Compute Engine network to the cluster will be part of. Conflicts with subnetwork. If neither is specified, this defaults to the \"default\" network."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_manager_tags`.\nA map of resource manager tags to add to all instances. Keys must be in the format tagKeys/{tag_key_id} and values in the format tagValues/{tag_value_id}."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe service account to be used by the Node VMs. If not specified, the \"default\" service account is used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_scopes`.\nThe set of Google API scopes to be made available on all of the node VMs under the service_account specified. These can be either FQDNs, or scope aliases."]
    pub fn set_service_account_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe name or self_link of the Google Compute Engine subnetwork the cluster will be part of. Conflicts with network."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nThe list of instance tags applied to instances in the cluster. Tags are used to identify valid sources or targets for network firewalls."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe GCP zone where your data is stored and used (i.e. where the master and the worker nodes will be created in). If region is set to 'global' (default) then zone is mandatory, otherwise GCP is able to make use of Auto Zone Placement to determine this automatically for you. Note: This setting additionally determines and restricts which computing resources are available for use with other configs such as cluster_config.master_config.machine_type and cluster_config.worker_config.machine_type."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidential_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_group_affinity`.\n"]
    pub fn set_node_group_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_group_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_GceClusterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_GceClusterConfig {}

impl BuildDataprocCluster_ClusterConfig_GceClusterConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_GceClusterConfig {
        DataprocCluster_ClusterConfig_GceClusterConfig {
            internal_ip_only: core::default::Default::default(),
            metadata: core::default::Default::default(),
            network: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_scopes: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            node_group_affinity: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_GceClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_GceClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_GceClusterConfigRef {
        DataprocCluster_ClusterConfig_GceClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_GceClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `internal_ip_only` after provisioning.\nBy default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. If set to true, all instances in the cluster will only have internal IP addresses. Note: Private Google Access (also known as privateIpGoogleAccess) must be enabled on the subnetwork that the cluster will be launched in."]
    pub fn internal_ip_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nA map of the Compute Engine metadata entries to add to all instances"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to the cluster will be part of. Conflicts with subnetwork. If neither is specified, this defaults to the \"default\" network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags to add to all instances. Keys must be in the format tagKeys/{tag_key_id} and values in the format tagValues/{tag_value_id}."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to be used by the Node VMs. If not specified, the \"default\" service account is used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_scopes` after provisioning.\nThe set of Google API scopes to be made available on all of the node VMs under the service_account specified. These can be either FQDNs, or scope aliases."]
    pub fn service_account_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_account_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork the cluster will be part of. Conflicts with network."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to instances in the cluster. Tags are used to identify valid sources or targets for network firewalls."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe GCP zone where your data is stored and used (i.e. where the master and the worker nodes will be created in). If region is set to 'global' (default) then zone is mandatory, otherwise GCP is able to make use of Auto Zone Placement to determine this automatically for you. Note: This setting additionally determines and restricts which computing resources are available for use with other configs such as cluster_config.master_config.machine_type and cluster_config.worker_config.machine_type."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_GceClusterConfig_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_group_affinity` after provisioning.\n"]
    pub fn node_group_affinity(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_GceClusterConfig_NodeGroupAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_group_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_GceClusterConfig_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_GceClusterConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_InitializationAction {
    script: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocCluster_ClusterConfig_InitializationAction {
    #[doc =
        "Set the field `timeout_sec`.\nThe maximum duration (in seconds) which script is allowed to take to execute its action. GCP will default to a predetermined computed value if not set (currently 300)."]
    pub fn set_timeout_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_InitializationAction {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_InitializationAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_InitializationAction {
    #[doc =
        "The script to be executed during initialization of the cluster. The script must be a GCS file with a gs:// prefix."]
    pub script: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_InitializationAction {
    pub fn build(self) -> DataprocCluster_ClusterConfig_InitializationAction {
        DataprocCluster_ClusterConfig_InitializationAction {
            script: self.script,
            timeout_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_InitializationActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_InitializationActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_InitializationActionRef {
        DataprocCluster_ClusterConfig_InitializationActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_InitializationActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nThe script to be executed during initialization of the cluster. The script must be a GCS file with a gs:// prefix."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nThe maximum duration (in seconds) which script is allowed to take to execute its action. GCP will default to a predetermined computed value if not set (currently 300)."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_LifecycleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_stop_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_delete_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_stop_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_ClusterConfig_LifecycleConfig {
    #[doc =
        "Set the field `auto_delete_time`.\nThe time when cluster will be auto-deleted. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_auto_delete_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_delete_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_stop_time`.\nThe time when cluster will be auto-stopped. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_auto_stop_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_stop_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `idle_delete_ttl`.\nThe duration to keep the cluster alive while idling (no jobs running). After this TTL, the cluster will be deleted. Valid range: [300s, 1209600s]."]
    pub fn set_idle_delete_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_delete_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `idle_stop_ttl`.\nThe duration to keep the cluster started while idling (no jobs running). After this TTL, the cluster will be stopped. Valid range: [10m, 14d]."]
    pub fn set_idle_stop_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_stop_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_LifecycleConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_LifecycleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_LifecycleConfig {}

impl BuildDataprocCluster_ClusterConfig_LifecycleConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_LifecycleConfig {
        DataprocCluster_ClusterConfig_LifecycleConfig {
            auto_delete_time: core::default::Default::default(),
            auto_stop_time: core::default::Default::default(),
            idle_delete_ttl: core::default::Default::default(),
            idle_stop_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_LifecycleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_LifecycleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_LifecycleConfigRef {
        DataprocCluster_ClusterConfig_LifecycleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_LifecycleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete_time` after provisioning.\nThe time when cluster will be auto-deleted. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn auto_delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_stop_time` after provisioning.\nThe time when cluster will be auto-stopped. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn auto_stop_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_stop_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_delete_ttl` after provisioning.\nThe duration to keep the cluster alive while idling (no jobs running). After this TTL, the cluster will be deleted. Valid range: [300s, 1209600s]."]
    pub fn idle_delete_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_delete_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_start_time` after provisioning.\nTime when the cluster became idle (most recent job finished) and became eligible for deletion due to idleness."]
    pub fn idle_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_stop_ttl` after provisioning.\nThe duration to keep the cluster started while idling (no jobs running). After this TTL, the cluster will be stopped. Valid range: [10m, 14d]."]
    pub fn idle_stop_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_stop_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
    #[doc = "Set the field `disk_size_gb`.\nSize of the attached disk, specified in GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nThe disk type of the attached disk. Such as \"pd-ssd\" or \"pd-standard\"."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe disk type of the attached disk. Currently only supports Hyperdisks: \"hyperdisk-balanced\", \"hyperdisk-extreme\", \"hyperdisk-ml\", \"hyperdisk-throughput\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
        DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef {
        DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the attached disk, specified in GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nThe disk type of the attached disk. Such as \"pd-ssd\" or \"pd-standard\"."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe disk type of the attached disk. Currently only supports Hyperdisks: \"hyperdisk-balanced\", \"hyperdisk-extreme\", \"hyperdisk-ml\", \"hyperdisk-throughput\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_MasterConfig_DiskConfigDynamic {
    attached_disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
    dynamic: DataprocCluster_ClusterConfig_MasterConfig_DiskConfigDynamic,
}

impl DataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
    #[doc =
        "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_disk_config`.\n"]
    pub fn set_attached_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attached_disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attached_disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_DiskConfig {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
        DataprocCluster_ClusterConfig_MasterConfig_DiskConfig {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
            attached_disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef {
        DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_disk_config` after provisioning.\n"]
    pub fn attached_disk_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    pub fn build(
        self,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
            machine_type: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigDynamic {
    attached_disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
    dynamic: DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigDynamic,
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
    #[doc =
        "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_ssd_interface`.\nInterface type of local SSDs (no Local SSDs or NVMe). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_disk_config`.\n"]
    pub fn set_attached_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attached_disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attached_disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
    pub fn build(
        self,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
            attached_disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The smallest allowed disk size is 10GB."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (no Local SSDs or NVMe). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_disk_config` after provisioning.\n"]
    pub fn attached_disk_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListDynamic {
    disk_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rank: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig>,
    >,
    dynamic: DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListDynamic,
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    #[doc = "Set the field `machine_types`.\nFull machine-type names, e.g. \"n1-standard-16\"."]
    pub fn set_machine_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.machine_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rank`.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn set_rank(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rank = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    type O =
        terrars
        ::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
            machine_types: core::default::Default::default(),
            rank: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_types` after provisioning.\nFull machine-type names, e.g. \"n1-standard-16\"."]
    pub fn machine_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rank` after provisioning.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn rank(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList_DiskConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyDynamic {
    instance_selection_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList>,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selection_list: std
    ::option
    ::Option<
        std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList>,
    >,
    dynamic: DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyDynamic,
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_selection_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy {
            instance_selection_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef {
        DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_selection_results` after provisioning.\nA list of instance selection results in the group."]
    pub fn instance_selection_results(
        &self,
    ) -> terrars
    ::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_results", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]
    pub fn instance_selection_list(
        &self,
    ) -> terrars
    ::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_MasterConfigDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    disk_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MasterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>>,
    dynamic: DataprocCluster_ClusterConfig_MasterConfigDynamic,
}

impl DataprocCluster_ClusterConfig_MasterConfig {
    #[doc = "Set the field `image_uri`.\nThe URI for the image to use for this master"]
    pub fn set_image_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the master. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nSpecifies the number of master nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MasterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MasterConfig {}

impl BuildDataprocCluster_ClusterConfig_MasterConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MasterConfig {
        DataprocCluster_ClusterConfig_MasterConfig {
            image_uri: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MasterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MasterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocCluster_ClusterConfig_MasterConfigRef {
        DataprocCluster_ClusterConfig_MasterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MasterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nThe URI for the image to use for this master"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nList of master instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the master. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of master nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_MetastoreConfig {
    dataproc_metastore_service: terrars::PrimField<std::string::String>,
}

impl DataprocCluster_ClusterConfig_MetastoreConfig { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_MetastoreConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_MetastoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_MetastoreConfig {
    #[doc = "Resource name of an existing Dataproc Metastore service."]
    pub dataproc_metastore_service: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_MetastoreConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_MetastoreConfig {
        DataprocCluster_ClusterConfig_MetastoreConfig { dataproc_metastore_service: self.dataproc_metastore_service }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_MetastoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_MetastoreConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_MetastoreConfigRef {
        DataprocCluster_ClusterConfig_MetastoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_MetastoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn dataproc_metastore_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_metastore_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigDynamic {
    attached_disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>>,
    dynamic: DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigDynamic,
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
    #[doc =
        "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each preemptible worker node, specified in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each preemptible worker node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each preemptible worker node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_disk_config`.\n"]
    pub fn set_attached_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attached_disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attached_disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {}

impl BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
            attached_disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each preemptible worker node, specified in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each preemptible worker node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each preemptible worker node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_disk_config` after provisioning.\n"]
    pub fn attached_disk_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig_AttachedDiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_capacity_base: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_capacity_percent_above_base: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    #[doc =
        "Set the field `standard_capacity_base`.\nThe base capacity that will always use Standard VMs to avoid risk of more preemption than the minimum capacity you need."]
    pub fn set_standard_capacity_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.standard_capacity_base = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `standard_capacity_percent_above_base`.\nThe percentage of target capacity that should use Standard VM. The remaining percentage will use Spot VMs."]
    pub fn set_standard_capacity_percent_above_base(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.standard_capacity_percent_above_base = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {}

impl BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    pub fn build(
        self,
    ) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
            standard_capacity_base: core::default::Default::default(),
            standard_capacity_percent_above_base: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `standard_capacity_base` after provisioning.\nThe base capacity that will always use Standard VMs to avoid risk of more preemption than the minimum capacity you need."]
    pub fn standard_capacity_base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard_capacity_base", self.base))
    }

    #[doc =
        "Get a reference to the value of field `standard_capacity_percent_above_base` after provisioning.\nThe percentage of target capacity that should use Standard VM. The remaining percentage will use Spot VMs."]
    pub fn standard_capacity_percent_above_base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard_capacity_percent_above_base", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyDynamic {
    instance_selection_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList>,
    >,
    provisioning_model_mix: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selection_list: std
    ::option
    ::Option<
        std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_model_mix: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix>,
    >,
    dynamic: DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyDynamic,
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_selection_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provisioning_model_mix`.\n"]
    pub fn set_provisioning_model_mix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioning_model_mix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioning_model_mix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
    type O =
        terrars::BlockAssignable<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {}

impl BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
    pub fn build(self) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy {
            instance_selection_list: core::default::Default::default(),
            provisioning_model_mix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_selection_results` after provisioning.\nA list of instance selection results in the group."]
    pub fn instance_selection_results(
        &self,
    ) -> terrars
    ::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_results", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]
    pub fn instance_selection_list(
        &self,
    ) -> terrars
    ::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_list", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_model_mix` after provisioning.\n"]
    pub fn provisioning_model_mix(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_model_mix", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfigDynamic {
    disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy>>,
    dynamic: DataprocCluster_ClusterConfig_PreemptibleWorkerConfigDynamic,
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
    #[doc = "Set the field `num_instances`.\nSpecifies the number of preemptible nodes to create. Defaults to 0."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preemptibility`.\nSpecifies the preemptibility of the secondary nodes. Defaults to PREEMPTIBLE."]
    pub fn set_preemptibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preemptibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig {}

impl BuildDataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfig {
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef {
        DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nList of preemptible instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of preemptible nodes to create. Defaults to 0."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptibility` after provisioning.\nSpecifies the preemptibility of the secondary nodes. Defaults to PREEMPTIBLE."]
    pub fn preemptibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptibility", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
    user_service_account_mapping: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig { }

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
    #[doc = "User to service account mappings for multi-tenancy."]
    pub user_service_account_mapping: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
        DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig {
            user_service_account_mapping: self.user_service_account_mapping,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef {
        DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_service_account_mapping` after provisioning.\nUser to service account mappings for multi-tenancy."]
    pub fn user_service_account_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_service_account_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_admin_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_kdc: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_realm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_shared_password_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_kerberos: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kdc_db_key_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_password_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keystore_password_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keystore_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    kms_key_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realm: std::option::Option<terrars::PrimField<std::string::String>>,
    root_principal_password_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tgt_lifetime_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    truststore_password_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    truststore_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
    #[doc =
        "Set the field `cross_realm_trust_admin_server`.\nThe admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_admin_server(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_admin_server = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_kdc`.\nThe KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_kdc(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_kdc = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_realm`.\nThe remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn set_cross_realm_trust_realm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_realm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_shared_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster\nKerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_shared_password_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_shared_password_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_kerberos`.\nFlag to indicate whether to Kerberize the cluster."]
    pub fn set_enable_kerberos(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_kerberos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kdc_db_key_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn set_kdc_db_key_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kdc_db_key_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_key_password_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_password_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keystore_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing\nthe password to the user provided keystore. For the self-signed certificate, this password is generated\nby Dataproc"]
    pub fn set_keystore_password_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keystore_password_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keystore_uri`.\nThe Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_keystore_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keystore_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `realm`.\nThe name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn set_realm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.realm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tgt_lifetime_hours`.\nThe lifetime of the ticket granting ticket, in hours."]
    pub fn set_tgt_lifetime_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tgt_lifetime_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `truststore_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_truststore_password_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.truststore_password_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `truststore_uri`.\nThe Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_truststore_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.truststore_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
    #[doc = "The uri of the KMS key used to encrypt various sensitive files."]
    pub kms_key_uri: terrars::PrimField<std::string::String>,
    #[doc = "The cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub root_principal_password_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
        DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig {
            cross_realm_trust_admin_server: core::default::Default::default(),
            cross_realm_trust_kdc: core::default::Default::default(),
            cross_realm_trust_realm: core::default::Default::default(),
            cross_realm_trust_shared_password_uri: core::default::Default::default(),
            enable_kerberos: core::default::Default::default(),
            kdc_db_key_uri: core::default::Default::default(),
            key_password_uri: core::default::Default::default(),
            keystore_password_uri: core::default::Default::default(),
            keystore_uri: core::default::Default::default(),
            kms_key_uri: self.kms_key_uri,
            realm: core::default::Default::default(),
            root_principal_password_uri: self.root_principal_password_uri,
            tgt_lifetime_hours: core::default::Default::default(),
            truststore_password_uri: core::default::Default::default(),
            truststore_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef {
        DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_admin_server` after provisioning.\nThe admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_admin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_admin_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_kdc` after provisioning.\nThe KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_kdc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_kdc", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_realm` after provisioning.\nThe remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn cross_realm_trust_realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_realm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_shared_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster\nKerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn cross_realm_trust_shared_password_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_shared_password_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_kerberos` after provisioning.\nFlag to indicate whether to Kerberize the cluster."]
    pub fn enable_kerberos(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_kerberos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kdc_db_key_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn kdc_db_key_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kdc_db_key_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn key_password_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_password_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keystore_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing\nthe password to the user provided keystore. For the self-signed certificate, this password is generated\nby Dataproc"]
    pub fn keystore_password_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore_password_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keystore_uri` after provisioning.\nThe Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn keystore_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_uri` after provisioning.\nThe uri of the KMS key used to encrypt various sensitive files."]
    pub fn kms_key_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `realm` after provisioning.\nThe name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_principal_password_uri` after provisioning.\nThe cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub fn root_principal_password_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_principal_password_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tgt_lifetime_hours` after provisioning.\nThe lifetime of the ticket granting ticket, in hours."]
    pub fn tgt_lifetime_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tgt_lifetime_hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `truststore_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn truststore_password_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.truststore_password_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `truststore_uri` after provisioning.\nThe Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn truststore_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.truststore_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_SecurityConfigDynamic {
    identity_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig>>,
    kerberos_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_SecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig>>,
    dynamic: DataprocCluster_ClusterConfig_SecurityConfigDynamic,
}

impl DataprocCluster_ClusterConfig_SecurityConfig {
    #[doc = "Set the field `identity_config`.\n"]
    pub fn set_identity_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kerberos_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kerberos_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_SecurityConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_SecurityConfig {}

impl BuildDataprocCluster_ClusterConfig_SecurityConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_SecurityConfig {
        DataprocCluster_ClusterConfig_SecurityConfig {
            identity_config: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_SecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_SecurityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_SecurityConfigRef {
        DataprocCluster_ClusterConfig_SecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_SecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_config` after provisioning.\n"]
    pub fn identity_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_SecurityConfig_IdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_SecurityConfig_KerberosConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_SoftwareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_components: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocCluster_ClusterConfig_SoftwareConfig {
    #[doc =
        "Set the field `image_version`.\nThe Cloud Dataproc image version to use for the cluster - this controls the sets of software versions installed onto the nodes when you create clusters. If not specified, defaults to the latest version."]
    pub fn set_image_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optional_components`.\nThe set of optional components to activate on the cluster."]
    pub fn set_optional_components(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.optional_components = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_properties`.\nA list of override and additional properties (key/value pairs) used to modify various aspects of the common configuration files used when creating a cluster."]
    pub fn set_override_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.override_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_SoftwareConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_SoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_SoftwareConfig {}

impl BuildDataprocCluster_ClusterConfig_SoftwareConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_SoftwareConfig {
        DataprocCluster_ClusterConfig_SoftwareConfig {
            image_version: core::default::Default::default(),
            optional_components: core::default::Default::default(),
            override_properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_SoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_SoftwareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_ClusterConfig_SoftwareConfigRef {
        DataprocCluster_ClusterConfig_SoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_SoftwareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_version` after provisioning.\nThe Cloud Dataproc image version to use for the cluster - this controls the sets of software versions installed onto the nodes when you create clusters. If not specified, defaults to the latest version."]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `optional_components` after provisioning.\nThe set of optional components to activate on the cluster."]
    pub fn optional_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.optional_components", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_properties` after provisioning.\nA list of override and additional properties (key/value pairs) used to modify various aspects of the common configuration files used when creating a cluster."]
    pub fn override_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.override_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA list of the properties used to set the daemon config files. This will include any values supplied by the user via cluster_config.software_config.override_properties"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfig_WorkerConfigDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    disk_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig_WorkerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>>,
    dynamic: DataprocCluster_ClusterConfig_WorkerConfigDynamic,
}

impl DataprocCluster_ClusterConfig_WorkerConfig {
    #[doc = "Set the field `image_uri`.\nThe URI for the image to use for this master/worker"]
    pub fn set_image_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master/worker"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the master/worker. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_num_instances`.\nThe minimum number of primary worker instances to create."]
    pub fn set_min_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nSpecifies the number of worker nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_DiskConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig_WorkerConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig_WorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig_WorkerConfig {}

impl BuildDataprocCluster_ClusterConfig_WorkerConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig_WorkerConfig {
        DataprocCluster_ClusterConfig_WorkerConfig {
            image_uri: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            min_num_instances: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfig_WorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfig_WorkerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocCluster_ClusterConfig_WorkerConfigRef {
        DataprocCluster_ClusterConfig_WorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfig_WorkerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nThe URI for the image to use for this master/worker"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nList of master/worker instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master/worker"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the master/worker. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_num_instances` after provisioning.\nThe minimum number of primary worker instances to create."]
    pub fn min_num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_num_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of worker nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfig_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_ClusterConfigDynamic {
    autoscaling_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_AutoscalingConfig>>,
    auxiliary_node_groups: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups>>,
    dataproc_metric_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_DataprocMetricConfig>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_EncryptionConfig>>,
    endpoint_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_EndpointConfig>>,
    gce_cluster_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_GceClusterConfig>>,
    initialization_action: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_InitializationAction>>,
    lifecycle_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_LifecycleConfig>>,
    master_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MasterConfig>>,
    metastore_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_MetastoreConfig>>,
    preemptible_worker_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig>>,
    security_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_SecurityConfig>>,
    software_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_SoftwareConfig>>,
    worker_config: std::option::Option<terrars::DynamicBlock<DataprocCluster_ClusterConfig_WorkerConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_ClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_AutoscalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auxiliary_node_groups: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_metric_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_DataprocMetricConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_EndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_cluster_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_GceClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialization_action: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_InitializationAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_LifecycleConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MasterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_MetastoreConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible_worker_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_SecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_SoftwareConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_config: std::option::Option<std::vec::Vec<DataprocCluster_ClusterConfig_WorkerConfig>>,
    dynamic: DataprocCluster_ClusterConfigDynamic,
}

impl DataprocCluster_ClusterConfig {
    #[doc = "Set the field `cluster_tier`.\nSpecifies the tier of the cluster created."]
    pub fn set_cluster_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_type`.\nThe type of the cluster."]
    pub fn set_cluster_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\nSpecifies the engine of the cluster created."]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `staging_bucket`.\nThe Cloud Storage staging bucket used to stage files, such as Hadoop jars, between client machines and the cluster. Note: If you don't explicitly specify a staging_bucket then GCP will auto create / assign one for you. However, you are not guaranteed an auto generated bucket which is solely dedicated to your cluster; it may be shared with other clusters in the same region/zone also choosing to use the auto generation option."]
    pub fn set_staging_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.staging_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temp_bucket`.\nThe Cloud Storage temp bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. Note: If you don't explicitly specify a temp_bucket then GCP will auto create / assign one for you."]
    pub fn set_temp_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temp_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_AutoscalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auxiliary_node_groups`.\n"]
    pub fn set_auxiliary_node_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_AuxiliaryNodeGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auxiliary_node_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_node_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataproc_metric_config`.\n"]
    pub fn set_dataproc_metric_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_DataprocMetricConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataproc_metric_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataproc_metric_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_config`.\n"]
    pub fn set_endpoint_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_EndpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gce_cluster_config`.\n"]
    pub fn set_gce_cluster_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_GceClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initialization_action`.\n"]
    pub fn set_initialization_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_InitializationAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initialization_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initialization_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_config`.\n"]
    pub fn set_lifecycle_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_LifecycleConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_config`.\n"]
    pub fn set_master_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MasterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metastore_config`.\n"]
    pub fn set_metastore_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_MetastoreConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metastore_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preemptible_worker_config`.\n"]
    pub fn set_preemptible_worker_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_PreemptibleWorkerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preemptible_worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preemptible_worker_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_SecurityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_SoftwareConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocCluster_ClusterConfig_WorkerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_ClusterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_ClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_ClusterConfig {}

impl BuildDataprocCluster_ClusterConfig {
    pub fn build(self) -> DataprocCluster_ClusterConfig {
        DataprocCluster_ClusterConfig {
            cluster_tier: core::default::Default::default(),
            cluster_type: core::default::Default::default(),
            engine: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            temp_bucket: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            auxiliary_node_groups: core::default::Default::default(),
            dataproc_metric_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            endpoint_config: core::default::Default::default(),
            gce_cluster_config: core::default::Default::default(),
            initialization_action: core::default::Default::default(),
            lifecycle_config: core::default::Default::default(),
            master_config: core::default::Default::default(),
            metastore_config: core::default::Default::default(),
            preemptible_worker_config: core::default::Default::default(),
            security_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            worker_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_ClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocCluster_ClusterConfigRef {
        DataprocCluster_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_ClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\n The name of the cloud storage bucket ultimately used to house the staging data for the cluster. If staging_bucket is specified, it will contain this value, otherwise it will be the auto generated name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_tier` after provisioning.\nSpecifies the tier of the cluster created."]
    pub fn cluster_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of the cluster."]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `engine` after provisioning.\nSpecifies the engine of the cluster created."]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc =
        "Get a reference to the value of field `staging_bucket` after provisioning.\nThe Cloud Storage staging bucket used to stage files, such as Hadoop jars, between client machines and the cluster. Note: If you don't explicitly specify a staging_bucket then GCP will auto create / assign one for you. However, you are not guaranteed an auto generated bucket which is solely dedicated to your cluster; it may be shared with other clusters in the same region/zone also choosing to use the auto generation option."]
    pub fn staging_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temp_bucket` after provisioning.\nThe Cloud Storage temp bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. Note: If you don't explicitly specify a temp_bucket then GCP will auto create / assign one for you."]
    pub fn temp_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `auxiliary_node_groups` after provisioning.\n"]
    pub fn auxiliary_node_groups(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_AuxiliaryNodeGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auxiliary_node_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `dataproc_metric_config` after provisioning.\n"]
    pub fn dataproc_metric_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_DataprocMetricConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataproc_metric_config", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_EndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_cluster_config` after provisioning.\n"]
    pub fn gce_cluster_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_GceClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_cluster_config", self.base))
    }

    #[doc = "Get a reference to the value of field `initialization_action` after provisioning.\n"]
    pub fn initialization_action(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_InitializationActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialization_action", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config` after provisioning.\n"]
    pub fn lifecycle_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_LifecycleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_config", self.base))
    }

    #[doc = "Get a reference to the value of field `master_config` after provisioning.\n"]
    pub fn master_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_MasterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_config", self.base))
    }

    #[doc = "Get a reference to the value of field `metastore_config` after provisioning.\n"]
    pub fn metastore_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_MetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metastore_config", self.base))
    }

    #[doc = "Get a reference to the value of field `preemptible_worker_config` after provisioning.\n"]
    pub fn preemptible_worker_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_ClusterConfig_PreemptibleWorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preemptible_worker_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> terrars::ListRef<DataprocCluster_ClusterConfig_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_Timeouts {
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

impl terrars::ToListMappable for DataprocCluster_Timeouts {
    type O = terrars::BlockAssignable<DataprocCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_Timeouts {}

impl BuildDataprocCluster_Timeouts {
    pub fn build(self) -> DataprocCluster_Timeouts {
        DataprocCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocCluster_TimeoutsRef {
        DataprocCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_TimeoutsRef {
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
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_metastore_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
    #[doc = "Set the field `dataproc_metastore_service`.\nThe Hive Metastore configuration for this workload."]
    pub fn set_dataproc_metastore_service(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dataproc_metastore_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
    type O = terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {}

impl BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig {
            dataproc_metastore_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_metastore_service` after provisioning.\nThe Hive Metastore configuration for this workload."]
    pub fn dataproc_metastore_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_metastore_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
    #[doc =
        "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataproc_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {}

impl BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigDynamic {
    metastore_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig>>,
    spark_history_server_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_history_server_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig>>,
    dynamic: DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigDynamic,
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
    #[doc = "Set the field `metastore_config`.\n"]
    pub fn set_metastore_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metastore_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_history_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
    type O = terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {}

impl BuildDataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig {
            metastore_config: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef {
        DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metastore_config` after provisioning.\n"]
    pub fn metastore_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_MetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metastore_config", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig_SparkHistoryServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
    #[doc =
        "Set the field `max_node_count`.\nThe maximum number of nodes in the node pool. Must be >= minNodeCount, and must be > 0."]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_node_count`.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
    pub fn build(
        self,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling {
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum number of nodes in the node pool. Must be >= minNodeCount, and must be > 0."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
    #[doc =
        "Set the field `local_ssd_count`.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."]
    pub fn set_local_ssd_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.local_ssd_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe name of a Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or a newer CPU platform. Specify the friendly names of CPU platforms, such as \"Intel Haswell\" or \"Intel Sandy Bridge\"."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preemptible`.\nWhether the nodes are created as preemptible VM instances. Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role)."]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `spot`.\nSpot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag."]
    pub fn set_spot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.spot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
    pub fn build(
        self,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config {
            local_ssd_count: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            spot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `local_ssd_count` after provisioning.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."]
    pub fn local_ssd_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Compute Engine machine type."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or a newer CPU platform. Specify the friendly names of CPU platforms, such as \"Intel Haswell\" or \"Intel Sandy Bridge\"."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptible` after provisioning.\nWhether the nodes are created as preemptible VM instances. Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role)."]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spot` after provisioning.\nSpot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag."]
    pub fn spot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigDynamic {
    autoscaling: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling,
        >,
    >,
    config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
    locations: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config,
        >,
    >,
    dynamic: DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigDynamic,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Autoscaling,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_Config,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
    #[doc =
        "The list of Compute Engine zones where node pool nodes associated with a Dataproc on GKE virtual cluster will be located."]
    pub locations: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
    pub fn build(
        self,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig {
            locations: self.locations,
            autoscaling: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `locations` after provisioning.\nThe list of Compute Engine zones where node pool nodes associated with a Dataproc on GKE virtual cluster will be located."]
    pub fn locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_AutoscalingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig_ConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetDynamic {
    node_pool_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
    node_pool: terrars::PrimField<std::string::String>,
    roles: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig,
        >,
    >,
    dynamic: DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetDynamic,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
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
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfig,
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

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
    type O =
        terrars
        ::BlockAssignable<
            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
    #[doc =
        "The target GKE node pool. Format: 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{nodePool}'"]
    pub node_pool: terrars::PrimField<std::string::String>,
    #[doc = "The roles associated with the GKE node pool."]
    pub roles: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
    pub fn build(
        self,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget {
            node_pool: self.node_pool,
            roles: self.roles,
            node_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_pool` after provisioning.\nThe target GKE node pool. Format: 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{nodePool}'"]
    pub fn node_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nThe roles associated with the GKE node pool."]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"]
    pub fn node_pool_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget_NodePoolConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigDynamic {
    node_pool_target: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget>,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_cluster_target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool_target: std
    ::option
    ::Option<
        std::vec::Vec<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget>,
    >,
    dynamic: DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigDynamic,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
    #[doc =
        "Set the field `gke_cluster_target`.\nA target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster (the GKE cluster can be zonal or regional). Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn set_gke_cluster_target(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gke_cluster_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_pool_target`.\n"]
    pub fn set_node_pool_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTarget,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_pool_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig {
            gke_cluster_target: core::default::Default::default(),
            node_pool_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gke_cluster_target` after provisioning.\nA target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster (the GKE cluster can be zonal or regional). Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn gke_cluster_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster_target", self.base))
    }

    #[doc = "Get a reference to the value of field `node_pool_target` after provisioning.\n"]
    pub fn node_pool_target(
        &self,
    ) -> terrars
    ::ListRef<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig_NodePoolTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
    component_version: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
    #[doc =
        "Set the field `properties`.\nThe properties to set on daemon config files. Property keys are specified in prefix:property format, for example spark:spark.kubernetes.container.image."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
    #[doc =
        "The components that should be installed in this Dataproc cluster. The key must be a string from the KubernetesComponent enumeration. The value is the version of the software to be installed."]
    pub component_version: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig {
            component_version: self.component_version,
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `component_version` after provisioning.\nThe components that should be installed in this Dataproc cluster. The key must be a string from the KubernetesComponent enumeration. The value is the version of the software to be installed."]
    pub fn component_version(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.component_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nThe properties to set on daemon config files. Property keys are specified in prefix:property format, for example spark:spark.kubernetes.container.image."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigDynamic {
    gke_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig>>,
    kubernetes_software_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_cluster_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_software_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig>>,
    dynamic: DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigDynamic,
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
    #[doc =
        "Set the field `kubernetes_namespace`.\nA namespace within the Kubernetes cluster to deploy into. If this namespace does not exist, it is created. If it exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it. If not specified, the name of the Dataproc Cluster is used."]
    pub fn set_kubernetes_namespace(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kubernetes_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_cluster_config`.\n"]
    pub fn set_gke_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_software_config`.\n"]
    pub fn set_kubernetes_software_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes_software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_software_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {}

impl BuildDataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig {
            kubernetes_namespace: core::default::Default::default(),
            gke_cluster_config: core::default::Default::default(),
            kubernetes_software_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef {
        DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kubernetes_namespace` after provisioning.\nA namespace within the Kubernetes cluster to deploy into. If this namespace does not exist, it is created. If it exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it. If not specified, the name of the Dataproc Cluster is used."]
    pub fn kubernetes_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_cluster_config` after provisioning.\n"]
    pub fn gke_cluster_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_GkeClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_cluster_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes_software_config` after provisioning.\n"]
    pub fn kubernetes_software_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig_KubernetesSoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_software_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocCluster_VirtualClusterConfigDynamic {
    auxiliary_services_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig>>,
    kubernetes_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig>>,
}

#[derive(Serialize)]
pub struct DataprocCluster_VirtualClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auxiliary_services_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_cluster_config: std
    ::option
    ::Option<std::vec::Vec<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig>>,
    dynamic: DataprocCluster_VirtualClusterConfigDynamic,
}

impl DataprocCluster_VirtualClusterConfig {
    #[doc =
        "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket."]
    pub fn set_staging_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.staging_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auxiliary_services_config`.\n"]
    pub fn set_auxiliary_services_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auxiliary_services_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_services_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_cluster_config`.\n"]
    pub fn set_kubernetes_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocCluster_VirtualClusterConfig {
    type O = terrars::BlockAssignable<DataprocCluster_VirtualClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocCluster_VirtualClusterConfig {}

impl BuildDataprocCluster_VirtualClusterConfig {
    pub fn build(self) -> DataprocCluster_VirtualClusterConfig {
        DataprocCluster_VirtualClusterConfig {
            staging_bucket: core::default::Default::default(),
            auxiliary_services_config: core::default::Default::default(),
            kubernetes_cluster_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocCluster_VirtualClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocCluster_VirtualClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocCluster_VirtualClusterConfigRef {
        DataprocCluster_VirtualClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocCluster_VirtualClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket."]
    pub fn staging_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `auxiliary_services_config` after provisioning.\n"]
    pub fn auxiliary_services_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_AuxiliaryServicesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auxiliary_services_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes_cluster_config` after provisioning.\n"]
    pub fn kubernetes_cluster_config(
        &self,
    ) -> terrars::ListRef<DataprocCluster_VirtualClusterConfig_KubernetesClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_cluster_config", self.base))
    }
}
