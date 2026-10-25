#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiReasoningEngineData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiReasoningEngine_Timeouts>,
    dynamic: VertexAiReasoningEngineDynamic,
}

struct VertexAiReasoningEngine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiReasoningEngineData_>,
}

#[derive(Clone)]
pub struct VertexAiReasoningEngine(std::rc::Rc<VertexAiReasoningEngine_>);

impl VertexAiReasoningEngine {
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
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the ReasoningEngine."]
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
        "Set the field `labels`.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the reasoning engine. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context_spec`.\n"]
    pub fn set_context_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().context_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.context_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiReasoningEngine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the ReasoningEngine."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the ReasoningEngine."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe generated name of the ReasoningEngine, in the format\nprojects/{project}/locations/{location}/reasoningEngines/{reasoningEngine}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the reasoning engine. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\"\nformat, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context_spec` after provisioning.\n"]
    pub fn context_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiReasoningEngine_TimeoutsRef {
        VertexAiReasoningEngine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiReasoningEngine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiReasoningEngine { }

impl terrars::ToListMappable for VertexAiReasoningEngine {
    type O = terrars::ListRef<VertexAiReasoningEngineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiReasoningEngine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_reasoning_engine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiReasoningEngine {
    pub tf_id: std::string::String,
    #[doc = "The display name of the ReasoningEngine."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiReasoningEngine {
        let out = VertexAiReasoningEngine(std::rc::Rc::new(VertexAiReasoningEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiReasoningEngineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                context_spec: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiReasoningEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiReasoningEngineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/vertex_ai_reasoning_engine.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the ReasoningEngine."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the ReasoningEngine."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this ReasoningEngine. You can use these to\norganize and group your ReasoningEngines.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe generated name of the ReasoningEngine, in the format\nprojects/{project}/locations/{location}/reasoningEngines/{reasoningEngine}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the reasoning engine. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\"\nformat, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context_spec` after provisioning.\n"]
    pub fn context_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiReasoningEngine_TimeoutsRef {
        VertexAiReasoningEngine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngineDynamic {
    context_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec>>,
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_EncryptionSpec>>,
    spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revisions_per_candidate_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
    #[doc = "Set the field `revisions_per_candidate_count`.\nNumber of revisions to consider per candidate count."]
    pub fn set_revisions_per_candidate_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.revisions_per_candidate_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig {
            revisions_per_candidate_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `revisions_per_candidate_count` after provisioning.\nNumber of revisions to consider per candidate count."]
    pub fn revisions_per_candidate_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revisions_per_candidate_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    word: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
    #[doc = "Set the field `end_offset`.\nEnd offset in time of the word relative to the start of the audio."]
    pub fn set_end_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_offset`.\nStart offset in time of the word relative to the start of the audio."]
    pub fn set_start_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
    #[doc = "Transcript of the word."]
    pub word: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words {
            end_offset: core::default::Default::default(),
            start_offset: core::default::Default::default(),
            word: self.word,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_offset` after provisioning.\nEnd offset in time of the word relative to the start of the audio."]
    pub fn end_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_offset` after provisioning.\nStart offset in time of the word relative to the start of the audio."]
    pub fn start_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `word` after provisioning.\nTranscript of the word."]
    pub fn word(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.word", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionDynamic {
    words: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speaker_label: std::option::Option<terrars::PrimField<std::string::String>>,
    text: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    words: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
    #[doc =
        "Set the field `speaker_label`.\nA label identifying the speaker of this audio segment (e.g. spk_1, spk_2). Present when diarization is set."]
    pub fn set_speaker_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.speaker_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `words`.\n"]
    pub fn set_words(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_Words,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.words = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.words = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
    #[doc = "The transcription text of this audio segment."]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription {
            speaker_label: core::default::Default::default(),
            text: self.text,
            words: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `speaker_label` after provisioning.\nA label identifying the speaker of this audio segment (e.g. spk_1, spk_2). Present when diarization is set."]
    pub fn speaker_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speaker_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nThe transcription text of this audio segment."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `words` after provisioning.\n"]
    pub fn words(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription_WordsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    outcome: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
    #[doc = "Set the field `id`.\nThe identifier of the ExecutableCode part this result is for."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output`.\nContains stdout when code execution is successful, stderr or other description otherwise."]
    pub fn set_output(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
    #[doc =
        "Outcome of the code execution. Possible values: [\"OUTCOME_UNSPECIFIED\", \"OUTCOME_OK\", \"OUTCOME_FAILED\", \"OUTCOME_DEADLINE_EXCEEDED\"]"]
    pub outcome: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult {
            id: core::default::Default::default(),
            outcome: self.outcome,
            output: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe identifier of the ExecutableCode part this result is for."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `outcome` after provisioning.\nOutcome of the code execution. Possible values: [\"OUTCOME_UNSPECIFIED\", \"OUTCOME_OK\", \"OUTCOME_FAILED\", \"OUTCOME_DEADLINE_EXCEEDED\"]"]
    pub fn outcome(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outcome", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output` after provisioning.\nContains stdout when code execution is successful, stderr or other description otherwise."]
    pub fn output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
    code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    language: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
    #[doc = "Set the field `id`.\nUnique identifier of the ExecutableCode part."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
    #[doc = "The code to be executed."]
    pub code: terrars::PrimField<std::string::String>,
    #[doc =
        "Supported programming languages for the generated code. Possible values: [\"LANGUAGE_UNSPECIFIED\", \"PYTHON\", \"BASH\"]"]
    pub language: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode {
            code: self.code,
            id: core::default::Default::default(),
            language: self.language,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\nThe code to be executed."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the ExecutableCode part."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language` after provisioning.\nSupported programming languages for the generated code. Possible values: [\"LANGUAGE_UNSPECIFIED\", \"PYTHON\", \"BASH\"]"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
    file_uri: terrars::PrimField<std::string::String>,
    mime_type: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {

}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
    #[doc = "The URI of the file in Google Cloud Storage."]
    pub file_uri: terrars::PrimField<std::string::String>,
    #[doc = "The IANA standard MIME type of the source data."]
    pub mime_type: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData {
            file_uri: self.file_uri,
            mime_type: self.mime_type,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_uri` after provisioning.\nThe URI of the file in Google Cloud Storage."]
    pub fn file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nThe IANA standard MIME type of the source data."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
    #[doc = "Set the field `args`.\nThe function parameters and values in JSON object format."]
    pub fn set_args(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nThe unique id of the function call."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the function to call."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall {
            args: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe function parameters and values in JSON object format."]
    pub fn args(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique id of the function call."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the function to call."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
    #[doc = "Set the field `id`.\nThe id of the function call this response is for."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\nThe function response in JSON object format."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
    #[doc = "The name of the function to call."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse {
            id: core::default::Default::default(),
            name: self.name,
            response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe id of the function call this response is for."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the function to call."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe function response in JSON object format."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
    data: terrars::PrimField<std::string::String>,
    mime_type: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {

}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
    #[doc = "Raw bytes, which should be base64-encoded."]
    pub data: terrars::PrimField<std::string::String>,
    #[doc = "The IANA standard MIME type of the source data."]
    pub mime_type: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData {
            data: self.data,
            mime_type: self.mime_type,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nRaw bytes, which should be base64-encoded."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nThe IANA standard MIME type of the source data."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_offset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
    #[doc = "Set the field `end_offset`.\nThe end offset of the video."]
    pub fn set_end_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_offset`.\nThe start offset of the video."]
    pub fn set_start_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata {
            end_offset: core::default::Default::default(),
            start_offset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_offset` after provisioning.\nThe end offset of the video."]
    pub fn end_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `start_offset` after provisioning.\nThe start offset of the video."]
    pub fn start_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_offset", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsDynamic {
    audio_transcription: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription,
        >,
    >,
    code_execution_result: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult,
        >,
    >,
    executable_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode,
        >,
    >,
    file_data: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData,
        >,
    >,
    function_call: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall,
        >,
    >,
    function_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse,
        >,
    >,
    inline_data: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData,
        >,
    >,
    video_metadata: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thought: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_transcription: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_execution_result: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executable_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_data: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_call: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_response: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_data: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_metadata: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
    #[doc = "Set the field `text`.\nThe text content of the part."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thought`.\nIndicates whether the part represents the model's thought process or reasoning."]
    pub fn set_thought(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.thought = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_transcription`.\n"]
    pub fn set_audio_transcription(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscription,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_transcription = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_transcription = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_execution_result`.\n"]
    pub fn set_code_execution_result(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResult,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_execution_result = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_execution_result = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `executable_code`.\n"]
    pub fn set_executable_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.executable_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.executable_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_data`.\n"]
    pub fn set_file_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileData,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_call`.\n"]
    pub fn set_function_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCall,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.function_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.function_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_response`.\n"]
    pub fn set_function_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.function_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.function_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_data`.\n"]
    pub fn set_inline_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineData,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_metadata`.\n"]
    pub fn set_video_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadata,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts {
            text: core::default::Default::default(),
            thought: core::default::Default::default(),
            audio_transcription: core::default::Default::default(),
            code_execution_result: core::default::Default::default(),
            executable_code: core::default::Default::default(),
            file_data: core::default::Default::default(),
            function_call: core::default::Default::default(),
            function_response: core::default::Default::default(),
            inline_data: core::default::Default::default(),
            video_metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe text content of the part."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thought` after provisioning.\nIndicates whether the part represents the model's thought process or reasoning."]
    pub fn thought(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thought", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_transcription` after provisioning.\n"]
    pub fn audio_transcription(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_AudioTranscriptionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_transcription", self.base))
    }

    #[doc = "Get a reference to the value of field `code_execution_result` after provisioning.\n"]
    pub fn code_execution_result(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_CodeExecutionResultRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_execution_result", self.base))
    }

    #[doc = "Get a reference to the value of field `executable_code` after provisioning.\n"]
    pub fn executable_code(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_ExecutableCodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.executable_code", self.base))
    }

    #[doc = "Get a reference to the value of field `file_data` after provisioning.\n"]
    pub fn file_data(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FileDataRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_data", self.base))
    }

    #[doc = "Get a reference to the value of field `function_call` after provisioning.\n"]
    pub fn function_call(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionCallRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_call", self.base))
    }

    #[doc = "Get a reference to the value of field `function_response` after provisioning.\n"]
    pub fn function_response(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_FunctionResponseRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_response", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_data` after provisioning.\n"]
    pub fn inline_data(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_InlineDataRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_data", self.base))
    }

    #[doc = "Get a reference to the value of field `video_metadata` after provisioning.\n"]
    pub fn video_metadata(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts_VideoMetadataRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentDynamic {
    parts: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parts: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
    #[doc =
        "Set the field `role`.\nThe producer of the content. Must be either 'user' or 'model'. If not set, the service will default to 'user'."]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parts`.\n"]
    pub fn set_parts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_Parts,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content {
            role: core::default::Default::default(),
            parts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe producer of the content. Must be either 'user' or 'model'. If not set, the service will default to 'user'."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `parts` after provisioning.\n"]
    pub fn parts(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content_PartsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parts", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsDynamic {
    content: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_Content,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events {
            content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events_ContentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceDynamic {
    events: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_Events,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.events = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.events = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource {
            events: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource_EventsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_memory_topic_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_memory_topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
    #[doc = "Set the field `custom_memory_topic_label`.\nRepresents the custom memory topic label."]
    pub fn set_custom_memory_topic_label(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_memory_topic_label = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `managed_memory_topic`.\nRepresents the managed memory topic. Possible values: [\"USER_PERSONAL_INFO\", \"USER_PREFERENCES\", \"KEY_CONVERSATION_DETAILS\", \"EXPLICIT_INSTRUCTIONS\"]"]
    pub fn set_managed_memory_topic(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_memory_topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics {
            custom_memory_topic_label: core::default::Default::default(),
            managed_memory_topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_memory_topic_label` after provisioning.\nRepresents the custom memory topic label."]
    pub fn custom_memory_topic_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_memory_topic_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_memory_topic` after provisioning.\nRepresents the managed memory topic. Possible values: [\"USER_PERSONAL_INFO\", \"USER_PREFERENCES\", \"KEY_CONVERSATION_DETAILS\", \"EXPLICIT_INSTRUCTIONS\"]"]
    pub fn managed_memory_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_memory_topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesDynamic {
    topics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
    fact: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_Topics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.topics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.topics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
    #[doc = "Represents the fact to generate a memory from."]
    pub fact: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories {
            fact: self.fact,
            topics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fact` after provisioning.\nRepresents the fact to generate a memory from."]
    pub fn fact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fact", self.base))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories_TopicsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesDynamic {
    conversation_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource,
        >,
    >,
    generated_memories: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generated_memories: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
    #[doc = "Set the field `conversation_source`.\n"]
    pub fn set_conversation_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generated_memories`.\n"]
    pub fn set_generated_memories(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemories,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generated_memories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generated_memories = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples {
            conversation_source: core::default::Default::default(),
            generated_memories: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conversation_source` after provisioning.\n"]
    pub fn conversation_source(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_ConversationSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_source", self.base))
    }

    #[doc = "Get a reference to the value of field `generated_memories` after provisioning.\n"]
    pub fn generated_memories(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples_GeneratedMemoriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generated_memories", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
    #[doc = "Set the field `description`.\nDescription of custom memory topic."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\nLabel of custom memory topic."]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic {
            description: core::default::Default::default(),
            label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of custom memory topic."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\nLabel of custom memory topic."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_topic_enum: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
    #[doc = "Set the field `managed_topic_enum`.\nManaged topic enum (e.g. USER_PREFERENCES, EXPLICIT_INSTRUCTIONS)."]
    pub fn set_managed_topic_enum(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_topic_enum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic {
            managed_topic_enum: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `managed_topic_enum` after provisioning.\nManaged topic enum (e.g. USER_PREFERENCES, EXPLICIT_INSTRUCTIONS)."]
    pub fn managed_topic_enum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_topic_enum", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsDynamic {
    custom_memory_topic: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic,
        >,
    >,
    managed_memory_topic: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_memory_topic: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_memory_topic: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
    #[doc = "Set the field `custom_memory_topic`.\n"]
    pub fn set_custom_memory_topic(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopic,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_memory_topic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_memory_topic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_memory_topic`.\n"]
    pub fn set_managed_memory_topic(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopic,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_memory_topic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_memory_topic = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
    type O =
        terrars
        ::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics {
            custom_memory_topic: core::default::Default::default(),
            managed_memory_topic: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_memory_topic` after provisioning.\n"]
    pub fn custom_memory_topic(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_CustomMemoryTopicRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_memory_topic", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_memory_topic` after provisioning.\n"]
    pub fn managed_memory_topic(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics_ManagedMemoryTopicRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_memory_topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsDynamic {
    consolidation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig>,
    >,
    generate_memories_examples: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples,
        >,
    >,
    memory_topics: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_natural_language_memories: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_third_person_memories: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consolidation_config: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate_memories_examples: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_topics: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics>>,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
    #[doc =
        "Set the field `disable_natural_language_memories`.\nIndicates whether natural language memory generation should be disabled."]
    pub fn set_disable_natural_language_memories(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_natural_language_memories = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_third_person_memories`.\nOptional. Generate memories in the third person if set to true."]
    pub fn set_enable_third_person_memories(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_third_person_memories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope_keys`.\nOptional. List of scope keys that this customization config applies to."]
    pub fn set_scope_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scope_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consolidation_config`.\n"]
    pub fn set_consolidation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consolidation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consolidation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generate_memories_examples`.\n"]
    pub fn set_generate_memories_examples(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamples,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generate_memories_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generate_memories_examples = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_topics`.\n"]
    pub fn set_memory_topics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_topics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_topics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs {
            disable_natural_language_memories: core::default::Default::default(),
            enable_third_person_memories: core::default::Default::default(),
            scope_keys: core::default::Default::default(),
            consolidation_config: core::default::Default::default(),
            generate_memories_examples: core::default::Default::default(),
            memory_topics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_natural_language_memories` after provisioning.\nIndicates whether natural language memory generation should be disabled."]
    pub fn disable_natural_language_memories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_natural_language_memories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_third_person_memories` after provisioning.\nOptional. Generate memories in the third person if set to true."]
    pub fn enable_third_person_memories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_third_person_memories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope_keys` after provisioning.\nOptional. List of scope keys that this customization config applies to."]
    pub fn scope_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `consolidation_config` after provisioning.\n"]
    pub fn consolidation_config(
        &self,
    ) -> terrars
    ::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_ConsolidationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consolidation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `generate_memories_examples` after provisioning.\n"]
    pub fn generate_memories_examples(
        &self,
    ) -> terrars
    ::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_GenerateMemoriesExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generate_memories_examples", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_topics` after provisioning.\n"]
    pub fn memory_topics(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs_MemoryTopicsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_topics", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overlap_event_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
    #[doc =
        "Set the field `event_count`.\nOptional. Specifies to trigger generation when the event count reaches this limit."]
    pub fn set_event_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.event_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fixed_interval`.\nOptional. Specifies to trigger generation at a fixed interval. The duration\nmust have a minute-level granularity."]
    pub fn set_fixed_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `idle_duration`.\nOptional. Specifies to trigger generation if the stream is inactive for the\nspecified duration after the most recent event. The duration must have a\nminute-level granularity."]
    pub fn set_idle_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `overlap_event_count`.\nOptional. Re-include the last N already-processed events in the next window."]
    pub fn set_overlap_event_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.overlap_event_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule {
            event_count: core::default::Default::default(),
            fixed_interval: core::default::Default::default(),
            idle_duration: core::default::Default::default(),
            overlap_event_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_count` after provisioning.\nOptional. Specifies to trigger generation when the event count reaches this limit."]
    pub fn event_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fixed_interval` after provisioning.\nOptional. Specifies to trigger generation at a fixed interval. The duration\nmust have a minute-level granularity."]
    pub fn fixed_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_duration` after provisioning.\nOptional. Specifies to trigger generation if the stream is inactive for the\nspecified duration after the most recent event. The duration must have a\nminute-level granularity."]
    pub fn idle_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `overlap_event_count` after provisioning.\nOptional. Re-include the last N already-processed events in the next window."]
    pub fn overlap_event_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overlap_event_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigDynamic {
    generation_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule,
        >,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_rule: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule,
        >,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
    #[doc = "Set the field `generation_rule`.\n"]
    pub fn set_generation_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
    type O =
        terrars
        ::BlockAssignable<
            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
    pub fn build(
        self,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig {
            generation_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `generation_rule` after provisioning.\n"]
    pub fn generation_rule(
        &self,
    ) -> terrars
    ::ListRef<
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig_GenerationRuleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigDynamic {
    generation_trigger_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
    model: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_trigger_config: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig>,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
    #[doc = "Set the field `generation_trigger_config`.\n"]
    pub fn set_generation_trigger_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_trigger_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_trigger_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
    #[doc =
        "The model used to generate memories. Format: projects/{project}/locations/{location}/publishers/google/models/{model}."]
    pub model: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig {
            model: self.model,
            generation_trigger_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nThe model used to generate memories. Format: projects/{project}/locations/{location}/publishers/google/models/{model}."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_trigger_config` after provisioning.\n"]
    pub fn generation_trigger_config(
        &self,
    ) -> terrars
    ::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig_GenerationTriggerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_trigger_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
    embedding_model: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig { }

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
    #[doc =
        "The model used to generate embeddings to lookup similar memories. Format: projects/{project}/locations/{location}/publishers/google/models/{model}."]
    pub embedding_model: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig {
            embedding_model: self.embedding_model,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `embedding_model` after provisioning.\nThe model used to generate embeddings to lookup similar memories. Format: projects/{project}/locations/{location}/publishers/google/models/{model}."]
    pub fn embedding_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
    #[doc =
        "Set the field `memory_schema`.\nOptional. The memory schema defined as an OpenAPI Schema Object JSON string."]
    pub fn set_memory_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
    type O =
        terrars
        ::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
    #[doc = "Required. Unique ID identifying the memory schema."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs {
            id: self.id,
            memory_schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. Unique ID identifying the memory schema."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_schema` after provisioning.\nOptional. The memory schema defined as an OpenAPI Schema Object JSON string."]
    pub fn memory_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsDynamic {
    schema_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_configs: std
    ::option
    ::Option<
        std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs>,
    >,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
    #[doc = "Set the field `scope_keys`.\nOptional. List of scope keys that this structured memory config applies to."]
    pub fn set_scope_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scope_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_configs`.\n"]
    pub fn set_schema_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs {
            scope_keys: core::default::Default::default(),
            schema_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope_keys` after provisioning.\nOptional. List of scope keys that this structured memory config applies to."]
    pub fn scope_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_configs` after provisioning.\n"]
    pub fn schema_configs(
        &self,
    ) -> terrars
    ::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs_SchemaConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate_created_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate_updated_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
    #[doc = "Set the field `create_ttl`.\nThe TTL duration for memories uploaded via CreateMemory."]
    pub fn set_create_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `generate_created_ttl`.\nThe TTL duration for memories newly generated via GenerateMemories."]
    pub fn set_generate_created_ttl(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.generate_created_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generate_updated_ttl`.\nThe TTL duration for memories updated via GenerateMemories."]
    pub fn set_generate_updated_ttl(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.generate_updated_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
    type O =
        terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig {
            create_ttl: core::default::Default::default(),
            generate_created_ttl: core::default::Default::default(),
            generate_updated_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_ttl` after provisioning.\nThe TTL duration for memories uploaded via CreateMemory."]
    pub fn create_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generate_created_ttl` after provisioning.\nThe TTL duration for memories newly generated via GenerateMemories."]
    pub fn generate_created_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_created_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generate_updated_ttl` after provisioning.\nThe TTL duration for memories updated via GenerateMemories."]
    pub fn generate_updated_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_updated_ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigDynamic {
    granular_ttl_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_revision_default_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    granular_ttl_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig>>,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
    #[doc = "Set the field `default_ttl`.\nThe default TTL duration of the memories in the Memory Bank."]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory_revision_default_ttl`.\nThe default TTL duration of the memory revisions in the Memory Bank."]
    pub fn set_memory_revision_default_ttl(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.memory_revision_default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `granular_ttl_config`.\n"]
    pub fn set_granular_ttl_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.granular_ttl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.granular_ttl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig {
            default_ttl: core::default::Default::default(),
            memory_revision_default_ttl: core::default::Default::default(),
            granular_ttl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_ttl` after provisioning.\nThe default TTL duration of the memories in the Memory Bank."]
    pub fn default_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_revision_default_ttl` after provisioning.\nThe default TTL duration of the memory revisions in the Memory Bank."]
    pub fn memory_revision_default_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_revision_default_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `granular_ttl_config` after provisioning.\n"]
    pub fn granular_ttl_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig_GranularTtlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.granular_ttl_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfigDynamic {
    customization_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs>>,
    generation_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig>>,
    similarity_search_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig>>,
    structured_memory_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs>>,
    ttl_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_memory_revisions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customization_configs: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    similarity_search_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    structured_memory_configs: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl_config: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig>>,
    dynamic: VertexAiReasoningEngine_ContextSpec_MemoryBankConfigDynamic,
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
    #[doc =
        "Set the field `disable_memory_revisions`.\nIf true, no memory revisions will be created for any requests to the Memory Bank."]
    pub fn set_disable_memory_revisions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_memory_revisions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customization_configs`.\n"]
    pub fn set_customization_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customization_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customization_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generation_config`.\n"]
    pub fn set_generation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `similarity_search_config`.\n"]
    pub fn set_similarity_search_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.similarity_search_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.similarity_search_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `structured_memory_configs`.\n"]
    pub fn set_structured_memory_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.structured_memory_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.structured_memory_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ttl_config`.\n"]
    pub fn set_ttl_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig {}

impl BuildVertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfig {
            disable_memory_revisions: core::default::Default::default(),
            customization_configs: core::default::Default::default(),
            generation_config: core::default::Default::default(),
            similarity_search_config: core::default::Default::default(),
            structured_memory_configs: core::default::Default::default(),
            ttl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef {
        VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_memory_revisions` after provisioning.\nIf true, no memory revisions will be created for any requests to the Memory Bank."]
    pub fn disable_memory_revisions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_memory_revisions", self.base))
    }

    #[doc = "Get a reference to the value of field `customization_configs` after provisioning.\n"]
    pub fn customization_configs(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_CustomizationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customization_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_config` after provisioning.\n"]
    pub fn generation_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_GenerationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `similarity_search_config` after provisioning.\n"]
    pub fn similarity_search_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_SimilaritySearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.similarity_search_config", self.base))
    }

    #[doc = "Get a reference to the value of field `structured_memory_configs` after provisioning.\n"]
    pub fn structured_memory_configs(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_StructuredMemoryConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.structured_memory_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl_config` after provisioning.\n"]
    pub fn ttl_config(&self) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig_TtlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_ContextSpecDynamic {
    memory_bank_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_ContextSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bank_config: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig>>,
    dynamic: VertexAiReasoningEngine_ContextSpecDynamic,
}

impl VertexAiReasoningEngine_ContextSpec {
    #[doc = "Set the field `memory_bank_config`.\n"]
    pub fn set_memory_bank_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec_MemoryBankConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_bank_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_bank_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_ContextSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_ContextSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_ContextSpec {}

impl BuildVertexAiReasoningEngine_ContextSpec {
    pub fn build(self) -> VertexAiReasoningEngine_ContextSpec {
        VertexAiReasoningEngine_ContextSpec {
            memory_bank_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_ContextSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_ContextSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_ContextSpecRef {
        VertexAiReasoningEngine_ContextSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_ContextSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `memory_bank_config` after provisioning.\n"]
    pub fn memory_bank_config(&self) -> terrars::ListRef<VertexAiReasoningEngine_ContextSpec_MemoryBankConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_bank_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiReasoningEngine_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_EncryptionSpec {
    #[doc =
        "Required. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect a resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the compute resource\nis created."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_EncryptionSpec {
    pub fn build(self) -> VertexAiReasoningEngine_EncryptionSpec {
        VertexAiReasoningEngine_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_EncryptionSpecRef {
        VertexAiReasoningEngine_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect a resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the compute resource\nis created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_BuildSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Spec_BuildSpec {
    #[doc = "Set the field `service_account`.\nOptional. The service account that the Cloud Build builder runs as."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_pool`.\nOptional. The resource name of the Cloud Build WorkerPool to use for the build."]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_BuildSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_BuildSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_BuildSpec {}

impl BuildVertexAiReasoningEngine_Spec_BuildSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_BuildSpec {
        VertexAiReasoningEngine_Spec_BuildSpec {
            service_account: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_BuildSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_BuildSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_Spec_BuildSpecRef {
        VertexAiReasoningEngine_Spec_BuildSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_BuildSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. The service account that the Cloud Build builder runs as."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nOptional. The resource name of the Cloud Build WorkerPool to use for the build."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_ContainerSpec {
    image_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiReasoningEngine_Spec_ContainerSpec {
    #[doc =
        "Set the field `port`.\nOptional. The port that the container listens on for incoming requests. If not specified, defaults to 8080."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_ContainerSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_ContainerSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_ContainerSpec {
    #[doc =
        "The Artifact Registry Docker image URI (e.g.,\n'us-central1-docker.pkg.dev/my-project/my-repo/my-image:tag') of the\ncontainer image that is to be run on each worker replica."]
    pub image_uri: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_ContainerSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_ContainerSpec {
        VertexAiReasoningEngine_Spec_ContainerSpec {
            image_uri: self.image_uri,
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_ContainerSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_ContainerSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_Spec_ContainerSpecRef {
        VertexAiReasoningEngine_Spec_ContainerSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_ContainerSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nThe Artifact Registry Docker image URI (e.g.,\n'us-central1-docker.pkg.dev/my-project/my-repo/my-image:tag') of the\ncontainer image that is to be run on each worker replica."]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nOptional. The port that the container listens on for incoming requests. If not specified, defaults to 8080."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
    agent_gateway: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
    type O =
        terrars
        ::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
    #[doc = "Required. The resource name of the Agent Gateway for outbound traffic."]
    pub agent_gateway: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig {
            agent_gateway: self.agent_gateway,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_gateway` after provisioning.\nRequired. The resource name of the Agent Gateway for outbound traffic."]
    pub fn agent_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
    agent_gateway: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
    type O =
        terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
    #[doc = "Required. The resource name of the Agent Gateway to use for inbound traffic."]
    pub agent_gateway: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig {
            agent_gateway: self.agent_gateway,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_gateway` after provisioning.\nRequired. The resource name of the Agent Gateway to use for inbound traffic."]
    pub fn agent_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_gateway", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigDynamic {
    agent_to_anywhere_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig>,
    >,
    client_to_agent_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_to_anywhere_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_to_agent_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig>>,
    dynamic: VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigDynamic,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
    #[doc = "Set the field `agent_to_anywhere_config`.\n"]
    pub fn set_agent_to_anywhere_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_to_anywhere_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_to_anywhere_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_to_agent_config`.\n"]
    pub fn set_client_to_agent_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_to_agent_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_to_agent_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig {
            agent_to_anywhere_config: core::default::Default::default(),
            client_to_agent_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_to_anywhere_config` after provisioning.\n"]
    pub fn agent_to_anywhere_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_AgentToAnywhereConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_to_anywhere_config", self.base))
    }

    #[doc = "Get a reference to the value of field `client_to_agent_config` after provisioning.\n"]
    pub fn client_to_agent_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig_ClientToAgentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_to_agent_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_Env {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_Env { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_Env {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_Env {
    #[doc = "The name of the environment variable. Must be a valid\nC identifier."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Variables that reference a $(VAR_NAME) are expanded using\nthe previous defined environment variables in the container\nand any service environment variables. If a variable cannot\nbe resolved, the reference in the input string will be\nunchanged. The $(VAR_NAME) syntax can be escaped with a\ndouble $$, ie: $$(VAR_NAME). Escaped references will never\nbe expanded, regardless of whether the variable exists\nor not."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_Env {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_Env {
        VertexAiReasoningEngine_Spec_DeploymentSpec_Env {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_EnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_EnvRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the environment variable. Must be a valid\nC identifier."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded using\nthe previous defined environment variables in the container\nand any service environment variables. If a variable cannot\nbe resolved, the reference in the input string will be\nunchanged. The $(VAR_NAME) syntax can be escaped with a\ndouble $$, ie: $$(VAR_NAME). Escaped references will never\nbe expanded, regardless of whether the variable exists\nor not."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
    domain: terrars::PrimField<std::string::String>,
    target_network: terrars::PrimField<std::string::String>,
    target_project: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
    type O =
        terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
    #[doc =
        "Required. The DNS name suffix of the zone being peered\nto, e.g., \"my-internal-domain.corp.\".\nMust end with a dot."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The VPC network name in the targetProject\nwhere the DNS zone specified by 'domain' is visible."]
    pub target_network: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The project id hosting the Cloud DNS managed\nzone that contains the 'domain'.\nThe Vertex AI service Agent requires the dns.peer role\non this project."]
    pub target_project: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
        VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs {
            domain: self.domain,
            target_network: self.target_network,
            target_project: self.target_project,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nRequired. The DNS name suffix of the zone being peered\nto, e.g., \"my-internal-domain.corp.\".\nMust end with a dot."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_network` after provisioning.\nRequired. The VPC network name in the targetProject\nwhere the DNS zone specified by 'domain' is visible."]
    pub fn target_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_project` after provisioning.\nRequired. The project id hosting the Cloud DNS managed\nzone that contains the 'domain'.\nThe Vertex AI service Agent requires the dns.peer role\non this project."]
    pub fn target_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_project", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigDynamic {
    dns_peering_configs: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_peering_configs: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs>>,
    dynamic: VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigDynamic,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
    #[doc =
        "Set the field `network_attachment`.\nOptional. The name of the Compute Engine network attachment\nto attach to the resource within the region and user project.\nTo specify this field, you must have already created a network attachment.\nThis field is only used for resources using PSC-Interface."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_peering_configs`.\n"]
    pub fn set_dns_peering_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_peering_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_peering_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
        VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig {
            network_attachment: core::default::Default::default(),
            dns_peering_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nOptional. The name of the Compute Engine network attachment\nto attach to the resource within the region and user project.\nTo specify this field, you must have already created a network attachment.\nThis field is only used for resources using PSC-Interface."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_peering_configs` after provisioning.\n"]
    pub fn dns_peering_configs(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig_DnsPeeringConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_peering_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
    #[doc =
        "Set the field `version`.\nThe Cloud Secret Manager secret version. Can be 'latest'\nfor the latest version, an integer for a specific\nversion, or a version alias."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
    #[doc = "The name of the secret in Cloud Secret Manager.\nFormat: {secret_name}."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef {
            secret: self.secret,
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nThe name of the secret in Cloud Secret Manager.\nFormat: {secret_name}."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Cloud Secret Manager secret version. Can be 'latest'\nfor the latest version, an integer for a specific\nversion, or a version alias."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvDynamic {
    secret_ref: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_ref: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef>>,
    dynamic: VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvDynamic,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
    #[doc = "Set the field `secret_ref`.\n"]
    pub fn set_secret_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRef>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_ref = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
    #[doc = "The name of the environment variable. Must be a valid C\nidentifier."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
        VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv {
            name: self.name,
            secret_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvRef {
        VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the environment variable. Must be a valid C\nidentifier."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_ref` after provisioning.\n"]
    pub fn secret_ref(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv_SecretRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_ref", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_DeploymentSpecDynamic {
    agent_gateway_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig>>,
    env: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_Env>>,
    psc_interface_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig>>,
    secret_env: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_gateway_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_env: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv>>,
    dynamic: VertexAiReasoningEngine_Spec_DeploymentSpecDynamic,
}

impl VertexAiReasoningEngine_Spec_DeploymentSpec {
    #[doc =
        "Set the field `container_concurrency`.\nOptional. Concurrency for each container and agent server.\nRecommended value: 2 * cpu + 1. Defaults to 9."]
    pub fn set_container_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_instances`.\nOptional. The maximum number of application instances that can be\nlaunched to handle increased traffic. Defaults to 100.\nRange: [1, 1000]. If VPC-SC or PSC-I is enabled, the acceptable\nrange is [1, 100]."]
    pub fn set_max_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_instances`.\nOptional. The minimum number of application instances that will be\nkept running at all times. Defaults to 1. Range: [0, 10]."]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_limits`.\nOptional. Resource limits for each container.\nOnly 'cpu' and 'memory' keys are supported.\n\nDefaults to {\"cpu\": \"4\", \"memory\": \"4Gi\"}.\n\nThe only supported values for CPU are '1', '2', '4', '6' and '8'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/cpu.\n\nThe only supported values for memory are '1Gi', '2Gi', ... '32 Gi'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/memory-limits."]
    pub fn set_resource_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_gateway_config`.\n"]
    pub fn set_agent_gateway_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_gateway_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_gateway_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_Env>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_interface_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_interface_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec_SecretEnv>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_env = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_DeploymentSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_DeploymentSpec {}

impl BuildVertexAiReasoningEngine_Spec_DeploymentSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_DeploymentSpec {
        VertexAiReasoningEngine_Spec_DeploymentSpec {
            container_concurrency: core::default::Default::default(),
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
            agent_gateway_config: core::default::Default::default(),
            env: core::default::Default::default(),
            psc_interface_config: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_DeploymentSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_DeploymentSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_DeploymentSpecRef {
        VertexAiReasoningEngine_Spec_DeploymentSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_DeploymentSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_concurrency` after provisioning.\nOptional. Concurrency for each container and agent server.\nRecommended value: 2 * cpu + 1. Defaults to 9."]
    pub fn container_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_concurrency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nOptional. The maximum number of application instances that can be\nlaunched to handle increased traffic. Defaults to 100.\nRange: [1, 1000]. If VPC-SC or PSC-I is enabled, the acceptable\nrange is [1, 100]."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nOptional. The minimum number of application instances that will be\nkept running at all times. Defaults to 1. Range: [0, 10]."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_limits` after provisioning.\nOptional. Resource limits for each container.\nOnly 'cpu' and 'memory' keys are supported.\n\nDefaults to {\"cpu\": \"4\", \"memory\": \"4Gi\"}.\n\nThe only supported values for CPU are '1', '2', '4', '6' and '8'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/cpu.\n\nThe only supported values for memory are '1Gi', '2Gi', ... '32 Gi'.\nFor more information, go to\nhttps://cloud.google.com/run/docs/configuring/memory-limits."]
    pub fn resource_limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_gateway_config` after provisioning.\n"]
    pub fn agent_gateway_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_AgentGatewayConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_gateway_config", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpec_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_PackageSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dependency_files_gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pickle_object_gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Spec_PackageSpec {
    #[doc =
        "Set the field `dependency_files_gcs_uri`.\nOptional. The Cloud Storage URI of the dependency files in tar.gz\nformat."]
    pub fn set_dependency_files_gcs_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dependency_files_gcs_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pickle_object_gcs_uri`.\nOptional. The Cloud Storage URI of the pickled python object."]
    pub fn set_pickle_object_gcs_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pickle_object_gcs_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_version`.\nOptional. The Python version. Currently support 3.8, 3.9, 3.10,\n3.11, 3.12, 3.13. If not specified, default value is 3.10."]
    pub fn set_python_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirements_gcs_uri`.\nOptional. The Cloud Storage URI of the requirements.txtfile"]
    pub fn set_requirements_gcs_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.requirements_gcs_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_PackageSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_PackageSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_PackageSpec {}

impl BuildVertexAiReasoningEngine_Spec_PackageSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_PackageSpec {
        VertexAiReasoningEngine_Spec_PackageSpec {
            dependency_files_gcs_uri: core::default::Default::default(),
            pickle_object_gcs_uri: core::default::Default::default(),
            python_version: core::default::Default::default(),
            requirements_gcs_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_PackageSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_PackageSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_Spec_PackageSpecRef {
        VertexAiReasoningEngine_Spec_PackageSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_PackageSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dependency_files_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the dependency files in tar.gz\nformat."]
    pub fn dependency_files_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dependency_files_gcs_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pickle_object_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the pickled python object."]
    pub fn pickle_object_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pickle_object_gcs_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_version` after provisioning.\nOptional. The Python version. Currently support 3.8, 3.9, 3.10,\n3.11, 3.12, 3.13. If not specified, default value is 3.10."]
    pub fn python_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requirements_gcs_uri` after provisioning.\nOptional. The Cloud Storage URI of the requirements.txtfile"]
    pub fn requirements_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_gcs_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig {
    json_config: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig {
    #[doc = "Required. The value of the ADK config in JSON format."]
    pub json_config: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig { json_config: self.json_config }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_config` after provisioning.\nRequired. The value of the ADK config in JSON format."]
    pub fn json_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
    source_archive: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
    #[doc =
        "Required. Input only. The application source code archive, provided as a compressed tarball (.tar.gz) file."]
    pub source_archive: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource {
            source_archive: self.source_archive,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_archive` after provisioning.\nRequired. Input only. The application source code archive, provided as a compressed tarball (.tar.gz) file."]
    pub fn source_archive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceDynamic {
    adk_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig>>,
    inline_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adk_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_source: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource>>,
    dynamic: VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceDynamic,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
    #[doc = "Set the field `adk_config`.\n"]
    pub fn set_adk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.adk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.adk_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_source`.\n"]
    pub fn set_inline_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource {
            adk_config: core::default::Default::default(),
            inline_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adk_config` after provisioning.\n"]
    pub fn adk_config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_AdkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_source` after provisioning.\n"]
    pub fn inline_source(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource_InlineSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_source", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
    dir: terrars::PrimField<std::string::String>,
    git_repository_link: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<std::string::String>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config { }

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
    #[doc = "Directory, relative to the source root, in which to run the build."]
    pub dir: terrars::PrimField<std::string::String>,
    #[doc =
        "The Developer Connect Git repository link, formatted as projects/*/locations/*/connections/*/gitRepositoryLink/*."]
    pub git_repository_link: terrars::PrimField<std::string::String>,
    #[doc = "The revision to fetch from the Git repository such as a branch, a tag, a commit SHA, or any Git ref."]
    pub revision: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config {
            dir: self.dir,
            git_repository_link: self.git_repository_link,
            revision: self.revision,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build."]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `git_repository_link` after provisioning.\nThe Developer Connect Git repository link, formatted as projects/*/locations/*/connections/*/gitRepositoryLink/*."]
    pub fn git_repository_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nThe revision to fetch from the Git repository such as a branch, a tag, a commit SHA, or any Git ref."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceDynamic {
    config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config>>,
    dynamic: VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceDynamic,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_Config>,
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

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource {
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_args: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {
    #[doc = "Set the field `build_args`.\nBuild arguments to be used. They will be passed through --build-arg flags."]
    pub fn set_build_args(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.build_args = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec { build_args: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_args` after provisioning.\nBuild arguments to be used. They will be passed through --build-arg flags."]
    pub fn build_args(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.build_args", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_archive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
    #[doc =
        "Set the field `source_archive`.\nRequired. Input only.\nThe application source code archive, provided as a compressed\ntarball (.tar.gz) file. A base64-encoded string."]
    pub fn set_source_archive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_archive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource {
            source_archive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_archive` after provisioning.\nRequired. Input only.\nThe application source code archive, provided as a compressed\ntarball (.tar.gz) file. A base64-encoded string."]
    pub fn source_archive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint_module: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
    #[doc =
        "Set the field `entrypoint_module`.\nOptional. The Python module to load as the entrypoint,\nspecified as a fully qualified module name. For example:\npath.to.agent. If not specified, defaults to \"agent\".\nThe project root will be added to Python sys.path, allowing\nimports to be specified relative to the root."]
    pub fn set_entrypoint_module(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint_module = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entrypoint_object`.\nOptional. The name of the callable object within the\nentrypointModule to use as the application If not specified,\ndefaults to \"root_agent\"."]
    pub fn set_entrypoint_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint_object = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requirements_file`.\nOptional. The path to the requirements file, relative to the\nsource root. If not specified, defaults to \"requirements.txt\"."]
    pub fn set_requirements_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirements_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nOptional. The version of Python to use. Support version\nincludes 3.9, 3.10, 3.11, 3.12, 3.13. If not specified,\ndefault value is 3.10."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec {
            entrypoint_module: core::default::Default::default(),
            entrypoint_object: core::default::Default::default(),
            requirements_file: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `entrypoint_module` after provisioning.\nOptional. The Python module to load as the entrypoint,\nspecified as a fully qualified module name. For example:\npath.to.agent. If not specified, defaults to \"agent\".\nThe project root will be added to Python sys.path, allowing\nimports to be specified relative to the root."]
    pub fn entrypoint_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint_module", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint_object` after provisioning.\nOptional. The name of the callable object within the\nentrypointModule to use as the application If not specified,\ndefaults to \"root_agent\"."]
    pub fn entrypoint_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint_object", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requirements_file` after provisioning.\nOptional. The path to the requirements file, relative to the\nsource root. If not specified, defaults to \"requirements.txt\"."]
    pub fn requirements_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. The version of Python to use. Support version\nincludes 3.9, 3.10, 3.11, 3.12, 3.13. If not specified,\ndefault value is 3.10."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_Spec_SourceCodeSpecDynamic {
    agent_config_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource>>,
    developer_connect_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource>>,
    image_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec>>,
    inline_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource>>,
    python_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_config_source: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_connect_source: std
    ::option
    ::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_source: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec>>,
    dynamic: VertexAiReasoningEngine_Spec_SourceCodeSpecDynamic,
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpec {
    #[doc = "Set the field `agent_config_source`.\n"]
    pub fn set_agent_config_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_config_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_config_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `developer_connect_source`.\n"]
    pub fn set_developer_connect_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.developer_connect_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.developer_connect_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_spec`.\n"]
    pub fn set_image_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_source`.\n"]
    pub fn set_inline_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `python_spec`.\n"]
    pub fn set_python_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec_SourceCodeSpec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec_SourceCodeSpec {}

impl BuildVertexAiReasoningEngine_Spec_SourceCodeSpec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec_SourceCodeSpec {
        VertexAiReasoningEngine_Spec_SourceCodeSpec {
            agent_config_source: core::default::Default::default(),
            developer_connect_source: core::default::Default::default(),
            image_spec: core::default::Default::default(),
            inline_source: core::default::Default::default(),
            python_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_Spec_SourceCodeSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_Spec_SourceCodeSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiReasoningEngine_Spec_SourceCodeSpecRef {
        VertexAiReasoningEngine_Spec_SourceCodeSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_Spec_SourceCodeSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_config_source` after provisioning.\n"]
    pub fn agent_config_source(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_AgentConfigSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_config_source", self.base))
    }

    #[doc = "Get a reference to the value of field `developer_connect_source` after provisioning.\n"]
    pub fn developer_connect_source(
        &self,
    ) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_DeveloperConnectSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.developer_connect_source", self.base))
    }

    #[doc = "Get a reference to the value of field `image_spec` after provisioning.\n"]
    pub fn image_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_ImageSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_source` after provisioning.\n"]
    pub fn inline_source(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_InlineSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_source", self.base))
    }

    #[doc = "Get a reference to the value of field `python_spec` after provisioning.\n"]
    pub fn python_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpec_PythonSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiReasoningEngine_SpecDynamic {
    build_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_BuildSpec>>,
    container_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_ContainerSpec>>,
    deployment_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_DeploymentSpec>>,
    package_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_PackageSpec>>,
    source_code_spec: std::option::Option<terrars::DynamicBlock<VertexAiReasoningEngine_Spec_SourceCodeSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_framework: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    class_methods: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_BuildSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_ContainerSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_DeploymentSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_PackageSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_code_spec: std::option::Option<std::vec::Vec<VertexAiReasoningEngine_Spec_SourceCodeSpec>>,
    dynamic: VertexAiReasoningEngine_SpecDynamic,
}

impl VertexAiReasoningEngine_Spec {
    #[doc = "Set the field `agent_framework`.\nOptional. The OSS agent framework used to develop the agent."]
    pub fn set_agent_framework(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_framework = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `class_methods`.\nOptional. Declarations for object class methods in OpenAPI\nspecification format.\n\n**Note**: When deploying via Terraform, this field must be populated manually.\nOtherwise, client SDKs (like 'agent_engines.get()') will not be able to discover the methods, and calls to the engine (or A2A integrations) will fail.\n\nDepending on the template/framework used ('agent_framework'), the required class methods and their parameters differ:\n\n**Warning**: The configuration snippets below are illustrative, may not be exhaustive, and could stop working over time. For the most up-to-date method lists and schemas, please consult the respective SDK source code:\n* For Google ADK: See [ADK Python SDK cli_deploy.py](https://github.com/google/adk-python/blob/68a780306e3bdd648a882ef34c0abf8e5148353e/src/google/adk/cli/cli_deploy.py#L109).\n* For Langchain: See [Vertex AI Python SDK langchain.py](https://github.com/googleapis/python-aiplatform/blob/c8a38a085931b01f4d6071f0ab7a64cb42851829/agentplatform/agent_engines/templates/langchain.py#L642-L717).\n\n### 1. Langchain Template\n* 'query' (api_mode = \"sync\" or empty)\n* 'stream_query' (api_mode = \"stream\")\n\nExample for Langchain:\n'''hcl\nclass_methods = jsonencode([\n  {\n    name        = \"query\"\n    api_mode    = \"sync\"\n    description = \"Queries the reasoning engine\"\n    parameters  = {\n      type       = \"object\"\n      required   = [\"input\"]\n      properties = {\n        input = {\n          type        = \"string\"\n          description = \"The input prompt\"\n        }\n      }\n    }\n  },\n  {\n    name        = \"stream_query\"\n    api_mode    = \"stream\"\n    description = \"Streams queries from the reasoning engine\"\n    parameters  = {\n      type       = \"object\"\n      required   = [\"input\"]\n      properties = {\n        input = {\n          type        = \"string\"\n          description = \"The input prompt\"\n        }\n      }\n    }\n  }\n])\n'''\n\n### 2. Google ADK Template (Standard - No A2A)\nFor standard Google ADK (Agent Development Kit) deployments, you must define the following 11 methods:\n\nExample for Standard ADK:\n'''hcl\nclass_methods = jsonencode([\n  {\n    name        = \"get_session\"\n    api_mode    = \"\"\n    description = \"Retrieve session by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_get_session\"\n    api_mode    = \"async\"\n    description = \"Retrieve session asynchronously by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"list_sessions\"\n    api_mode    = \"\"\n    description = \"List all sessions for a user\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_list_sessions\"\n    api_mode    = \"async\"\n    description = \"List all sessions for a user asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"create_session\"\n    api_mode    = \"\"\n    description = \"Create a new session\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"async_create_session\"\n    api_mode    = \"async\"\n    description = \"Create a new session asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"delete_session\"\n    api_mode    = \"\"\n    description = \"Delete session by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_delete_session\"\n    api_mode    = \"async\"\n    description = \"Delete session asynchronously by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"stream_query\"\n    api_mode    = \"stream\"\n    description = \"Stream queries from the agent\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"message\", \"user_id\"]\n      properties = {\n        message    = { description = \"Message string or object\" }\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        run_config = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"async_stream_query\"\n    api_mode    = \"async_stream\"\n    description = \"Stream queries asynchronously from the agent\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"message\", \"user_id\"]\n      properties = {\n        message        = { description = \"Message string or object\" }\n        user_id        = { type = \"string\" }\n        session_id     = { type = \"string\" }\n        session_events = { type = \"array\", items = { type = \"object\" } }\n        run_config     = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"streaming_agent_run_with_events\"\n    api_mode    = \"async_stream\"\n    description = \"Stream agent run with events asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"request_json\"]\n      properties = {\n        request_json = { type = \"string\" }\n      }\n    }\n  }\n])\n'''\n\n### 3. Google ADK Template (A2A-Enabled)\nIf the agent integrates with the Gemini Enterprise Agent Registry (A2A), you must inject the 'a2a_agent_card' JSON metadata as a string **specifically inside the 'async_create_session' method definition**:\n\nExample for A2A-Enabled ADK:\n'''hcl\nlocals {\n  # Construct the A2A endpoint URL\n  a2a_url = \"https://us-central1-aiplatform.googleapis.com/v1/projects/my-project/locations/us-central1/reasoningEngines/my-agent/a2a\"\n\n  agent_card = {\n    name                 = \"my-agent\"\n    description          = \"A2A Agent\"\n    version              = \"1.0.0\"\n    preferred_transport  = \"HTTP_JSON\"\n    supported_interfaces = [{ url = local.a2a_url, protocol_binding = \"HTTP_JSON\" }]\n    capabilities         = { streaming = true }\n  }\n}\n\n# In class_methods, append \"a2a_agent_card\" key ONLY to the \"async_create_session\" method:\nclass_methods = jsonencode([\n  # ... other 10 standard methods (same as Standard ADK) ...\n  {\n    name        = \"async_create_session\"\n    api_mode    = \"async\"\n    description = \"Create a new session asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n    # Inject the serialized Agent Card here\n    a2a_agent_card = jsonencode(local.agent_card)\n  }\n])\n'''"]
    pub fn set_class_methods(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.class_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nOptional. The identity type to use for the Reasoning Engine.\nIf not specified, the 'service_account' field will be used if set,\notherwise the default Vertex AI Reasoning Engine Service Agent in the project will be used.\nPossible values:\n* 'SERVICE_ACCOUNT': Use a custom service account if the 'service_account' field is set, otherwise use the default Vertex AI Reasoning Engine Service Agent in the project.\n* 'AGENT_IDENTITY': Use Agent Identity. The 'service_account' field must not be set. Possible values: [\"SERVICE_ACCOUNT\", \"AGENT_IDENTITY\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. The service account that the Reasoning Engine artifact runs\nas. It should have \"roles/storage.objectViewer\" for reading the user\nproject's Cloud Storage and \"roles/aiplatform.user\" for using Vertex\nextensions. If not specified, the Vertex AI Reasoning Engine service\nAgent in the project will be used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_spec`.\n"]
    pub fn set_build_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_BuildSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.build_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.build_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_spec`.\n"]
    pub fn set_container_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_ContainerSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_spec`.\n"]
    pub fn set_deployment_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_DeploymentSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `package_spec`.\n"]
    pub fn set_package_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_PackageSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.package_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.package_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_code_spec`.\n"]
    pub fn set_source_code_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiReasoningEngine_Spec_SourceCodeSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_code_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_code_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiReasoningEngine_Spec {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Spec {}

impl BuildVertexAiReasoningEngine_Spec {
    pub fn build(self) -> VertexAiReasoningEngine_Spec {
        VertexAiReasoningEngine_Spec {
            agent_framework: core::default::Default::default(),
            class_methods: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            service_account: core::default::Default::default(),
            build_spec: core::default::Default::default(),
            container_spec: core::default::Default::default(),
            deployment_spec: core::default::Default::default(),
            package_spec: core::default::Default::default(),
            source_code_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_SpecRef {
        VertexAiReasoningEngine_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_framework` after provisioning.\nOptional. The OSS agent framework used to develop the agent."]
    pub fn agent_framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_framework", self.base))
    }

    #[doc =
        "Get a reference to the value of field `class_methods` after provisioning.\nOptional. Declarations for object class methods in OpenAPI\nspecification format.\n\n**Note**: When deploying via Terraform, this field must be populated manually.\nOtherwise, client SDKs (like 'agent_engines.get()') will not be able to discover the methods, and calls to the engine (or A2A integrations) will fail.\n\nDepending on the template/framework used ('agent_framework'), the required class methods and their parameters differ:\n\n**Warning**: The configuration snippets below are illustrative, may not be exhaustive, and could stop working over time. For the most up-to-date method lists and schemas, please consult the respective SDK source code:\n* For Google ADK: See [ADK Python SDK cli_deploy.py](https://github.com/google/adk-python/blob/68a780306e3bdd648a882ef34c0abf8e5148353e/src/google/adk/cli/cli_deploy.py#L109).\n* For Langchain: See [Vertex AI Python SDK langchain.py](https://github.com/googleapis/python-aiplatform/blob/c8a38a085931b01f4d6071f0ab7a64cb42851829/agentplatform/agent_engines/templates/langchain.py#L642-L717).\n\n### 1. Langchain Template\n* 'query' (api_mode = \"sync\" or empty)\n* 'stream_query' (api_mode = \"stream\")\n\nExample for Langchain:\n'''hcl\nclass_methods = jsonencode([\n  {\n    name        = \"query\"\n    api_mode    = \"sync\"\n    description = \"Queries the reasoning engine\"\n    parameters  = {\n      type       = \"object\"\n      required   = [\"input\"]\n      properties = {\n        input = {\n          type        = \"string\"\n          description = \"The input prompt\"\n        }\n      }\n    }\n  },\n  {\n    name        = \"stream_query\"\n    api_mode    = \"stream\"\n    description = \"Streams queries from the reasoning engine\"\n    parameters  = {\n      type       = \"object\"\n      required   = [\"input\"]\n      properties = {\n        input = {\n          type        = \"string\"\n          description = \"The input prompt\"\n        }\n      }\n    }\n  }\n])\n'''\n\n### 2. Google ADK Template (Standard - No A2A)\nFor standard Google ADK (Agent Development Kit) deployments, you must define the following 11 methods:\n\nExample for Standard ADK:\n'''hcl\nclass_methods = jsonencode([\n  {\n    name        = \"get_session\"\n    api_mode    = \"\"\n    description = \"Retrieve session by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_get_session\"\n    api_mode    = \"async\"\n    description = \"Retrieve session asynchronously by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"list_sessions\"\n    api_mode    = \"\"\n    description = \"List all sessions for a user\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_list_sessions\"\n    api_mode    = \"async\"\n    description = \"List all sessions for a user asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"create_session\"\n    api_mode    = \"\"\n    description = \"Create a new session\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"async_create_session\"\n    api_mode    = \"async\"\n    description = \"Create a new session asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"delete_session\"\n    api_mode    = \"\"\n    description = \"Delete session by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"async_delete_session\"\n    api_mode    = \"async\"\n    description = \"Delete session asynchronously by ID\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\", \"session_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n      }\n    }\n  },\n  {\n    name        = \"stream_query\"\n    api_mode    = \"stream\"\n    description = \"Stream queries from the agent\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"message\", \"user_id\"]\n      properties = {\n        message    = { description = \"Message string or object\" }\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        run_config = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"async_stream_query\"\n    api_mode    = \"async_stream\"\n    description = \"Stream queries asynchronously from the agent\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"message\", \"user_id\"]\n      properties = {\n        message        = { description = \"Message string or object\" }\n        user_id        = { type = \"string\" }\n        session_id     = { type = \"string\" }\n        session_events = { type = \"array\", items = { type = \"object\" } }\n        run_config     = { type = \"object\" }\n      }\n    }\n  },\n  {\n    name        = \"streaming_agent_run_with_events\"\n    api_mode    = \"async_stream\"\n    description = \"Stream agent run with events asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"request_json\"]\n      properties = {\n        request_json = { type = \"string\" }\n      }\n    }\n  }\n])\n'''\n\n### 3. Google ADK Template (A2A-Enabled)\nIf the agent integrates with the Gemini Enterprise Agent Registry (A2A), you must inject the 'a2a_agent_card' JSON metadata as a string **specifically inside the 'async_create_session' method definition**:\n\nExample for A2A-Enabled ADK:\n'''hcl\nlocals {\n  # Construct the A2A endpoint URL\n  a2a_url = \"https://us-central1-aiplatform.googleapis.com/v1/projects/my-project/locations/us-central1/reasoningEngines/my-agent/a2a\"\n\n  agent_card = {\n    name                 = \"my-agent\"\n    description          = \"A2A Agent\"\n    version              = \"1.0.0\"\n    preferred_transport  = \"HTTP_JSON\"\n    supported_interfaces = [{ url = local.a2a_url, protocol_binding = \"HTTP_JSON\" }]\n    capabilities         = { streaming = true }\n  }\n}\n\n# In class_methods, append \"a2a_agent_card\" key ONLY to the \"async_create_session\" method:\nclass_methods = jsonencode([\n  # ... other 10 standard methods (same as Standard ADK) ...\n  {\n    name        = \"async_create_session\"\n    api_mode    = \"async\"\n    description = \"Create a new session asynchronously\"\n    parameters  = {\n      type     = \"object\"\n      required = [\"user_id\"]\n      properties = {\n        user_id    = { type = \"string\" }\n        session_id = { type = \"string\" }\n        state      = { type = \"object\" }\n      }\n    }\n    # Inject the serialized Agent Card here\n    a2a_agent_card = jsonencode(local.agent_card)\n  }\n])\n'''"]
    pub fn class_methods(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_identity` after provisioning.\nThe identity to use for the Reasoning Engine."]
    pub fn effective_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nOptional. The identity type to use for the Reasoning Engine.\nIf not specified, the 'service_account' field will be used if set,\notherwise the default Vertex AI Reasoning Engine Service Agent in the project will be used.\nPossible values:\n* 'SERVICE_ACCOUNT': Use a custom service account if the 'service_account' field is set, otherwise use the default Vertex AI Reasoning Engine Service Agent in the project.\n* 'AGENT_IDENTITY': Use Agent Identity. The 'service_account' field must not be set. Possible values: [\"SERVICE_ACCOUNT\", \"AGENT_IDENTITY\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. The service account that the Reasoning Engine artifact runs\nas. It should have \"roles/storage.objectViewer\" for reading the user\nproject's Cloud Storage and \"roles/aiplatform.user\" for using Vertex\nextensions. If not specified, the Vertex AI Reasoning Engine service\nAgent in the project will be used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_BuildSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_ContainerSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_spec` after provisioning.\n"]
    pub fn deployment_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_DeploymentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `package_spec` after provisioning.\n"]
    pub fn package_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_PackageSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `source_code_spec` after provisioning.\n"]
    pub fn source_code_spec(&self) -> terrars::ListRef<VertexAiReasoningEngine_Spec_SourceCodeSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_code_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiReasoningEngine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiReasoningEngine_Timeouts {
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

impl terrars::ToListMappable for VertexAiReasoningEngine_Timeouts {
    type O = terrars::BlockAssignable<VertexAiReasoningEngine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiReasoningEngine_Timeouts {}

impl BuildVertexAiReasoningEngine_Timeouts {
    pub fn build(self) -> VertexAiReasoningEngine_Timeouts {
        VertexAiReasoningEngine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiReasoningEngine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiReasoningEngine_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiReasoningEngine_TimeoutsRef {
        VertexAiReasoningEngine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiReasoningEngine_TimeoutsRef {
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
