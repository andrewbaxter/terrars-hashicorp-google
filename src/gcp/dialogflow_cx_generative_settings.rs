#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxGenerativeSettingsData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_settings: std::option::Option<std::vec::Vec<DialogflowCxGenerativeSettings_FallbackSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_safety_settings: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxGenerativeSettings_GenerativeSafetySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_connector_settings: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxGenerativeSettings_KnowledgeConnectorSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_model_settings: std::option::Option<std::vec::Vec<DialogflowCxGenerativeSettings_LlmModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxGenerativeSettings_Timeouts>,
    dynamic: DialogflowCxGenerativeSettingsDynamic,
}

struct DialogflowCxGenerativeSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxGenerativeSettingsData_>,
}

#[derive(Clone)]
pub struct DialogflowCxGenerativeSettings(std::rc::Rc<DialogflowCxGenerativeSettings_>);

impl DialogflowCxGenerativeSettings {
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

    #[doc =
        "Set the field `parent`.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fallback_settings`.\n"]
    pub fn set_fallback_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerativeSettings_FallbackSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fallback_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fallback_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generative_safety_settings`.\n"]
    pub fn set_generative_safety_settings(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxGenerativeSettings_GenerativeSafetySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generative_safety_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generative_safety_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `knowledge_connector_settings`.\n"]
    pub fn set_knowledge_connector_settings(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxGenerativeSettings_KnowledgeConnectorSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_connector_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_connector_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_model_settings`.\n"]
    pub fn set_llm_model_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxGenerativeSettings_LlmModelSettings>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxGenerativeSettings_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage for this settings."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the generativeSettings.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/generativeSettings."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_settings` after provisioning.\n"]
    pub fn fallback_settings(&self) -> terrars::ListRef<DialogflowCxGenerativeSettings_FallbackSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generative_safety_settings` after provisioning.\n"]
    pub fn generative_safety_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_safety_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_connector_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxGenerativeSettings_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerativeSettings_TimeoutsRef {
        DialogflowCxGenerativeSettings_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DialogflowCxGenerativeSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxGenerativeSettings { }

impl terrars::ToListMappable for DialogflowCxGenerativeSettings {
    type O = terrars::ListRef<DialogflowCxGenerativeSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxGenerativeSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_generative_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxGenerativeSettings {
    pub tf_id: std::string::String,
    #[doc = "Language for this settings."]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxGenerativeSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxGenerativeSettings {
        let out = DialogflowCxGenerativeSettings(std::rc::Rc::new(DialogflowCxGenerativeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxGenerativeSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                language_code: self.language_code,
                parent: core::default::Default::default(),
                fallback_settings: core::default::Default::default(),
                generative_safety_settings: core::default::Default::default(),
                knowledge_connector_settings: core::default::Default::default(),
                llm_model_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxGenerativeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxGenerativeSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage for this settings."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the generativeSettings.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/generativeSettings."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_settings` after provisioning.\n"]
    pub fn fallback_settings(&self) -> terrars::ListRef<DialogflowCxGenerativeSettings_FallbackSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generative_safety_settings` after provisioning.\n"]
    pub fn generative_safety_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_safety_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_connector_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_model_settings` after provisioning.\n"]
    pub fn llm_model_settings(&self) -> terrars::ListRef<DialogflowCxGenerativeSettings_LlmModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxGenerativeSettings_TimeoutsRef {
        DialogflowCxGenerativeSettings_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxGenerativeSettingsDynamic {
    fallback_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_FallbackSettings>>,
    generative_safety_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_GenerativeSafetySettings>>,
    knowledge_connector_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_KnowledgeConnectorSettings>>,
    llm_model_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_LlmModelSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frozen: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
    #[doc = "Set the field `display_name`.\nPrompt name."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `frozen`.\nIf the flag is true, the prompt is frozen and cannot be modified by users."]
    pub fn set_frozen(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.frozen = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prompt_text`.\nPrompt text that is sent to a LLM on no-match default, placeholders are filled downstream. For example: \"Here is a conversation $conversation, a response is: \""]
    pub fn set_prompt_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {}

impl BuildDialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
    pub fn build(self) -> DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
        DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates {
            display_name: core::default::Default::default(),
            frozen: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef {
        DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nPrompt name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `frozen` after provisioning.\nIf the flag is true, the prompt is frozen and cannot be modified by users."]
    pub fn frozen(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frozen", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prompt_text` after provisioning.\nPrompt text that is sent to a LLM on no-match default, placeholders are filled downstream. For example: \"Here is a conversation $conversation, a response is: \""]
    pub fn prompt_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxGenerativeSettings_FallbackSettingsDynamic {
    prompt_templates: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates>>,
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_FallbackSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_templates: std::option::Option<std::vec::Vec<DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates>>,
    dynamic: DialogflowCxGenerativeSettings_FallbackSettingsDynamic,
}

impl DialogflowCxGenerativeSettings_FallbackSettings {
    #[doc = "Set the field `selected_prompt`.\nDisplay name of the selected prompt."]
    pub fn set_selected_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.selected_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_templates`.\n"]
    pub fn set_prompt_templates(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplates>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt_templates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_templates = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_FallbackSettings {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_FallbackSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_FallbackSettings {}

impl BuildDialogflowCxGenerativeSettings_FallbackSettings {
    pub fn build(self) -> DialogflowCxGenerativeSettings_FallbackSettings {
        DialogflowCxGenerativeSettings_FallbackSettings {
            selected_prompt: core::default::Default::default(),
            prompt_templates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_FallbackSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_FallbackSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_FallbackSettingsRef {
        DialogflowCxGenerativeSettings_FallbackSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_FallbackSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `selected_prompt` after provisioning.\nDisplay name of the selected prompt."]
    pub fn selected_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selected_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_templates` after provisioning.\n"]
    pub fn prompt_templates(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_FallbackSettings_PromptTemplatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_templates", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
    language_code: terrars::PrimField<std::string::String>,
    text: terrars::PrimField<std::string::String>,
}

impl DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases { }

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
    #[doc = "Language code of the phrase."]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = "Text input which can be used for prompt or banned phrases."]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
    pub fn build(self) -> DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
        DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases {
            language_code: self.language_code,
            text: self.text,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef {
        DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nLanguage code of the phrase."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nText input which can be used for prompt or banned phrases."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxGenerativeSettings_GenerativeSafetySettingsDynamic {
    banned_phrases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases>>,
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_GenerativeSafetySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_banned_phrase_match_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_phrases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases>>,
    dynamic: DialogflowCxGenerativeSettings_GenerativeSafetySettingsDynamic,
}

impl DialogflowCxGenerativeSettings_GenerativeSafetySettings {
    #[doc =
        "Set the field `default_banned_phrase_match_strategy`.\nOptional. Default phrase match strategy for banned phrases.\nSee [PhraseMatchStrategy](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/GenerativeSettings#phrasematchstrategy) for valid values."]
    pub fn set_default_banned_phrase_match_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_banned_phrase_match_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `banned_phrases`.\n"]
    pub fn set_banned_phrases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrases>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.banned_phrases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.banned_phrases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_GenerativeSafetySettings {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_GenerativeSafetySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_GenerativeSafetySettings {}

impl BuildDialogflowCxGenerativeSettings_GenerativeSafetySettings {
    pub fn build(self) -> DialogflowCxGenerativeSettings_GenerativeSafetySettings {
        DialogflowCxGenerativeSettings_GenerativeSafetySettings {
            default_banned_phrase_match_strategy: core::default::Default::default(),
            banned_phrases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef {
        DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_GenerativeSafetySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_banned_phrase_match_strategy` after provisioning.\nOptional. Default phrase match strategy for banned phrases.\nSee [PhraseMatchStrategy](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/GenerativeSettings#phrasematchstrategy) for valid values."]
    pub fn default_banned_phrase_match_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_banned_phrase_match_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `banned_phrases` after provisioning.\n"]
    pub fn banned_phrases(
        &self,
    ) -> terrars::ListRef<DialogflowCxGenerativeSettings_GenerativeSafetySettings_BannedPhrasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_phrases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_data_store_fallback: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
    #[doc = "Set the field `agent`.\nName of the virtual agent. Used for LLM prompt. Can be left empty."]
    pub fn set_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_identity`.\nIdentity of the agent, e.g. \"virtual agent\", \"AI assistant\"."]
    pub fn set_agent_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `agent_scope`.\nAgent scope, e.g. \"Example company website\", \"internal Example company website for employees\", \"manual of car owner\"."]
    pub fn set_agent_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business`.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn set_business(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.business = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_description`.\nCompany description, used for LLM prompt, e.g. \"a family company selling freshly roasted coffee beans\".''"]
    pub fn set_business_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.business_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_data_store_fallback`.\nWhether to disable fallback to Data Store search results (in case the LLM couldn't pick a proper answer). Per default the feature is enabled."]
    pub fn set_disable_data_store_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_data_store_fallback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_KnowledgeConnectorSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_KnowledgeConnectorSettings {}

impl BuildDialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
    pub fn build(self) -> DialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
        DialogflowCxGenerativeSettings_KnowledgeConnectorSettings {
            agent: core::default::Default::default(),
            agent_identity: core::default::Default::default(),
            agent_scope: core::default::Default::default(),
            business: core::default::Default::default(),
            business_description: core::default::Default::default(),
            disable_data_store_fallback: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef {
        DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_KnowledgeConnectorSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nName of the virtual agent. Used for LLM prompt. Can be left empty."]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `agent_identity` after provisioning.\nIdentity of the agent, e.g. \"virtual agent\", \"AI assistant\"."]
    pub fn agent_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `agent_scope` after provisioning.\nAgent scope, e.g. \"Example company website\", \"internal Example company website for employees\", \"manual of car owner\"."]
    pub fn agent_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `business` after provisioning.\nName of the company, organization or other entity that the agent represents. Used for knowledge connector LLM prompt and for knowledge search."]
    pub fn business(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business", self.base))
    }

    #[doc =
        "Get a reference to the value of field `business_description` after provisioning.\nCompany description, used for LLM prompt, e.g. \"a family company selling freshly roasted coffee beans\".''"]
    pub fn business_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_data_store_fallback` after provisioning.\nWhether to disable fallback to Data Store search results (in case the LLM couldn't pick a proper answer). Per default the feature is enabled."]
    pub fn disable_data_store_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_data_store_fallback", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxGenerativeSettings_LlmModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerativeSettings_LlmModelSettings {
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

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_LlmModelSettings {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_LlmModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_LlmModelSettings {}

impl BuildDialogflowCxGenerativeSettings_LlmModelSettings {
    pub fn build(self) -> DialogflowCxGenerativeSettings_LlmModelSettings {
        DialogflowCxGenerativeSettings_LlmModelSettings {
            model: core::default::Default::default(),
            prompt_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_LlmModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_LlmModelSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxGenerativeSettings_LlmModelSettingsRef {
        DialogflowCxGenerativeSettings_LlmModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_LlmModelSettingsRef {
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
pub struct DialogflowCxGenerativeSettings_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxGenerativeSettings_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxGenerativeSettings_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxGenerativeSettings_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxGenerativeSettings_Timeouts {}

impl BuildDialogflowCxGenerativeSettings_Timeouts {
    pub fn build(self) -> DialogflowCxGenerativeSettings_Timeouts {
        DialogflowCxGenerativeSettings_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxGenerativeSettings_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxGenerativeSettings_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxGenerativeSettings_TimeoutsRef {
        DialogflowCxGenerativeSettings_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxGenerativeSettings_TimeoutsRef {
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
