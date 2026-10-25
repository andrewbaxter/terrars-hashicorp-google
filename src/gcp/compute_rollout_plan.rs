#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRolloutPlanData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRolloutPlan_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waves: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves>>,
    dynamic: ComputeRolloutPlanDynamic,
}

struct ComputeRolloutPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRolloutPlanData_>,
}

#[derive(Clone)]
pub struct ComputeRolloutPlan(std::rc::Rc<ComputeRolloutPlan_>);

impl ComputeRolloutPlan {
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

    #[doc =
        "Set the field `location_scope`.\nThe location scope of the rollout plan. Possible values: [\"LOCATION_SCOPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn set_location_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRolloutPlan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waves`.\n"]
    pub fn set_waves(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().waves = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.waves = std::option::Option::Some(d);
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_scope` after provisioning.\nThe location scope of the rollout plan. Possible values: [\"LOCATION_SCOPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn location_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035."]
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRolloutPlan_TimeoutsRef {
        ComputeRolloutPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waves` after provisioning.\n"]
    pub fn waves(&self) -> terrars::ListRef<ComputeRolloutPlan_WavesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waves", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRolloutPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRolloutPlan { }

impl terrars::ToListMappable for ComputeRolloutPlan {
    type O = terrars::ListRef<ComputeRolloutPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRolloutPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_rollout_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRolloutPlan {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRolloutPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRolloutPlan {
        let out = ComputeRolloutPlan(std::rc::Rc::new(ComputeRolloutPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRolloutPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location_scope: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                waves: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRolloutPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRolloutPlanRef {
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_scope` after provisioning.\nThe location scope of the rollout plan. Possible values: [\"LOCATION_SCOPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn location_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035."]
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRolloutPlan_TimeoutsRef {
        ComputeRolloutPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waves` after provisioning.\n"]
    pub fn waves(&self) -> terrars::ListRef<ComputeRolloutPlan_WavesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waves", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeRolloutPlanDynamic {
    waves: std::option::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves>>,
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRolloutPlan_Timeouts {
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

impl terrars::ToListMappable for ComputeRolloutPlan_Timeouts {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Timeouts {}

impl BuildComputeRolloutPlan_Timeouts {
    pub fn build(self) -> ComputeRolloutPlan_Timeouts {
        ComputeRolloutPlan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRolloutPlan_TimeoutsRef {
        ComputeRolloutPlan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
    #[doc =
        "Set the field `delimiter`.\nControls whether the delay should only be added between batches of projects corresponding to different locations, or also between batches of projects corresponding to the same location. Possible values: [\"DELIMITER_UNSPECIFIED\", \"DELIMITER_LOCATION\", \"DELIMITER_BATCH\"]"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\nThe duration of the delay, if any, to be added between batches of projects."]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nControls whether the specified duration is to be added at the end of each batch, or if the total processing time for each batch will be padded if needed to meet the specified duration. Possible values: [\"TYPE_UNSPECIFIED\", \"TYPE_OFFSET\", \"TYPE_MINIMUM\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_OrchestrationOptions_Delays>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_OrchestrationOptions_Delays {}

impl BuildComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
    pub fn build(self) -> ComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
        ComputeRolloutPlan_Waves_OrchestrationOptions_Delays {
            delimiter: core::default::Default::default(),
            duration: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef {
        ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nControls whether the delay should only be added between batches of projects corresponding to different locations, or also between batches of projects corresponding to the same location. Possible values: [\"DELIMITER_UNSPECIFIED\", \"DELIMITER_LOCATION\", \"DELIMITER_BATCH\"]"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe duration of the delay, if any, to be added between batches of projects."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nControls whether the specified duration is to be added at the end of each batch, or if the total processing time for each batch will be padded if needed to meet the specified duration. Possible values: [\"TYPE_UNSPECIFIED\", \"TYPE_OFFSET\", \"TYPE_MINIMUM\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRolloutPlan_Waves_OrchestrationOptionsDynamic {
    delays: std::option::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_OrchestrationOptions_Delays>>,
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_OrchestrationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_locations: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_resources_per_location: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delays: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves_OrchestrationOptions_Delays>>,
    dynamic: ComputeRolloutPlan_Waves_OrchestrationOptionsDynamic,
}

impl ComputeRolloutPlan_Waves_OrchestrationOptions {
    #[doc = "Set the field `max_concurrent_locations`.\nMaximum number of locations to be orchestrated in parallel."]
    pub fn set_max_concurrent_locations(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrent_resources_per_location`.\nMaximum number of resources to be orchestrated per location in parallel."]
    pub fn set_max_concurrent_resources_per_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_concurrent_resources_per_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delays`.\n"]
    pub fn set_delays(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_OrchestrationOptions_Delays>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delays = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delays = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_OrchestrationOptions {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_OrchestrationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_OrchestrationOptions {}

impl BuildComputeRolloutPlan_Waves_OrchestrationOptions {
    pub fn build(self) -> ComputeRolloutPlan_Waves_OrchestrationOptions {
        ComputeRolloutPlan_Waves_OrchestrationOptions {
            max_concurrent_locations: core::default::Default::default(),
            max_concurrent_resources_per_location: core::default::Default::default(),
            delays: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_OrchestrationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_OrchestrationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRolloutPlan_Waves_OrchestrationOptionsRef {
        ComputeRolloutPlan_Waves_OrchestrationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_OrchestrationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_locations` after provisioning.\nMaximum number of locations to be orchestrated in parallel."]
    pub fn max_concurrent_locations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_locations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_resources_per_location` after provisioning.\nMaximum number of resources to be orchestrated per location in parallel."]
    pub fn max_concurrent_resources_per_location(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_resources_per_location", self.base))
    }

    #[doc = "Get a reference to the value of field `delays` after provisioning.\n"]
    pub fn delays(&self) -> terrars::ListRef<ComputeRolloutPlan_Waves_OrchestrationOptions_DelaysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delays", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_Selectors_LocationSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRolloutPlan_Waves_Selectors_LocationSelector {
    #[doc = "Set the field `included_locations`.\nExample: \"us-central1-a\""]
    pub fn set_included_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_Selectors_LocationSelector {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors_LocationSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_Selectors_LocationSelector {}

impl BuildComputeRolloutPlan_Waves_Selectors_LocationSelector {
    pub fn build(self) -> ComputeRolloutPlan_Waves_Selectors_LocationSelector {
        ComputeRolloutPlan_Waves_Selectors_LocationSelector { included_locations: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef {
        ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `included_locations` after provisioning.\nExample: \"us-central1-a\""]
    pub fn included_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_folders: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_organizations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
    #[doc = "Set the field `included_folders`.\nFormat: \"folders/{folder_id}\""]
    pub fn set_included_folders(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_folders = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `included_organizations`.\nFormat: \"organizations/{organization_id}\""]
    pub fn set_included_organizations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_organizations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `included_projects`.\nFormat: \"projects/{project_id}\""]
    pub fn set_included_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_projects = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {}

impl BuildComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
    pub fn build(self) -> ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
        ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector {
            included_folders: core::default::Default::default(),
            included_organizations: core::default::Default::default(),
            included_projects: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef {
        ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `included_folders` after provisioning.\nFormat: \"folders/{folder_id}\""]
    pub fn included_folders(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_folders", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_organizations` after provisioning.\nFormat: \"organizations/{organization_id}\""]
    pub fn included_organizations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_organizations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_projects` after provisioning.\nFormat: \"projects/{project_id}\""]
    pub fn included_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_projects", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRolloutPlan_Waves_SelectorsDynamic {
    location_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_Selectors_LocationSelector>>,
    resource_hierarchy_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector>>,
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_Selectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_selector: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves_Selectors_LocationSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_hierarchy_selector: std
    ::option
    ::Option<std::vec::Vec<ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector>>,
    dynamic: ComputeRolloutPlan_Waves_SelectorsDynamic,
}

impl ComputeRolloutPlan_Waves_Selectors {
    #[doc = "Set the field `location_selector`.\n"]
    pub fn set_location_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors_LocationSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_hierarchy_selector`.\n"]
    pub fn set_resource_hierarchy_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_hierarchy_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_hierarchy_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_Selectors {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_Selectors {}

impl BuildComputeRolloutPlan_Waves_Selectors {
    pub fn build(self) -> ComputeRolloutPlan_Waves_Selectors {
        ComputeRolloutPlan_Waves_Selectors {
            location_selector: core::default::Default::default(),
            resource_hierarchy_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_SelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_SelectorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRolloutPlan_Waves_SelectorsRef {
        ComputeRolloutPlan_Waves_SelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_SelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_selector` after provisioning.\n"]
    pub fn location_selector(&self) -> terrars::ListRef<ComputeRolloutPlan_Waves_Selectors_LocationSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_hierarchy_selector` after provisioning.\n"]
    pub fn resource_hierarchy_selector(
        &self,
    ) -> terrars::ListRef<ComputeRolloutPlan_Waves_Selectors_ResourceHierarchySelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_hierarchy_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
    #[doc =
        "Set the field `wait_duration`.\nThe duration that the system waits in between waves. This wait starts\nafter all changes in the wave are rolled out."]
    pub fn set_wait_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wait_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {}

impl BuildComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
    pub fn build(self) -> ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
        ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata {
            wait_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef {
        ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `wait_duration` after provisioning.\nThe duration that the system waits in between waves. This wait starts\nafter all changes in the wave are rolled out."]
    pub fn wait_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRolloutPlan_Waves_ValidationDynamic {
    time_based_validation_metadata: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata>>,
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves_Validation {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_validation_metadata: std
    ::option
    ::Option<std::vec::Vec<ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata>>,
    dynamic: ComputeRolloutPlan_Waves_ValidationDynamic,
}

impl ComputeRolloutPlan_Waves_Validation {
    #[doc = "Set the field `time_based_validation_metadata`.\n"]
    pub fn set_time_based_validation_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_based_validation_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_validation_metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves_Validation {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves_Validation {
    #[doc =
        "The type of the validation. Possible values:\n\"manual\": The system waits for an end-user approval API before progressing to the next wave.\n\"time\": The system waits for a user specified duration before progressing to the next wave."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeRolloutPlan_Waves_Validation {
    pub fn build(self) -> ComputeRolloutPlan_Waves_Validation {
        ComputeRolloutPlan_Waves_Validation {
            type_: self.type_,
            time_based_validation_metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_Waves_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_Waves_ValidationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRolloutPlan_Waves_ValidationRef {
        ComputeRolloutPlan_Waves_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_Waves_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the validation. Possible values:\n\"manual\": The system waits for an end-user approval API before progressing to the next wave.\n\"time\": The system waits for a user specified duration before progressing to the next wave."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_validation_metadata` after provisioning.\n"]
    pub fn time_based_validation_metadata(
        &self,
    ) -> terrars::ListRef<ComputeRolloutPlan_Waves_Validation_TimeBasedValidationMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_validation_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRolloutPlan_WavesDynamic {
    orchestration_options: std::option::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_OrchestrationOptions>>,
    selectors: std::option::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_Selectors>>,
    validation: std::option::Option<terrars::DynamicBlock<ComputeRolloutPlan_Waves_Validation>>,
}

#[derive(Serialize)]
pub struct ComputeRolloutPlan_Waves {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orchestration_options: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves_OrchestrationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selectors: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves_Selectors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<ComputeRolloutPlan_Waves_Validation>>,
    dynamic: ComputeRolloutPlan_WavesDynamic,
}

impl ComputeRolloutPlan_Waves {
    #[doc = "Set the field `display_name`.\nThe display name of this wave of the rollout plan."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orchestration_options`.\n"]
    pub fn set_orchestration_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_OrchestrationOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.orchestration_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.orchestration_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_Selectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selectors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRolloutPlan_Waves_Validation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRolloutPlan_Waves {
    type O = terrars::BlockAssignable<ComputeRolloutPlan_Waves>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRolloutPlan_Waves {}

impl BuildComputeRolloutPlan_Waves {
    pub fn build(self) -> ComputeRolloutPlan_Waves {
        ComputeRolloutPlan_Waves {
            display_name: core::default::Default::default(),
            orchestration_options: core::default::Default::default(),
            selectors: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRolloutPlan_WavesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRolloutPlan_WavesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRolloutPlan_WavesRef {
        ComputeRolloutPlan_WavesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRolloutPlan_WavesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of this wave of the rollout plan."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `number` after provisioning.\nThe wave number."]
    pub fn number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }

    #[doc = "Get a reference to the value of field `orchestration_options` after provisioning.\n"]
    pub fn orchestration_options(&self) -> terrars::ListRef<ComputeRolloutPlan_Waves_OrchestrationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestration_options", self.base))
    }

    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> terrars::ListRef<ComputeRolloutPlan_Waves_SelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selectors", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<ComputeRolloutPlan_Waves_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}
