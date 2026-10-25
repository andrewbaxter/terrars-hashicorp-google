#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocAutoscalingPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_algorithm: std::option::Option<std::vec::Vec<DataprocAutoscalingPolicy_BasicAlgorithm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_worker_config: std::option::Option<std::vec::Vec<DataprocAutoscalingPolicy_SecondaryWorkerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocAutoscalingPolicy_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_config: std::option::Option<std::vec::Vec<DataprocAutoscalingPolicy_WorkerConfig>>,
    dynamic: DataprocAutoscalingPolicyDynamic,
}

struct DataprocAutoscalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocAutoscalingPolicyData_>,
}

#[derive(Clone)]
pub struct DataprocAutoscalingPolicy(std::rc::Rc<DataprocAutoscalingPolicy_>);

impl DataprocAutoscalingPolicy {
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

    #[doc =
        "Set the field `location`.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_algorithm`.\n"]
    pub fn set_basic_algorithm(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocAutoscalingPolicy_BasicAlgorithm>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_algorithm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_algorithm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_worker_config`.\n"]
    pub fn set_secondary_worker_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocAutoscalingPolicy_SecondaryWorkerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_worker_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocAutoscalingPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocAutoscalingPolicy_WorkerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_config = std::option::Option::Some(d);
            },
        }
        self
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
        "Get a reference to the value of field `location` after provisioning.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe \"resource name\" of the autoscaling policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_algorithm` after provisioning.\n"]
    pub fn basic_algorithm(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_BasicAlgorithmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_SecondaryWorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_worker_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocAutoscalingPolicy_TimeoutsRef {
        DataprocAutoscalingPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocAutoscalingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocAutoscalingPolicy { }

impl terrars::ToListMappable for DataprocAutoscalingPolicy {
    type O = terrars::ListRef<DataprocAutoscalingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocAutoscalingPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_autoscaling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocAutoscalingPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub policy_id: terrars::PrimField<std::string::String>,
}

impl BuildDataprocAutoscalingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocAutoscalingPolicy {
        let out = DataprocAutoscalingPolicy(std::rc::Rc::new(DataprocAutoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocAutoscalingPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                policy_id: self.policy_id,
                project: core::default::Default::default(),
                basic_algorithm: core::default::Default::default(),
                secondary_worker_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                worker_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocAutoscalingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocAutoscalingPolicyRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe \"resource name\" of the autoscaling policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_algorithm` after provisioning.\n"]
    pub fn basic_algorithm(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_BasicAlgorithmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_SecondaryWorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_worker_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocAutoscalingPolicy_TimeoutsRef {
        DataprocAutoscalingPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocAutoscalingPolicyDynamic {
    basic_algorithm: std::option::Option<terrars::DynamicBlock<DataprocAutoscalingPolicy_BasicAlgorithm>>,
    secondary_worker_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocAutoscalingPolicy_SecondaryWorkerConfig>>,
    worker_config: std::option::Option<terrars::DynamicBlock<DataprocAutoscalingPolicy_WorkerConfig>>,
}

#[derive(Serialize)]
pub struct DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
    graceful_decommission_timeout: terrars::PrimField<std::string::String>,
    scale_down_factor: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_min_worker_fraction: std::option::Option<terrars::PrimField<f64>>,
    scale_up_factor: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_up_min_worker_fraction: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
    #[doc =
        "Set the field `scale_down_min_worker_fraction`.\nMinimum scale-down threshold as a fraction of total cluster size before scaling occurs.\nFor example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must\nrecommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0\nmeans the autoscaler will scale down on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn set_scale_down_min_worker_fraction(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_down_min_worker_fraction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scale_up_min_worker_fraction`.\nMinimum scale-up threshold as a fraction of total cluster size before scaling\noccurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler\nmust recommend at least a 2-worker scale-up for the cluster to scale. A threshold of\n0 means the autoscaler will scale up on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn set_scale_up_min_worker_fraction(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_up_min_worker_fraction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
    type O = terrars::BlockAssignable<DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
    #[doc =
        "Timeout for YARN graceful decommissioning of Node Managers. Specifies the\nduration to wait for jobs to complete before forcefully removing workers\n(and potentially interrupting jobs). Only applicable to downscaling operations.\n\nBounds: [0s, 1d]."]
    pub graceful_decommission_timeout: terrars::PrimField<std::string::String>,
    #[doc =
        "Fraction of average pending memory in the last cooldown period for which to\nremove workers. A scale-down factor of 1 will result in scaling down so that there\nis no available memory remaining after the update (more aggressive scaling).\nA scale-down factor of 0 disables removing workers, which can be beneficial for\nautoscaling a single job.\n\nBounds: [0.0, 1.0]."]
    pub scale_down_factor: terrars::PrimField<f64>,
    #[doc =
        "Fraction of average pending memory in the last cooldown period for which to\nadd workers. A scale-up factor of 1.0 will result in scaling up so that there\nis no pending memory remaining after the update (more aggressive scaling).\nA scale-up factor closer to 0 will result in a smaller magnitude of scaling up\n(less aggressive scaling).\n\nBounds: [0.0, 1.0]."]
    pub scale_up_factor: terrars::PrimField<f64>,
}

impl BuildDataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
    pub fn build(self) -> DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
        DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig {
            graceful_decommission_timeout: self.graceful_decommission_timeout,
            scale_down_factor: self.scale_down_factor,
            scale_down_min_worker_fraction: core::default::Default::default(),
            scale_up_factor: self.scale_up_factor,
            scale_up_min_worker_fraction: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef {
        DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nTimeout for YARN graceful decommissioning of Node Managers. Specifies the\nduration to wait for jobs to complete before forcefully removing workers\n(and potentially interrupting jobs). Only applicable to downscaling operations.\n\nBounds: [0s, 1d]."]
    pub fn graceful_decommission_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graceful_decommission_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_down_factor` after provisioning.\nFraction of average pending memory in the last cooldown period for which to\nremove workers. A scale-down factor of 1 will result in scaling down so that there\nis no available memory remaining after the update (more aggressive scaling).\nA scale-down factor of 0 disables removing workers, which can be beneficial for\nautoscaling a single job.\n\nBounds: [0.0, 1.0]."]
    pub fn scale_down_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_factor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_down_min_worker_fraction` after provisioning.\nMinimum scale-down threshold as a fraction of total cluster size before scaling occurs.\nFor example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must\nrecommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0\nmeans the autoscaler will scale down on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn scale_down_min_worker_fraction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_min_worker_fraction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_up_factor` after provisioning.\nFraction of average pending memory in the last cooldown period for which to\nadd workers. A scale-up factor of 1.0 will result in scaling up so that there\nis no pending memory remaining after the update (more aggressive scaling).\nA scale-up factor closer to 0 will result in a smaller magnitude of scaling up\n(less aggressive scaling).\n\nBounds: [0.0, 1.0]."]
    pub fn scale_up_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_up_factor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale_up_min_worker_fraction` after provisioning.\nMinimum scale-up threshold as a fraction of total cluster size before scaling\noccurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler\nmust recommend at least a 2-worker scale-up for the cluster to scale. A threshold of\n0 means the autoscaler will scale up on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn scale_up_min_worker_fraction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_up_min_worker_fraction", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocAutoscalingPolicy_BasicAlgorithmDynamic {
    yarn_config: std::option::Option<terrars::DynamicBlock<DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig>>,
}

#[derive(Serialize)]
pub struct DataprocAutoscalingPolicy_BasicAlgorithm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yarn_config: std::option::Option<std::vec::Vec<DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig>>,
    dynamic: DataprocAutoscalingPolicy_BasicAlgorithmDynamic,
}

impl DataprocAutoscalingPolicy_BasicAlgorithm {
    #[doc =
        "Set the field `cooldown_period`.\nDuration between scaling events. A scaling period starts after the\nupdate operation from the previous event has completed.\n\nBounds: [2m, 1d]. Default: 2m."]
    pub fn set_cooldown_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cooldown_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `yarn_config`.\n"]
    pub fn set_yarn_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.yarn_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.yarn_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocAutoscalingPolicy_BasicAlgorithm {
    type O = terrars::BlockAssignable<DataprocAutoscalingPolicy_BasicAlgorithm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocAutoscalingPolicy_BasicAlgorithm {}

impl BuildDataprocAutoscalingPolicy_BasicAlgorithm {
    pub fn build(self) -> DataprocAutoscalingPolicy_BasicAlgorithm {
        DataprocAutoscalingPolicy_BasicAlgorithm {
            cooldown_period: core::default::Default::default(),
            yarn_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocAutoscalingPolicy_BasicAlgorithmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicy_BasicAlgorithmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocAutoscalingPolicy_BasicAlgorithmRef {
        DataprocAutoscalingPolicy_BasicAlgorithmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocAutoscalingPolicy_BasicAlgorithmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cooldown_period` after provisioning.\nDuration between scaling events. A scaling period starts after the\nupdate operation from the previous event has completed.\n\nBounds: [2m, 1d]. Default: 2m."]
    pub fn cooldown_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown_period", self.base))
    }

    #[doc = "Get a reference to the value of field `yarn_config` after provisioning.\n"]
    pub fn yarn_config(&self) -> terrars::ListRef<DataprocAutoscalingPolicy_BasicAlgorithm_YarnConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yarn_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocAutoscalingPolicy_SecondaryWorkerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocAutoscalingPolicy_SecondaryWorkerConfig {
    #[doc =
        "Set the field `max_instances`.\nMaximum number of instances for this group. Note that by default, clusters will not use\nsecondary workers. Required for secondary workers if the minimum secondary instances is set.\nBounds: [minInstances, ). Defaults to 0."]
    pub fn set_max_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_instances`.\nMinimum number of instances for this group. Bounds: [0, maxInstances]. Defaults to 0."]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocAutoscalingPolicy_SecondaryWorkerConfig {
    type O = terrars::BlockAssignable<DataprocAutoscalingPolicy_SecondaryWorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocAutoscalingPolicy_SecondaryWorkerConfig {}

impl BuildDataprocAutoscalingPolicy_SecondaryWorkerConfig {
    pub fn build(self) -> DataprocAutoscalingPolicy_SecondaryWorkerConfig {
        DataprocAutoscalingPolicy_SecondaryWorkerConfig {
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocAutoscalingPolicy_SecondaryWorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicy_SecondaryWorkerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocAutoscalingPolicy_SecondaryWorkerConfigRef {
        DataprocAutoscalingPolicy_SecondaryWorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocAutoscalingPolicy_SecondaryWorkerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances for this group. Note that by default, clusters will not use\nsecondary workers. Required for secondary workers if the minimum secondary instances is set.\nBounds: [minInstances, ). Defaults to 0."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances for this group. Bounds: [0, maxInstances]. Defaults to 0."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocAutoscalingPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocAutoscalingPolicy_Timeouts {
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

impl terrars::ToListMappable for DataprocAutoscalingPolicy_Timeouts {
    type O = terrars::BlockAssignable<DataprocAutoscalingPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocAutoscalingPolicy_Timeouts {}

impl BuildDataprocAutoscalingPolicy_Timeouts {
    pub fn build(self) -> DataprocAutoscalingPolicy_Timeouts {
        DataprocAutoscalingPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocAutoscalingPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocAutoscalingPolicy_TimeoutsRef {
        DataprocAutoscalingPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocAutoscalingPolicy_TimeoutsRef {
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
pub struct DataprocAutoscalingPolicy_WorkerConfig {
    max_instances: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocAutoscalingPolicy_WorkerConfig {
    #[doc =
        "Set the field `min_instances`.\nMinimum number of instances for this group. Bounds: [2, maxInstances]. Defaults to 2."]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocAutoscalingPolicy_WorkerConfig {
    type O = terrars::BlockAssignable<DataprocAutoscalingPolicy_WorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocAutoscalingPolicy_WorkerConfig {
    #[doc = "Maximum number of instances for this group."]
    pub max_instances: terrars::PrimField<f64>,
}

impl BuildDataprocAutoscalingPolicy_WorkerConfig {
    pub fn build(self) -> DataprocAutoscalingPolicy_WorkerConfig {
        DataprocAutoscalingPolicy_WorkerConfig {
            max_instances: self.max_instances,
            min_instances: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocAutoscalingPolicy_WorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocAutoscalingPolicy_WorkerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocAutoscalingPolicy_WorkerConfigRef {
        DataprocAutoscalingPolicy_WorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocAutoscalingPolicy_WorkerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances for this group."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances for this group. Bounds: [2, maxInstances]. Defaults to 2."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
