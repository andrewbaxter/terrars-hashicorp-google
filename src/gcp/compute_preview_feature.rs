#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputePreviewFeatureData_ {
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
    activation_status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_operation: std::option::Option<std::vec::Vec<ComputePreviewFeature_RolloutOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputePreviewFeature_Timeouts>,
    dynamic: ComputePreviewFeatureDynamic,
}

struct ComputePreviewFeature_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputePreviewFeatureData_>,
}

#[derive(Clone)]
pub struct ComputePreviewFeature(std::rc::Rc<ComputePreviewFeature_>);

impl ComputePreviewFeature {
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

    #[doc = "Set the field `rollout_operation`.\n"]
    pub fn set_rollout_operation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePreviewFeature_RolloutOperation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout_operation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputePreviewFeature_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `activation_status` after provisioning.\nThe activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub fn activation_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the preview feature."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> terrars::ListRef<ComputePreviewFeature_RolloutOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePreviewFeature_TimeoutsRef {
        ComputePreviewFeature_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputePreviewFeature {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputePreviewFeature { }

impl terrars::ToListMappable for ComputePreviewFeature {
    type O = terrars::ListRef<ComputePreviewFeatureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputePreviewFeature_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_preview_feature".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputePreviewFeature {
    pub tf_id: std::string::String,
    #[doc =
        "The activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub activation_status: terrars::PrimField<std::string::String>,
    #[doc = "The name of the preview feature."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputePreviewFeature {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputePreviewFeature {
        let out = ComputePreviewFeature(std::rc::Rc::new(ComputePreviewFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputePreviewFeatureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                activation_status: self.activation_status,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                rollout_operation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputePreviewFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePreviewFeatureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputePreviewFeatureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activation_status` after provisioning.\nThe activation status of the preview feature. Possible values: [\"ENABLED\", \"ACTIVATION_STATE_UNSPECIFIED\"]"]
    pub fn activation_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the preview feature."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> terrars::ListRef<ComputePreviewFeature_RolloutOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePreviewFeature_TimeoutsRef {
        ComputePreviewFeature_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputePreviewFeatureDynamic {
    rollout_operation: std::option::Option<terrars::DynamicBlock<ComputePreviewFeature_RolloutOperation>>,
}

#[derive(Serialize)]
pub struct ComputePreviewFeature_RolloutOperation_RolloutInput {
    predefined_rollout_plan: terrars::PrimField<std::string::String>,
}

impl ComputePreviewFeature_RolloutOperation_RolloutInput { }

impl terrars::ToListMappable for ComputePreviewFeature_RolloutOperation_RolloutInput {
    type O = terrars::BlockAssignable<ComputePreviewFeature_RolloutOperation_RolloutInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePreviewFeature_RolloutOperation_RolloutInput {
    #[doc = "Predefined rollout plans. Possible values: [\"ROLLOUT_PLAN_FAST_ROLLOUT\"]"]
    pub predefined_rollout_plan: terrars::PrimField<std::string::String>,
}

impl BuildComputePreviewFeature_RolloutOperation_RolloutInput {
    pub fn build(self) -> ComputePreviewFeature_RolloutOperation_RolloutInput {
        ComputePreviewFeature_RolloutOperation_RolloutInput { predefined_rollout_plan: self.predefined_rollout_plan }
    }
}

#[allow(dead_code)]
pub struct ComputePreviewFeature_RolloutOperation_RolloutInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePreviewFeature_RolloutOperation_RolloutInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePreviewFeature_RolloutOperation_RolloutInputRef {
        ComputePreviewFeature_RolloutOperation_RolloutInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePreviewFeature_RolloutOperation_RolloutInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `predefined_rollout_plan` after provisioning.\nPredefined rollout plans. Possible values: [\"ROLLOUT_PLAN_FAST_ROLLOUT\"]"]
    pub fn predefined_rollout_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_rollout_plan", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePreviewFeature_RolloutOperationDynamic {
    rollout_input: std::option::Option<terrars::DynamicBlock<ComputePreviewFeature_RolloutOperation_RolloutInput>>,
}

#[derive(Serialize)]
pub struct ComputePreviewFeature_RolloutOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_input: std::option::Option<std::vec::Vec<ComputePreviewFeature_RolloutOperation_RolloutInput>>,
    dynamic: ComputePreviewFeature_RolloutOperationDynamic,
}

impl ComputePreviewFeature_RolloutOperation {
    #[doc = "Set the field `rollout_input`.\n"]
    pub fn set_rollout_input(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePreviewFeature_RolloutOperation_RolloutInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollout_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputePreviewFeature_RolloutOperation {
    type O = terrars::BlockAssignable<ComputePreviewFeature_RolloutOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePreviewFeature_RolloutOperation {}

impl BuildComputePreviewFeature_RolloutOperation {
    pub fn build(self) -> ComputePreviewFeature_RolloutOperation {
        ComputePreviewFeature_RolloutOperation {
            rollout_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePreviewFeature_RolloutOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePreviewFeature_RolloutOperationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePreviewFeature_RolloutOperationRef {
        ComputePreviewFeature_RolloutOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePreviewFeature_RolloutOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rollout_input` after provisioning.\n"]
    pub fn rollout_input(&self) -> terrars::ListRef<ComputePreviewFeature_RolloutOperation_RolloutInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_input", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePreviewFeature_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputePreviewFeature_Timeouts {
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

impl terrars::ToListMappable for ComputePreviewFeature_Timeouts {
    type O = terrars::BlockAssignable<ComputePreviewFeature_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePreviewFeature_Timeouts {}

impl BuildComputePreviewFeature_Timeouts {
    pub fn build(self) -> ComputePreviewFeature_Timeouts {
        ComputePreviewFeature_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePreviewFeature_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePreviewFeature_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePreviewFeature_TimeoutsRef {
        ComputePreviewFeature_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePreviewFeature_TimeoutsRef {
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
