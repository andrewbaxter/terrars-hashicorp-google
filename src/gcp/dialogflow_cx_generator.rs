#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxGeneratorData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_model_settings: std::option::Option<std::vec::Vec<DialogflowCxGenerator_LlmModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_parameter: std::option::Option<std::vec::Vec<DialogflowCxGenerator_ModelParameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placeholders: std::option::Option<std::vec::Vec<DialogflowCxGenerator_Placeholders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_text: std::option::Option<std::vec::Vec<DialogflowCxGenerator_PromptText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxGenerator_Timeouts>,
    dynamic: DialogflowCxGeneratorDynamic,
}

struct DialogflowCxGenerator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxGeneratorData_>,
}

#[derive(Clone)]
pub struct DialogflowCxGenerator(std::rc::Rc<DialogflowCxGenerator_>);

impl DialogflowCxGenerator {
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
        "Set the field `language_code`.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn set_language_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerator_LlmModelSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_model_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_parameter`.\n"]
    pub fn set_model_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerator_ModelParameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placeholders`.\n"]
    pub fn set_placeholders(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerator_Placeholders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placeholders = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placeholders = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prompt_text`.\n"]
    pub fn set_prompt_text(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerator_PromptText>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().prompt_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.prompt_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxGenerator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the generator, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Generator.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/generators/<Generator ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxGenerator_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_parameter` after provisioning.\n"]
    pub fn model_parameter(&self) -> terrars::ListRef<DialogflowCxGenerator_ModelParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_parameter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placeholders` after provisioning.\n"]
    pub fn placeholders(&self) -> terrars::ListRef<DialogflowCxGenerator_PlaceholdersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placeholders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\n"]
    pub fn prompt_text(&self) -> terrars::ListRef<DialogflowCxGenerator_PromptTextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerator_TimeoutsRef {
        DialogflowCxGenerator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxGenerator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxGenerator { }

impl terrars::ToListMappable for DialogflowCxGenerator {
    type O = terrars::ListRef<DialogflowCxGeneratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxGenerator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_generator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxGenerator {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the generator, unique within the agent."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxGenerator {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxGenerator {
        let out = DialogflowCxGenerator(std::rc::Rc::new(DialogflowCxGenerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxGeneratorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                model_parameter: core::default::Default::default(),
                placeholders: core::default::Default::default(),
                prompt_text: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxGeneratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGeneratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxGeneratorRef {
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
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the generator, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language to create generators for the following fields:\n* Generator.prompt_text.text\nIf not specified, the agent's default language is used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Generator.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/generators/<Generator ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Generator for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxGenerator_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_parameter` after provisioning.\n"]
    pub fn model_parameter(&self) -> terrars::ListRef<DialogflowCxGenerator_ModelParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_parameter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placeholders` after provisioning.\n"]
    pub fn placeholders(&self) -> terrars::ListRef<DialogflowCxGenerator_PlaceholdersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placeholders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\n"]
    pub fn prompt_text(&self) -> terrars::ListRef<DialogflowCxGenerator_PromptTextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerator_TimeoutsRef {
        DialogflowCxGenerator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxGeneratorDynamic {
    llm_model_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerator_LlmModelSettings>>,
    model_parameter: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerator_ModelParameter>>,
    placeholders: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerator_Placeholders>>,
    prompt_text: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerator_PromptText>>,
}

#[derive(Serialize)]
pub struct DialogflowCxGenerator_LlmModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerator_LlmModelSettings {
    #[doc = "Set the field `model`.\nThe selected LLM model."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_text`.\nThe custom prompt to use."]
    pub fn set_prompt_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerator_LlmModelSettings {
    type O = terrars::BlockAssignable<DialogflowCxGenerator_LlmModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerator_LlmModelSettings {}

impl BuildDialogflowCxGenerator_LlmModelSettings {
    pub fn build(self) -> DialogflowCxGenerator_LlmModelSettings {
        DialogflowCxGenerator_LlmModelSettings {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerator_LlmModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerator_LlmModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerator_LlmModelSettingsRef {
        DialogflowCxGenerator_LlmModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerator_LlmModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe selected LLM model."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_text` after provisioning.\nThe custom prompt to use."]
    pub fn prompt_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerator_ModelParameter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_decode_steps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxGenerator_ModelParameter {
    #[doc = "Set the field `max_decode_steps`.\nThe maximum number of tokens to generate."]
    pub fn set_max_decode_steps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_decode_steps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nThe temperature used for sampling. Temperature sampling occurs after both topP and topK have been applied.\nValid range: [0.0, 1.0] Low temperature = less random. High temperature = more random."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `top_k`.\nIf set, the sampling process in each step is limited to the topK tokens with highest probabilities.\nValid range: [1, 40] or 1000+. Small topK = less random. Large topK = more random."]
    pub fn set_top_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_k = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `top_p`.\nIf set, only the tokens comprising the top topP probability mass are considered.\nIf both topP and topK are set, topP will be used for further refining candidates selected with topK.\nValid range: (0.0, 1.0]. Small topP = less random. Large topP = more random."]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerator_ModelParameter {
    type O = terrars::BlockAssignable<DialogflowCxGenerator_ModelParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerator_ModelParameter {}

impl BuildDialogflowCxGenerator_ModelParameter {
    pub fn build(self) -> DialogflowCxGenerator_ModelParameter {
        DialogflowCxGenerator_ModelParameter {
            max_decode_steps: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerator_ModelParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerator_ModelParameterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerator_ModelParameterRef {
        DialogflowCxGenerator_ModelParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerator_ModelParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_decode_steps` after provisioning.\nThe maximum number of tokens to generate."]
    pub fn max_decode_steps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_decode_steps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nThe temperature used for sampling. Temperature sampling occurs after both topP and topK have been applied.\nValid range: [0.0, 1.0] Low temperature = less random. High temperature = more random."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top_k` after provisioning.\nIf set, the sampling process in each step is limited to the topK tokens with highest probabilities.\nValid range: [1, 40] or 1000+. Small topK = less random. Large topK = more random."]
    pub fn top_k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top_p` after provisioning.\nIf set, only the tokens comprising the top topP probability mass are considered.\nIf both topP and topK are set, topP will be used for further refining candidates selected with topK.\nValid range: (0.0, 1.0]. Small topP = less random. Large topP = more random."]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerator_Placeholders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerator_Placeholders {
    #[doc = "Set the field `id`.\nUnique ID used to map custom placeholder to parameters in fulfillment."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nCustom placeholder value in the prompt text."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerator_Placeholders {
    type O = terrars::BlockAssignable<DialogflowCxGenerator_Placeholders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerator_Placeholders {}

impl BuildDialogflowCxGenerator_Placeholders {
    pub fn build(self) -> DialogflowCxGenerator_Placeholders {
        DialogflowCxGenerator_Placeholders {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerator_PlaceholdersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerator_PlaceholdersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerator_PlaceholdersRef {
        DialogflowCxGenerator_PlaceholdersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerator_PlaceholdersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique ID used to map custom placeholder to parameters in fulfillment."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nCustom placeholder value in the prompt text."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerator_PromptText {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerator_PromptText {
    #[doc = "Set the field `text`.\nText input which can be used for prompt or banned phrases."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerator_PromptText {
    type O = terrars::BlockAssignable<DialogflowCxGenerator_PromptText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerator_PromptText {}

impl BuildDialogflowCxGenerator_PromptText {
    pub fn build(self) -> DialogflowCxGenerator_PromptText {
        DialogflowCxGenerator_PromptText { text: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerator_PromptTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerator_PromptTextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerator_PromptTextRef {
        DialogflowCxGenerator_PromptTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerator_PromptTextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nText input which can be used for prompt or banned phrases."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerator_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxGenerator_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxGenerator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerator_Timeouts {}

impl BuildDialogflowCxGenerator_Timeouts {
    pub fn build(self) -> DialogflowCxGenerator_Timeouts {
        DialogflowCxGenerator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerator_TimeoutsRef {
        DialogflowCxGenerator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerator_TimeoutsRef {
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
