use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionAutoscalerData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policy: std::option::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionAutoscalerTimeoutsEl>,
    dynamic: ComputeRegionAutoscalerDynamic,
}

struct ComputeRegionAutoscaler_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionAutoscalerData>,
}

#[derive(Clone)]
pub struct ComputeRegionAutoscaler(std::rc::Rc<ComputeRegionAutoscaler_>);

impl ComputeRegionAutoscaler {
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

    #[doc = "Set the field `region`.\nURL of the region where the instance group resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionAutoscalerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nURL of the region where the instance group resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionAutoscalerTimeoutsElRef {
        ComputeRegionAutoscalerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRegionAutoscaler {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionAutoscaler { }

impl terrars::ToListMappable for ComputeRegionAutoscaler {
    type O = terrars::ListRef<ComputeRegionAutoscalerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionAutoscaler_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_autoscaler".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionAutoscaler {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "URL of the managed instance group that this autoscaler will scale."]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionAutoscaler {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionAutoscaler {
        let out = ComputeRegionAutoscaler(std::rc::Rc::new(ComputeRegionAutoscaler_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionAutoscalerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                target: self.target,
                autoscaling_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionAutoscalerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionAutoscalerRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nURL of the region where the instance group resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionAutoscalerTimeoutsElRef {
        ComputeRegionAutoscalerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_method: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<f64>,
}

impl ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[doc =
        "Set the field `predictive_method`.\nIndicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:\n\n- NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.\n\n- OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand."]
    pub fn set_predictive_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.predictive_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[doc =
        "The target CPU utilization that the autoscaler should maintain.\nMust be a float value in the range (0, 1]. If not specified, the\ndefault is 0.6.\n\nIf the CPU level is below the target utilization, the autoscaler\nscales down the number of instances until it reaches the minimum\nnumber of instances you specified or until the average CPU of\nyour instances reaches the target utilization.\n\nIf the average CPU is above the target utilization, the autoscaler\nscales up until it reaches the maximum number of instances you\nspecified or until the average utilization reaches the target\nutilization."]
    pub target: terrars::PrimField<f64>,
}

impl BuildComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
        ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl {
            predictive_method: core::default::Default::default(),
            target: self.target,
        }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
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
pub struct ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    target: terrars::PrimField<f64>,
}

impl ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl { }

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    #[doc =
        "Fraction of backend capacity utilization (set in HTTP(s) load\nbalancing configuration) that autoscaler should maintain. Must\nbe a positive float value. If not defined, the default is 0.8."]
    pub target: terrars::PrimField<f64>,
}

impl BuildComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
        ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl { target: self.target }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
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
pub struct ComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
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

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElMetricEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
    #[doc =
        "The identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
        ComputeRegionAutoscalerAutoscalingPolicyElMetricEl {
            name: self.name,
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
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
pub struct ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
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

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    type O =
        terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {}

impl BuildComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
        ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
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
struct ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElDynamic {
    max_scaled_in_replicas: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_window_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_scaled_in_replicas: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>>,
    dynamic: ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElDynamic,
}

impl ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
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
                        ::BlockAssignable<
                            ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl,
                        >,
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

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {}

impl BuildComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
        ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl {
            time_window_sec: core::default::Default::default(),
            max_scaled_in_replicas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef {
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
    ) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_scaled_in_replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
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

impl ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
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

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
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

impl BuildComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
        ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl {
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

pub struct ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
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
struct ComputeRegionAutoscalerAutoscalingPolicyElDynamic {
    cpu_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
    load_balancing_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>>,
    metric: std::option::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElMetricEl>>,
    scale_in_control: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl>>,
    scaling_schedules: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionAutoscalerAutoscalingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown_period: std::option::Option<terrars::PrimField<f64>>,
    max_replicas: terrars::PrimField<f64>,
    min_replicas: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization: std::option::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_utilization: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElMetricEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_control: std::option::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_schedules: std::option::Option<std::vec::Vec<ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
    dynamic: ComputeRegionAutoscalerAutoscalingPolicyElDynamic,
}

impl ComputeRegionAutoscalerAutoscalingPolicyEl {
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

    #[doc = "Set the field `cpu_utilization`.\n"]
    pub fn set_cpu_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>,
                    >,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElMetricEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
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

impl terrars::ToListMappable for ComputeRegionAutoscalerAutoscalingPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerAutoscalingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerAutoscalingPolicyEl {
    #[doc =
        "The maximum number of instances that the autoscaler can scale up\nto. This is required when creating or updating an autoscaler. The\nmaximum number of replicas should not be lower than minimal number\nof replicas."]
    pub max_replicas: terrars::PrimField<f64>,
    #[doc =
        "The minimum number of replicas that the autoscaler can scale down\nto. This cannot be less than 0. If not provided, autoscaler will\nchoose a default value depending on maximum number of instances\nallowed."]
    pub min_replicas: terrars::PrimField<f64>,
}

impl BuildComputeRegionAutoscalerAutoscalingPolicyEl {
    pub fn build(self) -> ComputeRegionAutoscalerAutoscalingPolicyEl {
        ComputeRegionAutoscalerAutoscalingPolicyEl {
            cooldown_period: core::default::Default::default(),
            max_replicas: self.max_replicas,
            min_replicas: self.min_replicas,
            mode: core::default::Default::default(),
            cpu_utilization: core::default::Default::default(),
            load_balancing_utilization: core::default::Default::default(),
            metric: core::default::Default::default(),
            scale_in_control: core::default::Default::default(),
            scaling_schedules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionAutoscalerAutoscalingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerAutoscalingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionAutoscalerAutoscalingPolicyElRef {
        ComputeRegionAutoscalerAutoscalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerAutoscalingPolicyElRef {
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

    #[doc = "Get a reference to the value of field `cpu_utilization` after provisioning.\n"]
    pub fn cpu_utilization(&self) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElCpuUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancing_utilization` after provisioning.\n"]
    pub fn load_balancing_utilization(
        &self,
    ) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancing_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElMetricElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_control` after provisioning.\n"]
    pub fn scale_in_control(&self) -> terrars::ListRef<ComputeRegionAutoscalerAutoscalingPolicyElScaleInControlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_in_control", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionAutoscalerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionAutoscalerTimeoutsEl {
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

impl terrars::ToListMappable for ComputeRegionAutoscalerTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeRegionAutoscalerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionAutoscalerTimeoutsEl {}

impl BuildComputeRegionAutoscalerTimeoutsEl {
    pub fn build(self) -> ComputeRegionAutoscalerTimeoutsEl {
        ComputeRegionAutoscalerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionAutoscalerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionAutoscalerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionAutoscalerTimeoutsElRef {
        ComputeRegionAutoscalerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionAutoscalerTimeoutsElRef {
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
struct ComputeRegionAutoscalerDynamic {
    autoscaling_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionAutoscalerAutoscalingPolicyEl>>,
}
