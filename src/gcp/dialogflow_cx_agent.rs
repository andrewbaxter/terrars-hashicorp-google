use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxAgentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_uri: Option<PrimField<String>>,
    default_language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_spell_correction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_language_codes: Option<ListField<PrimField<String>>>,
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_integration_settings: Option<Vec<DialogflowCxAgentGitIntegrationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_to_text_settings: Option<Vec<DialogflowCxAgentSpeechToTextSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_to_speech_settings: Option<Vec<DialogflowCxAgentTextToSpeechSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxAgentTimeoutsEl>,
    dynamic: DialogflowCxAgentDynamic,
}

struct DialogflowCxAgent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxAgentData>,
}

#[derive(Clone)]
pub struct DialogflowCxAgent(Rc<DialogflowCxAgent_>);

impl DialogflowCxAgent {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `avatar_uri`.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn set_avatar_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_spell_correction`.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn set_enable_spell_correction(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_spell_correction = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_stackdriver_logging`.\nDetermines whether this agent should log conversation queries."]
    pub fn set_enable_stackdriver_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `security_settings`.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `supported_language_codes`.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn set_supported_language_codes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_language_codes = Some(v.into());
        self
    }

    #[doc= "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(self, v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git_integration_settings`.\n"]
    pub fn set_git_integration_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentGitIntegrationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_integration_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_integration_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `speech_to_text_settings`.\n"]
    pub fn set_speech_to_text_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentSpeechToTextSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().speech_to_text_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.speech_to_text_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `text_to_speech_settings`.\n"]
    pub fn set_text_to_speech_settings(
        self,
        v: impl Into<BlockAssignable<DialogflowCxAgentTextToSpeechSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().text_to_speech_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.text_to_speech_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxAgentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(&self) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git_integration_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> ListRef<DialogflowCxAgentSpeechToTextSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.speech_to_text_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> ListRef<DialogflowCxAgentTextToSpeechSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DialogflowCxAgent {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DialogflowCxAgent { }

impl ToListMappable for DialogflowCxAgent {
    type O = ListRef<DialogflowCxAgentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DialogflowCxAgent_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_agent".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxAgent {
    pub tf_id: String,
    #[doc= "The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub default_language_code: PrimField<String>,
    #[doc= "The human-readable name of the agent, unique within the location."]
    pub display_name: PrimField<String>,
    #[doc= "The name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub location: PrimField<String>,
    #[doc= "The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub time_zone: PrimField<String>,
}

impl BuildDialogflowCxAgent {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxAgent {
        let out = DialogflowCxAgent(Rc::new(DialogflowCxAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxAgentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                avatar_uri: core::default::Default::default(),
                default_language_code: self.default_language_code,
                description: core::default::Default::default(),
                display_name: self.display_name,
                enable_spell_correction: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                supported_language_codes: core::default::Default::default(),
                time_zone: self.time_zone,
                advanced_settings: core::default::Default::default(),
                git_integration_settings: core::default::Default::default(),
                speech_to_text_settings: core::default::Default::default(),
                text_to_speech_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxAgentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxAgentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(&self) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git_integration_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> ListRef<DialogflowCxAgentSpeechToTextSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.speech_to_text_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> ListRef<DialogflowCxAgentTextToSpeechSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}

impl DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    #[doc= "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {}

impl BuildDialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl {
        DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl { uri: core::default::Default::default() }
    }
}

pub struct DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
        DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_digit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_digits: Option<PrimField<f64>>,
}

impl DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    #[doc= "Set the field `enabled`.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `finish_digit`.\nThe digit that terminates a DTMF digit sequence."]
    pub fn set_finish_digit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.finish_digit = Some(v.into());
        self
    }

    #[doc= "Set the field `max_digits`.\nMax length of DTMF digits."]
    pub fn set_max_digits(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_digits = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {}

impl BuildDialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."]
    pub fn finish_digit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }

    #[doc= "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."]
    pub fn max_digits(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_digits", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgentAdvancedSettingsElDynamic {
    audio_export_gcs_destination: Option<
        DynamicBlock<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>,
    >,
    dtmf_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxAgentAdvancedSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_export_gcs_destination: Option<Vec<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_settings: Option<Vec<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxAgentAdvancedSettingsElDynamic,
}

impl DialogflowCxAgentAdvancedSettingsEl {
    #[doc= "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_export_gcs_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_export_gcs_destination = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentAdvancedSettingsElDtmfSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dtmf_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DialogflowCxAgentAdvancedSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentAdvancedSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentAdvancedSettingsEl {}

impl BuildDialogflowCxAgentAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxAgentAdvancedSettingsEl {
        DialogflowCxAgentAdvancedSettingsEl {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DialogflowCxAgentAdvancedSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentAdvancedSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentAdvancedSettingsElRef {
        DialogflowCxAgentAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentAdvancedSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> ListRef<DialogflowCxAgentAdvancedSettingsElAudioExportGcsDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audio_export_gcs_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> ListRef<DialogflowCxAgentAdvancedSettingsElDtmfSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_branch: Option<PrimField<String>>,
}

impl DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    #[doc= "Set the field `access_token`.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }

    #[doc= "Set the field `branches`.\nA list of branches configured to be used from Dialogflow."]
    pub fn set_branches(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.branches = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\nThe unique repository display name for the GitHub repository."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_uri`.\nThe GitHub repository URI related to the agent."]
    pub fn set_repository_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `tracking_branch`.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn set_tracking_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tracking_branch = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {}

impl BuildDialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
    pub fn build(self) -> DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
        DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl {
            access_token: core::default::Default::default(),
            branches: core::default::Default::default(),
            display_name: core::default::Default::default(),
            repository_uri: core::default::Default::default(),
            tracking_branch: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
        DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_token` after provisioning.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc= "Get a reference to the value of field `branches` after provisioning.\nA list of branches configured to be used from Dialogflow."]
    pub fn branches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe unique repository display name for the GitHub repository."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc= "Get a reference to the value of field `repository_uri` after provisioning.\nThe GitHub repository URI related to the agent."]
    pub fn repository_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `tracking_branch` after provisioning.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn tracking_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgentGitIntegrationSettingsElDynamic {
    github_settings: Option<DynamicBlock<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxAgentGitIntegrationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    github_settings: Option<Vec<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
    dynamic: DialogflowCxAgentGitIntegrationSettingsElDynamic,
}

impl DialogflowCxAgentGitIntegrationSettingsEl {
    #[doc= "Set the field `github_settings`.\n"]
    pub fn set_github_settings(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DialogflowCxAgentGitIntegrationSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentGitIntegrationSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentGitIntegrationSettingsEl {}

impl BuildDialogflowCxAgentGitIntegrationSettingsEl {
    pub fn build(self) -> DialogflowCxAgentGitIntegrationSettingsEl {
        DialogflowCxAgentGitIntegrationSettingsEl {
            github_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DialogflowCxAgentGitIntegrationSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentGitIntegrationSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentGitIntegrationSettingsElRef {
        DialogflowCxAgentGitIntegrationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentGitIntegrationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `github_settings` after provisioning.\n"]
    pub fn github_settings(&self) -> ListRef<DialogflowCxAgentGitIntegrationSettingsElGithubSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentSpeechToTextSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_speech_adaptation: Option<PrimField<bool>>,
}

impl DialogflowCxAgentSpeechToTextSettingsEl {
    #[doc= "Set the field `enable_speech_adaptation`.\nWhether to use speech adaptation for speech recognition."]
    pub fn set_enable_speech_adaptation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_speech_adaptation = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentSpeechToTextSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentSpeechToTextSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentSpeechToTextSettingsEl {}

impl BuildDialogflowCxAgentSpeechToTextSettingsEl {
    pub fn build(self) -> DialogflowCxAgentSpeechToTextSettingsEl {
        DialogflowCxAgentSpeechToTextSettingsEl { enable_speech_adaptation: core::default::Default::default() }
    }
}

pub struct DialogflowCxAgentSpeechToTextSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentSpeechToTextSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentSpeechToTextSettingsElRef {
        DialogflowCxAgentSpeechToTextSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentSpeechToTextSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_speech_adaptation` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn enable_speech_adaptation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_speech_adaptation", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentTextToSpeechSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesize_speech_configs: Option<PrimField<String>>,
}

impl DialogflowCxAgentTextToSpeechSettingsEl {
    #[doc= "Set the field `synthesize_speech_configs`.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn set_synthesize_speech_configs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.synthesize_speech_configs = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentTextToSpeechSettingsEl {
    type O = BlockAssignable<DialogflowCxAgentTextToSpeechSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentTextToSpeechSettingsEl {}

impl BuildDialogflowCxAgentTextToSpeechSettingsEl {
    pub fn build(self) -> DialogflowCxAgentTextToSpeechSettingsEl {
        DialogflowCxAgentTextToSpeechSettingsEl { synthesize_speech_configs: core::default::Default::default() }
    }
}

pub struct DialogflowCxAgentTextToSpeechSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentTextToSpeechSettingsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentTextToSpeechSettingsElRef {
        DialogflowCxAgentTextToSpeechSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentTextToSpeechSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `synthesize_speech_configs` after provisioning.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn synthesize_speech_configs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.synthesize_speech_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DialogflowCxAgentTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxAgentTimeoutsEl {
    type O = BlockAssignable<DialogflowCxAgentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgentTimeoutsEl {}

impl BuildDialogflowCxAgentTimeoutsEl {
    pub fn build(self) -> DialogflowCxAgentTimeoutsEl {
        DialogflowCxAgentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxAgentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxAgentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxAgentTimeoutsElRef {
        DialogflowCxAgentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgentTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgentDynamic {
    advanced_settings: Option<DynamicBlock<DialogflowCxAgentAdvancedSettingsEl>>,
    git_integration_settings: Option<DynamicBlock<DialogflowCxAgentGitIntegrationSettingsEl>>,
    speech_to_text_settings: Option<DynamicBlock<DialogflowCxAgentSpeechToTextSettingsEl>>,
    text_to_speech_settings: Option<DynamicBlock<DialogflowCxAgentTextToSpeechSettingsEl>>,
}
