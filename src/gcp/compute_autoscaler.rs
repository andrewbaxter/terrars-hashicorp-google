#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeAutoscalerData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policy: std::option::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeAutoscaler_Timeouts>,
    dynamic: ComputeAutoscalerDynamic,
}

struct ComputeAutoscaler_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeAutoscalerData_>,
}

#[derive(Clone)]
pub struct ComputeAutoscaler(std::rc::Rc<ComputeAutoscaler_>);

impl ComputeAutoscaler {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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

    #[doc = "Set the field `zone`.\nURL of the zone where the instance group resides."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeAutoscaler_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nURL of the managed instance group that this autoscaler will scale."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nURL of the zone where the instance group resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAutoscaler_TimeoutsRef {
        ComputeAutoscaler_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeAutoscaler {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeAutoscaler { }

impl terrars::ToListMappable for ComputeAutoscaler {
    type O = terrars::ListRef<ComputeAutoscalerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeAutoscaler_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_autoscaler".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeAutoscaler {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "URL of the managed instance group that this autoscaler will scale."]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildComputeAutoscaler {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeAutoscaler {
        let out = ComputeAutoscaler(std::rc::Rc::new(ComputeAutoscaler_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeAutoscalerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target: self.target,
                zone: core::default::Default::default(),
                autoscaling_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeAutoscalerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscalerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeAutoscalerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nURL of the managed instance group that this autoscaler will scale."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nURL of the zone where the instance group resides."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAutoscaler_TimeoutsRef {
        ComputeAutoscaler_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeAutoscalerDynamic {
    autoscaling_policy: std::option::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy>>,
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_method: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<f64>,
}

impl ComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
    #[doc =
        "Set the field `predictive_method`.\nIndicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:\n\n- NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.\n\n- OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand."]
    pub fn set_predictive_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.predictive_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_CpuUtilization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
    #[doc =
        "The target CPU utilization that the autoscaler should maintain.\nMust be a float value in the range (0, 1]. If not specified, the\ndefault is 0.6.\n\nIf the CPU level is below the target utilization, the autoscaler\nscales down the number of instances until it reaches the minimum\nnumber of instances you specified or until the average CPU of\nyour instances reaches the target utilization.\n\nIf the average CPU is above the target utilization, the autoscaler\nscales up until it reaches the maximum number of instances you\nspecified or until the average utilization reaches the target\nutilization."]
    pub target: terrars::PrimField<f64>,
}

impl BuildComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
        ComputeAutoscaler_AutoscalingPolicy_CpuUtilization {
            predictive_method: core::default::Default::default(),
            target: self.target,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef {
        ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `predictive_method` after provisioning.\nIndicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:\n\n- NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.\n\n- OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand."]
    pub fn predictive_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predictive_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe target CPU utilization that the autoscaler should maintain.\nMust be a float value in the range (0, 1]. If not specified, the\ndefault is 0.6.\n\nIf the CPU level is below the target utilization, the autoscaler\nscales down the number of instances until it reaches the minimum\nnumber of instances you specified or until the average CPU of\nyour instances reaches the target utilization.\n\nIf the average CPU is above the target utilization, the autoscaler\nscales up until it reaches the maximum number of instances you\nspecified or until the average utilization reaches the target\nutilization."]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization {
    target: terrars::PrimField<f64>,
}

impl ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization { }

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization {
    #[doc =
        "Fraction of backend capacity utilization (set in HTTP(s) load\nbalancing configuration) that autoscaler should maintain. Must\nbe a positive float value. If not defined, the default is 0.8."]
    pub target: terrars::PrimField<f64>,
}

impl BuildComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization {
        ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization { target: self.target }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef {
        ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nFraction of backend capacity utilization (set in HTTP(s) load\nbalancing configuration) that autoscaler should maintain. Must\nbe a positive float value. If not defined, the default is 0.8."]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_Metric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_instance_assignment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeAutoscaler_AutoscalingPolicy_Metric {
    #[doc =
        "Set the field `filter`.\nA filter string to be used as the filter string for\na Stackdriver Monitoring TimeSeries.list API call.\nThis filter is used to select a specific TimeSeries for\nthe purpose of autoscaling and to determine whether the metric\nis exporting per-instance or per-group data.\n\nYou can only use the AND operator for joining selectors.\nYou can only use direct equality comparison operator (=) without\nany functions for each selector.\nYou can specify the metric in both the filter string and in the\nmetric field. However, if specified in both places, the metric must\nbe identical.\n\nThe monitored resource type determines what kind of values are\nexpected for the metric. If it is a gce_instance, the autoscaler\nexpects the metric to include a separate TimeSeries for each\ninstance in a group. In such a case, you cannot filter on resource\nlabels.\n\nIf the resource type is any other value, the autoscaler expects\nthis metric to contain values that apply to the entire autoscaled\ninstance group and resource label filtering can be performed to\npoint autoscaler at the correct TimeSeries to scale upon.\nThis is called a per-group metric for the purpose of autoscaling.\n\nIf not specified, the type defaults to gce_instance.\n\nYou should provide a filter that is selective enough to pick just\none TimeSeries for the autoscaled group or for each of the instances\n(if you are using gce_instance resource type). If multiple\nTimeSeries are returned upon the query execution, the autoscaler\nwill sum their respective values to obtain its scaling value."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `single_instance_assignment`.\nIf scaling is based on a per-group metric value that represents the\ntotal amount of work to be done or resource usage, set this value to\nan amount assigned for a single instance of the scaled group.\nThe autoscaler will keep the number of instances proportional to the\nvalue of this metric, the metric itself should not change value due\nto group resizing.\n\nFor example, a good metric to use with the target is\n'pubsub.googleapis.com/subscription/num_undelivered_messages'\nor a custom metric exporting the total number of requests coming to\nyour instances.\n\nA bad example would be a metric exporting an average or median\nlatency, since this value can't include a chunk assignable to a\nsingle instance, it could be better used with utilization_target\ninstead."]
    pub fn set_single_instance_assignment(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.single_instance_assignment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target`.\nThe target value of the metric that autoscaler should\nmaintain. This must be a positive value. A utilization\nmetric scales number of virtual machines handling requests\nto increase or decrease proportionally to the metric.\n\nFor example, a good metric to use as a utilizationTarget is\nwww.googleapis.com/compute/instance/network/received_bytes_count.\nThe autoscaler will work to keep this value constant for each\nof the instances."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nDefines how target utilization value is expressed for a\nStackdriver Monitoring metric. Possible values: [\"GAUGE\", \"DELTA_PER_SECOND\", \"DELTA_PER_MINUTE\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_Metric {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_Metric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_Metric {
    #[doc =
        "The identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeAutoscaler_AutoscalingPolicy_Metric {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_Metric {
        ComputeAutoscaler_AutoscalingPolicy_Metric {
            filter: core::default::Default::default(),
            name: self.name,
            single_instance_assignment: core::default::Default::default(),
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_MetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_MetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeAutoscaler_AutoscalingPolicy_MetricRef {
        ComputeAutoscaler_AutoscalingPolicy_MetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_MetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter string to be used as the filter string for\na Stackdriver Monitoring TimeSeries.list API call.\nThis filter is used to select a specific TimeSeries for\nthe purpose of autoscaling and to determine whether the metric\nis exporting per-instance or per-group data.\n\nYou can only use the AND operator for joining selectors.\nYou can only use direct equality comparison operator (=) without\nany functions for each selector.\nYou can specify the metric in both the filter string and in the\nmetric field. However, if specified in both places, the metric must\nbe identical.\n\nThe monitored resource type determines what kind of values are\nexpected for the metric. If it is a gce_instance, the autoscaler\nexpects the metric to include a separate TimeSeries for each\ninstance in a group. In such a case, you cannot filter on resource\nlabels.\n\nIf the resource type is any other value, the autoscaler expects\nthis metric to contain values that apply to the entire autoscaled\ninstance group and resource label filtering can be performed to\npoint autoscaler at the correct TimeSeries to scale upon.\nThis is called a per-group metric for the purpose of autoscaling.\n\nIf not specified, the type defaults to gce_instance.\n\nYou should provide a filter that is selective enough to pick just\none TimeSeries for the autoscaled group or for each of the instances\n(if you are using gce_instance resource type). If multiple\nTimeSeries are returned upon the query execution, the autoscaler\nwill sum their respective values to obtain its scaling value."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `single_instance_assignment` after provisioning.\nIf scaling is based on a per-group metric value that represents the\ntotal amount of work to be done or resource usage, set this value to\nan amount assigned for a single instance of the scaled group.\nThe autoscaler will keep the number of instances proportional to the\nvalue of this metric, the metric itself should not change value due\nto group resizing.\n\nFor example, a good metric to use with the target is\n'pubsub.googleapis.com/subscription/num_undelivered_messages'\nor a custom metric exporting the total number of requests coming to\nyour instances.\n\nA bad example would be a metric exporting an average or median\nlatency, since this value can't include a chunk assignable to a\nsingle instance, it could be better used with utilization_target\ninstead."]
    pub fn single_instance_assignment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_instance_assignment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe target value of the metric that autoscaler should\nmaintain. This must be a positive value. A utilization\nmetric scales number of virtual machines handling requests\nto increase or decrease proportionally to the metric.\n\nFor example, a good metric to use as a utilizationTarget is\nwww.googleapis.com/compute/instance/network/received_bytes_count.\nThe autoscaler will work to keep this value constant for each\nof the instances."]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDefines how target utilization value is expressed for a\nStackdriver Monitoring metric. Possible values: [\"GAUGE\", \"DELTA_PER_SECOND\", \"DELTA_PER_MINUTE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed number of VM instances. This must be a positive\ninteger."]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive.\nFor example, specify 80 for 80%."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {}

impl BuildComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
        ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef {
        ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive\ninteger."]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive.\nFor example, specify 80 for 80%."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeAutoscaler_AutoscalingPolicy_ScaleInControlDynamic {
    max_scaled_in_replicas: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas>>,
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_window_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_scaled_in_replicas: std
    ::option
    ::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas>>,
    dynamic: ComputeAutoscaler_AutoscalingPolicy_ScaleInControlDynamic,
}

impl ComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
    #[doc =
        "Set the field `time_window_sec`.\nHow long back autoscaling should look when computing recommendations\nto include directives regarding slower scale down, as described above."]
    pub fn set_time_window_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.time_window_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_scaled_in_replicas`.\n"]
    pub fn set_max_scaled_in_replicas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicas>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_scaled_in_replicas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_scaled_in_replicas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_ScaleInControl {}

impl BuildComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
        ComputeAutoscaler_AutoscalingPolicy_ScaleInControl {
            time_window_sec: core::default::Default::default(),
            max_scaled_in_replicas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef {
        ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_window_sec` after provisioning.\nHow long back autoscaling should look when computing recommendations\nto include directives regarding slower scale down, as described above."]
    pub fn time_window_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_window_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `max_scaled_in_replicas` after provisioning.\n"]
    pub fn max_scaled_in_replicas(
        &self,
    ) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl_MaxScaledInReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_scaled_in_replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    duration_sec: terrars::PrimField<f64>,
    min_required_replicas: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    schedule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
    #[doc = "Set the field `description`.\nA description of a scaling schedule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nA boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nThe time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
    #[doc =
        "The duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300."]
    pub duration_sec: terrars::PrimField<f64>,
    #[doc =
        "Minimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule."]
    pub min_required_replicas: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field)."]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
        ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            duration_sec: self.duration_sec,
            min_required_replicas: self.min_required_replicas,
            name: self.name,
            schedule: self.schedule,
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicy_ScalingSchedulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicy_ScalingSchedulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeAutoscaler_AutoscalingPolicy_ScalingSchedulesRef {
        ComputeAutoscaler_AutoscalingPolicy_ScalingSchedulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicy_ScalingSchedulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of a scaling schedule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nA boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration_sec` after provisioning.\nThe duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300."]
    pub fn duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_required_replicas` after provisioning.\nMinimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule."]
    pub fn min_required_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_required_replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nThe start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field)."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeAutoscaler_AutoscalingPolicyDynamic {
    cpu_utilization: std::option::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_CpuUtilization>>,
    load_balancing_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization>>,
    metric: std::option::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_Metric>>,
    scale_in_control: std::option::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl>>,
    scaling_schedules: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules>>,
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_AutoscalingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown_period: std::option::Option<terrars::PrimField<f64>>,
    max_replicas: terrars::PrimField<f64>,
    min_replicas: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stabilization_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization: std::option::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_CpuUtilization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_utilization: std
    ::option
    ::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_Metric>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_control: std::option::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_schedules: std::option::Option<std::vec::Vec<ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules>>,
    dynamic: ComputeAutoscaler_AutoscalingPolicyDynamic,
}

impl ComputeAutoscaler_AutoscalingPolicy {
    #[doc =
        "Set the field `cooldown_period`.\nThe number of seconds that the autoscaler should wait before it\nstarts collecting information from a new instance. This prevents\nthe autoscaler from collecting information when the instance is\ninitializing, during which the collected usage would not be\nreliable. The default time autoscaler waits is 60 seconds.\n\nVirtual machine initialization times might vary because of\nnumerous factors. We recommend that you test how long an\ninstance may take to initialize. To do this, create an instance\nand time the startup process."]
    pub fn set_cooldown_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooldown_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nDefines operating mode for this policy."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stabilization_period`.\nThe number of seconds that the autoscaler waits for load stabilization\nbefore making scale-in decisions.\n\nThis might appear as a delay in scaling in but it is an important mechanism\nfor your application to not have fluctuating size due to short term load\nfluctuations."]
    pub fn set_stabilization_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.stabilization_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_utilization`.\n"]
    pub fn set_cpu_utilization(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_CpuUtilization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_utilization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancing_utilization`.\n"]
    pub fn set_load_balancing_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.load_balancing_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.load_balancing_utilization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_Metric>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scale_in_control`.\n"]
    pub fn set_scale_in_control(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScaleInControl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scale_in_control = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_in_control = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_schedules`.\n"]
    pub fn set_scaling_schedules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy_ScalingSchedules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scaling_schedules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scaling_schedules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeAutoscaler_AutoscalingPolicy {
    type O = terrars::BlockAssignable<ComputeAutoscaler_AutoscalingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_AutoscalingPolicy {
    #[doc =
        "The maximum number of instances that the autoscaler can scale up\nto. This is required when creating or updating an autoscaler. The\nmaximum number of replicas should not be lower than minimal number\nof replicas."]
    pub max_replicas: terrars::PrimField<f64>,
    #[doc =
        "The minimum number of replicas that the autoscaler can scale down\nto. This cannot be less than 0. If not provided, autoscaler will\nchoose a default value depending on maximum number of instances\nallowed."]
    pub min_replicas: terrars::PrimField<f64>,
}

impl BuildComputeAutoscaler_AutoscalingPolicy {
    pub fn build(self) -> ComputeAutoscaler_AutoscalingPolicy {
        ComputeAutoscaler_AutoscalingPolicy {
            cooldown_period: core::default::Default::default(),
            max_replicas: self.max_replicas,
            min_replicas: self.min_replicas,
            mode: core::default::Default::default(),
            stabilization_period: core::default::Default::default(),
            cpu_utilization: core::default::Default::default(),
            load_balancing_utilization: core::default::Default::default(),
            metric: core::default::Default::default(),
            scale_in_control: core::default::Default::default(),
            scaling_schedules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_AutoscalingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_AutoscalingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeAutoscaler_AutoscalingPolicyRef {
        ComputeAutoscaler_AutoscalingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_AutoscalingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cooldown_period` after provisioning.\nThe number of seconds that the autoscaler should wait before it\nstarts collecting information from a new instance. This prevents\nthe autoscaler from collecting information when the instance is\ninitializing, during which the collected usage would not be\nreliable. The default time autoscaler waits is 60 seconds.\n\nVirtual machine initialization times might vary because of\nnumerous factors. We recommend that you test how long an\ninstance may take to initialize. To do this, create an instance\nand time the startup process."]
    pub fn cooldown_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_replicas` after provisioning.\nThe maximum number of instances that the autoscaler can scale up\nto. This is required when creating or updating an autoscaler. The\nmaximum number of replicas should not be lower than minimal number\nof replicas."]
    pub fn max_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replicas` after provisioning.\nThe minimum number of replicas that the autoscaler can scale down\nto. This cannot be less than 0. If not provided, autoscaler will\nchoose a default value depending on maximum number of instances\nallowed."]
    pub fn min_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nDefines operating mode for this policy."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stabilization_period` after provisioning.\nThe number of seconds that the autoscaler waits for load stabilization\nbefore making scale-in decisions.\n\nThis might appear as a delay in scaling in but it is an important mechanism\nfor your application to not have fluctuating size due to short term load\nfluctuations."]
    pub fn stabilization_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stabilization_period", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_utilization` after provisioning.\n"]
    pub fn cpu_utilization(&self) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicy_CpuUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancing_utilization` after provisioning.\n"]
    pub fn load_balancing_utilization(
        &self,
    ) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicy_LoadBalancingUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancing_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicy_MetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_control` after provisioning.\n"]
    pub fn scale_in_control(&self) -> terrars::ListRef<ComputeAutoscaler_AutoscalingPolicy_ScaleInControlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_in_control", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeAutoscaler_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeAutoscaler_Timeouts {
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

impl terrars::ToListMappable for ComputeAutoscaler_Timeouts {
    type O = terrars::BlockAssignable<ComputeAutoscaler_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeAutoscaler_Timeouts {}

impl BuildComputeAutoscaler_Timeouts {
    pub fn build(self) -> ComputeAutoscaler_Timeouts {
        ComputeAutoscaler_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeAutoscaler_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeAutoscaler_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeAutoscaler_TimeoutsRef {
        ComputeAutoscaler_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeAutoscaler_TimeoutsRef {
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
