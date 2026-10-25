#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiDeploymentResourcePoolData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_resources: std::option::Option<std::vec::Vec<VertexAiDeploymentResourcePool_DedicatedResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiDeploymentResourcePool_Timeouts>,
    dynamic: VertexAiDeploymentResourcePoolDynamic,
}

struct VertexAiDeploymentResourcePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiDeploymentResourcePoolData_>,
}

#[derive(Clone)]
pub struct VertexAiDeploymentResourcePool(std::rc::Rc<VertexAiDeploymentResourcePool_>);

impl VertexAiDeploymentResourcePool {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of deployment resource pool. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiDeploymentResourcePool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of deployment resource pool. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> terrars::ListRef<VertexAiDeploymentResourcePool_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiDeploymentResourcePool_TimeoutsRef {
        VertexAiDeploymentResourcePool_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiDeploymentResourcePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiDeploymentResourcePool { }

impl terrars::ToListMappable for VertexAiDeploymentResourcePool {
    type O = terrars::ListRef<VertexAiDeploymentResourcePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiDeploymentResourcePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_deployment_resource_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiDeploymentResourcePool {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiDeploymentResourcePool {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiDeploymentResourcePool {
        let out = VertexAiDeploymentResourcePool(std::rc::Rc::new(VertexAiDeploymentResourcePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiDeploymentResourcePoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                dedicated_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiDeploymentResourcePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiDeploymentResourcePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiDeploymentResourcePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of deployment resource pool. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> terrars::ListRef<VertexAiDeploymentResourcePool_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiDeploymentResourcePool_TimeoutsRef {
        VertexAiDeploymentResourcePool_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiDeploymentResourcePoolDynamic {
    dedicated_resources: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiDeploymentResourcePool_DedicatedResources>>,
}

#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
    #[doc =
        "Set the field `target`.\nThe target resource utilization in percentage (1% - 100%) for the given metric; once the real usage deviates from the target by a certain percentage, the machine replicas change. The default value is 60 (representing 60%) if not provided."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
    type O = terrars::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
    #[doc =
        "The resource metric name. Supported metrics: For Online Prediction: * 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle' * 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub metric_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
    pub fn build(self) -> VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
        VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs {
            metric_name: self.metric_name,
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef {
        VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metric_name` after provisioning.\nThe resource metric name. Supported metrics: For Online Prediction: * 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle' * 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe target resource utilization in percentage (1% - 100%) for the given metric; once the real usage deviates from the target by a certain percentage, the machine replicas change. The default value is 60 (representing 60%) if not provided."]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nThe type of accelerator(s) that may be attached to the machine as per accelerator_count. See possible values [here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec#AcceleratorType)."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
    type O = terrars::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {}

impl BuildVertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
    pub fn build(self) -> VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
        VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef {
        VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators to attach to the machine."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of accelerator(s) that may be attached to the machine as per accelerator_count. See possible values [here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec#AcceleratorType)."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiDeploymentResourcePool_DedicatedResourcesDynamic {
    autoscaling_metric_specs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs>>,
    machine_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePool_DedicatedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    min_replica_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_metric_specs: std
    ::option
    ::Option<std::vec::Vec<VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<std::vec::Vec<VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec>>,
    dynamic: VertexAiDeploymentResourcePool_DedicatedResourcesDynamic,
}

impl VertexAiDeploymentResourcePool_DedicatedResources {
    #[doc =
        "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type)."]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_metric_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_metric_specs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources_MachineSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiDeploymentResourcePool_DedicatedResources {
    type O = terrars::BlockAssignable<VertexAiDeploymentResourcePool_DedicatedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiDeploymentResourcePool_DedicatedResources {
    #[doc =
        "The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed."]
    pub min_replica_count: terrars::PrimField<f64>,
}

impl BuildVertexAiDeploymentResourcePool_DedicatedResources {
    pub fn build(self) -> VertexAiDeploymentResourcePool_DedicatedResources {
        VertexAiDeploymentResourcePool_DedicatedResources {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiDeploymentResourcePool_DedicatedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiDeploymentResourcePool_DedicatedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiDeploymentResourcePool_DedicatedResourcesRef {
        VertexAiDeploymentResourcePool_DedicatedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiDeploymentResourcePool_DedicatedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type)."]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]
    pub fn autoscaling_metric_specs(
        &self,
    ) -> terrars::ListRef<VertexAiDeploymentResourcePool_DedicatedResources_AutoscalingMetricSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_metric_specs", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<VertexAiDeploymentResourcePool_DedicatedResources_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiDeploymentResourcePool_Timeouts {
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
}

impl terrars::ToListMappable for VertexAiDeploymentResourcePool_Timeouts {
    type O = terrars::BlockAssignable<VertexAiDeploymentResourcePool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiDeploymentResourcePool_Timeouts {}

impl BuildVertexAiDeploymentResourcePool_Timeouts {
    pub fn build(self) -> VertexAiDeploymentResourcePool_Timeouts {
        VertexAiDeploymentResourcePool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiDeploymentResourcePool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiDeploymentResourcePool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiDeploymentResourcePool_TimeoutsRef {
        VertexAiDeploymentResourcePool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiDeploymentResourcePool_TimeoutsRef {
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
}
